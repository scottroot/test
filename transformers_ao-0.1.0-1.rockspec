package = "transformers_ao"
version = "0.1.0-1"

source = {
    url = "git+https://github.com/scottroot/test"
}

description = {
    summary = "A test library written in Rust",
    detailed = [[
        A Lua module written in Rust.
    ]],
    license = "MIT"
}

dependencies = {
    "luarocks-build-rust-mlua",
    "lua",
}

build = {
    type = "rust-mlua",
    -- install = {
    --     lua = {
    --         transformers_ao = "build/mymodule.lua"
    --     }
    -- },
    modules = {
        -- "transformers_ao"
        ["transformers_ao"] = "libtransformers_ao",
    },
    features = {"lua53"}
}