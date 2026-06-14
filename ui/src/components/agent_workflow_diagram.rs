use dioxus::prelude::*;

#[component]
pub fn AgentWorkflowDiagram() -> Element {
    rsx! {
        div { class: "diagram-container",
            div { class: "diagram-header",
                h3 { class: "diagram-title",
                    "Agent-First Workflow: Autonomous Agents → Human Gate → main"
                }
            }
            div { class: "svg-wrapper",
                svg {
                    view_box: "0 0 865 470",
                    class: "architecture-svg",
                    preserve_aspect_ratio: "xMidYMid meet",

                    defs {
                        marker {
                            id: "aw-arrow",
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
                        g { class: "boundary-group",
                            rect { x: "0", y: "20", width: "300", height: "430", rx: "10", class: "boundary-rect" }
                            text { x: "150", y: "42", class: "boundary-title", text_anchor: "middle",
                                "Scheduled Autonomous Agents"
                            }
                        }

                        g { transform: "translate(20, 65)",
                            rect { width: "260", height: "70", rx: "6", class: "node-rect" }
                            text { x: "130", y: "30", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                "Code Review"
                            }
                            text { x: "130", y: "50", class: "node-subtext", text_anchor: "middle", dominant_baseline: "middle",
                                "Diff analysis & comments"
                            }
                        }

                        g { transform: "translate(20, 160)",
                            rect { width: "260", height: "70", rx: "6", class: "node-rect" }
                            text { x: "130", y: "30", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                "Bug Triage & Fix PRs"
                            }
                            text { x: "130", y: "50", class: "node-subtext", text_anchor: "middle", dominant_baseline: "middle",
                                "Reproduce → patch → PR"
                            }
                        }

                        g { transform: "translate(20, 255)",
                            rect { width: "260", height: "70", rx: "6", class: "node-rect" }
                            text { x: "130", y: "30", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                "Dependency Security Sweep"
                            }
                            text { x: "130", y: "50", class: "node-subtext", text_anchor: "middle", dominant_baseline: "middle",
                                "CVE scan & bumps"
                            }
                        }

                        g { transform: "translate(20, 350)",
                            rect { width: "260", height: "70", rx: "6", class: "node-rect" }
                            text { x: "130", y: "30", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                "Architectural Drift Detection"
                            }
                            text { x: "130", y: "50", class: "node-subtext", text_anchor: "middle", dominant_baseline: "middle",
                                "Layer-boundary audit"
                            }
                        }

                        g { transform: "translate(390, 195)",
                            rect { width: "160", height: "80", rx: "6", class: "node-rect hub-node" }
                            text { x: "80", y: "35", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                "Pull Requests"
                            }
                            text { x: "80", y: "55", class: "node-subtext", text_anchor: "middle", dominant_baseline: "middle",
                                "+ impl. plans"
                            }
                        }

                        g { transform: "translate(620, 195)",
                            rect { width: "160", height: "80", rx: "6", class: "node-rect gate-node" }
                            text { x: "80", y: "35", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                "Human Review"
                            }
                            text { x: "80", y: "55", class: "node-subtext gate-subtext", text_anchor: "middle", dominant_baseline: "middle",
                                "mandatory gate"
                            }
                        }

                        g { transform: "translate(670, 360)",
                            rect { width: "160", height: "70", rx: "6", class: "node-rect main-node" }
                            text { x: "80", y: "30", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                "main branch"
                            }
                            text { x: "80", y: "50", class: "node-subtext", text_anchor: "middle", dominant_baseline: "middle",
                                "auto-deploy → dev"
                            }
                        }
                    }

                    g { class: "connections",
                        path { d: "M 280 100 C 340 100, 360 215, 390 215", class: "path-line static-path", marker_end: "url(#aw-arrow)" }
                        path { d: "M 280 100 C 340 100, 360 215, 390 215", class: "path-line pulse-path path-1" }

                        path { d: "M 280 195 C 340 195, 360 228, 390 228", class: "path-line static-path", marker_end: "url(#aw-arrow)" }
                        path { d: "M 280 195 C 340 195, 360 228, 390 228", class: "path-line pulse-path path-2" }

                        path { d: "M 280 290 C 340 290, 360 242, 390 242", class: "path-line static-path", marker_end: "url(#aw-arrow)" }
                        path { d: "M 280 290 C 340 290, 360 242, 390 242", class: "path-line pulse-path path-3" }

                        path { d: "M 280 385 C 340 385, 360 255, 390 255", class: "path-line static-path", marker_end: "url(#aw-arrow)" }
                        path { d: "M 280 385 C 340 385, 360 255, 390 255", class: "path-line pulse-path path-4" }

                        path { d: "M 550 235 L 620 235", class: "path-line static-path", marker_end: "url(#aw-arrow)" }
                        path { d: "M 550 235 L 620 235", class: "path-line pulse-path path-5" }

                        path { d: "M 720 275 C 730 320, 740 330, 745 360", class: "path-line static-path", marker_end: "url(#aw-arrow)" }
                        path { d: "M 720 275 C 730 320, 740 330, 745 360", class: "path-line pulse-path path-5" }
                        text { x: "800", y: "320", class: "path-label", "merge" }
                    }

                    g { class: "hard-guards",
                        rect { x: "320", y: "350", width: "320", height: "70", rx: "6", class: "guard-rect" }
                        text { x: "480", y: "378", class: "guard-title", text_anchor: "middle",
                            "HARD GUARDS: never auto-modified"
                        }
                        text { x: "480", y: "400", class: "guard-subtext", text_anchor: "middle",
                            "auth · payments · migrations"
                        }
                    }
                }
            }
        }
    }
}
