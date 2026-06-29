use dioxus::prelude::*;

use crate::audio::play_sound;
use crate::components::Close;
use crate::contexts::audio::use_audio_state;

struct Article {
    title: &'static str,
    publication: &'static str,
    date: &'static str,
    snippet: &'static str,
    url: &'static str,
}

static ARTICLES: &[Article] = &[
    Article {
        title: "How AI Software Development Changes The Premature Optimization Paradigm",
        publication: "Medium",
        date: "June 2026",
        snippet: "An essay on how AI collapses the cost of writing optimized code, why that inverts Knuth's old \"premature optimization\" tradeoff, and where the line still holds: the judgment of what is worth building, which AI never made cheap.",
        url: "https://medium.com/@zacharyernst/how-ai-software-development-changes-the-premature-optimization-paradigm-7a846446011a",
    },
    Article {
        title: "Codifying \"Less is More\" AI Enforcement",
        publication: "Medium",
        date: "June 2026",
        snippet: "An essay on why AI agents follow terse imperatives more reliably than reasoning-rich prose, and the pattern of splitting each architectural rule into two voicings: a context-heavy version for the humans deciding it, and a stripped directive version for the agents enforcing it.",
        url: "https://zacharyernst.medium.com/codifying-less-is-more-ai-enforcement-dfc6dc7e1829",
    },
    Article {
        title: "Architecture as Law",
        publication: "Medium",
        date: "May 2026",
        snippet: "An essay on architectural commitments as binding constraints rather than advisory documents, and what changes when teams treat their design decisions as load-bearing inside a codebase.",
        url: "https://medium.com/@zacharyernst/architecture-as-law-1f8101b7c046",
    },
];

#[component]
pub fn Writing() -> Element {
    let audio = use_audio_state();

    rsx! {
        div { class: "page-enter",
            div { id: "WritingContainer", class: "subpage-overlay",
                div { id: "WritingInnerContainer",
                    h1 { "Writing" }
                    p { class: "writing-lede",
                        "A growing collection of essays on AI architecture and AI-native development practice."
                    }

                    div { class: "writing-cards",
                        {ARTICLES.iter().map(|article| rsx! {
                            a {
                                key: "{article.title}",
                                class: "writing-card",
                                href: "{article.url}",
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
                                div { class: "writing-card-meta",
                                    span { class: "writing-card-publication", "{article.publication}" }
                                    span { class: "writing-card-dot", "·" }
                                    span { class: "writing-card-date", "{article.date}" }
                                }
                                h2 { class: "writing-card-title", "{article.title}" }
                                p { class: "writing-card-snippet", "{article.snippet}" }
                                span { class: "writing-card-cta", "Read on {article.publication} ↗" }
                            }
                        })}
                    }
                }
                Close {}
            }
        }
    }
}
