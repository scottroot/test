#cd /lua-${LUA_VERSION} && make clean && make generic CC='emcc -s WASM=1 -U LUA_32BITS';
mkdir -p /opt/node;
cp -rf /src/lua-to-aos-compiler/src/node /opt/;
cd /opt/node && npm install --omit="dev" && npm link;
cd /src
cp -rf /src/lua-to-aos-compiler/src/emcc-lua /usr/local/bin/emcc-lua;
echo "Updated emcc-lua"
chmod +x /usr/local/bin/emcc-lua;
mkdir -p /usr/local/emcc-lua/
cp -f /src/lua-to-aos-compiler/src/emcc_lua_lib/definition.py /usr/local/emcc-lua/emcc_lua_lib/definition.py;
cp -f /src/lua-to-aos-compiler/src/emcc_lua_lib/file.py /usr/local/emcc-lua/emcc_lua_lib/file.py;
cp -f /src/lua-to-aos-compiler/src/emcc_lua_lib/helper.py /usr/local/emcc-lua/emcc_lua_lib/helper.py;

cp -rf /src/lua-to-aos-compiler/src/pre.js /opt/pre.js;
cp -rf /src/lua-to-aos-compiler/src/definition.yml /opt/definition.yml;
cp -rf /src/lua-to-aos-compiler/src/loader.lua /opt/loader.lua;

cp -rf /src/lua-to-aos-compiler/src/json.lua /opt/src/json.lua;
cp -rf /src/lua-to-aos-compiler/src/ao.lua /opt/src/ao.lua;
cp -rf /src/lua-to-aos-compiler/src/main.c /opt/main.c;
cp -rf /src/lua-to-aos-compiler/src/main.lua /opt/main.lua;
emcc-lua