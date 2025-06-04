use crate::components::Ender;
use crate::helper_fun::set_meta_tags;
use crate::Route;
use dioxus::prelude::*;

const _ROBOTS_TXT: Asset = asset!("/assets/robots.txt");
const PROFESSIONAL_PHOTO_JPG: Asset = asset!("assets/professional_photo_2023.jpg");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    let PHOTO_FORMAT_URL = format!("https://darkicewolf50.pages.dev{PROFESSIONAL_PHOTO_JPG}");
    rsx! {
        document::Stylesheet { href: asset!("/assets/styling/navbar.css") }
        document::Stylesheet { href: asset!("assets/styling/standardColoursandFonts.css") }
        set_meta_tags {
            description: "Fourth year Software Engineering student specializing in full-stack development with a backend focus. Always improving through experience.",
            keywords: "webdev Rust software engineer projects blog darkicewol50",
        }
        document::Meta { name: "robots", content: "index, follow" }
        document::Link { rel: "canonical", href: "https://darkicewolf50.pages.dev/" }
        document::Meta {
            name: "google-site-verification",
            content: "lsAs9c2Pv7c6Sm26z1hd2YqR2depbp4sJddIDYKHkxY",
        }
        document::Meta {
            property: "og:title",
            content: "Brock Tomlinson - Software Engineering Student",
        }
        document::Meta {
            property: "og:description",
            content: "Fourth year Software Engineering student specializing in full-stack development with a backend focus. Always improving through experience.",
        }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:url", content: "https://darkicewolf50.pages.dev/" }

        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta {
            name: "twitter:title",
            content: "Brock Tomlinson - Software Engineer",
        }
        document::Meta {
            name: "twitter:description",
            content: "Fourth year Software Engineering student specializing in full-stack development with a backend focus. Always improving through experience.",
        }
        document::Meta { name: "twitter:image", content: PHOTO_FORMAT_URL.clone() }

        document::Meta { property: "og:image", content: PHOTO_FORMAT_URL.clone() }

        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Projects {}, "Projects" }
            Link { to: Route::Blogs { page_num: 0 }, "Blogs" }
            Link { to: Route::ContactMe {}, "Contact" }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}

        Ender {}
    }
}
