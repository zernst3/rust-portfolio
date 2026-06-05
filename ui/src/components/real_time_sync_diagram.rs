use dioxus::prelude::*;

// Tooltip overlay is disabled in the React source; mouse-enter/leave handlers
// and useState are omitted. CSS :hover on .interactive-node still works.
#[component]
pub fn RealTimeSyncDiagram() -> Element {
    rsx! {
        div { class: "diagram-container",
            div { class: "diagram-header",
                h3 { class: "diagram-title",
                    "System Architecture: Decoupled Message Ingestion"
                }
            }
            div { class: "svg-wrapper",
                svg {
                    view_box: "0 0 865 350",
                    class: "architecture-svg",
                    preserve_aspect_ratio: "xMidYMid meet",

                    defs {
                        marker {
                            id: "arrowhead",
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
                            rect { x: "0", y: "110", width: "200", height: "150", rx: "10", class: "boundary-rect" }
                            text { x: "100", y: "125", class: "boundary-title", text_anchor: "middle",
                                "Azure App Service"
                            }
                        }

                        g { transform: "translate(10, 150)",
                            rect { width: "180", height: "70", rx: "6", class: "node-rect frontend-node" }
                            text { x: "90", y: "30", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                "React Frontend"
                            }
                            text { x: "90", y: "50", class: "node-subtext", text_anchor: "middle", dominant_baseline: "middle",
                                "WebSocket Client"
                            }
                        }

                        g { class: "boundary-group",
                            rect { x: "230", y: "20", width: "630", height: "310", rx: "10", class: "boundary-rect" }
                            text { x: "545", y: "35", class: "boundary-title", text_anchor: "middle",
                                "Azure App Service Environment"
                            }
                        }

                        g { transform: "translate(250, 50)",
                            rect { width: "160", height: "70", rx: "6", class: "node-rect backend-node" }
                            text { x: "80", y: "40", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                ".NET Core API"
                            }
                        }

                        g { transform: "translate(480, 40)", class: "interactive-node",
                            rect { width: "160", height: "70", rx: "6", class: "node-rect msg-node" }
                            text { x: "80", y: "30", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                "Azure Service Bus"
                            }
                            text { x: "80", y: "50", class: "node-subtext", text_anchor: "middle", dominant_baseline: "middle",
                                "Message Queue"
                            }
                        }

                        g { transform: "translate(700, 40)", class: "interactive-node",
                            rect { width: "140", height: "70", rx: "6", class: "node-rect worker-node" }
                            text { x: "70", y: "30", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                "Azure WebJob"
                            }
                            text { x: "70", y: "50", class: "node-subtext", text_anchor: "middle", dominant_baseline: "middle",
                                "Background Worker"
                            }
                        }

                        g { transform: "translate(480, 250)",
                            rect { width: "160", height: "70", rx: "6", class: "node-rect hub-node" }
                            text { x: "80", y: "30", class: "node-text", text_anchor: "middle", dominant_baseline: "middle",
                                "SignalR Hub"
                            }
                            text { x: "80", y: "50", class: "node-subtext", text_anchor: "middle", dominant_baseline: "middle",
                                "WebSocket Server"
                            }
                        }
                    }

                    g { class: "connections",
                        path {
                            d: "M 190 185 C 220 185, 230 75, 250 75",
                            class: "path-line static-path",
                            marker_end: "url(#arrowhead)"
                        }
                        path { d: "M 190 185 C 220 185, 230 75, 250 75", class: "path-line pulse-path path-1" }
                        text { x: "210", y: "105", class: "path-label", "POST Data" }

                        path {
                            d: "M 410 75 L 480 75",
                            class: "path-line static-path",
                            marker_end: "url(#arrowhead)"
                        }
                        path { d: "M 410 75 L 480 75", class: "path-line pulse-path path-2" }
                        text { x: "445", y: "65", class: "path-label", "Enqueue" }

                        path {
                            d: "M 250 85 C 210 85, 180 160, 190 160",
                            class: "path-line return-path",
                            stroke_dasharray: "4,4",
                            marker_end: "url(#arrowhead)"
                        }
                        text { x: "180", y: "145", class: "path-label return-label", "202 Accepted" }

                        path {
                            d: "M 640 75 L 700 75",
                            class: "path-line static-path",
                            marker_end: "url(#arrowhead)"
                        }
                        path { d: "M 640 75 L 700 75", class: "path-line pulse-path path-3" }
                        text { x: "670", y: "65", class: "path-label", "Dequeue" }

                        path {
                            d: "M 770 110 C 770 285, 660 285, 640 285",
                            class: "path-line static-path",
                            marker_end: "url(#arrowhead)"
                        }
                        path { d: "M 770 110 C 770 285, 660 285, 640 285", class: "path-line pulse-path path-4" }
                        text { x: "735", y: "275", class: "path-label", "Broadcast (Okta S2S)" }

                        path {
                            d: "M 480 285 C 320 285, 300 200, 190 200",
                            class: "path-line static-path",
                            marker_end: "url(#arrowhead)"
                        }
                        path { d: "M 480 285 C 320 285, 300 200, 190 200", class: "path-line pulse-path path-5" }
                        text { x: "360", y: "235", class: "path-label", "Relay via WebSocket" }

                        path {
                            d: "M 190 215 C 320 215, 300 300, 480 300",
                            class: "path-line sub-path",
                            stroke_dasharray: "5,5",
                            marker_end: "url(#arrowhead)"
                        }
                        text { x: "360", y: "315", class: "path-label sub-label", "WebSocket Connection" }
                    }
                }
            }
        }
    }
}
