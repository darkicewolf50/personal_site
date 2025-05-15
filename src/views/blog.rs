use std::thread::Scope;

use crate::Route;
use dioxus::{html::script::r#async, logger::tracing, prelude::*};
use reqwest;
use serde::{Deserialize, Serialize};

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
            Link { to: Route::Blogs { page_num: 0 }, "Go Back" }
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
pub fn Blogs(page_num: u32) -> Element {
    let mut _num_limit: Signal<u8> = use_signal(|| 10);

    let blogs_resource: Resource<Vec<BlogPreview>> =
        use_resource(move || async move { get_blogs_preview(_num_limit(), page_num).await });

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }
        div { id: "blogs",
            title { "Blogs" }
            h1 { "Blogs" }
            p {
                "This is a collection of blog posts, ranging from tutorials, technologies I found interesting, and opinion pieces"
            }
            Link { to: Route::Home {},
                button { "Home" }
            }
            // Link {
            //     to: Route::Blog {
            //         blog_title: "Test_Blog".to_string(),
            //     },
            //     button { "To Test Blog" }
            // }
            div {
                if let Some(blogs) = &*blogs_resource.read() {
                    for blog in blogs.iter() {

                        Link {
                            to: Route::Blog {
                                blog_title: blog.blog_file_name.clone(),
                            },
                            div { dangerous_inner_html: blog.html_preview.as_str() }
                        }
                    }
                } else {
                    div { "Loading blogs..." }
                }
            }
            div {
                Link {
                    to: Route::Blogs {
                        page_num: page_num + 1,
                    },
                    "Next"
                }
                if page_num > 0 {
                    Link {
                        to: Route::Blogs {
                            page_num: page_num - 1,
                        },
                        "Go Back"
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct BlogPreview {
    pub blog_file_name: String,
    pub date_last_edit: String,
    pub html_preview: String,
}

async fn get_blogs_preview(_num_limit: u8, page_num: u32) -> Vec<BlogPreview> {
    let res = reqwest::get(format!(
        "http://localhost:8000/blogs/{}/{}",
        _num_limit, page_num
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap_or("".to_string());

    let json: serde_json::Value = serde_json::from_str(&res).unwrap();

    // Extract the "Blogs" array and deserialize it into Vec<BlogPreview>
    let blogs: Vec<BlogPreview> = serde_json::from_value(
        json.get("Blogs")
            .cloned()
            .unwrap_or(serde_json::Value::Null),
    )
    .unwrap_or_default();

    // tracing::info!("{:?}", blogs);
    blogs
}
