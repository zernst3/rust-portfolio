use dioxus::prelude::*;

use crate::audio::play_sound;
use crate::contexts::audio::use_audio_state;
use crate::contexts::page_transition::use_nav_with_transition;

/// Landing / starting screen shown at the root route.
///
/// Ported from `StartingScreen.tsx`. The framer-motion fade-in on `h1` and
/// `h3` is replaced by the `page-enter` CSS animation class (defined in
/// `assets/styles/transitions.css`) per decision #4 (vanilla CSS
/// transitions). Navigation to `/` goes through `use_nav_with_transition`
/// so the welcome screen does its fade+slide-up exit animation before the
/// home page mounts.
///
/// Per rust-dioxus-2 (functional component), PORT-CSS-1 (class names match
/// React originals), decision #4 (no dioxus-motion).
#[component]
pub fn StartingScreen() -> Element {
    let audio = use_audio_state();
    let transition = use_nav_with_transition();

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
                            play_sound("/static/sounds/select.mp3", 0.45);
                        }
                        transition.call("/");
                    },
                    "Click Here to Enter"
                }
            }
        }
    }
}
