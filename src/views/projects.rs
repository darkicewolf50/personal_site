use crate::helper_fun::get_tech_logos_from_str;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div {
            h2 { "Projects" }
            p { "Top Featured and Recent Projects" }
        }
        div { class: "project-section",
            ProjectCards {
                project_name: "Project Name",
                website_prop: "https://google.com",
                github_prop: "https://google.com",
                techs_used: vec!["Rust", "Rust", "Rust", "Rust", "Rust", "Rust", "Rust"],
                project_des: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla sit amet risus tristique nisi euismod elementum. Duis et est sed neque pulvinar sodales sit amet non purus. Nam ut ultrices enim. Vestibulum blandit sapien dui. Aliquam sit amet ex quis lectus consectetur tempor at non arcu. Curabitur placerat justo sed nulla lobortis molestie. Sed eget justo sit amet justo lobortis tempus. Phasellus laoreet leo est, in lacinia ante aliquet ut. Etiam ultricies fermentum dolor id pretium. Sed dictum nisl id felis pulvinar varius.",
            }
            ProjectCards {
                project_name: "Project Name 2",
                website_prop: "https://google.com",
                github_prop: "https://google.com",
                techs_used: vec!["Rust", "Rust", "Rust", "Rust", "Rust", "Rust", "Rust"],
                project_des: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla sit amet risus tristique nisi euismod elementum. Duis et est sed neque pulvinar sodales sit amet non purus. Nam ut ultrices enim. Vestibulum blandit sapien dui. Aliquam sit amet ex quis lectus consectetur tempor at non arcu. Curabitur placerat justo sed nulla lobortis molestie. Sed eget justo sit amet justo lobortis tempus. Phasellus laoreet leo est, in lacinia ante aliquet ut. Etiam ultricies fermentum dolor id pretium. Sed dictum nisl id felis pulvinar varius.",
            }
            ProjectCards {
                project_name: "Project Name 3",
                website_prop: "https://google.com",
                github_prop: "https://google.com",
                techs_used: vec!["Rust", "Rust", "Rust", "Rust", "Rust", "Rust", "Rust"],
                project_des: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla sit amet risus tristique nisi euismod elementum. Duis et est sed neque pulvinar sodales sit amet non purus. Nam ut ultrices enim. Vestibulum blandit sapien dui. Aliquam sit amet ex quis lectus consectetur tempor at non arcu. Curabitur placerat justo sed nulla lobortis molestie. Sed eget justo sit amet justo lobortis tempus. Phasellus laoreet leo est, in lacinia ante aliquet ut. Etiam ultricies fermentum dolor id pretium. Sed dictum nisl id felis pulvinar varius.",
            }
        }
    }
}

const PROJECT_CARDS_CSS: Asset = asset!("/assets/styling/projectCards.css");

#[component]
pub fn ProjectCards(
    website_prop: Option<&'static str>,
    github_prop: Option<&'static str>,
    project_name: &'static str,
    techs_used: Vec<&'static str>,
    project_des: &'static str,
) -> Element {
    rsx! {
        document::Link { href: PROJECT_CARDS_CSS, rel: "stylesheet" }
        div { class: "project-card",
            img {
                src: "https://picsum.photos/200",
                alt: "dashboard of project or the logo of the project",
            }
            div { class: "project-title-info",
                h3 { "{project_name}" }
                div {
                    if let Some(github_site) = github_prop {
                        a { href: "{github_site}",
                            get_tech_logos_from_str { used_tech: "Github" }
                        }
                    }
                    if let Some(site) = website_prop {
                        a { href: "{site}",
                            img {
                                src: "https://www.svgrepo.com/show/490809/internet.svg",
                                alt: "Internet icon",
                            }
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
