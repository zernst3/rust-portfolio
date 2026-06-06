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

struct Detail {
    heading: &'static str,
    body: DetailBody,
}

enum DetailBody {
    Text(&'static str),
    // Camerata detail has an embedded external link.
    Camerata,
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
                body: DetailBody::Text("The morning consolidator wrapper checks when the previous successful run completed. If more than 25 hours have passed, it sends a direct alert that the routine has been silently failing."),
            },
            Detail {
                heading: "Exit-code accountability",
                body: DetailBody::Text("The wrapper inspects the output and Slack-delivery result, not just the process exit code, so a routine that 'completed' but failed to deliver is detected and surfaced."),
            },
        ],
    },
    Tile {
        id: 2,
        title: "Routines",
        icon: ICON_SCHEDULE,
        summary: "Eight scheduled jobs do the routine work.",
        details: &[
            Detail {
                heading: "Daily review brief",
                body: DetailBody::Text("Scans the day's commits for architectural violations and posts a concise report."),
            },
            Detail {
                heading: "Bug triage (≤ 2 PRs/day)",
                body: DetailBody::Text("Reads open issues, reproduces the bug, opens a fix PR. Capped at two PRs per day to respect human-review bandwidth."),
            },
            Detail {
                heading: "Daily wrap-up summary",
                body: DetailBody::Text("Recaps the day's automated and human activity into a single digest for collaborators in other time zones."),
            },
            Detail {
                heading: "Morning consolidator",
                body: DetailBody::Text("Reads each overnight routine's inbox entries and writes one ranked digest as the day's first message: priority items first, then everything else. Replaces per-routine direct messaging (which the original architecture used and which produced too much noise) and ships with a 25-hour gap fail-safe that alerts if any expected consolidator run has been silently missed."),
            },
            Detail {
                heading: "Weekly architectural drift watcher",
                body: DetailBody::Text("Tracks where the live TypeScript codebase has drifted from the in-progress Rust port and logs the gap."),
            },
            Detail {
                heading: "Weekly product digest",
                body: DetailBody::Text("Summarizes what shipped, what opened and closed and which decisions are still outstanding."),
            },
            Detail {
                heading: "Weekly dependency security sweep",
                body: DetailBody::Text("Reads npm audit, attempts each high/critical bump, runs the full build and Jest suite, opens a PR only if green."),
            },
            Detail {
                heading: "Overnight Rust port",
                body: DetailBody::Text("Migrates the full TypeScript codebase to Rust under documented convention rules with auto-call escalation for novel architectural questions. The SeaORM entity layer, the entity-to-domain mappers, and the infrastructure repositories are populated; the routine is currently moving from infrastructure into the application-service layer, followed by workers, the HTTP server, and the UI. The goal is a complete language migration, not a partial coexistence."),
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
                body: DetailBody::Text("An ESLint rule blocks any database call (db.insert/update/delete/select) outside the repository layer. A controller that reaches for the ORM directly fails CI and can't merge."),
            },
            Detail {
                heading: "Per-resource permission flags",
                body: DetailBody::Text("For organization-scoped permissions specifically (who can manage which org), the front end is lint-banned from introspecting permission arrays. Each org response carries a server-stamped _can flag the UI simply reads. Global permissions still use the standard permission-array check; the rule narrows the dangerous client-side derivation pattern without touching the simple cases."),
            },
            Detail {
                heading: "Secret scanning",
                body: DetailBody::Text("gitleaks scans every PR for committed credentials. Failure blocks the merge."),
            },
            Detail {
                heading: "Migration safety + audit fields",
                body: DetailBody::Text("Migration linter catches dangerous DDL patterns; an audit-field rule enforces standard timestamping and modifier tracking on the tables that need them."),
            },
            Detail {
                heading: "1,000+ test merge gate",
                body: DetailBody::Text("Every merge — human or agent — passes the full Jest suite. The test gate is the same regardless of authorship."),
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
                heading: "Composed digests → outbox",
                body: DetailBody::Text("The morning consolidator writes its composed Slack message to a known outbox path. The wrapper reads the file and posts it via direct Slack API call."),
            },
            Detail {
                heading: "Direct curl over MCP",
                body: DetailBody::Text("Slack delivery uses a bot token and a curl POST instead of the Slack MCP server. This eliminates an entire class of 'silently broken delivery' failures that the prior architecture was vulnerable to."),
            },
            Detail {
                heading: "Single morning DM",
                body: DetailBody::Text("Individual routines write to an inbox during the night. One consolidator DM in the morning ranks the contents by priority and delivers the digest."),
            },
            Detail {
                heading: "Tiered merge",
                body: DetailBody::Text("Every machine-authored change lands as a pull request. Tight low-level PRs auto-merge once CI passes and CodeRabbit's AI review comes back clean. PRs that touch architectural surfaces or hard-guarded areas are flagged for human review before merging. The bot picks the channel based on PR scope so my time goes to the changes that actually need a person."),
            },
        ],
    },
];

#[component]
pub fn AIArchitecture() -> Element {
    let audio = use_audio_state();
    let mut focused: Signal<Option<usize>> = use_signal(|| None);

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
                        "The system I operate to make AI-written code trustworthy. Standards are encoded as enforceable rules; agents apply those rules on a schedule; I am the escalation path for the cases the rules don't cover. Click any tile to see the mechanism in detail."
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
                                    class: "tile-focused",
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
                                                if !*audio.is_muted.read() {
                                                    play_sound("/static/sounds/woosh2.mp3", 0.1);
                                                }
                                                focused.set(None);
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
                                                                "The codification approach above — rules with stable IDs, real alternatives, and an explicit \"why\" — is packaged as Camerata, my open-source library that lets any project declare these architectural commitments to its AI agents in a portable format. The repo ships ~96 starter principles and the schema for declaring your own. "
                                                                a {
                                                                    href: "https://github.com/zernst3/camerata-ai",
                                                                    target: "_blank",
                                                                    rel: "noopener noreferrer",
                                                                    class: "green-text",
                                                                    "github.com/zernst3/camerata-ai"
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
