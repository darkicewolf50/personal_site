use dioxus::prelude::*;

// use components::Hero;
use views::{Blog, Blogs, ContactMe, Home, Navbar, NewHome, Projects};

/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

/// Defines where to place all helper functions
mod helper_fun;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
///
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(Navbar)]
        // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
        // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
        #[route("/")]
        Home {},
        // The route attribute can include dynamic parameters that implement [`std::str::FromStr`] and [`std::fmt::Display`] with the `:` syntax.
        // In this case, id will match any integer like `/blog/123` or `/blog/-456`.
        #[route("/blogs/:page_num")]
        // Fields of the route variant will be passed to the component as props. In this case, the blog component must accept
        // an `id` prop of type `i32`.
        Blogs { page_num: u32 },

        #[route("/blogs/:blog_title")]
        Blog {blog_title: String},

        #[route("/projects")]
        Projects {},

        #[route("/contact")]
        ContactMe {},

        #[route("/new_home")]
        NewHome {},
        // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
        #[route("/:..route")]
        PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/styling/notFound.css") }
        div { id: "not-found",
            h1 { "Page not found" }
            p { "We are terribly sorry, but the page you requested doesn't exist." }
            dioxus::prelude::Link { to: Route::Home {},
                button { "Return Home Here" }
            }
        }
    }
}

// The server function at the endpoint "static_routes" will be called by the CLI to generate the list of static
// routes. You must explicitly set the endpoint to `"static_routes"` in the server function attribute instead of
// the default randomly generated endpoint.
// #[server(endpoint = "static_routes", output = server_fn::codec::Json)]
// async fn static_routes() -> Result<Vec<String>, ServerFnError> {
//     // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
//     Ok(Route::static_routes()
//         .iter()
//         .map(ToString::to_string)
//         .collect())
// }
