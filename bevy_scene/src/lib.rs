//! Bevy WASM scene for the portfolio background canvas.
//!
//! Compiles to a wasm32-unknown-unknown cdylib. JS in the Dioxus app loads
//! and starts it on `DOMContentLoaded`, mounting Bevy to the canvas element
//! at `#bevy-canvas` (sit at `z-index: -1` per PORT-BEVY-1).
//!
//! Bot fills in App::new() + the faithful three.js scene port per the
//! design memo at `docs/bevy-scene-spec-v0.1.md`, citing PORT-BEVY-1
//! and PORT-SCENE-1.

use wasm_bindgen::prelude::*;

/// JS-callable entry point. Replaced by the bot with the full Bevy `App`
/// construction in v0.1-bevy item 2.
#[wasm_bindgen(start)]
pub fn start() {
    // placeholder — bot replaces with App::new().add_plugins(...).run() etc.
}
