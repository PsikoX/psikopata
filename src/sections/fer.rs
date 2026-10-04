use crate::{
    components::{ButtonLink, SectionLabel},
    content,
};
use dioxus::prelude::*;

#[component]
pub fn FerReveal() -> Element {
    rsx! {
        section { class: "fer-section section-space", id: "fer", "aria-labelledby": "fer-title",
            div { class: "container fer-inner", SectionLabel { number: "00", label: content::copy::FER_LABEL }
                p { class: "fer-question", "{content::copy::FER_QUESTION}" }
                h2 { class: "fer-word", id: "fer-title", "aria-label": "FER",
                    span { class: "fer-glyph fer-glyph-f", "aria-hidden": "true", "F" }
                    span { class: "fer-glyph fer-glyph-e", "aria-hidden": "true", "E" }
                    span { class: "fer-glyph fer-glyph-r", "aria-hidden": "true", "R" }
                }
                p { class: "fer-meaning", span { "FETICHE" } span { "EDUCACIÓN" } span { "RELIGIÓN" } }
                p { class: "fer-trident", "EL TRIDENTE DE LA PROSPERIDAD" }
                p { class: "fer-status", "{content::copy::FER_STATUS}" }
                ButtonLink { href: "/fer/", label: "DESCUBRIR FER", secondary: true }
            }
        }
    }
}
