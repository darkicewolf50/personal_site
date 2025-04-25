use std::collections::HashMap;

use crate::helper_fun::{get_tech_logos_from_str, tech_table_lookup, TechDes};
use dioxus::prelude::*;

const ENDER_CSS: Asset = asset!("/assets/styling/ender.css");

#[component]
pub fn Ender() -> Element {
    // gets list of items to get
    let footer_info_to_get = vec!["Github", "Email", "LinkedIn", "Twitch", "Youtube"];

    // used so that I dont need to copy paste the same link/info everywhere
    let mut footer_info: HashMap<&str, TechDes> = HashMap::new();
    for used_tech_item in footer_info_to_get {
        footer_info.insert(used_tech_item, *tech_table_lookup(used_tech_item));
    }
    rsx! {
        document::Link { rel: "stylesheet", href: ENDER_CSS }
        footer {
            p { "Brock Tomlinson © 2025" }
            div {
                for (footer_name , footer_item) in footer_info {
                    a { href: "{footer_item.project_site}",
                        img {
                            src: "{footer_item.tech_logo}",
                            alt: "{footer_name}'s logo/icon",
                        }
                        p { "{footer_name}" }
                    }
                }
            }
        }
    }
}
