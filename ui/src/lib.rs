//! Dioxus UI crate for the portfolio site.
//!
//! Bot ports the React component tree from `~/Documents/Repos/MyPortfolioSite`
//! into Dioxus RSX components in this crate, one per session per the
//! work-queue. This file is a placeholder so the workspace builds.

use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        document::Title { "Zachary Ernst" }
        // Preconnect hint for Google Fonts glyph CDN
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        // JetBrains Mono — monospace accent font (code blocks, labels)
        document::Stylesheet {
            href: "https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800&display=swap"
        }
        // Merriweather — serif body font
        document::Stylesheet {
            href: "https://fonts.googleapis.com/css2?family=Merriweather:ital,wght@0,300;0,400;0,700;0,900;1,300;1,400;1,700;1,900&display=swap"
        }
        // Aldrich + Rajdhani — primary display and secondary body fonts
        document::Stylesheet {
            href: "https://fonts.googleapis.com/css2?family=Aldrich&family=Rajdhani:wght@300;400;500;600;700&display=swap"
        }
        // Base styles ported verbatim per PORT-CSS-1
        document::Stylesheet { href: "/assets/styles/index.css" }
        document::Stylesheet { href: "/assets/styles/App.css" }
        div { "rust-portfolio: scaffold placeholder. bot replaces during v0.1." }
    }
}
