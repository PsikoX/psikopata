use crate::{
    components::{BrandText, ButtonLink, ResponsiveImage, SectionHeading, SectionLabel},
    content,
};
use dioxus::prelude::*;

#[component]
pub fn Community() -> Element {
    rsx! {
        section { class: "community-section section-space", id: "community", "aria-labelledby": "community-title",
            div { class: "container community-grid",
                div { SectionLabel { number: "07", label: content::copy::COMMUNITY.label } SectionHeading { id: "community-title", lines: content::copy::COMMUNITY.heading } p { class: "body-copy", "{content::copy::COMMUNITY.paragraphs[0]}" } }
                div { class: "community-manifesto", p { "{content::copy::COMMUNITY_MANIFESTO[0]}" br {} em { "{content::copy::COMMUNITY_MANIFESTO[1]}" } } ul { for action in content::copy::COMMUNITY_ACTIONS { li { "{action}" } } } }
            }
            div { class: "container community-access",
                div { class: "community-access-photo", ResponsiveImage { image: content::ZOE_ALTERNATE, sizes: "(min-width: 900px) 300px, 92vw" } }
                div { class: "community-access-copy",
                    p { class: "eyebrow", "{content::copy::COMMUNITY_WHATSAPP_LABEL}" }
                    h3 { "{content::copy::COMMUNITY_WHATSAPP_HEADING}" }
                    p { BrandText { text: content::copy::COMMUNITY_WHATSAPP_BODY } }
                    if let Some(href) = content::COMMUNITY_WHATSAPP {
                        a { class: "community-whatsapp-link", href, target: "_blank", rel: "noopener noreferrer",
                            span { class: "whatsapp-mark", "aria-hidden": "true" }
                            "{content::copy::COMMUNITY_WHATSAPP_ACTION}"
                        }
                    } else {
                        p { class: "community-whatsapp-pending", span { class: "whatsapp-mark", "aria-hidden": "true" } "{content::copy::COMMUNITY_WHATSAPP_PENDING}" }
                    }
                }
            }
        }
        section { class: "seven-section section-space", id: "las-7", "aria-labelledby": "seven-title",
            div { class: "container seven-inner",
                SectionLabel { number: "08", label: content::copy::SEVEN.label }
                div { class: "seven-heading", SectionHeading { id: "seven-title", lines: content::copy::SEVEN.heading } div { h3 { "{content::copy::SEVEN_PROMISE[0]}" br {} "{content::copy::SEVEN_PROMISE[1]}" br {} "{content::copy::SEVEN_PROMISE[2]}" } p { class: "body-copy", "{content::copy::SEVEN.paragraphs[0]}" } } }
                ol { class: "seven-slots", "aria-label": "Las siete posiciones del programa inicial", for number in 1..=7 { li { span { "0{number}" } span { "aria-hidden": "true", "✦" } } } }
                div { class: "seven-bottom", p { "{content::copy::SEVEN_CLOSING}" } ButtonLink { href: "#contact", label: "DESCUBRIR EL PRÓXIMO CAPÍTULO", secondary: true } }
            }
        }
    }
}
