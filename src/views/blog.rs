use crate::{set_meta_tags, Route};
// use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};

// const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
///
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.
#[component]
pub fn Blog(blog_title: String) -> Element {
    let blog_resource = use_resource({
        let title = blog_title.clone();
        move || {
            let value = title.clone();
            async move {
                get_blog(value).await.unwrap_or(BlogContent {
                    blog_file_name: String::new(),
                    date_last_edit: "9999-12-01".to_string(),
                    blog_title: "Not Found".to_string(),
                    tags: vec!["#error".to_string()],
                    html_blog_content: "<p>Blog not found</p>".to_string(),
                })
            }
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/styling/blog.css") }
        document::Title { "Brock Tomlinson - {blog_title.clone()}" }
        document::Meta { name: "author", content: "Brock Tomlinson" }

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
            if let Some(blog_content) = &*blog_resource.read() {
                document::Meta { name: "description", content: "{blog_content.blog_title}" }
                document::Meta {
                    name: "keywords",
                    content: "webdev software engineer fullstack",
                }
                div { id: "blog_info",
                    h1 { "{blog_content.blog_title}" }
                    div {
                        div {
                            for tag in &blog_content.tags {
                                p { "{tag}" }
                            }
                        }
                        p { "{&blog_content.date_last_edit}" }
                    }
                }
                div {
                    id: "blog_content",
                    dangerous_inner_html: *&blog_content.html_blog_content.as_str(),
                }
            } else {
                p { "Loading..." }
            }
        }
    }
}

async fn get_blog(blog_name: String) -> Result<BlogContent, reqwest::Error> {
    let client = reqwest::Client::new();

    let res = client
        .get(format!("http://localhost:8000/blogs/blog/{}", blog_name))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await?
        .json::<BlogContent>()
        // .text()
        .await?;

    // tracing::info!("{:?}", res);
    Ok(res)

    // to be removed
    // let blog = serde_json::from_str(&res).unwrap_or(BlogContent {
    //     blog_file_name: "blog_title".to_string(),
    //     blog_title: "This is a blog #blog_title".to_string(),
    //     date_last_edit: "2025-5-20".to_string(),
    //     tags: "#test".to_string(),
    //     html_blog_content: "<p>
    //                 In blog #{blog_title}, we show how the Dioxus router works and
    //                 how URL parameters can be passed as props to our route components.
    //             </p>"
    //         .to_string(),
    // });
    // Ok(blog)
    // Ok(BlogContent {
    //     blog_file_name: "blog_title".to_string(),
    //     blog_title: "This is a blog #blog_title".to_string(),
    //     date_last_edit: "2025-5-20".to_string(),
    //     tags: "#test".to_string(),
    //     html_blog_content: "<p>
    //         In blog #{blog_title}, we show how the Dioxus router works and
    //         how URL parameters can be passed as props to our route components.
    //     </p>"
    //         .to_string(),
    // })
}

#[component]
pub fn Blogs(page_num: u32) -> Element {
    let mut _num_limit: Signal<u8> = use_signal(|| 10);

    let blogs_resource: Resource<Vec<BlogContent>> =
        use_resource(use_reactive!(|(_num_limit, page_num)| async move {
            get_blogs_preview(_num_limit(), page_num)
                .await
                .unwrap_or_else(|_| vec![])
        }));
    rsx! {
        document::Stylesheet { href: asset!("/assets/styling/blog.css") }
        div { id: "blogs",
            document::Title { "Brock Tomlinson - Blogs" }
            set_meta_tags {
                description: "This is a collection of blog posts, ranging from tutorials, technologies I found interesting, and opinion pieces",
                keywords: "blogs blog software engineer webdev",
            }
            div { id: "blogs-title",
                h1 { "Blogs" }
                p {
                    "This is a collection of blog posts, ranging from tutorials, technologies I found interesting, and opinion pieces"
                }
                p { "These blogs are my opinion and mine alone" }
            }
            div { id: "blogs-on-show",
                if let Some(blogs) = &*blogs_resource.read() {
                    if blogs.len() > 0 {
                        for blog in blogs.iter() {

                            Link {
                                class: "blog-preview",
                                to: Route::Blog {
                                    blog_title: blog.blog_file_name.clone(),
                                },
                                div { id: "blog_info",
                                    h1 { "{blog.blog_title}" }
                                    div {
                                        div {
                                            for tag in &blog.tags {
                                                p { "{tag}" }
                                            }
                                        }
                                        p { "{&blog.date_last_edit}" }
                                    }
                                }
                                div {
                                    id: "blog_content",
                                    dangerous_inner_html: *&blog.html_blog_content.as_str(),
                                }
                                button { "Read More Here" }
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
    pub tags: Vec<String>,
    pub html_blog_content: String,
}

async fn get_blogs_preview(
    _num_limit: u8,
    page_num: u32,
) -> Result<Vec<BlogContent>, reqwest::Error> {
    let client = reqwest::Client::new();

    let res = client
        .get(format!(
            "http://localhost:8000/blogs/{}/{}",
            _num_limit, page_num
        ))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await?
        .json::<Vec<BlogContent>>()
        // .text()
        .await?;
    Ok(res)
    // let json: serde_json::Value = serde_json::from_str(&res).unwrap();
    // let blogs: Vec<BlogContent> = serde_json::from_value(json).unwrap_or_default();

    // // Extract the "Blogs" array and deserialize it into Vec<BlogContent>
    // let blogs: Vec<BlogContent> = serde_json::from_value(
    //     json.get("Blogs")
    //         .cloned()
    //         .unwrap_or(serde_json::Value::Null),
    // )
    // .unwrap_or_default();
}
