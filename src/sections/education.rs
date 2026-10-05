use crate::{
    components::{SectionHeading, SectionLabel},
    content,
};
use dioxus::prelude::*;

#[component]
pub fn Education() -> Element {
    rsx! {
        section { class: "education-section section-space", id: "education", "aria-labelledby": "education-title",
            div { class: "container education-grid",
                div { class: "education-copy", SectionLabel { number: "06", label: content::copy::EDUCATION.label }
                    SectionHeading { id: "education-title", lines: content::copy::EDUCATION.heading }
                    p { class: "body-copy", "{content::copy::EDUCATION.paragraphs[0]}" }
                    p { class: "body-copy muted", "{content::copy::EDUCATION.paragraphs[1]}" }
                }
                div { class: "learning-list", p { class: "eyebrow", "{content::copy::LEARNING_KICKER}" }
                    ul { for (number, title) in content::LEARNING_AREAS { li { span { "{number}" } strong { "{title}" } } } }
                    p { class: "learning-note", "{content::copy::LEARNING_NOTE[0]}" br {} em { "{content::copy::LEARNING_NOTE[1]}" } }
                }
            }
        }
        section { class: "energy-section section-space", id: "energia", "aria-labelledby": "energy-title",
            div { class: "container energy-inner",
                p { class: "eyebrow", "{content::copy::ENERGY.label}" }
                SectionHeading { id: "energy-title", lines: content::copy::ENERGY.heading }
                p { class: "energy-qualities", "{content::copy::ENERGY.paragraphs[0]}" br {} "{content::copy::ENERGY.paragraphs[1]}" }
                p { class: "energy-closing", "{content::copy::ENERGY_CLOSING}" }
            }
        }
    }
}
