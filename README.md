## Evolutionary computing

### Infinite-Monkey-Theorem

https://de.wikipedia.org/wiki/Infinite-Monkey-Theorem

### Smart-Rockets

### Maze Solving



# Development

## install emscripten sdk:

git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest
source emsdk_env.sh
cd ..

# Building

EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sASSERTIONS=1 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1" PATH=$PATH:/usr/lib/emscripten  cargo build --target wasm32-unknown-emscripten --release
