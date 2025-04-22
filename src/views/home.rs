use crate::components::TechCat;
use crate::views::{Contact, Projects};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let languages: Vec<&'static str> = vec!["Rust", "Python", "JavaScript", "YAML", "C", "C++"];
    rsx!(
        h1 { "Hi I'm Brock" }
        TechCat { cat: "Languages".to_string(), tech_vec: languages }
        Contact {}
        Projects {}
    )
}

// https://yaml.org/favicon.svg
