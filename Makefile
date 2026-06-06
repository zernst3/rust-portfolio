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
# Served at: /static/portfolio_scene.js (repo static files are at /static/, NOT /assets/).
# The on-disk dir is still named `assets/` so --out-dir $(ASSETS_DIR) still works.
# When re-enabled, ui/src/lib.rs imports via: import('/static/portfolio_scene.js').then(m => m.default())
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
#
# RELEASE by default: the debug WASM bundle is ~50MB and JIT-compiling it on the
# browser main thread freezes the tab for 30-90s (looks like a hang). The release
# bundle is ~2MB and loads instantly. Release rebuilds are slower (~100s) — that
# is the deliberate tradeoff until the bundle-size story improves.
#
# SELF-CLEANING: kills any stale `dx serve` / spawned server first. A leftover
# dx serve holds port 8080; a new invocation then errors in ~1s and the browser
# keeps showing the OLD build (the "white screen is back" trap). Always kill first.
.PHONY: serve
serve:
	-pkill -f "dx serve" 2>/dev/null; pkill -f "target/dx" 2>/dev/null; sleep 1
	dx serve --platform web --release --package ui
