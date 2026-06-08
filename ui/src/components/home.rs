use dioxus::prelude::*;
use serde::Deserialize;

use crate::audio::play_sound;
use crate::contexts::audio::use_audio_state;
use crate::contexts::mobile_menu::use_mobile_menu;

/// Measured position of a nav item — used by the sliding selection bar.
#[derive(Clone, Deserialize)]
struct ItemPos {
    top: f64,
    height: f64,
}

struct NavItem {
    text: &'static str,
    link: &'static str,
    icon: &'static str,
}

struct LinkItem {
    label: &'static str,
    href: &'static str,
}

static NAV_ITEMS: &[NavItem] = &[
    NavItem {
        text: "Manifesto",
        link: "/manifesto",
        icon: ICON_ARCHITECTURE,
    },
    NavItem {
        text: "AI Architecture",
        link: "/ai-architecture",
        icon: ICON_HUB,
    },
    NavItem {
        text: "Work",
        link: "/work",
        icon: ICON_WORK_OUTLINE,
    },
    NavItem {
        text: "Writing",
        link: "/writing",
        icon: ICON_EDIT_NOTE,
    },
    NavItem {
        text: "Contact Me",
        link: "/contactme",
        icon: ICON_CONTACT_MAIL,
    },
];

static LINK_ITEMS: &[LinkItem] = &[
    LinkItem { label: "My Resume",        href: "https://1drv.ms/b/c/2284c063c81ee480/Ee6UQk1nDqxBrHc0NCMPHZoBLolQDGf2z0kSWkY_sE10qw?e=NRvCaC" },
    LinkItem { label: "My Github",        href: "https://github.com/zernst3" },
    LinkItem { label: "My LinkedIn",      href: "https://www.linkedin.com/in/zernst3/" },
    LinkItem { label: "My Medium",        href: "https://zacharyernst.medium.com/" },
    LinkItem { label: "My Stack Overflow", href: "https://stackoverflow.com/users/3048047/zernst" },
];

static HEADER_STRINGS: &[&str] = &[
    "Technical Lead @ S&P Global Market Intelligence | Enterprise Architecture",
    "Creator & Lead Engineer | Building The New Agora (Independent Project)",
    "AI-native development for clean, consistent and scalable code",
    "Aligning distributed contributors to one architecture across time zones",
    "Building robust digital infrastructure for the systems that shape our environment",
    "Ensuring resilient, secure and scalable ground-up systems",
    "Building in Rust through AI orchestration, this site included",
];

// ── Inline SVG icons (Material Design, 24×24 viewBox) ─────────────────────

const ICON_ARCHITECTURE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" style="width:24px;height:24px"><path d="M2.5 19h19v2h-19v-2zm7.18-1.73 4.35 1.16 6.04-6.04-1.78-1.78-4.35 1.16-2.08-2.09 3.03-5.25-1.77-1.77-5.25 3.02-2.09-2.08 1.16-4.35-1.78-1.78-6.04 6.04 1.16 4.35 2.08 2.09-3.03 5.25 1.77 1.77 5.25-3.02 2.09 2.09z"/></svg>"#;

const ICON_HUB: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" style="width:24px;height:24px"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 15v-4H7l5-8v4h4l-5 8z"/></svg>"#;

const ICON_WORK_OUTLINE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" style="width:24px;height:24px"><path d="M20 6h-2.18c.07-.44.18-.88.18-1.35C18 2.99 16.64 1.5 14.93 1.5c-1.26 0-2.18.72-2.93 1.5L12 3l-.01-.01C11.18 2.22 10.26 1.5 9.07 1.5 7.36 1.5 6 2.99 6 4.65c0 .47.11.91.18 1.35H4c-1.1 0-2 .9-2 2v11c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zM14.93 3.5c.74 0 1.07.63 1.07 1.15 0 .8-.73 1.69-2 2.35-1.27-.66-2-1.55-2-2.35 0-.52.33-1.15 1.07-1.15h1.86zM8 4.65c0-.52.33-1.15 1.07-1.15h1.86c.74 0 1.07.63 1.07 1.15 0 .8-.73 1.69-2 2.35-1.27-.66-2-1.55-2-2.35zM20 19H4V8h16v11z"/></svg>"#;

