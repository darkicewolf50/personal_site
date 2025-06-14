use crate::helper_fun::get_tech_logos_from_str;
use dioxus::prelude::*;

#[component]
pub fn Projects(#[props(default = true)] independent_page: bool) -> Element {
    rsx! {
        if independent_page {
            document::Title { "Brock Tomlinson - Projects" }
        }
        div {
            h2 { "Projects" }
            p { "Top Featured and Recent Projects" }
        }
        div { class: "project-section",
            ProjectCards {
                project_name: "Personal Backend",
                gitea_link: "https://gitea.bajacloud.duckdns.org/darkicewolf50/darkicewolf50Cloud",
                dockerhub_link: "https://hub.docker.com/r/darkicewolf50/darkicewolf50cloud",
                project_img: "https://actix.rs/img/logo.png",
                techs_used: vec![
                    "Rust",
                    "Actix",
                    "Github Actions",
                    "Docker",
                    "Traefik",
                    "Gitea",
                    "Git",
                    "Github",
                ],
                project_des: "I find that this is a much better option in compareision to FastAPI as it does not require a post request
            to input data instead it give the option for the url to give the parameters it needs. I don't have any complains about using Actix, its mature stable and fairly popular. 
            This backend application also uses comrak to convert markdown blogs into html docuemnts that are then seen by you the user. 
            This honestly was a fun challenge getting the blogs previews, ensuring correctness and not allowing for any unknown states, 
            this will serve as a great basis for any future backend requirements that I may have.",
            }
            ProjectCards {
                project_name: "Portfolio Site Version 2.0.1",
                website_link: "https://darkicewolf50.pages.dev",
                gitea_link: "https://gitea.bajacloud.duckdns.org/darkicewolf50/personal_site",
                project_img: "https://res.cloudinary.com/dpgrgsh7g/image/upload/v1745630861/Portfolio_site_k4mhmj.png",
                techs_used: vec!["Rust", "CSS", "Dioxus", "Git", "Gitea"],
                project_des: "This project was a great test of my newly learned Rust.
            This major update added functionality for the contact me, the blogs part of the site, as well as many minor 
            UI consistencies to ensure that all of the buttons and links felt like buttons and links.
            I was surprise how easy it was to set up a discord webhook using the 'reqwest' crate. 
            As I continue on I find myself struggling with how and why to use databases for content I generate. 
            I think using tools like disocrd webhooks and email notifications are great for users but certainly not great for reading data from. 
            I was very satisfied with serde, and comrak for converting markdown fiels into html. 
            I use this extensively for the blogs search menu and the blog itself to display the blog itself in a consistent way without needing to write a whole library.",
            }
            ProjectCards {
                project_name: "Portfolio Site 1.0.0",
                website_link: "https://darkicewolf50.github.io",
                github_link: "https://github.com/darkicewolf50/darkicewolf50.github.io",
                project_img: "https://res.cloudinary.com/dpgrgsh7g/image/upload/v1745630861/Portfolio_site_k4mhmj.png",
                techs_used: vec!["Rust", "CSS", "Dioxus", "Git", "Github"],
                project_des: "This project was a great test of my newly learned Rust.
                It was certainly interesting to go through all of the stages of front end web developement, while the orignal and new found scope is not currently achieved, it will be on a later pass through.
                I am very happy with how it turned out in compairison to my origanl site map, and wireframes.
                Considering this phase one was accomplished in 3 working days I believe it is an excellent show of my skill.",
            }
            ProjectCards {
                project_name: "UCalgary Baja Backend",
                project_img: "https://www.svgrepo.com/show/448221/docker.svg",
                dockerhub_link: "https://hub.docker.com/r/darkicewolf50/uofcbajacloud",
                techs_used: vec!["Python", "FastAPI", "Github Actions", "Docker", "Traefik", "Git", "Github"],
                project_des: "This is going to be extremely cost saving for the non-profit club UCalgary Baja.
                Using automated uploads and linting to check the Python and FastAPI code was excellent for learning how to self-host a web server.
                This was then upgraded later with the addition of treafik so that it could be SSL certified, this is also known as supporting HTTPS transmissions.
                Ultimately it will serve as a great stepping stone for both myself an anyone else in UCalgary Baja Software subteam. 
                This will lead into using Actixs in the migration Soon™ to be.",
            }
            ProjectCards {
                project_name: "UCalgary Baja Website",
                website_link: "https://uofcbaja.pages.dev",
                project_img: "https://res.cloudinary.com/dpgrgsh7g/image/upload/v1745633714/ucalgary-baja-site-April.png",
                techs_used: vec![
                    "HTML5",
                    "CSS",
                    "JavaScript",
                    "Markdown",
                    "YAML",
                    "React",
                    "Git",
                    "Github",
                    "Cloudflare",
                ],
                project_des: "The flexibility that we achieved using React,
                rather than a locked down platform or framework has allows all the Software members of UCalgary Baja to learn infinitely more.
                This isn't to say that it is faster or have additional perks of using 'non-code website builders'. 
                This is provided massive opportunities to learn teach and save on cost compaired to the website builders. 
                Overall I would say this will be worth it in the long run and opened my eyes to different website hosting providers, 
                with their associated perks and costs.
                When we change it, it will most likely we re-written in Vue as there is a good non-depreciated way to initalize the framework.",
            }
        }
    }
}

#[component]
pub fn ProjectCards(
    website_link: Option<&'static str>,
    github_link: Option<&'static str>,
    gitea_link: Option<&'static str>,
    dockerhub_link: Option<&'static str>,
    project_name: &'static str,
    techs_used: Vec<&'static str>,
    project_des: &'static str,
    #[props(default = "https://picsum.photos/200")] project_img: &'static str,
) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/styling/projectCards.css") }
        div { class: "project-card",
            img {
                src: "{project_img}",
                alt: "dashboard of project or the logo of the project",
            }
            div { class: "project-title-info",
                h3 { "{project_name}" }
                div {
                    if let Some(github_site) = github_link {
                        a { href: "{github_site}",
                            get_tech_logos_from_str { used_tech: "Github" }
                        }
                    }
                    if let Some(gitea_site) = gitea_link {
                        a { href: "{gitea_site}", id: "gitea",
                            get_tech_logos_from_str { used_tech: "Gitea" }
                        }
                    }
                    if let Some(dockerhub) = dockerhub_link {
                        a { href: "{dockerhub}", id: "dockerhub",
                            get_tech_logos_from_str { used_tech: "Docker" }
                        }
                    }
                    if let Some(site) = website_link {
                        a { href: "{site}",
                            get_tech_logos_from_str { used_tech: "Internet" }
                        }
                    }
                }
            }
            div {
                p { "{project_des}" }
            }
            div { class: "project-tech-logos",
                for tech in techs_used {
                    get_tech_logos_from_str { used_tech: tech }
                }
            }
        }
    }
}

// img {
//     src: "https://www.svgrepo.com/show/512317/github-142.svg",
//     alt: "Github logo",
// }
