use crate::components::TechCat;
use crate::views::{Contact, Projects};
use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Home() -> Element {
    let languages = vec![
        "Rust",
        "Python",
        "YAML",
        "C",
        "C++",
        "HTML5",
        "CSS",
        "JavaScript",
        "Markdown",
    ];
    let backend = vec!["Actix", "FastAPI", "Dioxus", "Diesel"];
    let frontend = vec!["React", "Dioxus", "Vue"];
    let databases = vec!["Sqlite", "PostgreSQL", "Mongodb", "DynamoDB"];
    let tools = vec![
        "Vs Code",
        "Git",
        "Prettier",
        "Firefox",
        "Github Actions",
        "Traefik",
        "Docker",
        "Kubernetes",
        "Terraform",
    ];
    let platforms = vec!["AWS", "Cloudflare", "Vercel", "Netlify", "Gitea", "Github"];
    rsx!(
        document::Link { rel: "stylesheet", href: HOME_CSS }
        div {
            h1 { "Hi I'm Brock" }
            div { class: "technologies",

                h2 { "Technology" }
                p { "Here is what I prefer to use." }
                div { class: "technologies-cat",
                    TechCat { cat: "Languages", tech_vec: languages }
                    TechCat { cat: "Backend", tech_vec: backend }
                    TechCat { cat: "Frontend", tech_vec: frontend }
                    TechCat { cat: "Databases", tech_vec: databases }
                    TechCat { cat: "Platforms", tech_vec: platforms }
                    TechCat { cat: "Tools", tech_vec: tools }
                }
            }
            Contact {}
            Projects {}
        }
    )
}

// https://yaml.org/favicon.svg