const ICON_EDIT_NOTE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" style="width:24px;height:24px"><path d="M3 10h11v2H3zm0-2h11V6H3zm0 8h7v-2H3zm11.41-2.83 1.17-1.17 1.42 1.42-1.17 1.17zm.71-3.54 2.12 2.12-5.3 5.3H10v-2.12z"/></svg>"#;

const ICON_CONTACT_MAIL: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" style="width:24px;height:24px"><path d="M21 8V7l-3 2-3-2v1l3 2zm1-5H2C.9 3 0 3.9 0 5v14c0 1.1.9 2 2 2h20c1.1 0 1.99-.9 1.99-2L24 5c0-1.1-.9-2-2-2zM8 6c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm6 12H2v-1c0-2 4-3.1 6-3.1s6 1.1 6 3.1v1zm8-6h-8V6h8v6z"/></svg>"#;

const ICON_MENU: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" style="width:24px;height:24px"><path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/></svg>"#;

const ICON_CLOSE_X: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" style="width:24px;height:24px"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>"#;

const ICON_VOLUME_UP: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" style="width:24px;height:24px"><path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02zM14 3.23v2.06c2.89.86 5 3.54 5 6.71s-2.11 5.85-5 6.71v2.06c4.01-.91 7-4.49 7-8.77s-2.99-7.86-7-8.77z"/></svg>"#;

const ICON_VOLUME_OFF: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" style="width:24px;height:24px"><path d="M16.5 12c0-1.77-1.02-3.29-2.5-4.03v2.21l2.45 2.45c.03-.2.05-.41.05-.63zm2.5 0c0 .94-.2 1.82-.54 2.64l1.51 1.51C20.63 14.91 21 13.5 21 12c0-4.28-2.99-7.86-7-8.77v2.06c2.89.86 5 3.54 5 6.71zM4.27 3L3 4.27 7.73 9H3v6h4l5 5v-6.73l4.25 4.25c-.67.52-1.42.93-2.25 1.18v2.06c1.38-.31 2.63-.95 3.69-1.81L19.73 21 21 19.73l-9-9L4.27 3zM12 4L9.91 6.09 12 8.18V4z"/></svg>"#;

const ICON_INFO: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" style="width:24px;height:24px"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"/></svg>"#;

