use dioxus::prelude::*;

#[component]
pub fn LayeredArchitectureDiagram() -> Element {
    rsx! {
        div { class: "diagram-container",
            div { class: "diagram-header",
                h3 { class: "diagram-title",
                    "End-to-End Stack: Application + Platform + Workflow"
                }
            }
            div { class: "svg-wrapper",
                svg {
                    view_box: "0 0 900 380",
                    class: "architecture-svg",
                    preserve_aspect_ratio: "xMidYMid meet",

                    defs {
                        marker {
                            id: "la-arrow",
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

                    // Top row: application stack
                    g { class: "nodes",
                        g { transform: "translate(90, 50)",
                            rect { width: "220", height: "90", rx: "6", class: "node-rect" }
                            text {
                                x: "110", y: "38", class: "node-text",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "React UI"
                            }
                            text {
                                x: "110", y: "60", class: "node-subtext",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "TypeScript + Tailwind"
                            }
                        }
                        g { transform: "translate(340, 50)",
                            rect { width: "220", height: "90", rx: "6", class: "node-rect repo-node" }
                            text {
                                x: "110", y: "38", class: "node-text",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "TypeScript API"
                            }
                            text {
                                x: "110", y: "60", class: "node-subtext",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "lint-enforced layered services"
                            }
                        }
                        g { transform: "translate(590, 50)",
                            rect { width: "220", height: "90", rx: "6", class: "node-rect data-node" }
                            text {
                                x: "110", y: "38", class: "node-text",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "PostgreSQL"
                            }
                            text {
                                x: "110", y: "60", class: "node-subtext",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "normalized schema"
                            }
                        }
                    }

                    // Connections — top row
                    g { class: "connections",
                        path {
                            d: "M 310 95 L 340 95",
                            class: "path-line static-path",
                            marker_end: "url(#la-arrow)"
                        }
                        path { d: "M 310 95 L 340 95", class: "path-line pulse-path path-1" }
                        path {
                            d: "M 560 95 L 590 95",
                            class: "path-line static-path",
                            marker_end: "url(#la-arrow)"
                        }
                        path { d: "M 560 95 L 590 95", class: "path-line pulse-path path-2" }
                    }

                    // Connector annotation
                    g {
                        text {
                            x: "450", y: "185", class: "connector-text", text_anchor: "middle",
                            "hosted, shipped, secured, and developed by"
                        }
                    }

                    // Bottom row: platform + workflow band
                    g { class: "nodes",
                        g { transform: "translate(30, 220)",
                            rect { width: "200", height: "100", rx: "6", class: "node-rect platform-node" }
                            text {
                                x: "100", y: "34", class: "node-text",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "Azure"
                            }
                            text {
                                x: "100", y: "58", class: "node-subtext",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "App Services + Postgres"
                            }
                            text {
                                x: "100", y: "76", class: "node-subtext",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "Terraform-managed"
                            }
                        }
                        g { transform: "translate(240, 220)",
                            rect { width: "200", height: "100", rx: "6", class: "node-rect platform-node" }
                            text {
                                x: "100", y: "34", class: "node-text",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "CI/CD"
                            }
                            text {
                                x: "100", y: "58", class: "node-subtext",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "build · test · lint"
                            }
                            text {
                                x: "100", y: "76", class: "node-subtext",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "secret scan · migration gates"
                            }
                        }
                        g { transform: "translate(450, 220)",
                            rect { width: "200", height: "100", rx: "6", class: "node-rect platform-node" }
                            text {
                                x: "100", y: "34", class: "node-text",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "Security"
                            }
                            text {
                                x: "100", y: "58", class: "node-subtext",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "RBAC + Azure Key Vault"
                            }
                            text {
                                x: "100", y: "76", class: "node-subtext",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "MFA (TOTP, OTP, passkeys)"
                            }
                        }
                        g { transform: "translate(660, 220)",
                            rect { width: "200", height: "100", rx: "6", class: "node-rect platform-node" }
                            text {
                                x: "100", y: "34", class: "node-text",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "AI Workflow"
                            }
                            text {
                                x: "100", y: "58", class: "node-subtext",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "scheduled agents · digests"
                            }
                            text {
                                x: "100", y: "76", class: "node-subtext",
                                text_anchor: "middle", dominant_baseline: "middle",
                                "drift watchers · gates"
                            }
                        }
                    }

                    // Dotted upward indicators
                    g { class: "platform-connectors",
                        path { d: "M 130 220 L 130 160", class: "platform-link", stroke_dasharray: "3 4" }
                        path { d: "M 340 220 L 340 160", class: "platform-link", stroke_dasharray: "3 4" }
                        path { d: "M 550 220 L 550 160", class: "platform-link", stroke_dasharray: "3 4" }
                        path { d: "M 760 220 L 760 160", class: "platform-link", stroke_dasharray: "3 4" }
                    }
                }
            }
        }
    }
}
