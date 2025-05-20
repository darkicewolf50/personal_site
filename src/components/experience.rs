use dioxus::{document, prelude::*};

#[component]
pub fn Experience(professional_jobs: bool) -> Element {
    let experience: [ExpDes; 4] = match professional_jobs {
        true => EXPERIENCE_JOBS,
        false => EXPERIENCE_VOL,
    };
    rsx! {
        div { class: "experience-comp",
            document::Stylesheet { href: asset!("/assets/styling/experience.css") }
            if professional_jobs {
                h3 { "Professional" }
            } else {
                h3 { "Volunteering" }
            }
            table {
                colgroup {
                    col { class: "symbol" }
                    col { class: "postion" }
                    col { class: "date-location" }
                }
                tbody {
                    for exp in experience {
                        tr {
                            td { class: "symbol", rowspan: 2,
                                span { class: "dot", "" }
                            }
                            td { "{exp.postition}" }
                            td { "{exp.start_month} - {exp.end_month}" }
                        }
                        tr {
                            td { "{exp.company}" }
                            td { "{exp.location}" }
                        }
                    }
                }
            }

        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct ExpDes {
    pub postition: &'static str,
    pub company: &'static str,
    pub location: &'static str,
    pub start_month: &'static str,
    pub end_month: &'static str,
}

const EXPERIENCE_JOBS: [ExpDes; 4] = [
    ExpDes {
        postition: "Project Coordinator",
        company: "Rally Engineering",
        location: "Ab",
        start_month: "Jan 2025",
        end_month: "May 2025",
    },
    ExpDes {
        postition: "Project Controller Student",
        company: "Rally Engineering",
        location: "Ab",
        start_month: "May 2024",
        end_month: "Jan 2025",
    },
    ExpDes {
        postition: "Staff",
        company: "Red Deer Farmer's Market",
        location: "Ab",
        start_month: "Mar 2013",
        end_month: "Present",
    },
    ExpDes {
        postition: "Staff",
        company: "Ghostrider Storage",
        location: "BC",
        start_month: "Mar 2020",
        end_month: "Present",
    },
];

const EXPERIENCE_VOL: [ExpDes; 4] = [
    ExpDes {
        postition: "Software Subteam Lead",
        company: "UCalgary Baja",
        location: "Ab",
        start_month: "Sept 2024",
        end_month: "Present",
    },
    ExpDes {
        postition: "Software, Logistics and Business Sub Team Lead",
        company: "Schulich Off-Road",
        location: "Ab",
        start_month: "May 2023",
        end_month: "Sept 2024",
    },
    ExpDes {
        postition: "Chassis Junior Member",
        company: "Schulich Off-Road",
        location: "Ab",
        start_month: "Sept 2022",
        end_month: "May 2023",
    },
    ExpDes {
        postition: "Coaching with Elk Valley Special Olympics",
        company: "Elk Valley Dolphins",
        location: "BC",
        start_month: "May 2019",
        end_month: "May 2019",
    },
];
