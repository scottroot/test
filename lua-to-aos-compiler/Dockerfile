FROM emscripten/emsdk:3.1.55

# Default working directory of emscripten/emsdk is /src
# WORKDIR /src


ENV LUA_VERSION 5.3.4
ENV LUAROCKS_VERSION 3.11.0


RUN apt-get update -qq -y
RUN apt-get install -y build-essential nano clang pkg-config libssl-dev libreadline-dev zlib1g-dev llvm-dev
RUN apt-get update -qq -y

# Intall yaml
RUN pip3 install pyyaml

# Install lua runtime
RUN cd / \
  && curl -L http://www.lua.org/ftp/lua-${LUA_VERSION}.tar.gz | tar xzf - \
  && cd /lua-${LUA_VERSION} \
  && make linux \
  && make install

# Install Luarocks
RUN cd / \
  && curl -L https://luarocks.org/releases/luarocks-${LUAROCKS_VERSION}.tar.gz | tar xzf - \
  && cd /luarocks-${LUAROCKS_VERSION} \
  && ./configure --with-lua-include=/lua-${LUA_VERSION}/src \
  && make build \
  && make install

# Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain add stable
RUN rustup target add wasm32-unknown-emscripten --toolchain stable

# Install Node
ENV NODE_VERSION=18.17.0
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
ENV NVM_DIR=/root/.nvm
RUN . "$NVM_DIR/nvm.sh" && nvm install ${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm use v${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm alias default v${NODE_VERSION}
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"
RUN node --version
RUN npm --version

RUN npm i -g https://get_ao.g8way.io

# And, re-compile lua with "generic WASM"
RUN cd /lua-${LUA_VERSION} && \
  make clean && \
  make generic CC='emcc -s WASM=1 -U LUA_32BITS'

#############################
##### Install Commands ######
#############################

# NOTE: Some commands do not require additional work
# when building the image, but are still commented
# here for posterity

###############
#### Init #####
###############
# The init command currently does not require running
# within the ao container, and so requires no additional setup

###############
#### Repl #####
###############
# The repl command simply fires up lua repl, which is installed
# as part of lua runtime above, and so requires no additional setup

###############
#### Build ####
###############

COPY ./src/emcc-lua /usr/local/bin/emcc-lua
COPY ./src/emcc_lua_lib /usr/local/emcc-lua/emcc_lua_lib

COPY ./src/pre.js /opt/pre.js
COPY ./src/definition.yml /opt/definition.yml
COPY ./src/loader.lua /opt/loader.lua

RUN mkdir -p /opt/src
COPY ./src/json.lua /opt/src/json.lua
COPY ./src/ao.lua /opt/src/ao.lua
# COPY ./src/pack.lua /opt/pack.lua
COPY ./src/main.c /opt/main.c
COPY ./src/main.lua /opt/main.lua
RUN chmod +x /usr/local/bin/emcc-lua

ENV CC 'emcc -s WASM=1'
ENV NM 'llvm-nm'

###########################################
### Publish, Bundler, Process, Contract ###
###########################################
# We first create a directory for the node impls to be placed
# and dependencies installed
# 
# By running npm link, we allow any commands exposed by
# the node module to be ran globally within the container
RUN mkdir -p /opt/node
COPY ./src/node /opt/node
RUN cd /opt/node && \
  npm install --omit="dev" && \
  npm link

