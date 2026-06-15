use dioxus::prelude::*;

/// Chorale data-flow diagram: where the live state lives (the adapter's signal)
/// and the two paths through the core (write: action → new state, read: state →
/// rendered ui). Reuses the shared diagram classes so it matches the other
/// architecture charts on the site (monospace nodes, blue strokes, pulse paths).
#[component]
pub fn ChoraleArchitectureDiagram() -> Element {
    rsx! {
        div { class: "diagram-container",
            div { class: "diagram-header",
                h3 { class: "diagram-title",
                    "Chorale: state lives in the signal · write + read paths"
                }
            }
            div { class: "svg-wrapper",
                svg {
                    view_box: "0 0 900 590",
                    class: "architecture-svg",
                    preserve_aspect_ratio: "xMidYMid meet",

                    defs {
                        marker {
                            id: "ch-arrow",
                            marker_width: "8",
                            marker_height: "8",
                            ref_x: "6",
                            ref_y: "3",
                            orient: "auto",
                            path { d: "M0,0 L7,3 L0,6 Z", style: "fill: rgb(var(--blue-mid))" }
                        }
                    }

                    // Boundary groups
                    rect { x: "64", y: "176", width: "772", height: "150", rx: "8", class: "boundary-rect" }
                    text { x: "78", y: "168", class: "boundary-title", "adapter (chorale-dioxus / leptos)" }
                    rect { x: "64", y: "392", width: "772", height: "128", rx: "8", class: "boundary-rect" }
                    text { x: "78", y: "384", class: "boundary-title", "core (chorale-core) · stateless" }

                    // ui / dom
                    rect { x: "320", y: "40", width: "260", height: "60", rx: "6", class: "node-rect" }
                    text { x: "450", y: "66", class: "node-text", text_anchor: "middle", "ui / dom" }
                    text { x: "450", y: "86", class: "node-subtext", text_anchor: "middle", "events: click, scroll, key" }

                    // useTableHandle — the live state (emphasized like the gate node)
                    rect { x: "90", y: "202", width: "320", height: "104", rx: "6", class: "node-rect gate-node" }
                    text { x: "250", y: "232", class: "node-text", text_anchor: "middle", "useTableHandle" }
                    text { x: "250", y: "254", class: "node-subtext", text_anchor: "middle", "holds Signal<TableState>" }
                    text { x: "250", y: "286", class: "gate-subtext", text_anchor: "middle", "the live state · single source of truth" }

                    // render · use_memo — derived cache
                    rect { x: "490", y: "202", width: "320", height: "104", rx: "6", class: "node-rect" }
                    text { x: "650", y: "232", class: "node-text", text_anchor: "middle", "render · use_memo" }
                    text { x: "650", y: "254", class: "node-subtext", text_anchor: "middle", "visible rows + window" }
                    text { x: "650", y: "284", class: "node-subtext", text_anchor: "middle", "derived from state · a cache" }

                    // transitions
                    rect { x: "90", y: "410", width: "320", height: "90", rx: "6", class: "node-rect" }
                    text { x: "250", y: "442", class: "node-text", text_anchor: "middle", "transitions" }
                    text { x: "250", y: "466", class: "node-subtext", text_anchor: "middle", "pure: state → new state" }

                    // views
                    rect { x: "490", y: "410", width: "320", height: "90", rx: "6", class: "node-rect" }
                    text { x: "650", y: "442", class: "node-text", text_anchor: "middle", "views" }
                    text { x: "650", y: "466", class: "node-subtext", text_anchor: "middle", "pure: state → render data" }

                    // ── write path (action → new state) ──
                    path { d: "M 415 100 L 308 200", class: "path-line static-path", marker_end: "url(#ch-arrow)" }
                    path { d: "M 415 100 L 308 200", class: "path-line pulse-path path-1" }
                    text { x: "360", y: "146", class: "path-label", "1 · user action" }

                    path { d: "M 205 306 L 205 406", class: "path-line static-path", marker_end: "url(#ch-arrow)" }
                    path { d: "M 205 306 L 205 406", class: "path-line pulse-path path-2" }
                    text { x: "150", y: "364", class: "path-label", "2 · state in" }

                    path { d: "M 285 408 L 285 310", class: "path-line static-path", marker_end: "url(#ch-arrow)" }
                    path { d: "M 285 408 L 285 310", class: "path-line pulse-path path-3" }
                    text { x: "362", y: "356", class: "path-label", "3 · new state" }
                    text { x: "362", y: "372", class: "path-label", "→ signal.set" }

                    // ── read path (state → rendered ui) ──
                    path { d: "M 410 252 L 488 252", class: "path-line static-path", marker_end: "url(#ch-arrow)" }
                    path { d: "M 410 252 L 488 252", class: "path-line pulse-path path-4" }
                    text { x: "449", y: "240", class: "path-label", "4 · read state" }

                    path { d: "M 605 306 L 605 406", class: "path-line static-path", marker_end: "url(#ch-arrow)" }
                    path { d: "M 605 306 L 605 406", class: "path-line pulse-path path-5" }
                    text { x: "562", y: "364", class: "path-label", "5 · read" }

                    path { d: "M 685 408 L 685 310", class: "path-line static-path", marker_end: "url(#ch-arrow)" }
                    path { d: "M 685 408 L 685 310", class: "path-line pulse-path path-1" }
                    text { x: "722", y: "364", class: "path-label", "6 · render data" }

                    path { d: "M 560 200 L 528 100", class: "path-line static-path", marker_end: "url(#ch-arrow)" }
                    path { d: "M 560 200 L 528 100", class: "path-line pulse-path path-2" }
                    text { x: "566", y: "146", class: "path-label", "7 · draw" }

                    // legend
                    text { x: "286", y: "556", class: "path-label", "1-3  write path: action → new state" }
                    text { x: "622", y: "556", class: "path-label", "4-7  read path: state → rendered ui" }
                }
            }
        }
    }
}
