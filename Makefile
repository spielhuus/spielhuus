SRC_DIR := src/bin
TARGET_DIR := target/release
WASM_TARGET_DIR := target/wasm32-unknown-emscripten/release
WWW_TARGET_DIR := www/static

BIN_SRC_FILES := $(shell find workspaces -type f -name main.rs)
BIN_NAMES := $(patsubst workspaces/%/src/main.rs,%,$(BIN_SRC_FILES))

BIN_TARGETS := $(addprefix $(TARGET_DIR)/, $(BIN_NAMES))
WASM_FILES := $(addsuffix .wasm, $(addprefix $(WASM_TARGET_DIR)/, $(BIN_NAMES)))
JS_FILES := $(addsuffix .js, $(addprefix $(WASM_TARGET_DIR)/, $(BIN_NAMES)))

TARGET_WASM_FILES := $(addprefix $(WWW_TARGET_DIR)/, $(notdir $(WASM_FILES)))
TARGET_JS_FILES := $(addprefix $(WWW_TARGET_DIR)/, $(notdir $(JS_FILES)))

EMCC_CFLAGS = "-O3 --bind --js-library src/library.js -sUSE_GLFW=3 -sASSERTIONS=1 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1 -sEXPORTED_RUNTIME_METHODS=ccall,cwrap,UTF8ToString"
EMSCRIPTEN_PATH = /usr/lib/emscripten

.PHONY: all build-native build-wasm copy-www clean

all: build-native build-wasm copy-www

target/emsdk: 
	git clone --depth 1 https://github.com/emscripten-core/emsdk.git target/emsdk
	cd target/emsdk && ./emsdk install latest
	cd target/emsdk && ./emsdk activate latest

# Native build
build-native: $(BIN_SRC_FILES)
	cargo build --release

# WASM build
build-wasm: $(BIN_SRC_FILES) target/emsdk
	# EMCC_CFLAGS=$(EMCC_CFLAGS) PATH=$(PATH):$(EMSCRIPTEN_PATH) cargo build --target wasm32-unknown-emscripten --release
	# EMCC_CFLAGS=$(EMCC_CFLAGS) source target/emsdk/emsdk_env.sh && cargo build --target wasm32-unknown-emscripten --release -vv
	. target/emsdk/emsdk_env.sh; cargo build --target wasm32-unknown-emscripten --release

# Copy to target/www
copy-www: build-wasm
	mkdir -p $(WWW_TARGET_DIR)
	cp $(WASM_FILES) $(JS_FILES) $(WWW_TARGET_DIR)

web: copy-www
	cd www && hugo build

serve: copy-www
	cd www && hugo serve

clean:
	rm -rf target
	rm -f $(TARGET_WASM_FILES) $(TARGET_JS_FILES)
	rm -rf www/public
