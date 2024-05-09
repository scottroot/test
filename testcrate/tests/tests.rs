use std::env;
use std::path::PathBuf;

use mlua::{Lua, Result};

#[test]
fn test_encode_text() -> Result<()> {
    let lua = make_lua()?;
    lua.load(
        r#"
        package.cpath = package.cpath .. ";/src/target/release/lib?.so"
        local transformers = require("transformers_ao")

        local function read_string(path)
            local file = io.open(path, "r")
            if file == nil then
                print(path .. " is nil")
                return nil
            end
            local content = file:read("*a")
            file:close()
            return content
        end

        local config = read_string("../src/models/bert/config.json")
        if config == nil then
            error("config is nil")
        end
        local tokenizer = read_string("../src/models/bert/tokenizer.json")
        if tokenizer == nil then
            error("tokenizer is nil")
        end
        local model = read_string("../src/models/bert/model.safetensors")
        if model == nil then
            error("model is nil")
        end
        local emb = transformers.encode_text(config, tokenizer, model)
        print(emb)

        assert(emb ~= nil)
    "#,
    )
    .exec()
}


fn make_lua() -> Result<Lua> {
    let (dylib_path, dylib_ext, separator);
    if cfg!(target_os = "macos") {
        dylib_path = env::var("DYLD_FALLBACK_LIBRARY_PATH").unwrap();
        dylib_ext = "dylib";
        separator = ":";
    } else if cfg!(target_os = "linux") {
        dylib_path = env::var("LD_LIBRARY_PATH").unwrap();
        dylib_ext = "so";
        separator = ":";
    } else if cfg!(target_os = "windows") {
        dylib_path = env::var("PATH").unwrap();
        dylib_ext = "dll";
        separator = ";";
    } else {
        panic!("unknown target os");
    };

    let mut cpath = dylib_path
        .split(separator)
        .take(3)
        .map(|p| {
            let mut path = PathBuf::from(p);
            path.push(format!("lib?.{}", dylib_ext));
            path.to_str().unwrap().to_owned()
        })
        .collect::<Vec<_>>()
        .join(";");

    if cfg!(target_os = "windows") {
        cpath = cpath.replace("\\", "\\\\");
        cpath = cpath.replace("lib?.", "?.");
    }

    let lua = unsafe { Lua::unsafe_new() }; // To be able to load C modules
    lua.load(&format!("package.cpath = \"{}\"", cpath)).exec()?;
    Ok(lua)
}