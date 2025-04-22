use std::collections::HashMap;
use std::hash::{BuildHasherDefault, DefaultHasher};

use dioxus::prelude::*;

const TECHS_CSS: Asset = asset!("/assets/styling/techs.css");

#[component]
pub fn TechCard(tech_props: &'static str) -> Element {
    let props_tech = tech_table_lookup(tech_props);

    rsx! {
        a { class: "tech-card", href: "{props_tech.project_site}",
            img { src: "{props_tech.lang_logo}", alt: "{tech_props}'s logo" }
            h4 { "{tech_props}" }
            progress { value: props_tech.skill_level, max: 100 }
        }
    }
}

#[component]
pub fn TechCat(cat: String, tech_vec: Vec<&'static str>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TECHS_CSS }
        div { class: "tech-cat",
            div {
                h3 { "{cat}" }
            }
            div { class: "tech-row",
                for tech in tech_vec {
                    TechCard { tech_props: tech }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone, Copy)]
pub struct TechDes {
    // to be removed soon
    pub lang_logo: &'static str,
    pub project_site: &'static str,
    pub skill_level: u8,
}

pub fn tech_table_lookup(to_lookup: &str) -> TechDes {
    let techs_tools_frameworks_lookup = HashMap::from([
        (
            "Rust",
            TechDes {
                lang_logo: "https://www.rust-lang.org/static/images/rust-logo-blk.svg",
                project_site: "https://www.rust-lang.org",
                skill_level: 40,
            },
        ),
        (
            "Python",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/452091/python.svg",
                project_site: "https://www.python.org",
                skill_level: 50,
            },
        ),
        (
            "JavaScript",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/303206/javascript-logo.svg",
                project_site: "https://www.python.org",
                skill_level: 60,
            },
        ),
        (
            "YAML",
            TechDes {
                lang_logo: "https://yaml.org/favicon.svg",
                project_site: "https://yaml.org",
                skill_level: 95,
            },
        ),
        (
            "C",
            TechDes {
                lang_logo: "https://www.c-language.org/logo.svg",
                project_site: "https://www.c-language.org",
                skill_level: 30,
            },
        ),
        (
            "C++",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/452183/cpp.svg",
                project_site: "https://cplusplus.com",
                skill_level: 30,
            },
        ),
    ]);

    techs_tools_frameworks_lookup[to_lookup]
}
