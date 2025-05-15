use crate::Route;
use dioxus::{logger::tracing, prelude::*};
use reqwest;

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
///
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.
#[component]
pub fn Blog(blog_title: String) -> Element {
    let blog_content = use_signal(|| {
        "<h1>This is a blog #blog_title</h1>
        <p>
            In blog #{blog_title}, we show how the Dioxus router works and 
            how URL parameters can be passed as props to our route components.
        </p>"
            .to_string()
    });
    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }

        div { id: "blog",

            // Content
            // h1 { "This is blog #{blog_title}!" }
            // p {
            //     "In blog #{blog_title}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            // }

            // // Navigation links
            // // The `Link` component lets us link to other routes inside our app. It takes a `to` prop of type `Route` and
            // // any number of child nodes.
            // Link {
            //     // The `to` prop is the route that the link should navigate to. We can use the `Route` enum to link to the
            //     // blog page with the id of -1. Since we are using an enum instead of a string, all of the routes will be checked
            //     // at compile time to make sure they are valid.
            //     to: Route::Blog { id: id - 1 },
            //     "Previous"
            // }
            // span { " <---> " }
            // Link { to: Route::Blog { id: id + 1 }, "Next" }
            Link { to: Route::Blogs { id: 0 }, "Go Back" }
            div { dangerous_inner_html: blog_content }
        }
    }
}

async fn get_blog(blog_name: String) {
    let res = reqwest::get("https://www.rust-lang.org")
        .await
        .unwrap()
        .text()
        .await
        .unwrap_or("".to_string());
    tracing::info!("{}", res);
}

#[component]
pub fn Blogs(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }
        div { id: "blogs",
            h1 { "Page Under Development" }
            p { "Please Try Again Later" }
            Link { to: Route::Home {},
                button { "Home" }
            }
                // Link {
        //     to: Route::Blog {
        //         blog_title: "Test_Blog".to_string(),
        //     },
        //     button { "To Test Blog" }
        // }
        }
    }
}
