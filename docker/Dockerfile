FROM emscripten/emsdk:latest
LABEL description="emscripten builder for rust lua"

ENV LUA_VERSION 5.3.4
ENV LUAROCKS_VERSION 3.11.0
#2.4.4

WORKDIR /src

RUN apt-get update -qq -y
RUN apt-get install -y build-essential clang pkg-config libssl-dev libreadline-dev
# apt install lsb-release wget software-properties-common gnupg
RUN apt-get update -qq -y

# Install lua runtime
RUN cd / \
  && curl -L http://www.lua.org/ftp/lua-${LUA_VERSION}.tar.gz | tar xzf - \
  && cd /lua-${LUA_VERSION} \
  && make linux test \
  && make install

# Install luarocks
RUN cd / \
  && curl -L https://luarocks.org/releases/luarocks-${LUAROCKS_VERSION}.tar.gz | tar xzf - \
  && cd /luarocks-${LUAROCKS_VERSION} \
  && ./configure --with-lua-include=/lua-${LUA_VERSION}/src \
  && make build \
  && make install

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain add stable
RUN rustup target add wasm32-unknown-emscripten --toolchain stable
RUN rustup target add wasm32-unknown-unknown --toolchain stable
RUN cargo install wasm-bindgen-cli


ENTRYPOINT ["sleep", "infinity"]
