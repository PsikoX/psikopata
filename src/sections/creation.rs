use crate::{
    components::{ResponsiveImage, SectionHeading, SectionLabel},
    content,
};
use dioxus::prelude::*;

#[component]
pub fn Creation() -> Element {
    rsx! {
        section { class: "creation-section section-space", id: "creation", "aria-labelledby": "creation-title",
            div { class: "container",
                SectionLabel { number: "05", label: content::copy::CREATION.label }
                div { class: "creation-heading", SectionHeading { id: "creation-title", lines: content::copy::CREATION.heading } p { class: "body-copy", "{content::copy::CREATION.paragraphs[0]}" } }
                div { class: "creation-board",
                    div { class: "creation-photo creation-photo-one", ResponsiveImage { image: content::ZOE, sizes: "(min-width: 900px) 24vw, 45vw" } }
                    div { class: "creation-statement", span { class: "eyebrow", "REAL × IMAGINADO" } p { "{content::copy::CREATION_HEADLINE[0]}" br {} "{content::copy::CREATION_HEADLINE[1]}" br {} em { "{content::copy::CREATION_HEADLINE[2]}" } } span { class: "creation-cross", "aria-hidden": "true", "✦" } }
                    div { class: "creation-photo creation-photo-two", ResponsiveImage { image: content::KAREN, sizes: "(min-width: 900px) 24vw, 45vw" } }
                }
                ul { class: "skill-tags", "aria-label": "Áreas del laboratorio creativo", for skill in content::CREATION_SKILLS { li { "{skill}" } } }
                p { class: "creative-principle", "{content::copy::CREATION_PRINCIPLE}" }
            }
        }
    }
}
