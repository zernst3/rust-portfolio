use dioxus::prelude::*;

use crate::components::Close;

struct Article {
    title: &'static str,
    publication: &'static str,
    date: &'static str,
    snippet: &'static str,
    url: &'static str,
}

static ARTICLES: &[Article] = &[
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
