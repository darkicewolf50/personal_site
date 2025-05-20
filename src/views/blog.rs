use crate::Route;
use dioxus::{logger::tracing, prelude::*};
use reqwest;
use serde::{Deserialize, Serialize};

// const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
///
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.
#[component]
pub fn Blog(blog_title: String) -> Element {
    let blog_content = use_signal(move || {
        "<h1>This is a blog #blog_title</h1><p>
            In blog #{blog_title}, we show how the Dioxus router works and 
            how URL parameters can be passed as props to our route components.
        </p>"
            .to_string()
    });
    // let blog_content = use_signal(move || BlogContent {
    //     blog_file_name: "blog_title".to_string(),
    //     blog_title: "This is a blog #blog_title".to_string(),
    //     date_last_edit: "2025-5-20".to_string(),
    //     tags: "#test".to_string(),
    //     html_blog_content: "<p>
    //         In blog #{blog_title}, we show how the Dioxus router works and
    //         how URL parameters can be passed as props to our route components.
    //     </p>"
    //         .to_string(),
    // });
    let blog = blog_content();

    // let last_edit = &blog.date_last_edit;
    // let tag = &blog.tags;

    rsx! {
        document::Stylesheet { href: asset!("/assets/styling/blog.css") }
        document::Title { "Brock Tomlinson - {blog_title}" }
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
            // div { dangerous_inner_html: *&blog.html_blog_content.as_str(),
            //     h1 { "{blog_title}" }
            //     div {
            //         p { "{&blog.tags}" }
            //         p { "{&blog.date_last_edit}" }
            //     }
            // }
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

    let blogs_resource: Resource<Vec<BlogContent>> = use_resource(move || async move {
        get_blogs_preview(_num_limit(), page_num)
            .await
            .unwrap_or_else(|_| vec![])
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/styling/blog.css") }
        div { id: "blogs",
            document::Title { "Brock Tomlinson - Blogs" }
            div { id: "blogs-title",
                h1 { "Blogs" }
                p {
                    "This is a collection of blog posts, ranging from tutorials, technologies I found interesting, and opinion pieces"
                }
                p { "These blogs are my opinion and mine alone" }
            }
            // Link { to: Route::Home {},
            //     button { "Home" }
            // }
            // Link {
            //     to: Route::Blog {
            //         blog_title: "Test_Blog".to_string(),
            //     },
            //     button { "To Test Blog" }
            // }
            div {
                if let Some(blogs) = &*blogs_resource.read() {
                    if blogs.len() > 0 {
                        for blog in blogs.iter() {

                            Link {
                                to: Route::Blog {
                                    blog_title: blog.blog_file_name.clone(),
                                },
                                div { dangerous_inner_html: blog.html_blog_content.as_str(),
                                    h1 { "{blog.blog_title}" }
                                    div {
                                        p { "{blog.tags}" }
                                        p { "{blog.date_last_edit}" }
                                    }
                                }
                            }
                        }
                    } else {
                        div { id: "blog-out-of",
                            p { "No more blogs available" }
                            Link { to: Route::Blogs { page_num: 0 },
                                button { "Go Back" }
                            }
                        }
                    }
                } else {
                    div { id: "blog-loading",
                        p { "Loading blogs..." }
                    }
                }
            }
            div { id: "blog-nav",
                if page_num > 0 {
                    Link {
                        to: Route::Blogs {
                            page_num: page_num - 1,
                        },
                        button { "<-- Go Back" }
                    }
                }
                div {
                    label { "display: " }
                    select {
                        onchange: move |event| {
                            tracing::info!("Change happened {:?}", event.value());
                            _num_limit.set(event.value().parse::<u8>().unwrap_or(10));
                        },
                        option { "10" }
                        option { "25" }
                        option { "50" }
                        option { "100" }
                    }
                }
                Link {
                    to: Route::Blogs {
                        page_num: page_num + 1,
                    },
                    button { "Next -->" }
                }

            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct BlogContent {
    pub blog_file_name: String,
    pub date_last_edit: String,
    pub blog_title: String,
    pub tags: String,
    pub html_blog_content: String,
}

async fn get_blogs_preview(
    _num_limit: u8,
    page_num: u32,
) -> Result<Vec<BlogContent>, reqwest::Error> {
    let client = reqwest::Client::new();

    let res: String = client
        .get(format!(
            "http://localhost:8000/blogs/{}/{}",
            _num_limit, page_num
        ))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await?
        .text()
        .await?;

    let json: serde_json::Value = serde_json::from_str(&res).unwrap();

    // Extract the "Blogs" array and deserialize it into Vec<BlogContent>
    let blogs: Vec<BlogContent> = serde_json::from_value(
        json.get("Blogs")
            .cloned()
            .unwrap_or(serde_json::Value::Null),
    )
    .unwrap_or_default();

    // tracing::info!("{:?}", blogs);
    Ok(blogs)
}
