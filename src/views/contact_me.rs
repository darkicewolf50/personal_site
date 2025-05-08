use std::collections::HashMap;

use dioxus::{logger::tracing, prelude::*};

use crate::views::Contact;

const CONTACTME_CSS: Asset = asset!("/assets/styling/contactme.css");

#[component]
pub fn ContactMe() -> Element {
    let mut pre_form: Signal<HashMap<&'static str, String>> = use_signal(|| {
        HashMap::from([
            ("Name", "".to_string()),
            ("Email", "".to_string()),
            ("Message", "".to_string()),
        ])
    });

    let mut error_box_message: Signal<String> = use_signal(|| "".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: CONTACTME_CSS }
        div { id: "ContactMe",
            div {
                h2 { "Get in Touch" }
                p {
                    "Please feel free to reach out about questions, opporunities or just want to connect.
                    Feel free to either fill out this form or contact me through one of the many of the platforms below"
                }
            }
            div {
                div { id: "contact-me",
                    label { "Name" }
                    input {
                        oninput: move |event| {
                            pre_form.write().insert("Name", event.value());
                        },
                    }
                    label { "Email" }
                    input {
                        r#type: "email",
                        oninput: move |event| {
                            pre_form.write().insert("Email", event.value());
                        },
                    }
                    label { "Message" }
                    textarea {
                        oninput: move |event| {
                            pre_form.write().insert("Message", event.value());
                        },
                    }
                    p { "{error_box_message}" }
                    button {
                        r#type: "submit",
                        onclick: move |_| tracing::info!("Clicked!\n{:?}", pre_form),
                        "Submit"
                    }
                }
            }
        }
        Contact {}
    }
}

// onsubmit:move |event| { log::info!("Submitted! {event:?}"),
// FormEvent {
//     value: "NameEmailMessageSubmit",
//     values: {
//         "name": FormValue(["asdasd"]),
//         "message": FormValue(["asdads"]),
//         "email": FormValue(["adasdad@asdasd.asdads"])
//     },
//     valid: false
// }
