pub mod audio;
pub mod components;
pub mod contexts;

use dioxus::prelude::*;

use contexts::audio::provide_audio_context;
use contexts::mobile_menu::provide_mobile_menu_context;

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

    rsx! {
        document::Title { "Zachary Ernst" }
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
        document::Stylesheet { href: "/assets/styles/index.css" }
        document::Stylesheet { href: "/assets/styles/App.css" }
        document::Stylesheet { href: "/assets/styles/transitions.css" }
        document::Stylesheet { href: "/assets/styles/components/Home/Home.css" }
        document::Stylesheet { href: "/assets/styles/components/StartingScreen/StartingScreen.css" }
        document::Stylesheet { href: "/assets/styles/components/Close/Close.css" }

        div { id: "App",
            components::Home {}
        }
    }
}
