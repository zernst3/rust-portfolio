# rust-portfolio build helpers
#
# Requirements: wasm-bindgen-cli matching the workspace wasm-bindgen version.
#   cargo install wasm-bindgen-cli --version 0.2.122
#
# Typical dev workflow:
#   make build-bevy    # rebuild the Bevy background canvas WASM
#   dx serve --platform fullstack   # run fullstack dev server

BEVY_WASM_RELEASE := target/wasm32-unknown-unknown/release/bevy_scene.wasm
ASSETS_DIR := assets

# Build the Bevy WASM scene and run wasm-bindgen to produce the JS loader
# and _bg.wasm file that the browser can import.
#
# Output: assets/bevy_scene.js + assets/bevy_scene_bg.wasm
# Referenced by ui/src/lib.rs via: import('/assets/bevy_scene.js').then(m => m.default())
.PHONY: build-bevy
build-bevy:
	cargo build --target wasm32-unknown-unknown -p bevy_scene --release
	wasm-bindgen --target web --out-dir $(ASSETS_DIR) $(BEVY_WASM_RELEASE)
	@echo "Bevy WASM ready: $(ASSETS_DIR)/bevy_scene.js + $(ASSETS_DIR)/bevy_scene_bg.wasm"

# Run all Rust quality gates (same checks required before every commit).
.PHONY: check
check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --workspace -- -D warnings
	cargo check --workspace
	cargo test --workspace

# Serve fullstack dev server via dx CLI.
.PHONY: serve
serve:
	dx serve --platform fullstack
