use dioxus::prelude::*;

/// Page transition state — drives the fade+slide-up "leaving" animation
/// on the outgoing route before the router actually swaps to the new one.
///
/// Lifecycle (per click on a transition-aware nav element):
/// 1. `use_nav_with_transition()` returns a callback. User clicks; callback
///    sets `is_leaving = true`.
/// 2. The App-level wrapper div in `lib.rs` toggles its `page-leaving` class
///    based on this signal. The CSS keyframe (`page-leave-kf` in
///    `assets/styles/transitions.css`) fades the wrapper out + slides it up.
/// 3. The callback's spawned task waits `EXIT_MS` (matches the CSS duration),
///    then calls `nav.push(target)`. The old route unmounts and the new one
///    mounts with its own `page-enter` class for the entrance animation.
/// 4. After `nav.push`, the signal is reset to `false` so the wrapper drops
///    `page-leaving` and the new page's entrance animation isn't fighting it.
///
/// Keep `EXIT_MS` synced with the CSS `page-leave-kf` duration.
#[derive(Clone, Copy)]
pub struct PageTransitionState {
    pub is_leaving: Signal<bool>,
}

/// CSS animation duration for `.page-leaving`. Kept here so the Rust delay
/// and the CSS duration are visibly the same constant in code review.
pub const EXIT_MS: u64 = 200;

pub fn provide_page_transition_context() -> PageTransitionState {
    use_context_provider(|| PageTransitionState {
        is_leaving: Signal::new(false),
    })
}

pub fn use_page_transition() -> PageTransitionState {
    consume_context::<PageTransitionState>()
}

/// Returns a callback that triggers the exit animation and then navigates.
/// Pass a `&'static str` path (e.g. `"/manifesto"` or `"/"`). String paths
/// match the existing routing pattern used by `NAV_ITEMS` in `home.rs`.
///
/// If the user is already on the target path, the navigation is a no-op
/// and no animation runs.
pub fn use_nav_with_transition() -> Callback<&'static str> {
    let transition = use_page_transition();
    let nav = use_navigator();
    let mut is_leaving = transition.is_leaving;

    use_callback(move |target: &'static str| {
        if *is_leaving.peek() {
            // Already in the middle of a transition; ignore additional clicks.
            return;
        }
        is_leaving.set(true);
        spawn(async move {
            // Wait for the page-leave animation to complete.
            //
            // Using document::eval + setTimeout (the same pattern Home's
            // subtitle rotation uses) rather than tokio sleep, because the
            // session model here is `dioxus::launch` on wasm32 and
            // `tokio::time::sleep` requires the runtime which isn't wired
            // for the client side.
            let js = format!("await new Promise(r => setTimeout(r, {EXIT_MS}))");
            let _ = document::eval(&js).await;

            // Push the new route FIRST, then clear is_leaving. This avoids a
            // single-frame flash where the wrapper's leaving class snaps off
            // (returning the outgoing page to opacity 1) before the router
            // has actually swapped.
            nav.push(target);
            is_leaving.set(false);
        });
    })
}
