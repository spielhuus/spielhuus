SRC_DIR := src/bin
TARGET_DIR := target/release
WASM_TARGET_DIR := target/wasm32-unknown-emscripten/release
WWW_TARGET_DIR := www/static

BIN_SRC_FILES := $(shell find workspaces -type f -name main.rs)
BIN_NAMES := $(patsubst workspaces/%/src/main.rs,%,$(BIN_SRC_FILES))

EMSCRIPTEN := wasm wasm_callback wasm_raylib nikolaus
EMSCRIPTEN_TARGETS := $(foreach svc,$(EMSCRIPTEN),emscripten-$(svc))

BINDGEN := monkey ahoi_wgpu ca game_of_life #voronoi 
BINDGEN_TARGETS := $(foreach svc,$(BINDGEN),bindgen-$(svc))

WASM_PACK := $(HOME)/.cargo/bin/wasm-pack

.PHONY: all emscripten bindgen build-native

all: emscripten bindgen build-native

$(WASM_PACK): 
	cargo install wasm-pack

target/emsdk: 
	git clone --depth 1 https://github.com/emscripten-core/emsdk.git target/emsdk
	cd target/emsdk && ./emsdk install latest
	cd target/emsdk && ./emsdk activate latest

emscripten: $(EMSCRIPTEN_TARGETS)

emscripten-%: target/emsdk
	@echo "--- Building service: $* ---"
	(cd target/emsdk && . ./emsdk_env.sh && cd ../.. && cargo build -p $* --target wasm32-unknown-emscripten --release)
	mkdir -p $(WWW_TARGET_DIR)/js/$*
	cp $(WASM_TARGET_DIR)/$*.wasm $(WASM_TARGET_DIR)/$*.js $(WWW_TARGET_DIR)/js/$*

.PHONY: bindgen
bindgen: $(BINDGEN_TARGETS)

bindgen-%: $(WASM_PACK)
	@echo "--- Building service: $* ---"
	mkdir -p $(WWW_TARGET_DIR)/js/$* 
	$(WASM_PACK) build workspaces/$* --target web --release -d ../../$(WWW_TARGET_DIR)/js/$*

build-native: $(BIN_SRC_FILES)
	cargo build --release

web: emscripten bindgen
	cd www && hugo build

serve: emscripten bindgen
	hugo --source www serve

clean:
	rm -rf target
	rm -f www/static/*.js www/static/*.wasm
	rm -rf www/public
	rm -rf www/resources/
