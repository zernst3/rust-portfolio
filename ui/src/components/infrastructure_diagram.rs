use dioxus::prelude::*;

// Tooltip overlay is disabled in the React source; onMouseEnter/Leave handlers
// and useState omitted. CSS :hover on .nodes g still provides the glow effect.
// SVG filter primitives (feDropShadow, feGaussianBlur, feComposite) are omitted;
// the node-glow filter referenced by CSS degrades gracefully to the stroke/fill
// CSS rules alone when the SVG filter is absent.
#[component]
pub fn InfrastructureDiagram() -> Element {
    rsx! {
        div { class: "diagram-container",
            div { class: "svg-wrapper",
                svg {
                    view_box: "0 0 865 530",
                    class: "architecture-svg",
                    preserve_aspect_ratio: "xMidYMid meet",

                    defs {
                        marker {
                            id: "arrowhead-deep",
                            marker_width: "8",
                            marker_height: "6",
                            ref_x: "8",
                            ref_y: "3",
                            orient: "auto",
                            polygon { points: "0 0, 8 3, 0 6", fill: "#00573F" }
                        }
                        marker {
                            id: "arrowhead-static",
                            marker_width: "8",
                            marker_height: "6",
                            ref_x: "8",
                            ref_y: "3",
                            orient: "auto",
                            polygon { points: "0 0, 8 3, 0 6", fill: "rgba(0, 87, 63, 0.4)" }
                        }
                    }

                    g { class: "connections",
                        path { d: "M 240 255 L 320 255", class: "path-line static-path", marker_end: "url(#arrowhead-static)" }
                        path { d: "M 240 255 L 320 255", class: "path-line pulse-path path-1" }

                        path { d: "M 425 220 L 425 150", class: "path-line static-path", marker_end: "url(#arrowhead-static)" }
                        path { d: "M 425 220 L 425 150", class: "path-line pulse-path path-2" }

                        path { d: "M 425 295 L 425 370", class: "path-line static-path", marker_end: "url(#arrowhead-static)" }
                        path { d: "M 425 295 L 425 370", class: "path-line pulse-path path-1" }

                        path { d: "M 530 240 L 610 240", class: "path-line static-path", marker_end: "url(#arrowhead-static)" }
                        path { d: "M 530 240 L 610 240", class: "path-line pulse-path path-3" }

                        path { d: "M 530 270 L 610 270", class: "path-line static-path", marker_end: "url(#arrowhead-static)" }
                        path { d: "M 530 270 L 610 270", class: "path-line pulse-path path-2" }

                        path { d: "M 715 220 L 715 150", class: "path-line static-path", marker_end: "url(#arrowhead-static)" }
                        path { d: "M 715 220 L 715 150", class: "path-line pulse-path path-4" }
                    }

                    g { class: "nodes",
                        g { transform: "translate(30, 220)",
                            rect { width: "210", height: "75", rx: "6", class: "node-rect frontend-node" }
                            text { x: "105", y: "32", class: "node-text", text_anchor: "middle",
                                "Agora UI"
                            }
                            text { x: "105", y: "52", class: "node-subtext", text_anchor: "middle",
                                "React SPA"
                            }
                        }

                        g { transform: "translate(320, 220)",
                            rect { width: "210", height: "75", rx: "6", class: "node-rect backend-node" }
                            text { x: "105", y: "32", class: "node-text", text_anchor: "middle",
                                "Agora API"
                            }
                            text { x: "105", y: "52", class: "node-subtext", text_anchor: "middle",
                                "Node.js & Express"
                            }
                        }

                        g { transform: "translate(320, 75)",
                            rect { width: "210", height: "75", rx: "6", class: "node-rect support-node" }
                            text { x: "105", y: "32", class: "node-text", text_anchor: "middle",
                                "Agora Key Vault"
                            }
                            text { x: "105", y: "52", class: "node-subtext", text_anchor: "middle",
                                "Secrets & Certificates"
                            }
                        }

                        g { transform: "translate(320, 370)",
                            rect { width: "210", height: "75", rx: "6", class: "node-rect data-node" }
                            text { x: "105", y: "32", class: "node-text", text_anchor: "middle",
                                "Agora Database"
                            }
                            text { x: "105", y: "52", class: "node-subtext", text_anchor: "middle",
                                "Azure PostgreSQL"
                            }
                        }

                        g { transform: "translate(610, 220)",
                            rect { width: "210", height: "75", rx: "6", class: "node-rect data-node" }
                            text { x: "105", y: "32", class: "node-text", text_anchor: "middle",
                                "Agora Storage"
                            }
                            text { x: "105", y: "52", class: "node-subtext", text_anchor: "middle",
                                "Blob Storage & Queues"
                            }
                        }

                        g { transform: "translate(610, 75)",
                            rect { width: "210", height: "75", rx: "6", class: "node-rect support-node" }
                            text { x: "105", y: "32", class: "node-text", text_anchor: "middle",
                                "Agora Functions"
                            }
                            text { x: "105", y: "52", class: "node-subtext", text_anchor: "middle",
                                "Queue & Timer Triggers"
                            }
                        }
                    }

                    g { class: "labels",
                        text { x: "280", y: "245", class: "path-label", "Request" }
                        text { x: "435", y: "190", class: "path-label", "Security Handshake" }
                        text { x: "435", y: "340", class: "path-label", "Async I/O" }
                        text { x: "570", y: "232", class: "path-label", "Enqueue JSON" }
                        text { x: "570", y: "285", class: "path-label", "Blob I/O" }
                        text { x: "725", y: "180", class: "path-label", "Queue Trigger" }
                        text { x: "725", y: "200", class: "path-label", "Timer Trigger" }
                    }
                }
            }
        }
    }
}
