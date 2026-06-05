use dioxus::prelude::*;

use crate::audio::play_sound;
use crate::contexts::audio::use_audio_state;
use crate::routes::Route;

/// Close / "Back" button used by detail pages (e.g. SingleExperience).
///
/// Ported from `Close.tsx`. Navigates to `/` via Dioxus `use_navigator`.
/// Plays `woosh2.mp3` at volume 0.1 on click when not muted.
///
/// Bug 4 fix: React renders `<button className="close">` (matched by
/// `#Close button { ... }` in Close.css). The prior Dioxus port used
/// `Link { class: "close" }` which renders as `<a>`, missing the CSS rule.
/// Native `<button>` restores the correct selector match (PORT-CSS-1).
///
/// Per rust-dioxus-2 (functional component), PORT-CSS-1 (class names match
/// React originals for CSS rule continuity).
#[component]
pub fn Close() -> Element {
    let audio = use_audio_state();
    let nav = use_navigator();

    rsx! {
        div { id: "Close",
            button {
                class: "close",
                r#type: "button",
                onclick: move |_| {
                    if !*audio.is_muted.read() {
                        play_sound("/assets/sounds/woosh2.mp3", 0.1);
                    }
                    nav.push(Route::Home {});
                },
                p { "Back" }
            }
        }
    }
}
