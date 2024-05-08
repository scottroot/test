package = "lua-aobert"
version = "0.1.0-1"

source = {
    url = "https://github.com/scottroot/test.git"
}

description = {
    summary = "A fast YAML library written in Rust using serde",
    detailed = [[
        The Lua module written in Rust that provides YAML support for Lua.
        Fast, pure Rust YAML 1.2 implementation using serde framework and yaml-rust parser.
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
        "aobert"
    },
}