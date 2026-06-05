//! Dioxus UI crate for the portfolio site.
//!
//! Bot ports the React component tree from `~/Documents/Repos/MyPortfolioSite`
//! into Dioxus RSX components in this crate, one per session per the
//! work-queue. This file is a placeholder so the workspace builds.

use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        div { "rust-portfolio: scaffold placeholder. bot replaces during v0.1." }
    }
}
