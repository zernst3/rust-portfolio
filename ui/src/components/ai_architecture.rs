use dioxus::prelude::*;

use crate::audio::play_sound;
use crate::components::Close;
use crate::contexts::audio::use_audio_state;

// Material Design SVG paths extracted verbatim from @mui/icons-material.
const ICON_BUILD: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="2rem" height="2rem" fill="currentColor"><path d="m22.7 19-9.1-9.1c.9-2.3.4-5-1.5-6.9-2-2-5-2.4-7.4-1.3L9 6 6 9 1.6 4.7C.4 7.1.9 10.1 2.9 12.1c1.9 1.9 4.6 2.4 6.9 1.5l9.1 9.1c.4.4 1 .4 1.4 0l2.3-2.3c.5-.4.5-1.1.1-1.4"/></svg>"#;
const ICON_SHIELD: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="2rem" height="2rem" fill="currentColor"><path d="M12 1 3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5z"/></svg>"#;
const ICON_SCHEDULE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="2rem" height="2rem" fill="currentColor"><path d="M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2M12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8"/><path d="M12.5 7H11v6l5.25 3.15.75-1.23-4.5-2.67z"/></svg>"#;
const ICON_GAVEL: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="2rem" height="2rem" fill="currentColor"><path d="m5.2494 8.0688 2.83-2.8269 14.1343 14.15-2.83 2.8269zm4.2363-4.2415 2.828-2.8289 5.6577 5.656-2.828 2.8289zM.9989 12.3147l2.8284-2.8285 5.6569 5.6569-2.8285 2.8284zM1 21h12v2H1z"/></svg>"#;
const ICON_VERIFIED: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="2rem" height="2rem" fill="currentColor"><path d="m23 12-2.44-2.79.34-3.69-3.61-.82-1.89-3.2L12 2.96 8.6 1.5 6.71 4.69 3.1 5.5l.34 3.7L1 12l2.44 2.79-.34 3.7 3.61.82L8.6 22.5l3.4-1.47 3.4 1.46 1.89-3.19 3.61-.82-.34-3.69zm-12.91 4.72-3.8-3.81 1.48-1.48 2.32 2.33 5.85-5.87 1.48 1.48z"/></svg>"#;
const ICON_FORWARD_TO_INBOX: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="2rem" height="2rem" fill="currentColor"><path d="M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h9v-2H4V8l8 5 8-5v5h2V6c0-1.1-.9-2-2-2m-8 7L4 6h16zm7 4 4 4-4 4v-3h-4v-2h4z"/></svg>"#;
const ICON_PERSON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="2.25rem" height="2.25rem" fill="currentColor"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4m0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4"/></svg>"#;
const ICON_VISIBILITY: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="2rem" height="2rem" fill="currentColor"><path d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5M12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5m0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3"/></svg>"#;

struct Detail {
    heading: &'static str,
    body: DetailBody,
}

