#include <lauxlib.h>
#include <lua.h>
#include <lualib.h>

#include <stdio.h>
#include <stdlib.h>
#include <signal.h>

#include <emscripten.h>

int boot_lua(lua_State* L);
static lua_State *wasm_lua_state = NULL;

// Pre-compiled lua loader program
static const unsigned char program[] = {__LUA_BASE__};
// Pre-compiled entry script which user wrote
static const unsigned char lua_main_program[] = {__LUA_MAIN__};

/*  This line will be injected by emcc-lua as export functions to WASM declaration */
__LUA_FUNCTION_DECLARATIONS__

/* Copied from lua.c */

static lua_State *globalL = NULL;

static void lstop (lua_State *L, lua_Debug *ar) {
  (void)ar;  /* unused arg. */
  lua_sethook(L, NULL, 0, 0);  /* reset hook */
  luaL_error(L, "interrupted!");
}

static void laction (int i) {
  signal(i, SIG_DFL); /* if another SIGINT happens, terminate process */
  lua_sethook(globalL, lstop, LUA_MASKCALL | LUA_MASKRET | LUA_MASKCOUNT, 1);
}

static int msghandler (lua_State *L) {
  const char *msg = lua_tostring(L, 1);
  if (msg == NULL) {  /* is error object not a string? */
    if (luaL_callmeta(L, 1, "__tostring") &&  /* does it have a metamethod */
        lua_type(L, -1) == LUA_TSTRING)  /* that produces a string? */
      return 1;  /* that is the message */
    else
      msg = lua_pushfstring(L, "(error object is a %s value)",
                               luaL_typename(L, 1));
  }
  /* Call debug.traceback() instead of luaL_traceback() for Lua 5.1 compatibility. */
  lua_getglobal(L, "debug");
  lua_getfield(L, -1, "traceback");
  /* debug */
  lua_remove(L, -2);
  lua_pushstring(L, msg);
  /* original msg */
  lua_remove(L, -3);
  lua_pushinteger(L, 2);  /* skip this function and traceback */
  lua_call(L, 2, 1); /* call debug.traceback */
  return 1;  /* return the traceback */
}

static int docall (lua_State *L, int narg, int nres) {
  int status;
  int base = lua_gettop(L) - narg;  /* function index */
  lua_pushcfunction(L, msghandler);  /* push message handler */
  lua_insert(L, base);  /* put it under function and args */
  globalL = L;  /* to be available to 'laction' */
  signal(SIGINT, laction);  /* set C-signal handler */
  status = lua_pcall(L, narg, nres, base);
  signal(SIGINT, SIG_DFL); /* reset C-signal handler */
  lua_remove(L, base);  /* remove message handler from the stack */
  return status;
}

// Boot function
int main(void) {
  if (wasm_lua_state != NULL) {
    return 0;
  }
  wasm_lua_state = luaL_newstate();
  if (boot_lua(wasm_lua_state)) {
    printf("failed to boot lua runtime\\n");
    lua_close(wasm_lua_state);
    return 1;
  }
  //printf("Boot Lua Webassembly!\n");
  return 0;
}

// boot lua runtime from compiled lua source
int boot_lua(lua_State* L) {
  luaL_openlibs(L);

  // Preload Rust mlua module
  luaL_getsubtable(L, LUA_REGISTRYINDEX, LUA_PRELOAD_TABLE);
  lua_pushcfunction(L, luaopen_transformers_ao); // Change "luaopen_transformers_ao" to the appropriate function for opening your Rust mlua module
  lua_setfield(L, -2, "transformers_ao"); // Change "transformers_ao" to the appropriate name for your Rust mlua module
  lua_pop(L, 1);  // Remove the PRELOAD table from the stack

  if (luaL_loadbuffer(L, (const char*)program, sizeof(program), "main")) {
    fprintf(stderr, "error on luaL_loadbuffer()\n");
    return 1;
  }
  lua_newtable(L);
  lua_pushlstring(L, (const char*)lua_main_program, sizeof(lua_main_program));
  lua_setfield(L, -2, "__lua_webassembly__");

  // This place will be injected by emcc-lua
  __INJECT_LUA_FILES__

  if (docall(L, 1, LUA_MULTRET)) {
    const char *errmsg = lua_tostring(L, 1);
    if (errmsg) {
      fprintf(stderr, "%s\n", errmsg);
    }
    lua_close(L);
    return 1;
  }
  return 0;
}