use dioxus::prelude::*;

use crate::audio::play_sound;
use crate::contexts::audio::use_audio_state;

/// Close / "Back" button used by detail pages (e.g. SingleExperience).
///
/// Ported from `Close.tsx`. Navigates to `link` (default `/`) via Dioxus
/// `Link`. Plays `woosh2.mp3` at volume 0.1 on click when not muted.
///
/// Per rust-dioxus-2 (functional component), PORT-CSS-1 (class names match
/// React originals for CSS rule continuity).
#[derive(Props, Clone, PartialEq)]
pub struct CloseProps {
    /// Navigation target. Defaults to `"/"`.
    #[props(default = "/".to_string())]
    pub link: String,
}

#[component]
pub fn Close(props: CloseProps) -> Element {
    let audio = use_audio_state();

    rsx! {
        div { id: "Close",
            Link {
                to: props.link,
                class: "close",
                onclick: move |_| {
                    if !*audio.is_muted.read() {
                        play_sound("/assets/sounds/woosh2.mp3", 0.1);
                    }
                },
                p { "Back" }
            }
        }
    }
}
