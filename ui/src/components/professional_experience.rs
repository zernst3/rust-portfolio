use dioxus::prelude::*;

use crate::audio::play_sound;
use crate::components::single_experience::SingleExperience;
use crate::components::Close;
use crate::components::ChoraleArchitectureDiagram;
use crate::contexts::audio::use_audio_state;

// All picker item keys in display order.
const SP_ITEMS: &[&str] = &[
    "scopeAndTrajectory",
    "aiNativeDeliveryTooling",
    "pythonServices",
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

// Chorale v0.1.0 → v0.2.* capability comparison, sourced from the chorale
// CHANGELOG (through v0.2.2, 2026-06-14). "✓" = included, "—" = not in that
// version, text = a qualified capability that grew between versions. The right
// column spans the whole 0.2.x line, so capabilities added in 0.2.1/0.2.2 (no
// new features landed in 0.2.1; row-set mutation and full keyboard navigation
// landed in 0.2.2) show "✓" there.
const CHORALE_FEATURES: &[(&str, &str, &str)] = &[
    ("Headless core + Dioxus", "✓", "✓"),
    ("Row virtualization", "Fixed height", "Fixed + variable"),
    ("Sorting", "Single column", "Multi-column"),
    ("Column filtering", "✓", "✓"),
    ("Pagination", "Pages", "Pages + infinite scroll"),
    ("Row selection", "✓", "✓"),
    ("Column show / hide", "✓", "✓"),
    ("Column resize", "✓", "✓"),
    ("CSV export", "✓", "✓"),
    ("Excel (XLSX) export", "—", "✓"),
    ("Column reorder", "—", "✓"),
    ("Frozen columns", "—", "✓"),
    ("In-cell editing", "—", "✓"),
    ("Range selection, fill & clipboard", "—", "✓"),
    ("Grouping + aggregation", "—", "✓"),
    ("Master / detail rows", "—", "✓"),
    ("Row-aware cell renderers", "—", "✓"),
    ("i18n / custom labels", "—", "✓"),
    ("Out-of-the-box light / dark themes", "—", "✓"),
    ("Leptos bindings", "—", "✓"),
    ("Derive macro (TableRow)", "—", "✓"),
    ("Row-set mutation (live data)", "—", "✓"),
    ("Keyboard navigation (incl. master/detail)", "—", "✓"),
];

/// Cell class for a feature-table value: accent for included, muted for absent,
/// neutral for a qualified text value.
fn feature_cell_class(value: &str) -> &'static str {
    match value {
        "✓" => "ft-yes",
        "—" => "ft-no",
        _ => "ft-val",
    }
}

