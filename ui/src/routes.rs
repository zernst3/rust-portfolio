use dioxus::prelude::*;

use crate::components;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    // Legacy alias — mirrors React's <Navigate to="/" replace />.
    #[route("/home")]
    HomeAlias {},
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
    // Legacy alias — mirrors React's <Navigate to="/work" replace />.
    #[route("/professional-experience")]
    ProfessionalExperienceAlias {},
    #[route("/writing")]
    Writing {},
    #[route("/contactme")]
    ContactMe {},
    // Explicit 404 page — mirrors React's <Route path="/404" element={<NotFound />} />.
    #[route("/404")]
    NotFoundPage {},
    // Catch-all redirects to /404 — mirrors React's <Navigate to="/404" replace />.
    #[route("/:..segments")]
    RedirectToNotFound { segments: Vec<String> },
}

// Thin wrappers that bridge Route variants → component module.
// Named to match their Route variant so the Routable derive can call them.

#[component]
fn Home() -> Element {
    rsx! { components::Home {} }
}

#[component]
fn HomeAlias() -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.replace(Route::Home {});
    });
    rsx! {}
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
fn ProfessionalExperienceAlias() -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.replace(Route::ProfessionalExperience {});
    });
    rsx! {}
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
fn NotFoundPage() -> Element {
    rsx! { components::NotFound {} }
}

#[component]
fn RedirectToNotFound(segments: Vec<String>) -> Element {
    let _ = segments;
    let nav = use_navigator();
    use_effect(move || {
        nav.replace(Route::NotFoundPage {});
    });
    rsx! {}
}
