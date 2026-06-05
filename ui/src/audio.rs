use dioxus::prelude::document;

/// Plays a sound asset via a JavaScript `Audio` element.
///
/// Uses `document::eval` (already in the `dioxus` dep) instead of
/// `web_sys::HtmlAudioElement` to avoid adding a new dependency per PORT-ROUTE-1.
/// On the server (SSR) the `NoOpEvaluator` discards the call silently.
/// Per PORT-AUDIO-1 (see CONVENTIONS.md).
pub fn play_sound(path: &str, volume: f64) {
    let js = format!(
        "(()=>{{var a=new Audio('{path}');a.volume={volume};a.play().catch(()=>{{}});}})()"
    );
    let _ = document::eval(&js);
}