fn item_label(key: &str) -> &'static str {
    match key {
        "credentials" => "Education & Certifications",
        "scopeAndTrajectory" => "Scope & Trajectory",
        "aiNativeDeliveryTooling" => "AI-Native Delivery Tooling",
        "pythonServices" => "Python Services",
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
    // Preview entrance: toggled off->on around each selection to force a fresh
    // mount, so the CSS fade-in (.preview-anim) restarts from zero every open.
    let mut preview_shown: Signal<bool> = use_signal(|| true);
    // Detail exit: while true, the preview container carries `.detail-leaving`
    // so it plays the fade+slide-down exit before `current` is cleared.
    let mut is_closing: Signal<bool> = use_signal(|| false);

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
                    current.set(Some("builtEndToEnd"));
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
            // Read the measured rect via recv(): in dioxus 0.7.9 `.await`/join()
            // returns the JS *return value*, not the `dioxus.send()` payload —
            // so the bar never became visible. recv() reads the sent value.
            let mut eval = document::eval(&js);
            if let Ok(val) = eval.recv::<serde_json::Value>().await {
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

    // Open a sub-experience: sound, select, slide the bar, and replay the
    // entrance animation. We briefly unmount the preview body (preview_shown
    // false -> true across a paint) so the .preview-anim keyframe restarts from
    // zero on every open. A real remount, not a transition from the current
    // opacity (which only produced an invisible flicker).
    let open_experience = use_callback(move |key: &'static str| {
        if !*audio.is_muted.read() {
            play_sound("/static/sounds/select.mp3", 0.45);
        }
        // Slide the picker highlight bar to the new item right away.
        measure_bar(key);

        // Desktop has no Back button — you switch directly between experiences.
        // So if one is ALREADY open and we're moving to a DIFFERENT one, play
        // the outgoing exit (`.detail-leaving`) first, then swap and replay the
        // entrance. First open (nothing showing) skips straight to the entrance.
        let switching = matches!(*current.peek(), Some(cur) if cur != key);
        let mut shown = preview_shown;
        // One paint with the body unmounted, then remount, so the entrance
        // keyframe restarts from zero.
        let restart_entrance = "await new Promise(function(r){requestAnimationFrame(function(){requestAnimationFrame(r);});});";
        if switching {
            is_closing.set(true);
            spawn(async move {
                // Outgoing exit — matches `.detail-leaving` (0.2s).
                document::eval("await new Promise(function(r){setTimeout(r,200);});")
                    .await
                    .ok();
                is_closing.set(false);
                current.set(Some(key));
                shown.set(false);
                document::eval(restart_entrance).await.ok();
                shown.set(true);
            });
        } else {
            current.set(Some(key));
            preview_shown.set(false);
            spawn(async move {
                document::eval(restart_entrance).await.ok();
                shown.set(true);
            });
        }
    });

    // Close the open experience with an exit animation: play the back sound,
    // flag `.detail-leaving` (fade + slide-down via transitions.css), wait the
    // 200ms exit, THEN clear the selection so the detail animates out instead of
    // vanishing. Centralizes the old inline `current.set(None)` that every Back
    // handler used to do directly.
    let close_experience = use_callback(move |_: ()| {
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
            current.set(None);
            bar_visible.set(false);
            is_closing.set(false);
        });
    });

    let preview_shown_val = *preview_shown.read();

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

                    // Always mounted (not gated on `is_selected`) so it fades
                    // out with the picker/h1 via the opacity transition below
                    // instead of unmounting instantly — which collapsed its
                    // space and made the page "jump" before the modal slid in.
                    // CSS fades it to opacity 0 under `.experience-selected`.
                    p { class: "empty-preview-prompt empty-preview-prompt-mobile",
                        "Select an item to preview details"
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

                                // ─── Independent Work (lead) ──────────────────
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
                                            on_select: move |k| open_experience.call(k),
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

                                // Camerata
                                div { class: "experience-group",
                                    div { class: "experience-header-picker",
                                        div { class: "header-top",
                                            span { class: "entity-name", "Camerata" }
                                            span { class: "dots" }
                                            span { class: "date", "Rust · governance engine" }
                                        }
                                    }
                                    PickerItem {
                                        exp_key: "camerata",
                                        current: current,
                                        on_select: move |k| open_experience.call(k),
                                        on_hover: move |k| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/woosh3.mp3", 0.25);
                                            }
                                            hovered.set(Some(k));
                                            measure_bar(k);
                                        },
                                    }
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
                                        on_select: move |k| open_experience.call(k),
                                        on_hover: move |k| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/woosh3.mp3", 0.25);
                                            }
                                            hovered.set(Some(k));
                                            measure_bar(k);
                                        },
                                    }
                                }

                                // ─── Professional Experience ──────────────────
                                div { class: "bucket-header", "Professional Experience" }

                                // ─── S&P Global (professional, supporting) ────
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
                                            on_select: move |k| open_experience.call(k),
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

                                // ─── Credentials ──────────────────────────────
                                div { class: "bucket-header", "Credentials" }

                                div { class: "experience-group",
                                    PickerItem {
                                        exp_key: "credentials",
                                        current: current,
                                        on_select: move |key| open_experience.call(key),
                                        on_hover: move |key| {
                                            if !*audio.is_muted.read() {
                                                play_sound("/static/sounds/woosh3.mp3", 0.25);
                                            }
                                            hovered.set(Some(key));
                                            measure_bar(key);
                                        },
                                    }
                                }
                            }
                        }

                        // ─── Preview panel ────────────────────────────────────
                        div {
                            class: "preview",
                            if preview_shown_val {
                            div {
                            class: if *is_closing.read() { "preview-anim detail-leaving" } else { "preview-anim" },
                            {match *current.read() {
                                None => rsx! {
                                    p { class: "empty-preview-prompt empty-preview-prompt-desktop",
                                        "Select an item to preview details"
                                    }
                                },
                                Some("credentials") => rsx! {
                                    CredentialsPreview {
                                        on_back: move |_| close_experience.call(()),
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
                                Some("chorale") => rsx! {
                                    ChoralePreview {
                                        on_back: move |_| close_experience.call(()),
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
                                        on_back: move |_| close_experience.call(()),
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
                                        on_back: move |_| close_experience.call(()),
                                    }
                                },
                            }}
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
    on_link_hover: EventHandler<()>,
    on_link_click: EventHandler<()>,
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
                    onmouseenter: move |_| props.on_link_hover.call(()),
                    onclick: move |_| props.on_link_click.call(()),
                    "Verify ↗"
                }
            }
            p {
                "Azure Fundamentals (AZ-900), June 2022. "
                a {
                    href: "https://www.credly.com/badges/16c63be7-621b-489f-a475-0a740c997b28?source=linked_in_profile",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    onmouseenter: move |_| props.on_link_hover.call(()),
                    onclick: move |_| props.on_link_click.call(()),
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
                span { class: "custom-preview-date", "v0.2.2 on crates.io" }
            }
            // v0.2.2 status snippet + live example.
            div { class: "chorale-status",
                p {
                    "v0.2.2 is "
                    strong { class: "green-text", "released on crates.io" }
                    ", now with "
                    strong { class: "green-text", "429 passing tests" }
                    ". The latest release adds full keyboard navigation for "
                    "master/detail child tables and a row-set mutation API "
                    "(append, insert, remove, reset) for live data."
                }
                a {
                    class: "custom-preview-cta",
                    href: "https://zernst3.github.io/rust-chorale/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    onmouseenter: move |_| props.on_link_hover.call(()),
                    onclick: move |_| props.on_link_click.call(()),
                    "Try the live example ↗"
                }
            }
            img {
                src: "/static/images/chorale-v0.2.0-dioxus-example.png",
                alt: "Chorale v0.2.0 rendered with the Dioxus adapter, showing a data table with sorting, filtering, virtualization, selection, column visibility, and export",
                class: "custom-preview-image",
            }
            p {
                "A headless, virtualized table library for "
                strong { class: "green-text", "Rust, with Dioxus and Leptos adapters" }
                ", published to crates.io."
            }
            img {
                src: "/static/images/chorale-v0.2.0-leptos-example.png",
                alt: "Chorale v0.2.0 rendered with the Leptos adapter, demonstrating the same headless core driving a second Rust UI framework",
                class: "custom-preview-image",
            }
            p {
                "I orchestrated it with AI end to end in an unfamiliar language, held to a production bar: "
                strong { class: "green-text", "clippy-pedantic-clean" }
                ", fully tested, unsafe forbidden. Built to demonstrate that AI orchestration can hold a real quality standard, not just move fast."
            }
            // Architecture diagram: where state lives + the write/read paths.
            h3 { class: "custom-preview-h3", "How it works" }
            ChoraleArchitectureDiagram {}
            // v0.1.0 vs v0.2.* capability comparison.
            h3 { class: "custom-preview-h3", "v0.1.0 vs v0.2.*" }
            table { class: "feature-table",
                thead {
                    tr {
                        th { "Capability" }
                        th { "v0.1.0" }
                        th { "v0.2.*" }
                    }
                }
                tbody {
                    {CHORALE_FEATURES.iter().map(|(feature, v01, v02)| rsx! {
                        tr { key: "{feature}",
                            td { class: "ft-feature", "{feature}" }
                            td { class: feature_cell_class(v01), "{v01}" }
                            td { class: feature_cell_class(v02), "{v02}" }
                        }
                    })}
                }
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
                span { class: "custom-preview-tag", "Governance Engine" }
                span { class: "custom-preview-dot", "·" }
                span { class: "custom-preview-date", "Rust" }
            }
            p {
                "I set out to build Camerata because managing multiple AI agents across multiple projects through conversational chat alone became unwieldy, and frankly cognitively tiring. The governance itself worked: my rules held and the agent's code quality stayed high. But routines would fail silently, and I often wouldn't know a run had failed until I went and asked the agent about it. The missing piece was never the governance, it is already baked into my process. What was missing was a structured place for it to live: status and state visible and managed in a real interface, instead of buried in chat. That is what Camerata is: a structured management layer for orchestrating AI agents across any number of projects, while enforcing rules and deterministic gates to maintain code quality."
            }
            img {
                src: "/static/images/camerata-proposed-ruleset.png",
                alt: "Camerata's proposed starter ruleset: the detected stack and suggested rules grouped by domain, each with a scope and a gate placement, ready to adopt or swap",
                class: "custom-preview-image",
            }
            p {
                "The brownfield audit is the working core. It runs a deterministic security floor (hardcoded secrets, raw SQL, secrets in URLs) alongside an AI architectural review, and you choose the model tier and scan mode with the cost shown before you run."
            }
            img {
                src: "/static/images/camerata-audit-run.png",
                alt: "Camerata's audit screen: an audit-model and calibration-model selector, a scan-mode selector, a live cost estimate, and parallel agent batches reporting progress",
                class: "custom-preview-image",
            }
            p {
                "Every finding lands in one table with its severity, its authority (deterministic and enforced, or advisory and review-only), and a disposition flow: fix it, waive it with a recorded reason, or accept it as tracked tech debt."
            }
            img {
                src: "/static/images/camerata-findings-table.png",
                alt: "Camerata's findings table, grouped by rule then file then line, with severity, authority, and enforcement columns and a toolbar for ignoring, resolving, or accepting findings",
                class: "custom-preview-image",
            }
            p {
                "Each finding explains itself against the actual code: the rule it broke, what the code does, and every other rule it violates at the same location."
            }
            img {
                src: "/static/images/camerata-finding-explanation.png",
                alt: "A Camerata finding detail: the violated rule, a code-grounded explanation of the layering violation, and the other rule it also violates at that location",
                class: "custom-preview-image",
            }
            img {
                src: "/static/images/camerata-finding-in-context.png",
                alt: "A Camerata finding explanation shown over the full findings list, with the deterministic security findings pinned at the top",
                class: "custom-preview-image",
            }
            p {
                "The architecture extends this toward development-time governance: a "
                strong { class: "green-text", "deny-before-execute MCP gateway" }
                " designed to block a disallowed agent action before it reaches disk, with a post-task structural check that bounces violations back for revision."
            }
            a {
                class: "custom-preview-cta",
                href: "https://github.com/zernst3/camerata-orchestrator",
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
