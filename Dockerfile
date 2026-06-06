# syntax=docker/dockerfile:1

###############################################################################
# Builder — compile the Dioxus fullstack bundle (server binary + public/).
###############################################################################
# Latest stable Rust on Debian bookworm; glibc here matches the bookworm-slim
# runtime so the dynamically-linked server binary loads without surprises.
FROM rust:bookworm AS builder

# wasm target for the hydration bundle (rust-toolchain.toml also requests it).
RUN rustup target add wasm32-unknown-unknown

# Install the Dioxus CLI (dx) pinned to 0.7.9 — MUST match the workspace's
# dioxus 0.7.9 (CLI/library version skew breaks the hydration handshake).
# Installed via cargo-binstall (prebuilt binary): a source `cargo install` of
# dioxus-cli fails to compile (auth-git2/git2 build error).
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
      https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall -y dioxus-cli@0.7.9

WORKDIR /build
COPY . .

# Emits target/dx/portfolio/release/web/{server,public}.
# (wasm-opt may log a non-fatal SIGABRT on some hosts; dx continues and the
#  bundle is valid — do not treat that line as a build failure.)
RUN dx bundle --platform web --release --package ui

###############################################################################
# Runtime — slim image with just the binary, public/, and repo assets/.
###############################################################################
FROM debian:bookworm-slim AS runtime

# ca-certificates: rustls needs the system trust store to verify Mailgun's TLS.
# tini: proper PID-1 signal handling for clean container shutdown.
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates tini \
 && rm -rf /var/lib/apt/lists/* \
 && useradd --create-home --uid 10001 app

WORKDIR /app

# The binary and public/ MUST be siblings: dioxus-server's serve_static_assets()
# reads `exe/../public/` at startup (it panics without it). With the binary at
# /app/server, that resolves to /app/public.
COPY --from=builder /build/target/dx/portfolio/release/web/server /app/server
COPY --from=builder /build/target/dx/portfolio/release/web/public /app/public
# Repo static files (CSS, images incl. nyc.jpg, sounds) are served at /static via
# ServeDir::new("assets"), resolved relative to the working directory (/app).
COPY --from=builder /build/assets /app/assets

# Azure Container Apps routes ingress to this port; main.rs binds 0.0.0.0:$PORT.
ENV PORT=8080
EXPOSE 8080
USER app

ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["/app/server"]