/// Home page / navigation sidebar.
///
/// Ported from `Home.tsx`. Key differences:
/// - MUI components replaced with native HTML + ported CSS (PORT-CSS-1).
/// - framer-motion replaced with `.page-enter` CSS keyframes (decision #4).
/// - Sliding nav highlight measured via `document::eval` on `li.onmouseenter`
///   (Link doesn't expose onmouseenter; hover sits on the li instead).
/// - Zustand `useAudioStore` → `AudioState` context (rust-dioxus-3/4).
/// - `MobileMenuContext` → `MobileMenuState` context (rust-dioxus-3/4).
/// - Header rotation: `use_future` timer (client-only; SSR renders header[0]).
/// - `useAudioAnalyzer` omitted — the animated background is the WebGL water
///   cinemagraph (background.rs), which doesn't consume the audio analyser.
///
/// Per rust-dioxus-2, rust-dioxus-7, rust-dioxus-8, PORT-CSS-1, decision #4.
#[component]
pub fn Home() -> Element {
    let audio = use_audio_state();
    let mobile_menu = use_mobile_menu();

    // Extract writable signal handles. Signal is Copy so these are independent
    // handles to the same underlying storage — mutations go through them.
    let mut is_muted = audio.is_muted;
    let mut is_hovering = audio.is_hovering;
    let mut menu_open = mobile_menu.is_open;

    let mut header_idx: Signal<usize> = use_signal(|| 0);
    let highlight: Signal<ItemPos> = use_signal(|| ItemPos {
        top: 0.0,
        height: 0.0,
    });
    let mut links_open: Signal<bool> = use_signal(|| false);
    let mut subtitle_opacity: Signal<f64> = use_signal(|| 1.0);
    let mut highlight_on: Signal<bool> = use_signal(|| false);

    // Rotate subtitle every 7 s.
    //
    // WHY use_effect + spawn instead of use_future:
    // use_future re-spawns its future on every re-render when its closure
    // captures a reactive signal. header_idx.set() inside the loop triggered
    // a re-render → re-spawn → another timer fires → another set() → infinite
    // spawn cascade. On WASM's single-threaded executor that saturated the JS
    // event loop (dead refresh, blank page, slow tab close).
    //
    // use_effect with no reactive reads in its body runs ONCE on mount and
    // never re-runs. The spawn() inside it creates exactly one long-lived async
    // task for the entire component lifetime, stopping the cascade entirely.
    //
    // ROOT-CAUSE FIX 2026-06-06: the original code used:
    //   document::eval("setTimeout(function(){dioxus.send(null)},7000)").await
    //
    // The eval PROMISE_WRAPPER wraps JS in an async IIFE that calls dioxus.close()
    // IMMEDIATELY AFTER the JS code runs — not after the timeout fires. So
    // dioxus.close() fires synchronously right after setTimeout() is registered,
    // the outer promise resolves on the next microtask tick, and the Rust .await
    // completes immediately. The loop then runs at microtask speed (~60fps),
    // saturating the JS event loop and permanently blocking the main thread.
    //
    // FIX: Use `await` inside the eval JS so the IIFE itself awaits the timer
    // before dioxus.close() is called. The outer promise resolves only after
    // the 7-second timer fires, giving the correct delay.
    use_effect(move || {
        spawn(async move {
            loop {
                document::eval("await new Promise(function(r){setTimeout(r,3000)});")
                    .await
                    .ok();
                subtitle_opacity.set(0.0);
                document::eval("await new Promise(function(r){setTimeout(r,400)});")
                    .await
                    .ok();
                let next = (*header_idx.peek() + 1) % HEADER_STRINGS.len();
                header_idx.set(next);
                subtitle_opacity.set(1.0);
            }
        });
    });

    // Shared sliding-highlight mover. Both the page links AND the Links button
    // call this so they all drive the SAME `.nav-highlight` bar (identical fade
    // + travel). `idx` is the item's position among `#mobile-menu-content
    // .navbarItem` (page links first, then the Links trigger). use_callback is
    // Copy, so every hover handler can call it without moving a fresh closure.
    let measure_highlight = use_callback(move |idx: usize| {
        let mut hl = highlight;
        let mut hl_on = highlight_on;
        spawn(async move {
            let js = format!(
                "(function(){{\
                  var items=document.querySelectorAll('#mobile-menu-content .navbarItem');\
                  var el=items[{idx}];\
                  if(!el){{dioxus.send(null);return;}}\
                  var parent=document.getElementById('mobile-menu-content');\
                  if(!parent){{dioxus.send(null);return;}}\
                  var pr=parent.getBoundingClientRect();\
                  var er=el.getBoundingClientRect();\
                  dioxus.send({{top:er.top-pr.top,height:er.height}});\
                }})()"
            );
            let mut ev = document::eval(&js);
            if let Ok(Some(pos)) = ev.recv::<Option<ItemPos>>().await {
                hl.set(pos);
                hl_on.set(true);
            }
        });
    });

    let is_muted_val = *is_muted.read();
    let is_open_val = *menu_open.read();
    let current_idx = *header_idx.read();
    let current_highlight = highlight.read().clone();
    let highlight_on_val = *highlight_on.read();
    let highlight_bar_opacity: f64 = if highlight_on_val { 1.0 } else { 0.0 };
    let subtitle_opacity_val = *subtitle_opacity.read();
    let links_open_val = *links_open.read();

    rsx! {
        div {
            class: "page-enter",
            onclick: move |_| {
                let _ = document::eval(
                    "(function(){var c=window.sharedAudioContext;\
                     if(c&&c.state==='suspended')c.resume();})()"
                );
            },

            div { id: "Home",
                div { class: "content",

                    // ── Navigation sidebar ─────────────────────────────────────
                    div {
                        class: if is_open_val { "menuContainer mobile-open" } else { "menuContainer" },

                        // Mobile-only toggle (hidden at desktop breakpoint via Home.css).
                        button {
                            r#type: "button",
                            class: "mobile-menu-toggle",
                            aria_expanded: "{is_open_val}",
                            aria_controls: "mobile-menu-content",
                            aria_label: if is_open_val { "Collapse menu" } else { "Expand menu" },
                            onclick: move |_| menu_open.toggle(),
                            span { dangerous_inner_html: if is_open_val { ICON_CLOSE_X } else { ICON_MENU } }
                            span { class: "mobile-menu-toggle-label",
                                if is_open_val { "Close" } else { "Menu" }
                            }
                        }

                        div {
                            id: "mobile-menu-content",
                            class: "mobile-menu-content",
                            // Leaving the whole menu hides the shared bar; moving
                            // BETWEEN sections (page links <-> Links) keeps it alive.
                            onmouseleave: move |_| {
                                highlight_on.set(false);
                                is_hovering.set(false);
                            },

                            // Single shared sliding highlight bar for the entire
                            // menu (page links AND Links). Absolutely positioned
                            // within #mobile-menu-content; CSS handles color/border.
                            div {
                                class: "nav-highlight",
                                style: "top: {current_highlight.top}px; \
                                        height: {current_highlight.height}px; \
                                        opacity: {highlight_bar_opacity}; \
                                        transition: top 0.28s cubic-bezier(0.22,1,0.36,1), \
                                                    height 0.28s cubic-bezier(0.22,1,0.36,1), \
                                                    opacity 0.2s ease;",
                            }

                            // ── Page links ─────────────────────────────────────
                            div {
                                class: "pageLinks",

                                for (idx, item) in NAV_ITEMS.iter().enumerate() {
                                    Link {
                                        to: item.link,
                                        class: "navbarItem",
                                        active_class: "active",
                                        onclick: move |_| {
                                            if !*is_muted.peek() {
                                                play_sound("/static/sounds/select.mp3", 0.45);
                                            }
                                        },
                                        li {
                                            // Hover events on li: Link doesn't expose onmouseenter.
                                            onmouseenter: move |_| {
                                                is_hovering.set(true);
                                                measure_highlight.call(idx);
                                                if !*is_muted.peek() {
                                                    spawn(async move {
                                                        // Same fix: dioxus.send() after 5 ms.
                                                        document::eval(
                                                            "setTimeout(function(){dioxus.send(null)},5)"
                                                        )
                                                        .await
                                                        .ok();
                                                        play_sound("/static/sounds/woosh3.mp3", 0.25);
                                                    });
                                                }
                                            },
                                            span { class: "listItem",
                                                dangerous_inner_html: item.icon,
                                            }
                                            span { class: "listItem", "{item.text}" }
                                        }
                                    }
                                }
                            }

                            hr {}

                            // ── Links external-URL dropdown ────────────────────
                            div { class: "links",
                                div {
                                    class: "navbarItem",
                                    style: "position: relative;",
                                    li {
                                        button {
                                            r#type: "button",
                                            style: "display: flex; align-items: center; gap: 15px; \
                                                    width: 100%; background: none; border: none; \
                                                    cursor: pointer; padding: 0; color: inherit;",
                                            onmouseenter: move |_| {
                                                is_hovering.set(true);
                                                // idx = NAV_ITEMS.len() → the Links
                                                // trigger is the last `.navbarItem`,
                                                // so it reuses the SAME shared bar.
                                                measure_highlight.call(NAV_ITEMS.len());
                                                if !*is_muted.peek() {
                                                    spawn(async move {
                                                        document::eval(
                                                            "setTimeout(function(){dioxus.send(null)},5)"
                                                        )
                                                        .await
                                                        .ok();
                                                        play_sound("/static/sounds/woosh3.mp3", 0.25);
                                                    });
                                                }
                                            },
                                            onmouseleave: move |_| is_hovering.set(false),
                                            onclick: move |_| {
                                                if !*is_muted.peek() {
                                                    play_sound("/static/sounds/select.mp3", 0.45);
                                                }
                                                links_open.toggle();
                                            },
                                            span { class: "listItem", dangerous_inner_html: ICON_INFO }
                                            span { class: "listItem", "Links" }
                                        }
                                    }

                                    if links_open_val {
                                        div {
                                            style: "position: absolute; bottom: 100%; left: 0; \
                                                    background-color: rgba(8,26,20,0.96); \
                                                    border-radius: 10px; min-width: 200px; \
                                                    z-index: 100; padding: 8px 0;",
                                            onclick: move |_| links_open.set(false),
                                            for link_item in LINK_ITEMS.iter() {
                                                a {
                                                    class: "menuLink",
                                                    href: link_item.href,
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    onmouseenter: move |_| {
                                                        if !*is_muted.peek() {
                                                            play_sound("/static/sounds/woosh3.mp3", 0.25);
                                                        }
                                                    },
                                                    onclick: move |_| {
                                                        if !*is_muted.peek() {
                                                            play_sound("/static/sounds/select.mp3", 0.45);
                                                        }
                                                    },
                                                    div {
                                                        style: "display:flex;align-items:center;\
                                                                gap:14px;padding:8px 16px;",
                                                        span { class: "listItem", "{link_item.label}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // ── Name + rotating subtitle ───────────────────────────────
                    div {
                        class: "textContainer",
                        style: "text-align: center;",
                        div {
                            class: "text",
                            style: "min-height: 200px; display: flex; flex-direction: column; \
                                    justify-content: center; align-items: center;",
                            h1 { class: "name-fade-in", style: "text-align: center;",
                                "Zachary Ernst"
                            }
                            div {
                                style: "position: relative; height: 40px; width: 100%; \
                                        display: flex; justify-content: center;",
                                h3 {
                                    style: "position: absolute; text-align: center; \
                                            margin: 0; width: 100%; \
                                            opacity: {subtitle_opacity_val}; \
                                            transition: opacity 0.4s ease-in-out;",
                                    "{HEADER_STRINGS[current_idx]}"
                                }
                            }
                        }
                    }
                }

                // ── Mute toggle (fixed, bottom-right) ──────────────────────────
                div {
                    style: "position: fixed; bottom: 30px; right: 30px; \
                            z-index: 1000; pointer-events: auto;",
                    button {
                        r#type: "button",
                        aria_label: "Toggle Mute",
                        style: "background-color: rgba(15,23,42,0.4); border: none; \
                                border-radius: 50%; padding: 8px; cursor: pointer; \
                                color: rgb(240,240,240); display: flex; align-items: center; \
                                justify-content: center;",
                        onclick: move |_| {
                            let _ = document::eval(
                                "(function(){var c=window.sharedAudioContext;\
                                 if(c&&c.state==='suspended')c.resume();})()"
                            );
                            is_muted.toggle();
                        },
                        span {
                            dangerous_inner_html: if is_muted_val { ICON_VOLUME_OFF } else { ICON_VOLUME_UP },
                        }
                    }
                }
            }
        }
    }
}
