use dioxus::prelude::*;

use crate::audio::play_sound;
use crate::contexts::audio::use_audio_state;

/// Landing / starting screen shown at the root route.
///
/// Ported from `StartingScreen.tsx`. The framer-motion fade-in on `h1` and
/// `h3` is replaced by the `page-enter` CSS animation class (defined in
/// `assets/styles/App.css`) per decision #4 (vanilla CSS transitions).
///
/// Per rust-dioxus-2 (functional component), PORT-CSS-1 (class names match
/// React originals), decision #4 (no dioxus-motion).
#[component]
pub fn StartingScreen() -> Element {
    let audio = use_audio_state();
    let navigator = use_navigator();

    rsx! {
        div { class: "page-enter",
            div { id: "StartingScreenContainer",
                h1 { class: "fade-in-delayed",
                    "Welcome"
                }
                h3 {
                    class: "fade-in-delayed-long",
                    style: "cursor: pointer",
                    onclick: move |_| {
                        if !*audio.is_muted.read() {
                            play_sound("/assets/sounds/select.mp3", 0.45);
                        }
                        navigator.push("/");
                    },
                    "Click Here to Enter"
                }
            }
        }
    }
}
