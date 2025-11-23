WWW_TARGET_DIR := target/wasm
BINDGEN := monkey ca game_of_life fertilization maze
BINDGEN_TARGETS := $(foreach svc,$(BINDGEN),bindgen-$(svc))
CLEAN_BINDGEN_TARGETS := $(foreach svc,$(BINDGEN),clean_bindgen-$(svc))

WASM_PACK := $(HOME)/.cargo/bin/wasm-pack

.PHONY: all web serve bindgen clean
all: web

$(WASM_PACK):
	@echo "--- Installing wasm-pack ---"
	cargo install wasm-pack

.PHONY: bindgen
bindgen: $(BINDGEN_TARGETS)

bindgen-%: $(WASM_PACK)
	@echo "--- Building service: $* ---"
	mkdir -p $(WWW_TARGET_DIR)/js/$* 
	$(WASM_PACK) build src/$* --target web --release -d js/$*

clean_bindgen-%:
	@echo "--- Removing generated files for: $* ---"
	rm -rf src/$*/js/$*

node_modules: package.json
	npm install

web: bindgen node_modules
	@echo "--- Building Hugo site ---"
	hugo build --gc --minify

serve: bindgen node_modules
	@echo "--- Run Hugo server ---"
	hugo serve --buildDrafts --buildFuture

clean: $(CLEAN_BINDGEN_TARGETS)
	@echo "--- cleanup files ---"
	rm -rf target
	rm -rf public
	rm -rf .hugo_build.lock
	rm -rf resources
	rm -rf node_modules
