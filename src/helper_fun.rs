use dioxus::prelude::*;
use std::{collections::HashMap, rc::Rc};

#[component]
pub fn get_tech_logos_from_str(used_tech: &'static str) -> Element {
    let raw_data: TechDes = *tech_table_lookup(used_tech);
    rsx! {
        img { src: "{raw_data.tech_logo}", alt: "{used_tech}'s logo/icon" }
    }
}

#[derive(PartialEq, Props, Clone, Copy)]
pub struct TechDes {
    pub tech_logo: &'static str,
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

pub fn tech_table_lookup(to_lookup: &str) -> Rc<TechDes> {
    let techs_tools_frameworks_lookup: Rc<HashMap<&'static str, TechDes>> = HashMap::from([
        (
            "Rust",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/374056/rust.svg",
                project_site: "https://www.rust-lang.org",
                skill_level: 60,
            },
        ),

        (
            "Python",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/452091/python.svg",
                project_site: "https://www.python.org",
                skill_level: 50,
            },
        ),
        (
            "JavaScript",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/303206/javascript-logo.svg",
                project_site: "https://www.python.org",
                skill_level: 60,
            },
        ),
        (
            "YAML",
            TechDes {
                tech_logo: "https://yaml.org/favicon.svg",
                project_site: "https://yaml.org",
                skill_level: 95,
            },
        ),
        (
            "C",
            TechDes {
                tech_logo: "https://www.c-language.org/logo.svg",
                project_site: "https://www.c-language.org",
                skill_level: 49,
            },
        ),
        (
            "C++",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/452183/cpp.svg",
                project_site: "https://cplusplus.com",
                skill_level: 49,
            },
        ),
        (
            "Github",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/512317/github-142.svg",
                project_site: "https://github.com/darkicewolf50",
                skill_level: 80,
            },
        ),
        (
            "Email",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/491226/email.svg",
                project_site: "mailto:darkicewolf50@gmail.com",
                skill_level: 100,
            },
        ),
        (
            "LinkedIn",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/521725/linkedin.svg",
                project_site: "https://www.linkedin.com/in/brock-tomlinson/",
                skill_level: 40,
            },
        ),
        (
            "Twitch",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/519925/twitch.svg",
                project_site: "https://www.twitch.tv/darkicewolf50",
                skill_level: 60,
            },
        ),
        (
            "Youtube",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/521936/youtube.svg",
                project_site: "https://www.youtube.com/@darkicewolf50",
                skill_level: 40,
            },
        ),
        (
            "Internet",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/490809/internet.svg",
                project_site: "https://google.com",
                skill_level: 99,
            },
        ),
        (
            "React",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/452092/react.svg",
                project_site: "https://react.dev",
                skill_level: 70,
            },
        ),
        (
            "Docker",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/448221/docker.svg",
                project_site: "https://www.docker.com",
                skill_level: 70,
            },
        ),
        (
            "FastAPI",
            TechDes {
                tech_logo: "https://fastapi.tiangolo.com/img/favicon.png",
                project_site: "https://fastapi.tiangolo.com",
                skill_level: 80,
            },
        ),
        (
            "Actix",
            TechDes {
                tech_logo: "https://actix.rs/img/logo.png",
                project_site: "https://actix.rs",
                skill_level: 20,
            },
        ),
        (
            "HTML5",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/452228/html-5.svg",
                project_site: "https://google.com",
                skill_level: 90,
            },
        ),
        (
            "CSS",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/452185/css-3.svg",
                project_site: "https://google.com",
                skill_level: 65,
            },
        ),
        (
            "Git",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/452210/git.svg",
                project_site: "https://git-scm.com",
                skill_level: 55,
            },
        ),
        (
            "Github Actions",
            TechDes {
                tech_logo: "https://cdn.simpleicons.org/githubactions/2088FF",
                project_site: "https://github.com/",
                skill_level: 50,
            },
        ),
        (
            "Vs Code",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/452129/vs-code.svg",
                project_site: "https://code.visualstudio.com",
                skill_level: 60,
            },
        ),
        (
            "Gitea",
            TechDes {
                tech_logo: "https://about.gitea.com/gitea.png",
                project_site: "https://about.gitea.com",
                skill_level: 85,
            },
        ),
        (
            "AWS",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/448266/aws.svg",
                project_site: "https://aws.amazon.com",
                skill_level: 30,
            },
        ),
        (
            "Firefox",
            TechDes {
                tech_logo:
                    "https://www.svgrepo.com/show/378808/firefox-developer-edition-57-70.svg",
                project_site: "https://www.mozilla.org/en-CA/firefox/developer/",
                skill_level: 80,
            },
        ),
        (
            "Markdown",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/510065/markdown.svg",
                project_site: "https://www.markdownguide.org",
                skill_level: 90,
            },
        ),
        (
            "Prettier",
            TechDes {
                tech_logo: "https://prettier.io/icon.png",
                project_site: "https://prettier.io",
                skill_level: 90,
            },
        ),
        (
            "DynamoDB",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/473526/amazondynamodb.svg",
                project_site: "https://aws.amazon.com/dynamodb/",
                skill_level: 70,
            },
        ),
        (
            "Cloudflare", 
            TechDes {
                tech_logo: "https://qualified-production.s3.us-east-1.amazonaws.com/uploads/3b522ef84c409e4457032e4b4e3b984abbc92522c6f100f4ccc55c0ccfd3062b.png", 
                project_site: "https://www.cloudflare.com/en-ca/", 
                skill_level: 65,
            },
        ),
        (
            "Netlify",
            TechDes {
                tech_logo: "https://qualified-production.s3.us-east-1.amazonaws.com/uploads/0f63ae7280d8d193e346973a1915bf99aea8c63e254eb062bad0bde99b43a9b7.png",
                project_site: "https://www.netlify.com",
                skill_level: 60,
            },
        ),
        (
            "Vercel",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/361653/vercel-logo.svg",
                project_site: "https://vercel.com/home",
                skill_level: 60,
            },
        ),
        (
            "Dioxus",
            TechDes {
                tech_logo: "https://dioxuslabs.com/assets/smalllogo-b1926fd214dc8427.png",
                project_site: "https://dioxuslabs.com",
                skill_level: 70,
            },
        ),
        (
            "Vue",
            TechDes {
                tech_logo: "https://vuejs.org/logo.svg",
                project_site: "https://vuejs.org",
                skill_level: 1,
            },
        ),
        (
            "Mongodb",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/331488/mongodb.svg",
                project_site: "https://www.mongodb.com",
                skill_level: 10,
            },
        ),
        (
            "Sqlite",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/374094/sqlite.svg",
                project_site: "https://www.sqlite.org",
                skill_level: 10,
            },
        ),
        (
            "PostgreSQL",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/303301/postgresql-logo.svg",
                project_site: "https://www.postgresql.org",
                skill_level: 10,
            },
        ),
        (
            "Diesel",
            TechDes {
                tech_logo: "https://res.cloudinary.com/dpgrgsh7g/image/upload/v1745443276/diesel_logo_ujtvia.png",
                project_site: "https://diesel.rs",
                skill_level: 10,
            },
        ),
        (
            "Kubernetes",
            TechDes {
                tech_logo: "https://kubernetes.io/images/kubernetes.png",
                project_site: "https://kubernetes.io",
                skill_level: 5,
            },
        ),
        (
            "Terraform",
            TechDes {
                tech_logo: "https://www.svgrepo.com/show/448253/terraform.svg",
                project_site: "https://www.terraform.io",
                skill_level: 15,
            },
        ),
        (
            "Traefik",
            TechDes {
                tech_logo: "https://hub.docker.com/api/media/repos_logo/v1/library%2Ftraefik",
                project_site: "https://traefik.io/traefik/",
                skill_level: 60,
            },
        ),
    ]).into();
    techs_tools_frameworks_lookup[to_lookup].into()
}

// pub fn techs_tools_frameworks_lookup_create() -> HashMap<&'static str, TechDes> {

//     techs_tools_frameworks_lookup
// }
