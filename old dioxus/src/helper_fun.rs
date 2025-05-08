use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn get_tech_logos_from_str(used_tech: &'static str) -> Element {
    let raw_data: TechDes = *tech_table_lookup(used_tech);
    rsx! {
        img { src: "{raw_data.tech_logo}", alt: "{used_tech}'s logo/icon" }
    }
}

#[derive(PartialEq, Props, Clone, Copy)]
pub struct TechDes {
    pub tech_name: &'static str,
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
    let mut tech_to_return: TechDes = TechDes {
                tech_name: "Not in table",
                tech_logo: "",
                project_site: "",
                skill_level: 0,
            }; 
            
    for tech in TECH_TABLE {
        if tech.tech_name == to_lookup {
            tech_to_return = tech.into();
        }
    }
    tech_to_return.into()
}

const TECH_TABLE:[TechDes; 37] = [
            TechDes {
                tech_name: "Rust",
                tech_logo: "https://www.svgrepo.com/show/374056/rust.svg",
                project_site: "https://www.rust-lang.org",
                skill_level: 60,
            },
            TechDes {
                tech_name: "Python",
                tech_logo: "https://www.svgrepo.com/show/452091/python.svg",
                project_site: "https://www.python.org",
                skill_level: 50,
            },
            
            TechDes {
                tech_name: "JavaScript",
                tech_logo: "https://www.svgrepo.com/show/303206/javascript-logo.svg",
                project_site: "https://www.python.org",
                skill_level: 60,
            },
            
            TechDes {
                tech_name: "YAML",
                tech_logo: "https://yaml.org/favicon.svg",
                project_site: "https://yaml.org",
                skill_level: 95,
            },
            TechDes {
                tech_name: "Github",
                tech_logo: "https://www.svgrepo.com/show/512317/github-142.svg",
                project_site: "https://github.com/darkicewolf50",
                skill_level: 80,
            },
            TechDes {
                tech_name: "Email",
                tech_logo: "https://www.svgrepo.com/show/491226/email.svg",
                project_site: "mailto:darkicewolf50@gmail.com",
                skill_level: 100,
            },

            TechDes {
                tech_name: "LinkedIn",
                tech_logo: "https://www.svgrepo.com/show/521725/linkedin.svg",
                project_site: "https://www.linkedin.com/in/brock-tomlinson/",
                skill_level: 40,
            },
            TechDes {
                tech_name: "Twitch",
                tech_logo: "https://www.svgrepo.com/show/519925/twitch.svg",
                project_site: "https://www.twitch.tv/darkicewolf50",
                skill_level: 60,
            },
            TechDes {
                tech_name: "Youtube",
                tech_logo: "https://www.svgrepo.com/show/521936/youtube.svg",
                project_site: "https://www.youtube.com/@darkicewolf50",
                skill_level: 40,
            },
            TechDes {
                tech_name: "Internet",
                tech_logo: "https://www.svgrepo.com/show/490809/internet.svg",
                project_site: "https://google.com",
                skill_level: 99,
            },
            TechDes {
                tech_name: "React",
                tech_logo: "https://www.svgrepo.com/show/452092/react.svg",
                project_site: "https://react.dev",
                skill_level: 70,
            },
            TechDes {
                tech_name: "Docker",
                tech_logo: "https://www.svgrepo.com/show/448221/docker.svg",
                project_site: "https://www.docker.com",
                skill_level: 70,
            },
            TechDes {
                tech_name: "FastAPI",
                tech_logo: "https://fastapi.tiangolo.com/img/favicon.png",
                project_site: "https://fastapi.tiangolo.com",
                skill_level: 80,
            },
            
            TechDes {
                tech_name: "Actix",
                tech_logo: "https://actix.rs/img/logo.png",
                project_site: "https://actix.rs",
                skill_level: 20,
            },
            TechDes {
                tech_name: "HTML5",
                tech_logo: "https://www.svgrepo.com/show/452228/html-5.svg",
                project_site: "https://google.com",
                skill_level: 90,
            },
            TechDes {
                tech_name: "CSS",
                tech_logo: "https://www.svgrepo.com/show/452185/css-3.svg",
                project_site: "https://google.com",
                skill_level: 65,
            },
            TechDes {
                tech_name: "Git",
                tech_logo: "https://www.svgrepo.com/show/452210/git.svg",
                project_site: "https://git-scm.com",
                skill_level: 55,
            },
            TechDes {
                tech_name: "Github Actions",
                tech_logo: "https://cdn.simpleicons.org/githubactions/2088FF",
                project_site: "https://github.com/",
                skill_level: 50,
            },
            TechDes {
                tech_name: "Vs Code",
                tech_logo: "https://www.svgrepo.com/show/452129/vs-code.svg",
                project_site: "https://code.visualstudio.com",
                skill_level: 60,
            },
            TechDes {
                tech_name: "Gitea",
                tech_logo: "https://about.gitea.com/gitea.png",
                project_site: "https://about.gitea.com",
                skill_level: 85,
            },
            TechDes {
                tech_name: "AWS",
                tech_logo: "https://www.svgrepo.com/show/448266/aws.svg",
                project_site: "https://aws.amazon.com",
                skill_level: 30,
            },
            TechDes {
                tech_name: "Firefox",
                tech_logo: "https://www.svgrepo.com/show/378808/firefox-developer-edition-57-70.svg",
                project_site: "https://www.mozilla.org/en-CA/firefox/developer/",
                skill_level: 80,
            },
            TechDes {
                tech_name: "Markdown",
                tech_logo: "https://www.svgrepo.com/show/510065/markdown.svg",
                project_site: "https://www.markdownguide.org",
                skill_level: 90,
            },
            TechDes {
                tech_name: "Prettier",
                tech_logo: "https://prettier.io/icon.png",
                project_site: "https://prettier.io",
                skill_level: 90,
            },
            TechDes {
                tech_name: "Cloudflare",
                tech_logo: "https://qualified-production.s3.us-east-1.amazonaws.com/uploads/3b522ef84c409e4457032e4b4e3b984abbc92522c6f100f4ccc55c0ccfd3062b.png", 
                project_site: "https://www.cloudflare.com/en-ca/", 
                skill_level: 65,
            },
            TechDes {
                tech_name: "Netlify",
                tech_logo: "https://qualified-production.s3.us-east-1.amazonaws.com/uploads/0f63ae7280d8d193e346973a1915bf99aea8c63e254eb062bad0bde99b43a9b7.png",
                project_site: "https://www.netlify.com",
                skill_level: 60,
            },
            TechDes {
                tech_name: "Vercel",
                tech_logo: "https://www.svgrepo.com/show/361653/vercel-logo.svg",
                project_site: "https://vercel.com/home",
                skill_level: 60,
            },
            TechDes {
                tech_name: "Dioxus",
                tech_logo: "https://dioxuslabs.com/assets/smalllogo-b1926fd214dc8427.png",
                project_site: "https://dioxuslabs.com",
                skill_level: 70,
            },
            TechDes {
                tech_name: "Vue",
                tech_logo: "https://vuejs.org/logo.svg",
                project_site: "https://vuejs.org",
                skill_level: 1,
            },
            TechDes {
                tech_name: "Mongodb",
                tech_logo: "https://www.svgrepo.com/show/331488/mongodb.svg",
                project_site: "https://www.mongodb.com",
                skill_level: 10,
            },
            TechDes {
                tech_name: "Sqlite",
                tech_logo: "https://www.svgrepo.com/show/374094/sqlite.svg",
                project_site: "https://www.sqlite.org",
                skill_level: 10,
            },
            TechDes {
                tech_name: "PostgreSQL",
                tech_logo: "https://www.svgrepo.com/show/303301/postgresql-logo.svg",
                project_site: "https://www.postgresql.org",
                skill_level: 10,
            },
            TechDes {
                tech_name: "DynamoDB",
                tech_logo: "https://www.svgrepo.com/show/473526/amazondynamodb.svg",
                project_site: "https://aws.amazon.com/dynamodb/",
                skill_level: 70,
            }, 
            TechDes {
                tech_name: "Diesel",
                tech_logo: "https://res.cloudinary.com/dpgrgsh7g/image/upload/v1745443276/diesel_logo_ujtvia.png",
                project_site: "https://diesel.rs",
                skill_level: 10,
            },
            TechDes {
                tech_name: "Kubernetes",
                tech_logo: "https://kubernetes.io/images/kubernetes.png",
                project_site: "https://kubernetes.io",
                skill_level: 5,
            },
            TechDes {
                tech_name: "Terraform",
                tech_logo: "https://www.svgrepo.com/show/448253/terraform.svg",
                project_site: "https://www.terraform.io",
                skill_level: 15,
            },
            TechDes {
                tech_name: "Traefik",
                tech_logo: "https://hub.docker.com/api/media/repos_logo/v1/library%2Ftraefik",
                project_site: "https://traefik.io/traefik/",
                skill_level: 60,
            },
    ];
