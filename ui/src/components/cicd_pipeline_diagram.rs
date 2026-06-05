use dioxus::prelude::*;

#[component]
pub fn CICDPipelineDiagram() -> Element {
    rsx! {
        div { class: "diagram-container",
            div { class: "diagram-header",
                h3 { class: "diagram-title",
                    "CI/CD Pipeline: Git Trigger → Safety Gates → Environment"
                }
            }
            div { class: "svg-wrapper",
                svg {
                    view_box: "0 0 865 420",
                    class: "architecture-svg",
                    preserve_aspect_ratio: "xMidYMid meet",

                    defs {
                        marker {
                            id: "ci-arrow",
                            marker_width: "8",
                            marker_height: "6",
                            ref_x: "7",
                            ref_y: "3",
                            orient: "auto",
                            polygon {
                                points: "0 0, 8 3, 0 6",
                                style: "fill: rgb(var(--blue-mid))"
                            }
                        }
                    }

                    g { class: "nodes",
                        text { x: "120", y: "35", class: "column-label", text_anchor: "middle",
                            "Git Trigger"
                        }

                        g { transform: "translate(20, 55)",
                            rect { width: "200", height: "60", rx: "6", class: "node-rect trigger-node" }
                            text { x: "100", y: "26", class: "node-text mono-sm", text_anchor: "middle", dominant_baseline: "middle",
                                "push → main"
                            }
                            text { x: "100", y: "44", class: "node-subtext", text_anchor: "middle", dominant_baseline: "middle",
                                "dev"
                            }
                        }

                        g { transform: "translate(20, 180)",
                            rect { width: "200", height: "60", rx: "6", class: "node-rect trigger-node" }
                            text { x: "100", y: "26", class: "node-text mono-sm", text_anchor: "middle", dominant_baseline: "middle",
                                "push → release/v*"
                            }
                            text { x: "100", y: "44", class: "node-subtext", text_anchor: "middle", dominant_baseline: "middle",
                                "staging"
                            }
                        }

                        g { transform: "translate(20, 305)",
                            rect { width: "200", height: "60", rx: "6", class: "node-rect trigger-node" }
                            text { x: "100", y: "26", class: "node-text mono-sm", text_anchor: "middle", dominant_baseline: "middle",
                                "tag v*"
                            }
                            text { x: "100", y: "44", class: "node-subtext", text_anchor: "middle", dominant_baseline: "middle",
                                "prod"
                            }
                        }

                        text { x: "432", y: "35", class: "column-label", text_anchor: "middle",
                            "Shared CI Gates"
                        }
                        g { transform: "translate(330, 55)",
                            rect { width: "205", height: "310", rx: "8", class: "gates-rect" }
                            g { transform: "translate(20, 40)",
                                rect { width: "165", height: "55", rx: "5", class: "node-rect gate-item" }
                                text { x: "82", y: "32", class: "gate-text", text_anchor: "middle", dominant_baseline: "middle",
                                    "Migration-safety lint"
                                }
                            }
                            g { transform: "translate(20, 125)",
                                rect { width: "165", height: "55", rx: "5", class: "node-rect gate-item" }
                                text { x: "82", y: "32", class: "gate-text", text_anchor: "middle", dominant_baseline: "middle",
                                    "Secret scan"
                                }
                            }
                            g { transform: "translate(20, 210)",
                                rect { width: "165", height: "55", rx: "5", class: "node-rect gate-item" }
                                text { x: "82", y: "32", class: "gate-text", text_anchor: "middle", dominant_baseline: "middle",
                                    "Build + test"
                                }
                            }
                        }

                        text { x: "712", y: "35", class: "column-label", text_anchor: "middle",
                            "Environment"
                        }

                        g { transform: "translate(620, 55)",
                            rect { width: "185", height: "60", rx: "6", class: "node-rect env-dev" }
                            text { x: "92", y: "34", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                "Dev"
                            }
                        }

                        g { transform: "translate(620, 180)",
                            rect { width: "185", height: "60", rx: "6", class: "node-rect env-staging" }
                            text { x: "92", y: "34", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                "Staging"
                            }
                        }

                        g { transform: "translate(620, 305)",
                            rect { width: "185", height: "60", rx: "6", class: "node-rect env-prod" }
                            text { x: "92", y: "34", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                "Prod"
                            }
                        }
                    }

                    g { class: "connections",
                        path { d: "M 220 85 C 280 85, 290 120, 330 120", class: "path-line static-path", marker_end: "url(#ci-arrow)" }
                        path { d: "M 220 85 C 280 85, 290 120, 330 120", class: "path-line pulse-path path-1" }

                        path { d: "M 220 210 L 330 210", class: "path-line static-path", marker_end: "url(#ci-arrow)" }
                        path { d: "M 220 210 L 330 210", class: "path-line pulse-path path-2" }

                        path { d: "M 220 335 C 280 335, 290 300, 330 300", class: "path-line static-path", marker_end: "url(#ci-arrow)" }
                        path { d: "M 220 335 C 280 335, 290 300, 330 300", class: "path-line pulse-path path-3" }

                        path { d: "M 535 120 C 575 120, 585 85, 620 85", class: "path-line static-path", marker_end: "url(#ci-arrow)" }
                        path { d: "M 535 120 C 575 120, 585 85, 620 85", class: "path-line pulse-path path-4" }

                        path { d: "M 535 210 L 620 210", class: "path-line static-path", marker_end: "url(#ci-arrow)" }
                        path { d: "M 535 210 L 620 210", class: "path-line pulse-path path-5" }

                        path { d: "M 535 300 C 575 300, 585 335, 620 335", class: "path-line static-path", marker_end: "url(#ci-arrow)" }
                        path { d: "M 535 300 C 575 300, 585 335, 620 335", class: "path-line pulse-path path-6" }
                    }
                }
            }
        }
    }
}
