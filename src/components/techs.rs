use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/techs.css");

#[component]
pub fn TechCard(props_tech: TechDes) -> Element {
    rsx! {
        a { class: "tech-card", href: "{props_tech.project_site}",
            img {
                src: "{props_tech.lang_logo}",
                alt: "{props_tech.lang_name}'s logo",
            }
            h4 { "{props_tech.lang_name}" }
            progress { value: props_tech.skill_level, max: 100 }
        }
    }
}

#[component]
pub fn TechCat(cat: String, tech_vec: Vec<TechDes>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        div { class: "tech-cat",
            div {
                h3 { "{cat}" }
            }
            div { class: "tech-row",
                for tech in tech_vec {
                    TechCard { props_tech: tech }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct TechDes {
    pub lang_name: &'static str,
    pub lang_logo: &'static str,
    pub project_site: &'static str,
    pub skill_level: u8,
}
