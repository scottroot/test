build-container:
	docker build -t transformers-ao-compiler:latest ./docker/
	docker run -d -v ./:/src --name transformers-ao-compiler transformers-ao-compiler:latest
exec:
	docker exec -it transformers-ao-compiler /bin/bash

run: build-container exec

stop:
	docker stop transformers-ao-compiler && docker remove transformers-ao-compiler;

.PHONY: clean
clean:
	-rm ./target/release/liblua_transformers_ao.so;
	-rm ./build/liblua_transformers_ao.so;

.PHONY: build
build: clean
	cargo build --release;
	cp ./target/release/libtransformers_ao.so ./build/libtransformers_ao.so;
	cd build && lua mymodule.lua;
