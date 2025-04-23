use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn get_tech_logos_from_str(used_tech: &'static str) -> Element {
    let raw_data: TechDes = tech_table_lookup(used_tech);
    rsx! {
        img { src: "{raw_data.lang_logo}", alt: "{used_tech}'s logo/icon" }
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
        (
            "Github",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/512317/github-142.svg",
                project_site: "https://github.com/darkicewolf50",
                skill_level: 80,
            },
        ),
        (
            "Email",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/491226/email.svg",
                project_site: "mailto:darkicewolf50@gmail.com",
                skill_level: 100,
            },
        ),
        (
            "LinkedIn",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/521725/linkedin.svg",
                project_site: "https://www.linkedin.com/in/brock-tomlinson/",
                skill_level: 40,
            },
        ),
        (
            "Twitch",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/519925/twitch.svg",
                project_site: "https://www.twitch.tv/darkicewolf50",
                skill_level: 60,
            },
        ),
        (
            "Youtube",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/521936/youtube.svg",
                project_site: "https://www.youtube.com/@darkicewolf50",
                skill_level: 40,
            },
        ),
        (
            "Internet",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/490809/internet.svg",
                project_site: "https://google.com",
                skill_level: 99,
            },
        ),
    ]);

    techs_tools_frameworks_lookup[to_lookup]
}
