use dioxus::prelude::*;

use crate::audio::play_sound;
use crate::components::Close;
use crate::contexts::audio::use_audio_state;

#[component]
pub fn Credentials() -> Element {
    let audio = use_audio_state();

    rsx! {
        div { class: "page-enter",
            div { id: "EducationAndWorkContainer",
                div { id: "EducationAndWorkInnerContainer",
                    h1 { "Credentials" }
                    div { class: "Info",
                        h2 { "Eligibility and Location" }
                        div { class: "FlexRow",
                            div { class: "Block",
                                p {
                                    strong { "Status:" }
                                    " United States Citizen"
                                    br {}
                                    strong { "Location:" }
                                    " New York City"
                                    br {}
                                    strong { "Availability:" }
                                    " Fully Remote"
                                }
                            }
                        }

                        h2 { "Education & Certifications" }
                        div { class: "FlexRow Education",
                            div { class: "Block",
                                h4 { "Microsoft" }
                                p { "2022 - 2024" }
                                p {
                                    "• "
                                    a {
                                        href: "https://learn.microsoft.com/api/credentials/share/en-us/ZacharyErnst-2786/3164FF9B17DAE905?sharingId",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        onmouseenter: move |_| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/woosh3.mp3", 0.25);
                                            }
                                        },
                                        onclick: move |_| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/select.mp3", 0.45);
                                            }
                                        },
                                        "Azure Developer Associate (AZ-204)"
                                    }
                                    " - Dec 2024"
                                    br {}
                                    "• "
                                    a {
                                        href: "https://www.credly.com/badges/16c63be7-621b-489f-a475-0a740c997b28?source=linked_in_profile",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        onmouseenter: move |_| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/woosh3.mp3", 0.25);
                                            }
                                        },
                                        onclick: move |_| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/select.mp3", 0.45);
                                            }
                                        },
                                        "Azure Fundamentals (AZ-900)"
                                    }
                                    " - June 2022"
                                }
                            }

                            div { class: "Block",
                                h4 { "FullStack Academy" }
                                p { "2020 - 2020" }
                                p {
                                    "• "
                                    strong { "Web Development Certificate" }
                                    " - New York, NY"
                                }
                            }

                            div { class: "Block",
                                h4 { "Rutgers University" }
                                p { "2015 - 2018" }
                                p {
                                    "• "
                                    strong { "B.S. in Public and Non-Profit Administration" }
                                    " - Summa Cum Laude"
                                }
                            }
                        }
                    }
                }
                Close {}
            }
        }
    }
}
