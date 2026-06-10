pub mod audio;
pub mod components;
pub mod contexts;
pub mod routes;

// WebGL water-cinemagraph background — wasm32-only (uses web-sys). Native/server
// builds don't compile it; the use_effect call site is cfg-gated to match.
#[cfg(target_arch = "wasm32")]
mod background;

// Server-only modules — excluded for wasm32 via target cfg.
// Per PORT-FULLSTACK-1: same crate compiles twice; target-based gating ensures
// axum / tokio / reqwest never enter the wasm32 dependency graph.
#[cfg(not(target_arch = "wasm32"))]
pub mod dto;
#[cfg(not(target_arch = "wasm32"))]
pub mod handlers;
#[cfg(not(target_arch = "wasm32"))]
pub mod mailgun;
#[cfg(not(target_arch = "wasm32"))]
mod server;

/// Re-export so `main.rs` and integration tests can call `ui::build_router()`.
#[cfg(not(target_arch = "wasm32"))]
pub use server::{api_router, build_router};

use dioxus::prelude::*;

use contexts::audio::provide_audio_context;
use contexts::mobile_menu::provide_mobile_menu_context;
use contexts::page_transition::provide_page_transition_context;
use routes::Route;

/// Root Dioxus component.
///
/// Provides AudioState and MobileMenuState contexts at the App root (above the
/// Router) so both contexts survive route navigation, matching the React
/// MobileMenuProvider + useAudioStore behavior.
///
/// Stylesheets are loaded in the document head per PORT-CSS-1 (verbatim port
/// of the React index.html <link> tags) and decision #4 (transitions.css
/// replaces framer-motion).
#[component]
pub fn App() -> Element {
    let audio = provide_audio_context();
    provide_mobile_menu_context();
    let transition = provide_page_transition_context();
    let is_leaving = transition.is_leaving;

    // Animated water cinemagraph background, authored in Rust (web-sys/WebGL).
    // Client-only: use_effect runs after hydration (never on SSR / the server
    // build). Renders the NYC photo into #background-canvas with only the river
    // animated; fails silently to a static image if WebGL is unavailable.
    // See background.rs. wasm32-only; the call is a no-op on native builds.
    use_effect(|| {
        #[cfg(target_arch = "wasm32")]
        background::mount();
    });

    // Background ambient tracks. Both start from t=0 the instant the user
    // unmutes; both stop and reset on mute. Water Waves loops continuously.
    // Rhapsody In Blue plays through, waits 3s, then restarts. Audio elements
    // and the rhapsody-restart timer are kept on `window` so re-runs of this
    // effect reuse them instead of double-creating instances.
    use_effect(move || {
        let muted = *audio.is_muted.read();
        let js = if muted {
            "(function(){\
              if(window.bgWaterAudio){\
                window.bgWaterAudio.pause();\
                window.bgWaterAudio.currentTime=0;\
              }\
              if(window.bgRhapsodyAudio){\
                window.bgRhapsodyAudio.pause();\
                window.bgRhapsodyAudio.currentTime=0;\
              }\
              if(window.bgRhapsodyTimer){\
                clearTimeout(window.bgRhapsodyTimer);\
                window.bgRhapsodyTimer=null;\
              }\
            })()"
        } else {
            "(function(){\
              if(!window.bgWaterAudio){\
                var w=new Audio('/static/sounds/Water%20Waves.mp3');\
                w.loop=true;\
                w.volume=0.35;\
                window.bgWaterAudio=w;\
              }\
              window.bgWaterAudio.currentTime=0;\
              window.bgWaterAudio.play().catch(function(){});\
              if(!window.bgRhapsodyAudio){\
                var r=new Audio('/static/sounds/Rhapsody%20In%20Blue.mp3');\
                r.loop=false;\
                r.volume=0.5;\
                r.addEventListener('ended',function(){\
                  window.bgRhapsodyTimer=setTimeout(function(){\
                    if(window.bgRhapsodyAudio){\
                      window.bgRhapsodyAudio.currentTime=0;\
                      window.bgRhapsodyAudio.play().catch(function(){});\
                    }\
                  },3000);\
                });\
                window.bgRhapsodyAudio=r;\
              }\
              window.bgRhapsodyAudio.currentTime=0;\
              window.bgRhapsodyAudio.play().catch(function(){});\
            })()"
        };
        let _ = document::eval(js);
    });

    rsx! {
        document::Title { "Zachary Ernst" }
        document::Link { rel: "icon", r#type: "image/svg+xml", href: "/static/images/favicon.svg" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Stylesheet {
            href: "https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800&display=swap"
        }
        document::Stylesheet {
            href: "https://fonts.googleapis.com/css2?family=Merriweather:ital,wght@0,300;0,400;0,700;0,900;1,300;1,400;1,700;1,900&display=swap"
        }
        document::Stylesheet {
            href: "https://fonts.googleapis.com/css2?family=Aldrich&family=Rajdhani:wght@300;400;500;600;700&display=swap"
        }
        document::Stylesheet { href: "/static/styles/index.css" }
        document::Stylesheet { href: "/static/styles/App.css" }
        document::Stylesheet { href: "/static/styles/transitions.css" }
        document::Stylesheet { href: "/static/styles/components/Home/Home.css" }
        document::Stylesheet { href: "/static/styles/components/StartingScreen/StartingScreen.css" }
        document::Stylesheet { href: "/static/styles/components/Close/Close.css" }
        document::Stylesheet { href: "/static/styles/components/Manifesto/Manifesto.css" }
        document::Stylesheet { href: "/static/styles/components/NotFound/NotFound.css" }
        document::Stylesheet { href: "/static/styles/components/Credentials/Credentials.css" }
        document::Stylesheet { href: "/static/styles/components/Writing/Writing.css" }
        document::Stylesheet { href: "/static/styles/components/AIArchitecture/AIArchitecture.css" }
        document::Stylesheet { href: "/static/styles/components/ContactMe/ContactMe.css" }
        document::Stylesheet {
            href: "/static/styles/components/ProfessionalExperience/ProfessionalExperience.css"
        }
        document::Stylesheet {
            href: "/static/styles/components/SingleExperience/SingleExperience.css"
        }
        document::Stylesheet {
            href: "/static/styles/components/ProfessionalExperience/AgentWorkflowDiagram/AgentWorkflowDiagram.css"
        }
        document::Stylesheet {
            href: "/static/styles/components/ProfessionalExperience/CICDPipelineDiagram/CICDPipelineDiagram.css"
        }
        document::Stylesheet {
            href: "/static/styles/components/ProfessionalExperience/LayeredArchitectureDiagram/LayeredArchitectureDiagram.css"
        }
        document::Stylesheet {
            href: "/static/styles/components/ProfessionalExperience/RealTimeSyncDiagram/RealTimeSyncDiagram.css"
        }
        document::Stylesheet {
            href: "/static/styles/components/ProfessionalExperience/InfrastructureDiagram/InfrastructureDiagram.css"
        }

        // Animated water cinemagraph renders here (WebGL via /static/background.js).
        canvas {
            id: "background-canvas",
            style: "position:fixed;inset:0;z-index:-2;width:100%;height:100%;pointer-events:none;background:#0a1f1c;"
        }
        // Dark scrim between the background and the content. Tune the rgba
        // freely: last value is opacity (0.82 = mostly opaque).
        div {
            id: "background-overlay",
            style: "position:fixed;inset:0;z-index:-1;pointer-events:none;background:rgba(8,12,16,0.72);"
        }

        div { id: "App",
            // Wrapper that toggles `page-leaving` while a route transition
            // is in flight. The outgoing page (still mounted under Router)
            // fades + slides up via CSS, then `use_nav_with_transition`
            // fires nav.push, the router swaps, and the new page runs its
            // own `page-enter` animation. See contexts/page_transition.rs.
            //
            // The conditional `class:` is read at the top of `App` (not via
            // an inline rsx! block here) so SSR and WASM-side render produce
            // the same tree shape — an inline rsx! interpolation breaks
            // hydration tree-walking and lands the mount on `<body>`, which
            // then panics on `length` (see PR commit message for context).
            div {
                class: if *is_leaving.read() { "page-active page-leaving" } else { "page-active" },
                Router::<Route> {}
            }
        }
    }
}
