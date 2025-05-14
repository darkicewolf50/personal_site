use crate::components::{Experience, TechCat};
use crate::views::{Contact, Projects};
use crate::Route;
use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Home() -> Element {
    let languages = vec![
        "Rust",
        "Python",
        "YAML",
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
        title { "Brock Tomlinson" }
        div {
            div { id: "home-intro",
                h1 { "Hi I'm Brock" }
                p {
                    "a fourth year Software Engineering Student specializing in full-stack development with a strong focus on backend technologies.
                I am developing the language of how to design, develop, and create programs that are to industry standards and reasonably efficent. 
                I bring the lessons learned from each project I have completed, 
                learning from the mistakes I have made and bringing improved versions forward into the next project."
                }
                p {
                    "As of writing this I intend to bring the knowledge learned from my time at university in Software Engineering onto a Baja SAE car,
                where we can collect data remotely and graph data for instantaneous and future analysis, 
                during vechile operation."
                }
                p {
                    "I grew up in a small ski town where, I started learning about programming, from of course Minecraft,
                where I thought the application of this was so futuristic and downright cool that I knew I wanted to persue it further. 
                While living there I spend a majority of my time outside of school swimming competitively, where I ranked top 10 in BC. 
                Along with swimming I spend a lot of time volunteering with fundraising events and coaching the local Special Olympics swim team."
                }
                p {
                    "I adore problem solving and building cool stuff, I'm happy to jump in and get started! "
                    Link { to: Route::ContactMe {},
                        button { "Let's create something great together!" }
                    }
                }
            }
            div { class: "technologies",

                h2 { "Technology" }
                p { "Here is what I prefer to use and their self assessed skill" }
                // p { "Here is what I developed skills in." }
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
            Projects { display_title: false }
            div { id: "experience",
                h2 { "Experience" }
                div {
                    Experience { professional_jobs: true }
                    Experience { professional_jobs: false }
                }
            }
        }
    )
}

// https://yaml.org/favicon.svg
