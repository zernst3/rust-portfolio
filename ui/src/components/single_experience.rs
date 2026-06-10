use dioxus::prelude::*;

use crate::audio::play_sound;
use crate::components::{
    AgentWorkflowDiagram, InfrastructureDiagram, LayeredArchitectureDiagram, RealTimeSyncDiagram,
};
use crate::contexts::audio::use_audio_state;
use crate::contexts::page_transition::use_nav_with_transition;

#[derive(Props, Clone, PartialEq)]
pub struct SingleExperienceProps {
    pub experience_key: &'static str,
    pub on_back: EventHandler<()>,
}

#[component]
pub fn SingleExperience(props: SingleExperienceProps) -> Element {
    let audio = use_audio_state();

    let on_back = move |_| {
        if !*audio.is_muted.read() {
            play_sound("/static/sounds/woosh2.mp3", 0.1);
        }
        props.on_back.call(());
    };

    rsx! {
        div {
            id: "SingleExperienceOuterContainer",
            // No open animation — kept consistent with the other previews
            // (Credentials / Chorale / Camerata), which render instantly.
            div { id: "SingleExperienceContainer",
                div { id: "SingleExperience",
                    // Mobile-only title — CSS hides on desktop
                    h1 { class: "mobile", "{experience_name(props.experience_key)}" }

                    // No image carousel: images array is empty for all entries
                    // in ExperienceList. Carousel omitted (PORT-AUTO-CALLS-1).

                    div { class: "experienceInfo",
                        ExperienceDescription { experience_key: props.experience_key }
                    }

                    // Links section (all current entries have empty links arrays)
                }
                div { class: "mobile",
                    button { onclick: on_back, "Back" }
                }
            }
        }
    }
}

fn experience_name(key: &str) -> &'static str {
    match key {
        "scopeAndTrajectory" => "Scope & Trajectory",
        "realTimeSyncLayer" => "Real-Time Sync Layer",
        "frontEndModernization" => "Front-End Modernization",
        "azureToAWSMigration" => "Azure-to-AWS Migration",
        "leadershipAndDelivery" => "Leadership & Delivery",
        "builtEndToEnd" => "Built End to End",
        "aiNativeWorkflow" => "AI-Native Workflow",
        "infrastructureAndSecurity" => "Infrastructure & Security",
        _ => "",
    }
}

#[derive(Props, Clone, PartialEq)]
struct ExperienceDescriptionProps {
    experience_key: &'static str,
}

