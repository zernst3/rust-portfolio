# rust-portfolio build helpers
#
# Requirements: wasm-bindgen-cli matching the workspace wasm-bindgen version.
#   cargo install wasm-bindgen-cli --version 0.2.122
#
# Typical dev workflow:
#   make build-bevy    # rebuild the Bevy background canvas WASM
#   dx serve --platform fullstack   # run fullstack dev server

BEVY_WASM_RELEASE := target/wasm32-unknown-unknown/release/portfolio_scene.wasm
ASSETS_DIR := assets

# Build the Bevy WASM scene and run wasm-bindgen to produce the JS loader
# and _bg.wasm file that the browser can import.
#
# Output: assets/portfolio_scene.js + assets/portfolio_scene_bg.wasm
# Referenced by ui/src/lib.rs via: import('/assets/portfolio_scene.js').then(m => m.default())
.PHONY: build-bevy
build-bevy:
	cargo build --target wasm32-unknown-unknown -p portfolio_scene --release
	wasm-bindgen --target web --out-dir $(ASSETS_DIR) $(BEVY_WASM_RELEASE)
	@echo "Bevy WASM ready: $(ASSETS_DIR)/portfolio_scene.js + $(ASSETS_DIR)/portfolio_scene_bg.wasm"

# Run all Rust quality gates (same checks required before every commit).
.PHONY: check
check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --workspace -- -D warnings
	cargo check --workspace
	cargo test --workspace

# Serve fullstack dev server via dx CLI.
# Dioxus 0.7: "fullstack" is a feature flag on the dioxus workspace dep (already
# enabled). The platform for a fullstack web app is `web` — dx builds the SSR
# server + the WASM hydration bundle + serves both with hot-reload.
.PHONY: serve
serve:
	dx serve --platform web --package server
