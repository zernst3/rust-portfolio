use dioxus::prelude::*;

/// Mobile menu open/closed state — mirrors the React `MobileMenuContext`.
///
/// Provided at the App root (above the Router) via [`provide_mobile_menu_context`]
/// so the state survives route navigation, matching the React behavior where the
/// `MobileMenuProvider` wraps the router and keeps menu state across nav clicks.
/// Resets on full-page refresh because the provider remounts on each SSR/hydration.
///
/// Per rust-dioxus-3 (Signals) and rust-dioxus-4 (context providers).
#[derive(Clone, Copy)]
pub struct MobileMenuState {
    pub is_open: Signal<bool>,
}

pub fn provide_mobile_menu_context() -> MobileMenuState {
    use_context_provider(|| MobileMenuState {
        is_open: Signal::new(false),
    })
}

pub fn use_mobile_menu() -> MobileMenuState {
    consume_context::<MobileMenuState>()
}