#[component]
fn ExperienceDescription(props: ExperienceDescriptionProps) -> Element {
    let audio = use_audio_state();
    let transition = use_nav_with_transition();
    match props.experience_key {
        "scopeAndTrajectory" => rsx! {
            p {
                "Technical lead on a six-engineer team building third-party risk-assessment software relied on by "
                strong { class: "green-text", "tier-1 US financial institutions" }
                ". I own the team's day-to-day technical decisions, standards, and design choices, while working from both the US and Europe."
            }
            hr {}
            p {
                "Joined in 2021 and was retained through the "
                strong { class: "green-text", "2023 acquisition" }
                " by S&P Global Market Intelligence, advancing from "
                strong { class: "green-text", "Applications Engineer" }
                " through Software Engineer I and II to "
                strong { class: "green-text", "Technical Lead" }
                "."
            }
            // CareerTimeline diagram — stub until v0.1-diagrams phase
            div { class: "diagram-stub", "data-diagram": "CareerTimeline" }
        },
        "realTimeSyncLayer" => rsx! {
            p {
                "Designed a "
                strong { class: "green-text", "SignalR" }
                " WebSocket layer linking the "
                strong { class: "green-text", "React" }
                " frontend, "
                strong { class: "green-text", ".NET" }
                " backend, and "
                strong { class: "green-text", "Azure WebJobs" }
                ", so users saw live updates without refreshing the page."
            }
            hr {}
            p {
                "Wrote a custom "
                strong { class: "green-text", "\"Service-as-Client\"" }
                " API proxy authenticated with "
                strong { class: "green-text", "Okta" }
                " service-to-service tokens, which let internal services reach the WebSocket hub securely."
            }
            RealTimeSyncDiagram {}
        },
        "frontEndModernization" => rsx! {
            p {
                "Earlier in my tenure, as a Software Engineer, drove the front-end build stack from "
                strong { class: "green-text", "Node 12 to Node 20" }
                " and the UI from "
                strong { class: "green-text", "React 16 to 18" }
                ", working through the full cascade of dependency breaking changes it set off and keeping a data-heavy "
                strong { class: "green-text", "AG Grid" }
                " stable."
            }
            hr {}
            p {
                "Migrated ~350 Enzyme tests to "
                strong { class: "green-text", "React Testing Library" }
                " with "
                strong { class: "green-text", "AI-assisted refactoring" }
                "."
            }
        },
        "azureToAWSMigration" => rsx! {
            p {
                "Turned a high-level "
                strong { class: "green-text", "Azure-to-AWS" }
                " migration strategy into the engineering plan the team executed, and coordinated a temporary group of "
                strong { class: "green-text", "7 contractors" }
                " by scoping their tasks and reviewing output."
            }
            hr {}
            p {
                "Built a Python "
                strong { class: "green-text", "FastAPI" }
                " service and its Dockerfile to replace a set of 14 "
                strong { class: "green-text", "Azure Functions" }
                ", now being deployed to "
                strong { class: "green-text", "AWS ECS" }
                ", while keeping the production Azure platform stable through the transition."
            }
        },
        "leadershipAndDelivery" => rsx! {
            p { "Unblock engineers, review code, and keep engineering standards consistent across the team." }
            hr {}
            p {
                "Ran "
                strong { class: "green-text", "QA" }
                " across the team when there was no dedicated QA engineer, and took over day-to-day "
                strong { class: "green-text", "Product Owner" }
                " duties after a vacancy, running ceremonies and managing the backlog to keep delivery on track."
            }
        },
        "builtEndToEnd" => rsx! {
            p {
                "Built and run the platform, every layer from the database to the UI, plus the deploy pipeline and the agent workflow that develops it. It is "
                strong { class: "green-text", "live in production" }
                ", with real applicants using it to apply to "
                strong { class: "green-text", "INTBAU" }
                " summer-school programs."
            }
            hr {}
            p {
                "Rebuilt an early "
                strong { class: "green-text", "Supabase" }
                " prototype onto a normalized "
                strong { class: "green-text", "PostgreSQL" }
                " schema, keeping the service layers strictly separated so I can add modules without the design drifting."
            }
            LayeredArchitectureDiagram {}
        },
        "aiNativeWorkflow" => rsx! {
            p {
                "Agora runs on an AI-orchestrated workflow. Five scheduled routines do the recurring work, each a single tightly scoped unattended job:"
            }
            ul { class: "routine-list",
                li {
                    strong { class: "green-text", "Morning consolidator" }
                    ": ranks every routine's overnight output into one prioritized digest."
                }
                li {
                    strong { class: "green-text", "Daily digest" }
                    ": a single evening recap of what shipped, what opened, and what is waiting on review."
                }
                li {
                    strong { class: "green-text", "Bug triage" }
                    ": reproduces an open issue and opens at most one fix PR per night, sized to my review bandwidth."
                }
                li {
                    strong { class: "green-text", "Weekly omnibus" }
                    ": one Sunday run that merges main into the Rust port branch, classifies the week's drift by phase, attempts security bumps, and surfaces decisions outstanding for the coming week."
                }
                li {
                    strong { class: "green-text", "Overnight Rust port" }
                    ": migrates the TypeScript codebase to Rust under documented conventions, including the UI primitives layer once the port reaches the UI phases."
                }
            }
            hr {}
            p {
                "Agora's rules are mechanical, not aspirational. An "
                strong { class: "green-text", "ESLint rule" }
                " blocks database calls outside the repository layer; organization permissions are gated by server-stamped "
                strong { class: "green-text", "_can flags" }
                " the front end is lint-banned from deriving itself; "
                strong { class: "green-text", "gitleaks" }
                " scans every PR for secrets; a migration linter catches dangerous DDL; and a "
                strong { class: "green-text", "1,000+ test Jest suite" }
                " gates every merge, human or agent."
            }
            hr {}
            p {
                "The largest routine is an overnight migration of the TypeScript backend to "
                strong { class: "green-text", "Rust" }
                ", running under documented convention rules with escalation for novel architectural calls. The SeaORM entity layer, the entity-to-domain mappers, and the repositories are done; it is now working through the application-service layer, with the workers, HTTP server, and UI to follow."
            }
            hr {}
            p {
                "High-risk, one-way-door changes (auth, payments, migrations, schema) always come to me before they merge."
            }
            AgentWorkflowDiagram {}
            p {
                // Link via the page-transition hook so the AI Architecture page
                // gets the standard fade+slide-up entrance instead of a hard
                // full-page reload. onclick_only: true keeps Dioxus's Link from
                // doing the router push so the transition can sequence first.
                Link {
                    to: "/ai-architecture",
                    class: "green-text",
                    onclick_only: true,
                    onclick: move |_| {
                        if !*audio.is_muted.read() {
                            play_sound("/static/sounds/select.mp3", 0.45);
                        }
                        transition.call("/ai-architecture");
                    },
                    "See AI Architecture →"
                }
                " for the general approach behind this: the orchestration, routines, rules, governance, and front-loaded decisions I apply on every project."
            }
        },
        "infrastructureAndSecurity" => rsx! {
            p {
                "Stood up all three environments (dev, staging, prod) on "
                strong { class: "green-text", "Azure" }
                " with "
                strong { class: "green-text", "Terraform" }
                ": App Services, managed Postgres, and networking."
            }
            InfrastructureDiagram {}
            hr {}
            p {
                "Designed "
                strong { class: "green-text", "RBAC" }
                " with system- and organization-scoped permissions over multi-factor auth ("
                strong { class: "green-text", "TOTP, email OTP, passkeys" }
                "), centralized secrets in "
                strong { class: "green-text", "Azure Key Vault" }
                " per environment, and wrote secret-injection tooling that keeps local development credential-free."
            }
        },
        _ => rsx! {},
    }
}
