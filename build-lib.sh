rm ./target/release/liblua_transformers_ao.so
rm ./build/liblua_transformers_ao.so
cargo build --release
cp ./target/release/liblua_transformers_ao.so ./build/liblua_transformers_ao.so
cd build
lua mymodule.lua
cd ../