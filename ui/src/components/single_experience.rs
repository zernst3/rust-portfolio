use dioxus::prelude::*;

use crate::audio::play_sound;
use crate::components::LayeredArchitectureDiagram;
use crate::contexts::audio::use_audio_state;

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
            play_sound("/assets/sounds/woosh2.mp3", 0.1);
        }
        props.on_back.call(());
    };

    rsx! {
        div {
            id: "SingleExperienceOuterContainer",
            style: "animation: fade-in-kf 0.3s ease-out 0.15s forwards; opacity: 0",
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
            // RealTimeSyncDiagram — stub until v0.1-diagrams phase
            div { class: "diagram-stub", "data-diagram": "RealTimeSyncDiagram" }
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
                " service and its Dockerfile to replace a set of "
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
                "Run the codebase through scheduled agents for code review, bug triage, and dependency-vulnerability sweeps. The architecture's rules live in "
                strong { class: "green-text", "lint and CI" }
                ": layer boundaries, migration-safety checks, secret scanning, and audit-field gates."
            }
            hr {}
            p {
                "High-risk, one-way-door changes (auth, payments, migrations, schema) always come to me before they merge."
            }
            // AgentWorkflowDiagram — stub until v0.1-diagrams phase
            div { class: "diagram-stub", "data-diagram": "AgentWorkflowDiagram" }
            p {
                a {
                    href: "/ai-architecture",
                    class: "green-text",
                    "See the full AI Architecture →"
                }
                " for the complete breakdown: tools, wrappers, routines, governance, enforcement and delivery."
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
            // InfrastructureDiagram — stub until v0.1-diagrams phase
            div { class: "diagram-stub", "data-diagram": "InfrastructureDiagram" }
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
