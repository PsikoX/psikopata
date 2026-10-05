use crate::{
    components::{BrandName, BrandText, ResponsiveImage, SectionHeading, SectionLabel},
    content,
};
use dioxus::prelude::*;

#[component]
pub fn Universe() -> Element {
    rsx! {
        section { class: "universe-section section-space", id: "universo", "aria-labelledby": "universe-title",
            div { class: "container universe-grid",
                div { class: "universe-copy", SectionLabel { number: "01", label: content::copy::UNIVERSE.label }
                    SectionHeading { id: "universe-title", lines: content::copy::UNIVERSE.heading }
                    p { class: "body-copy", "{content::copy::UNIVERSE.paragraphs[0]}" }
                    p { class: "body-copy muted", BrandText { text: content::copy::UNIVERSE.paragraphs[1] } }
                    a { class: "text-link", href: "#alter-ego", "DESCUBRE OTRA PARTE DE TI" }
                }
                figure { class: "artist-editorial", ResponsiveImage { image: content::ARTIST, sizes: "(min-width: 900px) 38vw, 85vw" }
                    figcaption { span { "EL ARTISTA / EL UNIVERSO" } strong { BrandName {} } }
                    span { class: "editorial-index", "aria-hidden": "true", "P. / 01" }
                }
            }
        }
    }
}
