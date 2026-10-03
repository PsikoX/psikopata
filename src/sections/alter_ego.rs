use crate::{
    components::{SectionHeading, SectionLabel},
    content,
};
use dioxus::prelude::*;

#[component]
pub fn AlterEgo() -> Element {
    rsx! {
        section { class: "alter-section section-space", id: "alter-ego", "aria-labelledby": "alter-title",
            div { class: "container alter-grid",
                div { class: "alter-copy", SectionLabel { number: "04", label: content::copy::ALTER_EGO.label }
                    SectionHeading { id: "alter-title", lines: content::copy::ALTER_EGO.heading }
                    p { class: "body-copy", "{content::copy::ALTER_EGO.paragraphs[0]}" }
                    p { class: "alter-footnote", "{content::copy::ALTER_EGO_FOOTNOTE}" }
                }
                ol { class: "identity-ladder", "aria-label": "Exploración creativa de identidad",
                    for (index, stage) in content::ALTER_EGO_STAGES.iter().enumerate() {
                        li { class: if index == 1 { "identity-active" } else { "" }, span { class: "identity-index", "0{index + 1}" } span { class: "identity-name", "{stage}" } span { class: "identity-symbol", "aria-hidden": "true", if index == 1 { "✦" } else { "+" } } }
                    }
                }
            }
        }
    }
}
