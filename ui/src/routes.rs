use dioxus::prelude::*;

use crate::components;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/start")]
    StartingScreen {},
    #[route("/manifesto")]
    Manifesto {},
    #[route("/ai-architecture")]
    AIArchitecture {},
    #[route("/credentials")]
    Credentials {},
    #[route("/work")]
    ProfessionalExperience {},
    #[route("/writing")]
    Writing {},
    #[route("/contactme")]
    ContactMe {},
    // Catch-all — renders NotFound for any unmatched path.
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

// Thin wrappers that bridge Route variants → component module.
// Named to match their Route variant so the Routable derive can call them.

#[component]
fn Home() -> Element {
    rsx! { components::Home {} }
}

#[component]
fn StartingScreen() -> Element {
    rsx! { components::StartingScreen {} }
}

#[component]
fn Manifesto() -> Element {
    rsx! { components::Manifesto {} }
}

#[component]
fn AIArchitecture() -> Element {
    rsx! { components::AIArchitecture {} }
}

#[component]
fn Credentials() -> Element {
    rsx! { components::Credentials {} }
}

#[component]
fn ProfessionalExperience() -> Element {
    rsx! { components::ProfessionalExperience {} }
}

#[component]
fn Writing() -> Element {
    rsx! { components::Writing {} }
}

#[component]
fn ContactMe() -> Element {
    rsx! { components::ContactMe {} }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let _ = segments;
    rsx! { components::NotFound {} }
}
