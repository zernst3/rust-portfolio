# rust-portfolio build helpers

# Run all Rust quality gates (same checks CI runs before every merge).
.PHONY: check
check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --workspace -- -D warnings
	cargo check --workspace
	cargo test --workspace

# Serve the fullstack dev server via dx CLI.
# Single `ui` crate compiles twice (server + wasm); target-cfg gating keeps
# server deps out of the wasm bundle (PORT-FULLSTACK-1).
#
# RELEASE by default: the debug WASM bundle is ~50MB and JIT-compiling it on the
# browser main thread freezes the tab for 30-90s. The release bundle is small
# and loads instantly. Release rebuilds are slower — the deliberate tradeoff.
#
# SELF-CLEANING: kills any stale `dx serve` first (a leftover holds port 8080,
# so a new invocation errors and the browser keeps showing the old build).
.PHONY: serve
serve:
	-pkill -9 -f "dx serve" 2>/dev/null; pkill -9 -f "target/dx" 2>/dev/null; lsof -ti:8080 | xargs -r kill -9 2>/dev/null; sleep 1
	dx serve --platform web --release --package ui

# Production bundle (what the Docker builder runs). Emits the server binary and
# the public/ dir (hydration bootstrap + wasm + dx assets) under
# target/dx/portfolio/release/web/{server,public}.
.PHONY: bundle
bundle:
	dx bundle --platform web --release --package ui
