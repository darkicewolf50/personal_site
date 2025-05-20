use crate::helper_fun::tech_table_lookup;
use dioxus::prelude::*;

#[component]
pub fn TechCard(tech_props: &'static str) -> Element {
    let props_tech = tech_table_lookup(tech_props);

    rsx! {
        a { class: "tech-card", href: "{props_tech.project_site}",
            img { src: "{props_tech.tech_logo}", alt: "{tech_props}'s logo" }
            h4 { "{tech_props}" }
            progress { value: props_tech.skill_level, max: 100 }
        }
    }
}

#[component]
pub fn TechCat(cat: &'static str, tech_vec: Vec<&'static str>) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/styling/techs.css") }
        div { class: "tech-cat",
            h3 { "{cat}" }
            div { class: "tech-row",
                for tech in tech_vec {
                    TechCard { tech_props: tech }
                }
            }
        }
    }
}
