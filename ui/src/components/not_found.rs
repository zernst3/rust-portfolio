use dioxus::prelude::*;

use crate::components::Close;

#[component]
pub fn NotFound() -> Element {
    rsx! {
        div { class: "page-enter",
            div { id: "NotFoundContainer",
                h1 { "Sorry, this page has not been found (Error: 404)" }
                Close {}
            }
        }
    }
}
