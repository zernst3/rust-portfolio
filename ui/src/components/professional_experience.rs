use dioxus::prelude::*;

use crate::audio::play_sound;
use crate::components::single_experience::SingleExperience;
use crate::components::Close;
use crate::contexts::audio::use_audio_state;

// All picker item keys in display order.
const SP_ITEMS: &[&str] = &[
    "scopeAndTrajectory",
    "realTimeSyncLayer",
    "azureToAWSMigration",
    "leadershipAndDelivery",
    "frontEndModernization",
];
const AGORA_ITEMS: &[&str] = &[
    "builtEndToEnd",
    "aiNativeWorkflow",
    "infrastructureAndSecurity",
];

fn item_label(key: &str) -> &'static str {
    match key {
        "credentials" => "Education & Certifications",
        "scopeAndTrajectory" => "Scope & Trajectory",
        "realTimeSyncLayer" => "Real-Time Sync Layer",
        "azureToAWSMigration" => "Azure-to-AWS Migration",
        "leadershipAndDelivery" => "Leadership & Delivery",
        "frontEndModernization" => "Front-End Modernization",
        "builtEndToEnd" => "Built End to End",
        "aiNativeWorkflow" => "AI-Native Workflow",
        "infrastructureAndSecurity" => "Infrastructure & Security",
        "chorale" => "Virtualized Tables for Rust",
        "camerata" => "Guardrails AI agents actually follow",
        _ => "",
    }
}

