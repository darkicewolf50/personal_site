use reqwest::Client;
use serde_json;

// use dioxus::logger::tracing;
use dioxus::prelude::*;

use crate::views::Contact;

#[component]
pub fn ContactMe() -> Element {
    let mut contact_me_name = use_signal(|| String::new());
    let mut contact_me_email = use_signal(|| String::new());
    let mut contact_me_message = use_signal(|| String::new());

    let mut _error_box_message = use_signal(|| String::new());

    rsx! {
        document::Stylesheet { href: asset!("/assets/styling/contactme.css") }
        document::Title { "Brock Tomlinson - Contact" }
        div { id: "ContactMe",
            div {
                h2 { "Get in Touch" }
                p {
                    "Please feel free to reach out about questions, opporunities or just want to connect.
                    Feel free to either fill out this form or contact me through one of the many of the platforms below"
                }
            }
            div {
                label { "Name" }
                input {
                    oninput: move |event| {
                        contact_me_name.set(event.value());
                    },
                }
                label { "Email" }
                input {
                    oninput: move |event| {
                        contact_me_email.set(event.value());
                    },
                }
                label { "Message" }
                textarea {
                    oninput: move |event| {
                        contact_me_message.set(event.value());
                    },
                }
                p { "{_error_box_message}" }
                button {
                    onclick: move |_| async move {
                        send_message(
                                contact_me_name(),
                                contact_me_email(),
                                contact_me_message(),
                                _error_box_message,
                            )
                            .await
                    },
                    "Submit"
                }
            }
        }
        Contact {}
    }
}

async fn send_message(name: String, email: String, message: String, mut recived: Signal<String>) {
    if name == "".to_string() && email == "".to_string() && message == "".to_string() {
        recived.set("Please fill fill out the form first".to_string());
        return ();
    } else if name == "".to_string() {
        recived.set("Please fill in your name so I know who I am contacting".to_string());
        return ();
    } else if email == "".to_string() {
        recived.set("Please fill in your email so I can get into contact".to_string());
        return ();
    } else if message == "".to_string() {
        recived.set("Please write a message so I know why you wanted to be in contact".to_string());
        return ();
    }

    if is_valid_email_basic(&email) == false {
        recived.set("Please write a vaild email".to_string());
        return ();
    }

    let json_to_send = serde_json::json!({
      "content": format!("***New Message***\n*Name*: {name}\n*Email*: [{email}](mailto:{email})\n*Message*: {message}")
    });

    let client = Client::new();
    let res = client
                .post("https://discord.com/api/webhooks/1371617469281861772/uARm18pvzzs4DVNLSYNYCyl7CQk_7eglqGmBabQASow2L7NHgGRHzQhkSAKaOIZmLnn1")
                .json(&json_to_send)
                .send()
                .await;
    match res {
        Ok(_) => {
            recived.set("Sent Sucessfully, I will be in contact with you soon".to_string());
        }
        Err(_) => {
            recived.set("An Error Occured".to_string());
        }
    }
}

fn is_valid_email_basic(email: &str) -> bool {
    // Find the position of '@'
    if let Some(at_pos) = email.find('@') {
        // Ensure there's only one '@'
        if email.rfind('@') != Some(at_pos) {
            return false;
        }

        // Split into local and domain parts
        let local = &email[..at_pos];
        let domain = &email[at_pos + 1..];

        // Check both parts are non-empty
        if local.is_empty() || domain.is_empty() {
            return false;
        }

        // Check domain contains at least one '.' and it's not at the start or end
        if let Some(dot_pos) = domain.find('.') {
            // '.' must not be at the beginning or end of the domain
            if dot_pos == 0 || dot_pos == domain.len() - 1 {
                return false;
            }
            return true;
        }
    }

    false
}
