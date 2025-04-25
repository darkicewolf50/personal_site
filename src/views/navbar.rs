use crate::components::Ender;
use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const STD_COLOUR_AND_FONTS_CSS: Asset = asset!("assets/styling/standardColoursandFonts.css");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        document::Link { rel: "stylesheet", href: STD_COLOUR_AND_FONTS_CSS }

        div { id: "navbar",
            Link { to: Route::NewHome {}, "Home" }
            Link { to: Route::Blogs { id: 0 }, "Blogs" }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}

        Ender {}
    }
}