#[component]
pub fn ProfessionalExperience() -> Element {
    let audio = use_audio_state();

    let mut current: Signal<Option<&'static str>> = use_signal(|| None);
    let mut hovered: Signal<Option<&'static str>> = use_signal(|| None);
    // Sliding bar position relative to the picker-scroll-container.
    let mut bar_top: Signal<f64> = use_signal(|| 0.0);
    let mut bar_height: Signal<f64> = use_signal(|| 0.0);
    let mut bar_visible: Signal<bool> = use_signal(|| false);

    // Auto-select "scopeAndTrajectory" on desktop on mount.
    //
    // WHY use_effect + spawn instead of use_future:
    // use_future's closure is replaced by use_callback on every re-render, and
    // use_hook_did_run calls task.resume() on every render. While resume() on a
    // finished task is a no-op, use_future captures `current` in its closure which
    // can subscribe to reactive tracking if Dioxus ever changes that behavior.
    // use_effect with no reactive reads in its body runs ONCE on mount and never
    // re-runs (same fix pattern as home.rs subtitle rotation, PORT-CC-1).
    use_effect(move || {
        spawn(async move {
            let eval = document::eval("dioxus.send(window.innerWidth > 1050)");
            if let Ok(v) = eval.await {
                if v.as_bool().unwrap_or(false) {
                    current.set(Some("scopeAndTrajectory"));
                }
            }
        });
    });

    let is_selected = current.read().is_some();

    // Measure the picker item position via JS and update the sliding bar.
    let measure_bar = move |key: &'static str| {
        spawn(async move {
            let js = format!(
                r#"
                const el = document.querySelector('[data-exp-key="{key}"]');
                const parent = el ? el.closest('.picker-scroll-container') : null;
                if (!el || !parent) {{ dioxus.send(null); return; }}
                const er = el.getBoundingClientRect();
                const pr = parent.getBoundingClientRect();
                dioxus.send({{ top: er.top - pr.top + parent.scrollTop, height: er.height }});
                "#
            );
            let eval = document::eval(&js);
            if let Ok(val) = eval.await {
                if let (Some(top), Some(height)) = (
                    val.get("top").and_then(|v| v.as_f64()),
                    val.get("height").and_then(|v| v.as_f64()),
                ) {
                    bar_top.set(top);
                    bar_height.set(height);
                    bar_visible.set(true);
                } else {
                    bar_visible.set(false);
                }
            }
        });
    };

    let bar_style = if *bar_visible.read() {
        format!(
            "top: {}px; height: {}px; opacity: 1; transition: top 0.18s cubic-bezier(0.4,0,0.2,1), height 0.18s ease, opacity 0.2s ease",
            bar_top.read(),
            bar_height.read()
        )
    } else {
        "opacity: 0".to_string()
    };

    rsx! {
        div { class: "page-enter",
            div {
                id: "ProfessionalExperienceContainer",
                class: if is_selected { "experience-selected" } else { "" },

                div { id: "ProfessionalExperienceInnerContainer",
                    h1 { "Work" }

                    if !is_selected {
                        p { class: "empty-preview-prompt empty-preview-prompt-mobile",
                            "Select an item to preview details"
                        }
                    }

                    div { id: "ProfessionalExperienceContent",
                        div { class: "picker",
                            div {
                                class: "picker-scroll-container",
                                onmouseleave: move |_| {
                                    hovered.set(None);
                                    let target = *current.read();
                                    if let Some(key) = target {
                                        measure_bar(key);
                                    } else {
                                        bar_visible.set(false);
                                    }
                                },

                                // Sliding highlight bar
                                div { class: "picker-highlight", style: "{bar_style}" }

                                // ─── Bucket 1: Credentials ───────────────────
                                div { class: "bucket-header", "Credentials" }

                                div { class: "experience-group",
                                    PickerItem {
                                        exp_key: "credentials",
                                        current: current,
                                        on_select: move |key| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/select.mp3", 0.45);
                                            }
                                            current.set(Some(key));
                                            measure_bar(key);
                                        },
                                        on_hover: move |key| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/woosh3.mp3", 0.25);
                                            }
                                            hovered.set(Some(key));
                                            measure_bar(key);
                                        },
                                    }
                                }

                                // ─── S&P Global ───────────────────────────────
                                div { class: "experience-group",
                                    div { class: "experience-header-picker",
                                        div { class: "header-top",
                                            span { class: "entity-name",
                                                img { src: "/static/images/finance.svg", alt: "", class: "header-icon-svg" }
                                                " S&P Global Market Intelligence"
                                            }
                                            span { class: "dots" }
                                            span { class: "date", "Feb 2021 – Present" }
                                        }
                                        div { class: "header-bottom",
                                            span { class: "title green-text", "Technical Lead" }
                                            span { class: "location", "Remote, USA & EMEA" }
                                        }
                                    }
                                    {SP_ITEMS.iter().map(|key| rsx! {
                                        PickerItem {
                                            key: "{key}",
                                            exp_key: key,
                                            current: current,
                                            on_select: move |k| {
                                                if !*audio.is_muted.read() {
                                                    play_sound("/static/sounds/select.mp3", 0.45);
                                                }
                                                current.set(Some(k));
                                                measure_bar(k);
                                            },
                                            on_hover: move |k| {
                                                if !*audio.is_muted.read() {
                                                    play_sound("/static/sounds/woosh3.mp3", 0.25);
                                                }
                                                hovered.set(Some(k));
                                                measure_bar(k);
                                            },
                                        }
                                    })}
                                }

                                // ─── Bucket 2: Independent Work ───────────────
                                div { class: "bucket-header", "Independent Work" }

                                // The New Agora
                                div { class: "experience-group",
                                    div { class: "experience-header-picker",
                                        div { class: "header-top",
                                            span { class: "entity-name",
                                                img { src: "/static/images/city.svg", alt: "", class: "header-icon-svg" }
                                                " The New Agora (Independent Project)"
                                            }
                                            span { class: "dots" }
                                            span { class: "date", "Nov 2025 – Present" }
                                        }
                                        div { class: "header-bottom",
                                            span { class: "title green-text", "Creator & Lead Engineer" }
                                            span { class: "location", "Remote / Global" }
                                        }
                                        a {
                                            class: "live-link",
                                            href: "https://agora.new",
                                            target: "_blank",
                                            rel: "noreferrer",
                                            "Visit agora.new ↗"
                                        }
                                    }
                                    {AGORA_ITEMS.iter().map(|key| rsx! {
                                        PickerItem {
                                            key: "{key}",
                                            exp_key: key,
                                            current: current,
                                            on_select: move |k| {
                                                if !*audio.is_muted.read() {
                                                    play_sound("/static/sounds/select.mp3", 0.45);
                                                }
                                                current.set(Some(k));
                                                measure_bar(k);
                                            },
                                            on_hover: move |k| {
                                                if !*audio.is_muted.read() {
                                                    play_sound("/static/sounds/woosh3.mp3", 0.25);
                                                }
                                                hovered.set(Some(k));
                                                measure_bar(k);
                                            },
                                        }
                                    })}
                                }

                                // Chorale
                                div { class: "experience-group",
                                    div { class: "experience-header-picker",
                                        div { class: "header-top",
                                            span { class: "entity-name", "Chorale" }
                                            span { class: "dots" }
                                            span { class: "date", "Rust · Dioxus · crates.io" }
                                        }
                                    }
                                    PickerItem {
                                        exp_key: "chorale",
                                        current: current,
                                        on_select: move |k| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/select.mp3", 0.45);
                                            }
                                            current.set(Some(k));
                                            measure_bar(k);
                                        },
                                        on_hover: move |k| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/woosh3.mp3", 0.25);
                                            }
                                            hovered.set(Some(k));
                                            measure_bar(k);
                                        },
                                    }
                                }

                                // Camerata
                                div { class: "experience-group",
                                    div { class: "experience-header-picker",
                                        div { class: "header-top",
                                            span { class: "entity-name", "Camerata" }
                                            span { class: "dots" }
                                            span { class: "date", "v0.1.0 · May 2026" }
                                        }
                                    }
                                    PickerItem {
                                        exp_key: "camerata",
                                        current: current,
                                        on_select: move |k| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/select.mp3", 0.45);
                                            }
                                            current.set(Some(k));
                                            measure_bar(k);
                                        },
                                        on_hover: move |k| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/woosh3.mp3", 0.25);
                                            }
                                            hovered.set(Some(k));
                                            measure_bar(k);
                                        },
                                    }
                                }
                            }
                        }

                        // ─── Preview panel ────────────────────────────────────
                        div { class: "preview",
                            {match *current.read() {
                                None => rsx! {
                                    p { class: "empty-preview-prompt empty-preview-prompt-desktop",
                                        "Select an item to preview details"
                                    }
                                },
                                Some("credentials") => rsx! {
                                    CredentialsPreview {
                                        on_back: move |_| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/woosh2.mp3", 0.1);
                                            }
                                            current.set(None);
                                            bar_visible.set(false);
                                        },
                                    }
                                },
                                Some("chorale") => rsx! {
                                    ChoralePreview {
                                        on_back: move |_| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/woosh2.mp3", 0.1);
                                            }
                                            current.set(None);
                                            bar_visible.set(false);
                                        },
                                        on_link_hover: move |_| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/woosh3.mp3", 0.25);
                                            }
                                        },
                                        on_link_click: move |_| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/select.mp3", 0.45);
                                            }
                                        },
                                    }
                                },
                                Some("camerata") => rsx! {
                                    CamerataPreview {
                                        on_back: move |_| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/woosh2.mp3", 0.1);
                                            }
                                            current.set(None);
                                            bar_visible.set(false);
                                        },
                                        on_link_hover: move |_| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/woosh3.mp3", 0.25);
                                            }
                                        },
                                        on_link_click: move |_| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/select.mp3", 0.45);
                                            }
                                        },
                                    }
                                },
                                Some(key) => rsx! {
                                    SingleExperience {
                                        experience_key: key,
                                        on_back: move |_| {
                                            current.set(None);
                                            bar_visible.set(false);
                                        },
                                    }
                                },
                            }}
                        }
                    }
                }
                Close {}
            }
        }
    }
}

