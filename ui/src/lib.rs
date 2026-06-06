pub mod audio;
pub mod components;
pub mod contexts;
pub mod routes;

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
pub use server::build_router;

use dioxus::prelude::*;

use contexts::audio::provide_audio_context;
use contexts::mobile_menu::provide_mobile_menu_context;
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
    provide_audio_context();
    provide_mobile_menu_context();

    // Load Bevy WASM scene on first client render (no-op on SSR per PORT-BEVY-1).
    // TEMPORARILY DISABLED 2026-06-05 to bisect a main-thread freeze (white page,
    // dead refresh, slow tab close). If the app is responsive with this off, the
    // Bevy event loop / 25MB Bevy bundle is the culprit. Re-enable once isolated.
    // Bevy files are served at /static/ (repo static files moved off /assets to
    // leave that prefix for dx's hydration bootstrap; see server.rs asset strategy).
    // use_effect(|| {
    //     let _ = document::eval(
    //         "import('/static/portfolio_scene.js')\
    //          .then(function(m){return m.default();})\
    //          .catch(function(e){console.warn('Bevy scene unavailable',e);});",
    //     );
    // });

    rsx! {
        document::Title { "Zachary Ernst" }
        document::Link { rel: "icon", href: "/static/images/favicon.ico" }
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

        // Background canvas — Bevy mounts here per PORT-BEVY-1.
        canvas {
            id: "bevy-canvas",
            style: "position:fixed;inset:0;z-index:-1;width:100%;height:100%;pointer-events:none;background:#0a1f1c;"
        }

        div { id: "App",
            Router::<Route> {}
        }
    }
}
