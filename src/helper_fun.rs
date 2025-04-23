use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn get_tech_logos_from_str(used_tech: &'static str) -> Element {
    let raw_data: TechDes = tech_table_lookup(used_tech);
    rsx! {
        img { src: "{raw_data.lang_logo}", alt: "{used_tech}'s logo/icon" }
    }
}

#[derive(PartialEq, Props, Clone, Copy)]
pub struct TechDes {
    pub lang_logo: &'static str,
    pub project_site: &'static str,
    pub skill_level: u8,
}

#[derive(PartialEq, Props, Clone)]
pub struct ProjectDes {
    website_prop: Option<&'static str>,
    github_prop: Option<&'static str>,
    project_name: &'static str,
    techs_used: Vec<&'static str>,
    project_des: &'static str,
}

pub fn tech_table_lookup(to_lookup: &str) -> TechDes {
    let techs_tools_frameworks_lookup = HashMap::from([
        (
            "Rust",
            TechDes {
                lang_logo: "https://www.rust-lang.org/static/images/rust-logo-blk.svg",
                project_site: "https://www.rust-lang.org",
                skill_level: 40,
            },
        ),
        (
            "Python",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/452091/python.svg",
                project_site: "https://www.python.org",
                skill_level: 50,
            },
        ),
        (
            "JavaScript",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/303206/javascript-logo.svg",
                project_site: "https://www.python.org",
                skill_level: 60,
            },
        ),
        (
            "YAML",
            TechDes {
                lang_logo: "https://yaml.org/favicon.svg",
                project_site: "https://yaml.org",
                skill_level: 95,
            },
        ),
        (
            "C",
            TechDes {
                lang_logo: "https://www.c-language.org/logo.svg",
                project_site: "https://www.c-language.org",
                skill_level: 30,
            },
        ),
        (
            "C++",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/452183/cpp.svg",
                project_site: "https://cplusplus.com",
                skill_level: 30,
            },
        ),
        (
            "Github",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/512317/github-142.svg",
                project_site: "https://github.com/darkicewolf50",
                skill_level: 80,
            },
        ),
        (
            "Email",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/491226/email.svg",
                project_site: "mailto:darkicewolf50@gmail.com",
                skill_level: 100,
            },
        ),
        (
            "LinkedIn",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/521725/linkedin.svg",
                project_site: "https://www.linkedin.com/in/brock-tomlinson/",
                skill_level: 40,
            },
        ),
        (
            "Twitch",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/519925/twitch.svg",
                project_site: "https://www.twitch.tv/darkicewolf50",
                skill_level: 60,
            },
        ),
        (
            "Youtube",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/521936/youtube.svg",
                project_site: "https://www.youtube.com/@darkicewolf50",
                skill_level: 40,
            },
        ),
        (
            "Internet",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/490809/internet.svg",
                project_site: "https://google.com",
                skill_level: 99,
            },
        ),
        (
            "React",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/452092/react.svg",
                project_site: "https://react.dev",
                skill_level: 60,
            },
        ),
        (
            "Docker",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/448221/docker.svg",
                project_site: "https://www.docker.com",
                skill_level: 70,
            },
        ),
        (
            "FastAPI",
            TechDes {
                lang_logo: "https://fastapi.tiangolo.com/img/favicon.png",
                project_site: "https://fastapi.tiangolo.com",
                skill_level: 80,
            },
        ),
        (
            "Actix",
            TechDes {
                lang_logo: "https://actix.rs/img/logo.png",
                project_site: "https://actix.rs",
                skill_level: 20,
            },
        ),
        (
            "HTML5",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/452228/html-5.svg",
                project_site: "https://google.com",
                skill_level: 90,
            },
        ),
        (
            "CSS",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/452185/css-3.svg",
                project_site: "https://google.com",
                skill_level: 40,
            },
        ),
        (
            "Git",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/452210/git.svg",
                project_site: "https://git-scm.com",
                skill_level: 50,
            },
        ),
        (
            "Github Actions",
            TechDes {
                lang_logo: "https://cdn.simpleicons.org/githubactions/2088FF",
                project_site: "https://github.com/",
                skill_level: 40,
            },
        ),
        (
            "Vs Code",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/452129/vs-code.svg",
                project_site: "https://code.visualstudio.com",
                skill_level: 60,
            },
        ),
        (
            "Gitea",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/349372/gitea.svg",
                project_site: "https://about.gitea.com",
                skill_level: 85,
            },
        ),
        (
            "AWS",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/448266/aws.svg",
                project_site: "https://aws.amazon.com",
                skill_level: 30,
            },
        ),
        (
            "Firefox",
            TechDes {
                lang_logo:
                    "https://www.svgrepo.com/show/378808/firefox-developer-edition-57-70.svg",
                project_site: "https://www.mozilla.org/en-CA/firefox/developer/",
                skill_level: 80,
            },
        ),
        (
            "Markdown",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/510065/markdown.svg",
                project_site: "https://www.markdownguide.org",
                skill_level: 90,
            },
        ),
        (
            "Prettier",
            TechDes {
                lang_logo: "https://prettier.io/icon.png
",
                project_site: "https://prettier.io",
                skill_level: 90,
            },
        ),
        (
            "DynamoDB",
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/473526/amazondynamodb.svg",
                project_site: "https://aws.amazon.com/dynamodb/",
                skill_level: 20,
            },
        ),
        (
            "Cloudflare", 
            TechDes {
                lang_logo: "https://qualified-production.s3.us-east-1.amazonaws.com/uploads/3b522ef84c409e4457032e4b4e3b984abbc92522c6f100f4ccc55c0ccfd3062b.png", 
                project_site: "https://www.cloudflare.com/en-ca/", 
                skill_level: 35,
            },
        ),
        (
            "Netlify",
            TechDes {
                lang_logo: "https://qualified-production.s3.us-east-1.amazonaws.com/uploads/0f63ae7280d8d193e346973a1915bf99aea8c63e254eb062bad0bde99b43a9b7.png",
                project_site: "https://www.netlify.com",
                skill_level: 34,
            },
        ),
        (
            "Vercel", 
            TechDes {
                lang_logo: "https://www.svgrepo.com/show/361653/vercel-logo.svg", 
                project_site: "https://vercel.com/home", 
                skill_level: 30
            },
        ), 
        (
            "Dioxus",
            TechDes {
                lang_logo: "https://dioxuslabs.com/assets/smalllogo-b1926fd214dc8427.png",
                project_site: "https://dioxuslabs.com",
                skill_level: 40,
            },
        ),
        (
            "Vue",
            TechDes {
                lang_logo: "https://vuejs.org/logo.svg",
                project_site: "https://vuejs.org",
                skill_level: 1,
            },
        ),
    ]);

    techs_tools_frameworks_lookup[to_lookup]
}
