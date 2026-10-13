use crate::{
    components::{BrandName, ResponsiveImage, SpotifyLink},
    content::releases,
};
use dioxus::prelude::*;

#[component]
pub fn LatestRelease() -> Element {
    let release = &releases::LATEST;
    rsx! {
        section { class: "latest-release section-space", id: "latest-release", "aria-labelledby": "latest-release-title",
            div { class: "container",
            div { class: "release-stage latest-release-grid",
                div { class: "latest-art",
                    span { class: "latest-disc", "aria-hidden": "true" }
                    ResponsiveImage { image: release.cover, sizes: "(min-width: 900px) 480px, calc(100vw - 40px)" }
                }
                div { class: "latest-copy",
                    p { class: "eyebrow", BrandName {} " MUSIC / ÚLTIMO LANZAMIENTO" }
                    h2 { id: "latest-release-title", "Flaquita" br {} em { "Pepita." } }
                    time { datetime: releases::LATEST_DATE, "{releases::LATEST_DATE_LABEL}" }
                    p { class: "latest-artist", BrandName {} }
                    div { class: "latest-actions",
                        AppleMusicLink { href: release.apple_music }
                        SpotifyLink { href: release.spotify_search, label: "BUSCAR EN SPOTIFY" }
                    }
                    p { class: "latest-caption", "UNA HISTORIA. UNA CANCIÓN." }
                }
            }
            }
        }
    }
}

#[component]
pub fn FeaturedMusic() -> Element {
    rsx! {
        div { class: "featured-music", "aria-labelledby": "featured-music-title",
            p { class: "eyebrow", BrandName {} " MUSIC / SELECCIÓN" }
            h3 { id: "featured-music-title", "Canciones destacadas." }
            div { class: "featured-release-grid",
                for release in releases::FEATURED {
                    article { class: "featured-release",
                        a { class: "featured-cover", href: release.apple_music, target: "_blank", rel: "noopener noreferrer", "aria-label": "Escuchar {release.title} en Apple Music",
                            ResponsiveImage { image: release.cover, sizes: "(min-width: 900px) 350px, (min-width: 600px) 45vw, calc(100vw - 40px)" }
                        }
                        div { class: "featured-release-copy",
                        h4 { "{release.title}" }
                        p { BrandName {} }
                        AppleMusicLink { href: release.apple_music }
                        SpotifyLink { href: release.spotify_search, label: "BUSCAR EN SPOTIFY" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AppleMusicLink(href: &'static str) -> Element {
    rsx! {
        a { class: "release-apple", href, target: "_blank", rel: "noopener noreferrer",
            span { class: "release-apple-mark", "aria-hidden": "true" }
            "ESCUCHAR EN APPLE MUSIC"
            span { class: "release-link-arrow", "aria-hidden": "true", "→" }
            span { class: "sr-only", " (abre una nueva pestaña)" }
        }
    }
}
