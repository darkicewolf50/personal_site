use dioxus::prelude::*;

const ENDER_CSS: Asset = asset!("/assets/styling/ender.css");

#[component]
pub fn Ender() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: ENDER_CSS }
        footer {
            p { "Brock Tomlinson © 2025" }
            div {
                a { href: "https://github.com/darkicewolf50",
                    img {
                        src: "https://www.svgrepo.com/show/512317/github-142.svg",
                        alt: "Github logo",
                    }
                    p { "Github" }
                }
                a { href: "mailto:darkicewolf50@gmail.com",
                    img {
                        src: "https://www.svgrepo.com/show/491226/email.svg",
                        alt: "Email logo/icon",
                    }
                    p { "Email" }
                }
                a { href: "https://www.linkedin.com/in/brock-tomlinson/",
                    img {
                        src: "https://www.svgrepo.com/show/521725/linkedin.svg",
                        alt: "LinkedIn logo",
                    }
                    p { "LinkedIn" }
                }
                a { href: "https://www.twitch.tv/darkicewolf50",
                    img {
                        src: "https://www.svgrepo.com/show/519925/twitch.svg",
                        alt: "Twitch logo",
                    }
                    p { "Twitch" }
                }
                a { href: "https://www.youtube.com/@darkicewolf50",
                    img {
                        src: "https://www.svgrepo.com/show/521936/youtube.svg",
                        alt: "Youtube logo",
                    }
                    p { "Youtube" }
                }
            }
        }
    }
}
