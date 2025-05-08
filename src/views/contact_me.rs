use std::collections::HashMap;

use dioxus::{logger::tracing, prelude::*};

use crate::views::Contact;

const CONTACTME_CSS: Asset = asset!("/assets/styling/contactme.css");

#[component]
pub fn ContactMe() -> Element {
    // let mut test_form = use_signal(|| {
    //     HashMap::from([
    //         ("Name", "".to_string()),
    //         ("Email", "".to_string()),
    //         ("Message", "".to_string()),
    //     ])
    // });

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
                form {
                    onsubmit: move |event| {
                        event.prevent_default();
                        tracing::info!("{:?}", event.data().values() ["name"]);
                    },
                    label { "Name" }
                    input { name: "name" }
                    label { "Email" }
                    input { name: "email", r#type: "email" }
                    label { "Message" }
                    textarea { name: "message" }
                    button {
                        r#type: "submit",
                        onclick: move |_| tracing::info!("Clicked!"),
                        "Submit"
                    }
                }
            }
        }
        div {
            h3 { "Form Submission" }
                // p { "{test_form.values().iter().map(|value| value.to_string)}" }
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
