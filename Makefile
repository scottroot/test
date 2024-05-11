build-container:
	docker build -t ao_compiler:latest ./docker/
	docker run -d -v ./:/src --name ao_compiler ao_compiler:latest
exec:
	docker exec -it ao_compiler /bin/bash

run: build-container exec

stop:
	docker stop ao_compiler && docker remove ao_compiler;

.PHONY: clean
clean:
	-rm ./target/release/liblua_transformers_ao.so;
	-rm ./output/liblua_transformers_ao.so;

cargo-build: clean
	cargo build --release --features lua53;
	cp ./target/release/libtransformers_ao.so ./output/libtransformers_ao.so;
	cd output && lua mymodule.lua && cd ../;

.PHONY: build
build: clean
	cargo build --release --features lua53;
	cp ./target/release/libtransformers_ao.so ./output/libtransformers_ao.so;
	cd output && lua mymodule.lua && cd ../;

wasm-emscripten:
	LUA_LIB=/src/lua-5.3.4/src LUA_LIB_NAME=lua LUA_LINK=static cargo build --release --target wasm32-unknown-emscripten -vv;

wasm:
	cargo build --release --target wasm32-unknown-unknown;

