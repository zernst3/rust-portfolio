use dioxus::prelude::*;

use crate::audio::play_sound;
use crate::components::Close;
use crate::contexts::audio::use_audio_state;

const ICON_ALTERNATE_EMAIL: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="1.2em" height="1.2em" fill="currentColor" style="vertical-align:middle;margin-right:4px"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10h5v-2h-5c-4.34 0-8-3.66-8-8s3.66-8 8-8 8 3.66 8 8v1.43c0 .79-.71 1.57-1.5 1.57s-1.5-.78-1.5-1.57V12c0-2.76-2.24-5-5-5s-5 2.24-5 5 2.24 5 5 5c1.38 0 2.64-.56 3.54-1.47.65.89 1.77 1.47 2.96 1.47 1.97 0 3.5-1.6 3.5-3.57V12c0-5.52-4.48-10-10-10m0 13c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3"/></svg>"#;

#[derive(Clone, PartialEq)]
enum SubmitState {
    Idle,
    Loading,
    Success,
    Error,
}

#[component]
pub fn ContactMe() -> Element {
    let audio = use_audio_state();

    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut subject = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut submit_state = use_signal(|| SubmitState::Idle);

    let form_empty = name.read().is_empty()
        || email.read().is_empty()
        || subject.read().is_empty()
        || message.read().is_empty();

    let on_submit = move |e: Event<FormData>| {
        e.prevent_default();
        if form_empty {
            return;
        }
        if !*audio.is_muted.read() {
            play_sound("/static/sounds/select.mp3", 0.75);
        }

        let name_val = name.read().clone();
        let email_val = email.read().clone();
        let subject_val = subject.read().clone();
        let message_val = message.read().clone();

        submit_state.set(SubmitState::Loading);

        // Use document::eval + JS fetch to POST to /api/contact without
        // adding any HTTP client dependency (PORT-ROUTE-1 dep gate).
        // JS receives fields via dioxus.recv() and returns the HTTP status.
        // PORT-AUTO-CALLS-1: self-made call — see auto-calls ledger.
        let mut eval = document::eval(
            r#"
            const name    = await dioxus.recv();
            const email   = await dioxus.recv();
            const subject = await dioxus.recv();
            const message = await dioxus.recv();
            try {
                const resp = await fetch('/api/contact', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ name, email, subject, message })
                });
                dioxus.send(resp.status);
            } catch(_) {
                dioxus.send(503);
            }
            "#,
        );
        eval.send(name_val).ok();
        eval.send(email_val).ok();
        eval.send(subject_val).ok();
        eval.send(message_val).ok();

        spawn(async move {
            let status: i64 = eval
                .recv::<serde_json::Value>()
                .await
                .ok()
                .and_then(|v| v.as_i64())
                .unwrap_or(503);

            if status == 204 {
                submit_state.set(SubmitState::Success);
            } else {
                submit_state.set(SubmitState::Error);
            }
        });
    };

    rsx! {
        div { class: "page-enter",
            div { id: "ContactMeContainer",
                div { id: "ContactMe",
                    h1 { "Contact Me" }

                    {match *submit_state.read() {
                        SubmitState::Loading => rsx! {
                            div { class: "page",
                                h2 { "Sending..." }
                            }
                        },
                        SubmitState::Success => rsx! {
                            div { class: "page",
                                h2 { "Thank you for your message!" }
                            }
                        },
                        SubmitState::Error => rsx! {
                            div { class: "page",
                                h2 {
                                    "There has been an error, please contact me directly instead: "
                                    a {
                                        href: "mailto:zernst3@gmail.com",
                                        rel: "noreferrer",
                                        target: "_blank",
                                        dangerous_inner_html: "{ICON_ALTERNATE_EMAIL}",
                                    }
                                    a {
                                        href: "mailto:zernst3@gmail.com",
                                        rel: "noreferrer",
                                        target: "_blank",
                                        "zernst3@gmail.com"
                                    }
                                }
                            }
                        },
                        SubmitState::Idle => rsx! {
                            div { class: "form",
                                form { onsubmit: on_submit,
                                    label { r#for: "name", "Your Name" }
                                    input {
                                        required: true,
                                        id: "name",
                                        r#type: "text",
                                        value: "{name}",
                                        oninput: move |e| name.set(e.value()),
                                    }

                                    label { r#for: "email", "Your Email" }
                                    input {
                                        required: true,
                                        id: "email",
                                        r#type: "email",
                                        value: "{email}",
                                        oninput: move |e| email.set(e.value()),
                                    }

                                    label { r#for: "subject", "Subject" }
                                    input {
                                        required: true,
                                        id: "subject",
                                        r#type: "text",
                                        value: "{subject}",
                                        oninput: move |e| subject.set(e.value()),
                                    }

                                    label { r#for: "message", "Message" }
                                    textarea {
                                        required: true,
                                        id: "message",
                                        rows: 4,
                                        value: "{message}",
                                        oninput: move |e| message.set(e.value()),
                                    }

                                    h4 { "*All fields are required" }

                                    if form_empty {
                                        button {
                                            r#type: "submit",
                                            class: "disabled",
                                            disabled: true,
                                            "Submit"
                                        }
                                    } else {
                                        button { r#type: "submit", "Submit" }
                                    }
                                }

                                div { id: "DirectContact",
                                    h3 { "Or, to get in touch with me directly, please contact me at:" }
                                    a {
                                        href: "mailto:zernst3@gmail.com",
                                        rel: "noreferrer",
                                        target: "_blank",
                                        span { dangerous_inner_html: "{ICON_ALTERNATE_EMAIL}" }
                                        " zernst3@gmail.com"
                                    }
                                }
                            }
                        },
                    }}
                }
                Close {}
            }
        }
    }
}