// ─── Picker item sub-component ────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct PickerItemProps {
    exp_key: &'static str,
    current: Signal<Option<&'static str>>,
    on_select: EventHandler<&'static str>,
    on_hover: EventHandler<&'static str>,
}

#[component]
fn PickerItem(props: PickerItemProps) -> Element {
    let key = props.exp_key;
    let is_active = *props.current.read() == Some(key);
    let class = if is_active {
        "center experienceActive"
    } else {
        "center"
    };

    rsx! {
        div {
            class: "{class}",
            style: "cursor: pointer; padding: 15px 10px; position: relative; z-index: 1;",
            "data-exp-key": "{key}",
            onclick: move |_| props.on_select.call(key),
            onmouseenter: move |_| props.on_hover.call(key),
            h3 { style: "text-align: center", "{item_label(key)}" }
        }
    }
}

// ─── Custom preview panels ────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct CredentialsPreviewProps {
    on_back: EventHandler<()>,
}

#[component]
fn CredentialsPreview(props: CredentialsPreviewProps) -> Element {
    rsx! {
        div { class: "custom-preview credentials-preview",
            h2 { "Education & Certifications" }
            p { class: "custom-preview-lede",
                "United States Citizen, based in New York City, fully remote available."
            }
            h3 { class: "custom-preview-h3", "Microsoft" }
            p { class: "custom-preview-h3-date", "2022 to 2024" }
            p {
                "Azure Developer Associate (AZ-204), December 2024. "
                a {
                    href: "https://learn.microsoft.com/api/credentials/share/en-us/ZacharyErnst-2786/3164FF9B17DAE905?sharingId",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Verify ↗"
                }
            }
            p {
                "Azure Fundamentals (AZ-900), June 2022. "
                a {
                    href: "https://www.credly.com/badges/16c63be7-621b-489f-a475-0a740c997b28?source=linked_in_profile",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Verify ↗"
                }
            }
            h3 { class: "custom-preview-h3", "FullStack Academy" }
            p { class: "custom-preview-h3-date", "2020" }
            p { "Web Development Certificate, New York, NY." }
            h3 { class: "custom-preview-h3", "Rutgers University" }
            p { class: "custom-preview-h3-date", "2015 to 2018" }
            p { "Bachelor of Science in Public and Non-Profit Administration, Summa Cum Laude." }
            div { class: "mobile custom-preview-back",
                button { onclick: move |_| props.on_back.call(()), "Back" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ChoralePreviewProps {
    on_back: EventHandler<()>,
    on_link_hover: EventHandler<()>,
    on_link_click: EventHandler<()>,
}

#[component]
fn ChoralePreview(props: ChoralePreviewProps) -> Element {
    rsx! {
        div { class: "custom-preview chorale-preview",
            h2 { "Chorale" }
            div { class: "custom-preview-meta",
                span { class: "custom-preview-tag", "Open Source" }
                span { class: "custom-preview-dot", "·" }
                span { class: "custom-preview-date", "Published to crates.io" }
            }
            img {
                src: "/static/images/chorale-qa-harness.png",
                alt: "Chorale QA harness showing a 10,010-row data table with sorting, filtering, virtualization, selection, column visibility, CSV export, and column resize enabled",
                class: "custom-preview-image",
            }
            p {
                "A headless, virtualized table library for "
                strong { class: "green-text", "Rust and Dioxus" }
                ", published to crates.io."
            }
            img {
                src: "/static/images/chorale-custom-cells.png",
                alt: "Chorale custom cell rendering example with Service, Status, and Health columns, showing badge variants and arbitrary Dioxus markup keyed by column id",
                class: "custom-preview-image",
            }
            p {
                "I orchestrated it with AI end to end in an unfamiliar language, held to a production bar: "
                strong { class: "green-text", "clippy-pedantic-clean" }
                ", fully tested (including "
                strong { class: "green-text", "137 passing unit tests" }
                "), unsafe forbidden. Built to demonstrate that AI orchestration can hold a real quality standard, not just move fast."
            }
            a {
                class: "custom-preview-cta",
                href: "https://github.com/zernst3/rust-chorale",
                target: "_blank",
                rel: "noopener noreferrer",
                onmouseenter: move |_| props.on_link_hover.call(()),
                onclick: move |_| props.on_link_click.call(()),
                "View on GitHub ↗"
            }
            a {
                class: "custom-preview-cta",
                href: "https://crates.io/crates/chorale-core",
                target: "_blank",
                rel: "noopener noreferrer",
                onmouseenter: move |_| props.on_link_hover.call(()),
                onclick: move |_| props.on_link_click.call(()),
                style: "margin-left: 1.5em",
                "View on crates.io ↗"
            }
            div { class: "mobile custom-preview-back",
                button { onclick: move |_| props.on_back.call(()), "Back" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CamerataPreviewProps {
    on_back: EventHandler<()>,
    on_link_hover: EventHandler<()>,
    on_link_click: EventHandler<()>,
}

#[component]
fn CamerataPreview(props: CamerataPreviewProps) -> Element {
    rsx! {
        div { class: "custom-preview camerata-preview",
            h2 { "Camerata" }
            div { class: "custom-preview-meta",
                span { class: "custom-preview-tag", "Open Source" }
                span { class: "custom-preview-dot", "·" }
                span { class: "custom-preview-date", "v0.1.0, May 2026" }
            }
            img {
                src: "/static/images/camerata-overview.png",
                alt: "Camerata desktop GUI showing the principle selection view, with domains and rules listed for adoption",
                class: "custom-preview-image",
            }
            p {
                "A personal project that codifies the architectural approach I use, packaged as a "
                strong { class: "green-text", "Rust CLI and Dioxus GUI" }
                ". It generates machine-enforceable AI-agent config (AGENTS.md, CONVENTIONS.md) from ~95 starter principles, each a TOML rule with explicit alternatives and a test for what qualifies."
            }
            img {
                src: "/static/images/camerata-principle.png",
                alt: "Camerata desktop GUI showing a single principle's detail view with summary, why, and adoption options",
                class: "custom-preview-image",
            }
            p { "Built to demonstrate how I think about standards-as-code, not as a maintained product." }
            a {
                class: "custom-preview-cta",
                href: "https://github.com/zernst3/camerata-ai",
                target: "_blank",
                rel: "noopener noreferrer",
                onmouseenter: move |_| props.on_link_hover.call(()),
                onclick: move |_| props.on_link_click.call(()),
                "View on GitHub ↗"
            }
            div { class: "mobile custom-preview-back",
                button { onclick: move |_| props.on_back.call(()), "Back" }
            }
        }
    }
}
