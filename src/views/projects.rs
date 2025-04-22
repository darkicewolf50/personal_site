use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div {
            h2 { "Projects" }
            p { "Top Featured and Recent Projects" }
        }
        div { ProjectCards {} }
    }
}

const PROJECT_CARDS_CSS: Asset = asset!("/assets/styling/projectCards.css");

#[component]
pub fn ProjectCards() -> Element {
    let website: Option<&'static str> = Some("https://google.com");
    let github: Option<&'static str> = Some("https://google.com");

    rsx! {
        document::Link { href: PROJECT_CARDS_CSS, rel: "stylesheet" }
        div { class: "project-card",
            img {
                src: "https://picsum.photos/200",
                alt: "dashboard of project or the logo of the project",
            }
            div { class: "project-title-info",
                h3 { "Project Name" }
                div {
                    if let Some(github_site) = github {
                        a { href: "{github_site}",
                            img {
                                src: "https://www.svgrepo.com/show/512317/github-142.svg",
                                alt: "Github logo",
                            }
                        }
                    }
                    if let Some(site) = website {
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
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla sit amet risus tristique nisi euismod elementum. Duis et est sed neque pulvinar sodales sit amet non purus. Nam ut ultrices enim. Vestibulum blandit sapien dui. Aliquam sit amet ex quis lectus consectetur tempor at non arcu. Curabitur placerat justo sed nulla lobortis molestie. Sed eget justo sit amet justo lobortis tempus. Phasellus laoreet leo est, in lacinia ante aliquet ut. Etiam ultricies fermentum dolor id pretium. Sed dictum nisl id felis pulvinar varius."
                }
            
            }
            div { class: "project-tech-logos",
                img { alt: "todo cards" }
                img { alt: "todo cards" }
                img { alt: "todo cards" }
                img { alt: "todo cards" }
                img { alt: "todo cards" }
                img { alt: "todo cards" }
            }
        
        }
    }
}

// if let Some(site) = website {
//     a { href: "{site}", {img {
//     src: "https://www.svgrepo.com/show/490809/internet.svg",
//     alt: "Internet icon",
// }
