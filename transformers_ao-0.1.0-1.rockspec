package = "transformers_ao"
version = "0.1.0-1"

source = {
    url = "https://github.com/scottroot/test.git"
}

description = {
    summary = "A test library written in Rust",
    detailed = [[
        A Lua module written in Rust.
    ]],
    license = "MIT"
}

dependencies = {
    "lua >= 5.3",
    "luarocks-build-rust-mlua",
}

build = {
    type = "rust-mlua",
    modules = {
        "transformers_ao"
    },
}