use crate::{
    components::{
        BrandName, ExternalLink, ResponsiveImage, SectionHeading, SectionLabel, SpotifyLink,
    },
    content,
};
use dioxus::prelude::*;

#[component]
pub fn Muses() -> Element {
    rsx! {
        section { class: "muses-section section-space", id: "muses", "aria-labelledby": "muses-title",
            div { class: "container",
                div { class: "section-heading", div { SectionLabel { number: "02", label: content::copy::MUSES.label } SectionHeading { id: "muses-title", lines: content::copy::MUSES.heading } } p { class: "section-aside", "{content::copy::MUSES.paragraphs[0]}" br {} "{content::copy::MUSES.paragraphs[1]}" } }
                div { class: "muse-grid",
                    for (index, muse) in content::MUSES.iter().enumerate() {
                        article { class: if index == 0 { "muse-card muse-karen" } else { "muse-card muse-zoe" },
                            a { class: "muse-image-link", href: "#{muse.track_id}", "aria-label": "Descubrir la canción {muse.track_title}",
                                ResponsiveImage { image: muse.image, sizes: "(min-width: 900px) 45vw, 100vw" }
                                span { class: "muse-overlay", "aria-hidden": "true" }
                                span { class: "muse-number", "0{index + 1} / MUSE" }
                                span { class: "muse-caption", strong { "{muse.name}" } span { "{content::copy::MUSE_CAPTION}" } }
                                span { class: "muse-plus", "aria-hidden": "true", "+" }
                            }
                            div { class: "muse-credit", span { BrandName {} " MUSIC" } a { href: "#{muse.track_id}", "{muse.track_title}" } }
                            SpotifyLink { href: muse.spotify_search, label: "BUSCAR EN SPOTIFY", class: "muse-spotify" }
                        }
                    }
                }
                p { class: "muses-closing", "{content::copy::MUSES_CLOSING[0]}" em { "{content::copy::MUSES_CLOSING[1]}" } }
                aside { class: "seeta-mention", "aria-label": "Seeta Live",
                    p { class: "eyebrow", "{content::copy::SEETA_LABEL}" }
                    div { h3 { "{content::copy::SEETA_HEADING}" } p { "{content::copy::SEETA_DESCRIPTION}" } }
                    ExternalLink { href: content::SEETA_GOOGLE_PLAY, label: content::copy::SEETA_ACTION }
                }
            }
        }
    }
}
