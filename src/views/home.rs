use crate::components::{TechCat, TechDes};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let languages = vec![
        TechDes {
            lang_name: "Rust",
            lang_logo: "https://www.rust-lang.org/static/images/rust-logo-blk.svg",
            project_site: "https://www.rust-lang.org",
            skill_level: 40,
        },
        TechDes {
            lang_name: "Python",
            lang_logo: "https://www.svgrepo.com/show/452091/python.svg",
            project_site: "https://www.python.org",
            skill_level: 50,
        },
        TechDes {
            lang_name: "JavaScript",
            lang_logo: "https://www.svgrepo.com/show/303206/javascript-logo.svg",
            project_site: "https://www.python.org",
            skill_level: 60,
        },
        TechDes {
            lang_name: "YAML",
            lang_logo: "https://yaml.org/favicon.svg",
            project_site: "https://yaml.org",
            skill_level: 95,
        },
        TechDes {
            lang_name: "C",
            lang_logo: "https://www.c-language.org/logo.svg",
            project_site: "https://www.c-language.org",
            skill_level: 30,
        },
        TechDes {
            lang_name: "C++",
            lang_logo: "https://www.svgrepo.com/show/452183/cpp.svg",
            project_site: "https://cplusplus.com",
            skill_level: 30,
        },
    ];
    rsx!(
        h1 { "Hi I'm Brock" }
        TechCat { cat: "Languages".to_string(), tech_vec: languages }
    )
}

// https://yaml.org/favicon.svg
