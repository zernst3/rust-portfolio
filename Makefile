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
# After v0.1-restructure-fullstack: server host code merged into `ui` crate.
# Target-based gating: [target.'cfg(not(target_arch="wasm32"))'.dependencies]
# ensures server deps (axum, tokio, reqwest) are excluded from the wasm32 bundle
# automatically — no @server/--features flags needed.
# Per PORT-FULLSTACK-1.
.PHONY: serve
serve:
	dx serve --platform web --package ui
