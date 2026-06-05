use dioxus::prelude::*;

use crate::components::Close;

struct Statement {
    parts: &'static [StatementPart],
}

enum StatementPart {
    Text(&'static str),
    Flair(&'static str),
    Em(&'static str),
}

use StatementPart::*;

static STATEMENTS: &[Statement] = &[
    Statement {
        parts: &[
            Text("I design and build enterprise-grade systems in two very different worlds: "),
            Flair("financial market intelligence"),
            Text(" and "),
            Flair("traditional urbanism"),
            Text("."),
        ],
    },
    Statement {
        parts: &[
            Flair("AI-native development"),
            Text(
                " is my default, and I use it to ship secure, maintainable software quickly and to hold architectural standards in place as a product grows.",
            ),
        ],
    },
    Statement {
        parts: &[
            Text(
                "I work asynchronously across time zones, keeping distributed contributors in the ",
            ),
            Em("United States, Europe, and India"),
            Text(" aligned to one architecture."),
        ],
    },
];

#[component]
pub fn Manifesto() -> Element {
    rsx! {
        div { class: "page-enter",
            div { id: "ManifestoContainer",
                div { id: "ManifestoInnerContainer",
                    h1 { style: "animation: name-fade-in-kf 1s ease-out forwards; opacity: 0",
                        "Architectural Manifesto"
                    }

                    div { class: "manifesto-thread",
                        div { class: "thread-line",
                            span { class: "thread-pulse" }
                        }
                        div { class: "story-container",
                            {STATEMENTS.iter().enumerate().map(|(i, stmt)| {
                                let delay = 0.5 + i as f32 * 0.7;
                                let style = format!(
                                    "animation: fade-in-kf 0.9s ease-out {delay}s forwards; opacity: 0"
                                );
                                rsx! {
                                    section {
                                        key: "{i}",
                                        class: "story-block",
                                        style: "{style}",
                                        span { class: "thread-node" }
                                        p {
                                            {stmt.parts.iter().map(|part| match part {
                                                Text(t) => rsx! { "{t}" },
                                                Flair(t) => rsx! { span { class: "flair", "{t}" } },
                                                Em(t) => rsx! { em { "{t}" } },
                                            })}
                                        }
                                    }
                                }
                            })}
                        }
                    }
                }
                Close {}
            }
        }
    }
}