enum DetailBody {
    Text(&'static str),
    // Camerata detail has an embedded external link.
    Camerata,
    // Chorale-bug detail (the "Judgment" tile) has an embedded external link.
    ChoraleBug,
}

struct Tile {
    id: usize,
    title: &'static str,
    icon: &'static str,
    summary: &'static str,
    details: &'static [Detail],
}

static TILES: &[Tile] = &[
    Tile {
        id: 0,
        title: "Tools",
        icon: ICON_BUILD,
        summary: "The AI engines I operate.",
        details: &[
            Detail {
                heading: "Claude Code (interactive)",
                body: DetailBody::Text("Paired with daily for architecture, design, debugging and code review. The conversational layer where decisions are talked through and recorded."),
            },
            Detail {
                heading: "claude -p (unattended)",
                body: DetailBody::Text("Non-interactive Claude runs invoked by cron jobs overnight. Each routine is a tightly scoped prompt run against the codebase without human supervision."),
            },
            Detail {
                heading: "Gemini",
                body: DetailBody::Text("Used for quick research on conventions and alternate perspectives where a second voice is useful. Not in the autonomous loop."),
            },
        ],
    },
    Tile {
        id: 1,
        title: "Wrappers",
        icon: ICON_SHIELD,
        summary: "Shell gates around every unattended run.",
        details: &[
            Detail {
                heading: "Budget caps",
                body: DetailBody::Text("Every scheduled run is wrapped by a shell script that enforces a hard nightly budget. Overruns terminate the routine cleanly."),
            },
            Detail {
                heading: "Pause flags",
                body: DetailBody::Text("When the bot logs an unresolved decision, it drops a flag file. The next scheduled invocation refuses to start until the flag is cleared by a human."),
            },
            Detail {
                heading: "Fail-safe gap detection",
                body: DetailBody::Text("A wrapper records when each routine last succeeded. If too long passes without a successful run, it sends a direct alert, so a routine that has been silently failing surfaces instead of going unnoticed."),
            },
            Detail {
                heading: "Exit-code accountability",
                body: DetailBody::Text("The wrapper inspects the actual output and the delivery result, not just the process exit code, so a routine that 'completed' but failed to deliver its result is still caught."),
            },
        ],
    },
    Tile {
        id: 2,
        title: "Routines",
        icon: ICON_SCHEDULE,
        summary: "Scheduled jobs handle the repetitive work.",
        details: &[
            Detail {
                heading: "One routine, one job",
                body: DetailBody::Text("Each routine is a single, tightly scoped prompt aimed at one kind of work, not a general-purpose agent told to be helpful. Narrow scope is what makes an unattended overnight run safe to trust."),
            },
            Detail {
                heading: "Work that maps onto a schedule",
                body: DetailBody::Text("A lot of recurring engineering work fits this shape cleanly: review passes over recent commits, bug triage that opens fix PRs, dependency-security sweeps, status digests for collaborators in other time zones, and long-running migrations that grind through a large codebase over many nights."),
            },
            Detail {
                heading: "Cadence by purpose",
                body: DetailBody::Text("Fast-signal jobs run daily; heavier analysis runs weekly. The schedule matches how often each kind of work actually produces something worth a person's attention, which keeps the noise low."),
            },
            Detail {
                heading: "Capped to human bandwidth",
                body: DetailBody::Text("Any routine that opens pull requests is rate-limited, so the machine never generates more review work in a day than a person can actually absorb."),
            },
        ],
    },
    Tile {
        id: 3,
        title: "Governance",
        icon: ICON_GAVEL,
        summary: "Architecture treated as enforceable code.",
        details: &[
            Detail {
                heading: "CONVENTIONS.md",
                body: DetailBody::Text("Every architectural rule has an ID, a one-line statement, a rationale and an example. Agents cite the rule ID in their commit messages, making every change traceable to the rule it applied."),
            },
            Detail {
                heading: "Decisions-needed ledger",
                body: DetailBody::Text("When the bot encounters a call it can't make, it appends the case to a decisions file: the options, the tradeoffs, its recommendation and a confidence level."),
            },
            Detail {
                heading: "Manual-work-needed log",
                body: DetailBody::Text("Items the bot skips because they need human design or sign-off accumulate here without halting the routine. Reviewed on the human's schedule, not the bot's."),
            },
            Detail {
                heading: "Auto-calls ledger + weekly lock",
                body: DetailBody::Text("Every self-made clear-winner call is logged with its alternatives. Once a week a digest is delivered and a lock halts all unattended runs until each call is accepted, modified or rejected."),
            },
            Detail {
                heading: "Camerata (open source)",
                body: DetailBody::Camerata,
            },
        ],
    },
    Tile {
        id: 4,
        title: "Enforcement",
        icon: ICON_VERIFIED,
        summary: "Mechanical guardrails that fail CI.",
        details: &[
            Detail {
                heading: "Lint-encoded layer boundaries",
                body: DetailBody::Text("Architectural boundaries are encoded as lint rules rather than left to convention. When a layer reaches past where it belongs, for example a controller touching the database directly, CI fails and the change cannot merge."),
            },
            Detail {
                heading: "Secret scanning",
                body: DetailBody::Text("Every pull request is scanned for committed credentials. A leaked secret blocks the merge before it can reach history."),
            },
            Detail {
                heading: "Migration safety + audit fields",
                body: DetailBody::Text("A migration linter catches dangerous schema-change patterns, and an audit-field rule enforces consistent timestamping and modifier tracking on the tables that need it."),
            },
            Detail {
                heading: "The test suite gates every merge",
                body: DetailBody::Text("The full test suite runs on every merge, human or agent, with no exception for authorship. Machine-written code clears exactly the same bar as code I write by hand."),
            },
        ],
    },
    Tile {
        id: 5,
        title: "Delivery",
        icon: ICON_FORWARD_TO_INBOX,
        summary: "Outputs reach me through controlled channels.",
        details: &[
            Detail {
                heading: "Compose, then deliver",
                body: DetailBody::Text("A routine writes its finished message to a known outbox path; a wrapper reads that file and sends it. Composing and delivering are separate steps, so a formatting bug can never silently swallow the result."),
            },
            Detail {
                heading: "Delivery over a controlled channel",
                body: DetailBody::Text("Messages go out through a direct, dependency-light API call rather than a heavier integration layer. That removes an entire class of failures where the work ran but the message never arrived."),
            },
            Detail {
                heading: "One digest, not a stream",
                body: DetailBody::Text("Individual routines write to an inbox through the night; one ranked digest arrives in the morning, priority first. One message to read instead of a trickle of notifications."),
            },
            Detail {
                heading: "Tiered merge",
                body: DetailBody::Text("Every machine-authored change lands as a pull request. Tight, low-level PRs auto-merge once CI and automated review pass; PRs that touch architectural or sensitive surfaces are held for a person. The routine picks the channel by scope, so my time goes to the changes that actually need it."),
            },
        ],
    },
    Tile {
        id: 6,
        title: "Judgment",
        icon: ICON_VISIBILITY,
        summary: "A real example of what the human review loop catches that the rules cannot.",
        details: &[
            Detail {
                heading: "A bug that passed every test",
                body: DetailBody::Text("In Chorale, the table re-ran a full filter, sort and clone of the entire dataset twice on every render, and triggered it again on every scroll event. On a ten-thousand-row grid, fast scrolling re-sorted ten thousand rows twice per scroll tick. Every unit test passed."),
            },
            Detail {
                heading: "Why the tests could not see it",
                body: DetailBody::Text("The defect lived in the wiring between pure functions, not inside any one of them. Each function was correct in isolation, so its unit test passed. The bug only existed in how they were composed, which is exactly the seam automated tests rarely cover."),
            },
            Detail {
                heading: "Caught in review, fixed, documented",
                body: DetailBody::ChoraleBug,
            },
        ],
    },
];

#[component]
pub fn AIArchitecture() -> Element {
    let audio = use_audio_state();
    let mut focused: Signal<Option<usize>> = use_signal(|| None);
    // While true, the focused tile carries `.detail-leaving` so it plays the
    // fade+slide-down exit before `focused` is cleared back to the grid.
    let mut is_closing: Signal<bool> = use_signal(|| false);

    let is_focused = focused.read().is_some();
    let grid_style = if is_focused {
        "opacity: 0; pointer-events: none; transition: opacity 0.25s ease"
    } else {
        "opacity: 1; pointer-events: auto; transition: opacity 0.25s ease"
    };
    let close_style = if is_focused {
        "opacity: 0; pointer-events: none; transition: opacity 0.25s ease; width: 100%"
    } else {
        "opacity: 1; pointer-events: auto; transition: opacity 0.25s ease; width: 100%"
    };

    rsx! {
        div { class: "page-enter",
            div { id: "AIArchitectureContainer",
                div { id: "AIArchitectureInnerContainer",
                    h1 { style: "animation: name-fade-in-kf 0.6s ease-out forwards; opacity: 0",
                        "AI Architecture"
                    }

                    p {
                        class: "intro",
                        style: "animation: fade-in-kf 0.6s ease-out 0.2s forwards; opacity: 0",
                        "How I build software by orchestrating AI instead of hand-writing it. The approach is the same on every project: settle the architecture up front and write it down as enforceable rules, let agents apply those rules on a schedule, and stay the escalation path for the cases the rules do not cover. The decisions that are expensive to reverse get made before any code is written, not discovered halfway through. Click any tile to see the mechanism."
                    }

                    div { class: "ai-arch-stage",
                        // Tile grid — always mounted; fades out when a tile is focused.
                        div { class: "tile-grid", style: "{grid_style}",
                            {TILES.iter().map(|tile| {
                                let tile_id = tile.id;
                                let icon_html = tile.icon;
                                rsx! {
                                    button {
                                        key: "{tile_id}",
                                        class: "tile",
                                        onclick: move |_| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/select.mp3", 0.45);
                                            }
                                            focused.set(Some(tile_id));
                                        },
                                        onmouseenter: move |_| {
                                            if !is_focused && !*audio.is_muted.read() {
                                                play_sound("/static/sounds/woosh3.mp3", 0.25);
                                            }
                                        },
                                        div {
                                            class: "tile-icon",
                                            dangerous_inner_html: "{icon_html}",
                                        }
                                        h2 { class: "tile-title", "{tile.title}" }
                                        p { class: "tile-summary", "{tile.summary}" }
                                    }
                                }
                            })}

                            // Human-in-the-loop card spans full grid width.
                            div { class: "human-card",
                                div {
                                    class: "human-icon",
                                    dangerous_inner_html: ICON_PERSON,
                                }
                                div { class: "human-text",
                                    div { class: "human-label", "Human in the loop" }
                                    div { class: "human-body",
                                        "The rules and CI gates are robust enough that tight low-level PRs auto-merge once they pass. I review the high-level and architectural PRs by hand, resolve escalations from the decisions ledger and accept or reject each auto-call in the weekly review. Human QA testing catches whatever the rules don't. The rule set grows exactly where my judgment is exercised."
                                    }
                                }
                            }
                        }

                        // Focused tile detail panel — conditionally rendered; CSS enter animation.
                        if let Some(idx) = *focused.read() {
                            if let Some(tile) = TILES.get(idx) {
                                div {
                                    class: if *is_closing.read() { "tile-focused detail-leaving" } else { "tile-focused" },
                                    style: "animation: tile-focused-enter-kf 0.3s cubic-bezier(0.4,0,0.2,1) forwards; opacity: 0",
                                    div { class: "tile-focused-header",
                                        div {
                                            class: "tile-icon tile-icon-large",
                                            dangerous_inner_html: "{tile.icon}",
                                        }
                                        h2 { class: "tile-focused-title", "{tile.title}" }
                                        button {
                                            class: "back-button",
                                            onclick: move |_| {
                                                if *is_closing.peek() {
                                                    return;
                                                }
                                                if !*audio.is_muted.read() {
                                                    play_sound("/static/sounds/woosh2.mp3", 0.1);
                                                }
                                                is_closing.set(true);
                                                spawn(async move {
                                                    // Match the `.detail-leaving` duration (0.2s) in transitions.css.
                                                    document::eval("await new Promise(function(r){setTimeout(r,200);});")
                                                        .await
                                                        .ok();
                                                    focused.set(None);
                                                    is_closing.set(false);
                                                });
                                            },
                                            "Back"
                                        }
                                    }

                                    p { class: "tile-focused-summary", "{tile.summary}" }

                                    ul { class: "tile-detail-list",
                                        {tile.details.iter().enumerate().map(|(i, detail)| {
                                            let delay = 0.1 + i as f32 * 0.05;
                                            let li_style = format!(
                                                "animation: fade-in-kf 0.3s ease-out {delay}s forwards; opacity: 0"
                                            );
                                            rsx! {
                                                li { key: "{detail.heading}", style: "{li_style}",
                                                    h3 { "{detail.heading}" }
                                                    {match &detail.body {
                                                        DetailBody::Text(t) => rsx! { p { "{t}" } },
                                                        DetailBody::Camerata => rsx! {
                                                            p {
                                                                "The codification approach above (rules with stable IDs, real alternatives, and an explicit \"why\") is the corpus inside Camerata, my governance engine for AI coding agents, built in Rust. It audits a codebase against these rules today, and its architecture is built to enforce them on agent output during development. The repo ships over 100 starter principles and the schema for declaring your own. "
                                                                a {
                                                                    href: "https://github.com/zernst3/camerata-orchestrator",
                                                                    target: "_blank",
                                                                    rel: "noopener noreferrer",
                                                                    class: "green-text",
                                                                    "github.com/zernst3/camerata-orchestrator"
                                                                }
                                                            }
                                                        },
                                                        DetailBody::ChoraleBug => rsx! {
                                                            p {
                                                                "It surfaced in a review pass, not from a failing test. I memoized the work behind a single derived view, removed the double pass, added a regression test, and kept the crate clippy-pedantic-clean, then documented the honest residual left for a later release. The doubt about AI-written code rests on the assumption that nobody is watching. The watching is the point. "
                                                                a {
                                                                    href: "https://github.com/zernst3/rust-chorale",
                                                                    target: "_blank",
                                                                    rel: "noopener noreferrer",
                                                                    class: "green-text",
                                                                    "github.com/zernst3/rust-chorale"
                                                                }
                                                            }
                                                        },
                                                    }}
                                                }
                                            }
                                        })}
                                    }
                                }
                            }
                        }
                    }
                }

                div { style: "{close_style}",
                    Close {}
                }
            }
        }
    }
}
