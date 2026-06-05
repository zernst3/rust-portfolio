use dioxus::prelude::*;

/// Global audio state — mirrors the React Zustand `useAudioStore`.
///
/// `is_muted` defaults to `true` (site loads silent, matching the React default).
/// `is_hovering` tracks whether the user is hovering a nav item (fed to the Bevy
/// background once the Bevy phase lands; no-op until then).
///
/// Provided at the App root via [`provide_audio_context`]; consumed anywhere in
/// the component tree via [`use_audio_state`]. Per rust-dioxus-3 (Signals) and
/// rust-dioxus-4 (context providers).
#[derive(Clone, Copy)]
pub struct AudioState {
    pub is_muted: Signal<bool>,
    pub is_hovering: Signal<bool>,
}

pub fn provide_audio_context() -> AudioState {
    use_context_provider(|| AudioState {
        is_muted: Signal::new(true),
        is_hovering: Signal::new(false),
    })
}

pub fn use_audio_state() -> AudioState {
    consume_context::<AudioState>()
}
