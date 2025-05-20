use dioxus::prelude::*;

const PROFESSIONAL_PHOTO_JPG: Asset = asset!("assets/professional_photo_2023.jpg");

#[component]
pub fn Contact() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/styling/contact.css") }
        h2 { "Contact" }
        div { id: "contact",
            div {
                div {
                    img {
                        src: PROFESSIONAL_PHOTO_JPG,
                        alt: "Brock's professional photo",
                    }
                }
            }
            div {
                div {
                    h4 { "Brock Tomlinson" }
                    ul {
                        li { "FullStack Webdev and Student Software Engineer" }
                        li {
                            a { href: "mailto:darkicewolf50@gmail.com",
                                img {
                                    src: "https://www.svgrepo.com/show/491226/email.svg",
                                    alt: "Email icon/logo",
                                }
                                div {
                                    p { "Email I check:" }
                                    p { "darkicewolf50@gmail.com" }
                                }
                            }
                        }
                        li {
                            a { href: "mailto:brock@eatsleepski.com",
                                img {
                                    src: "https://www.svgrepo.com/show/491226/email.svg",
                                    alt: "Email icon/logo",
                                }
                                div {
                                    p { "Professional Email:" }
                                    p { "brock@eatsleepski.com" }
                                }
                            }
                        }
                        li {
                            a {
                                img {
                                    src: "https://www.svgrepo.com/show/512317/github-142.svg",
                                    alt: "Github logo",
                                }
                                p { "darkicewolf50" }
                            }
                        }
                        li {
                            a {
                                img {
                                    src: "https://www.svgrepo.com/show/521725/linkedin.svg",
                                    alt: "LinkedIn logo",
                                }
                                p { "Brock Tomlinson" }
                            }
                        }
                        li {
                            a {
                                img {
                                    src: "https://www.svgrepo.com/show/519925/twitch.svg",
                                    alt: "Twitch logo",
                                }
                                p { "darkicewolf50" }
                            }
                        }
                        li {
                            a {
                                img {
                                    src: "https://www.svgrepo.com/show/521936/youtube.svg",
                                    alt: "Youtube logo",
                                }
                                p { "@darkicewolf50" }
                            }
                        }
                    }
                }
            }
        }
    }
}
