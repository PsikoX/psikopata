use crate::{
    components::{ButtonLink, SectionLabel},
    content,
    layouts::ClosingInvitation,
};
use dioxus::prelude::*;

#[component]
pub fn Fer() -> Element {
    rsx! {
        section { class: "fer-page-hero section-space", "aria-labelledby": "fer-page-title",
            div { class: "container", a { class: "text-link back-link", href: "/#fer", "VOLVER AL UNIVERSO" }
                SectionLabel { number: "F / E / R", label: content::copy::FER_STATUS }
                h1 { id: "fer-page-title", for line in content::copy::FER_HEADING { span { class: "editorial-line", "{line.before}" if let Some(text) = line.emphasis { em { "{text}" } } "{line.after}" } } }
                p { class: "fer-page-intro", "{content::copy::FER_INTRO[0]}" br {} "{content::copy::FER_INTRO[1]}" }
                p { class: "fer-page-name", "FER" }
                p { class: "fer-meaning", span { "FETICHE" } span { "EDUCACIÓN" } span { "RELIGIÓN" } }
            }
        }
        section { class: "fer-chapters", "aria-label": "Los tres elementos de FER",
            div { class: "container",
                for chapter in content::FER_CHAPTERS {
                    article { class: "fer-chapter", id: chapter.title,
                        span { class: "fer-chapter-letter", "aria-hidden": "true", "{chapter.letter}" }
                        div { class: "fer-chapter-copy", p { class: "eyebrow", "{chapter.title}" } h2 { "{chapter.subtitle}" } p { class: "body-copy", "{chapter.text}" }
                            ul { class: "fer-subjects", for subject in chapter.subjects { li { "{subject}" } } }
                        }
                    }
                }
            }
        }
        section { class: "fer-page-closing section-space", div { class: "container", p { "{content::copy::FER_CLOSING[0]}" br {} em { "{content::copy::FER_CLOSING[1]}" } } p { class: "body-copy", "{content::copy::FER_DEVELOPMENT}" } ClosingInvitation {} ButtonLink { href: "/", label: "ENTRAR AL UNIVERSO", secondary: true } } }
    }
}
