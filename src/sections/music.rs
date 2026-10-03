use crate::{
    components::{
        BrandName, BrandText, ExternalLink, ResponsiveImage, SectionHeading, SectionLabel,
    },
    content,
    models::Track,
};
use dioxus::prelude::*;

#[component]
pub fn Music() -> Element {
    rsx! {
        section { class: "music-section section-space", id: "music", "aria-labelledby": "music-title",
            div { class: "container",
                div { class: "music-heading", SectionLabel { number: "03", label: content::copy::MUSIC.label }
                    SectionHeading { id: "music-title", lines: content::copy::MUSIC.heading }
                    p { class: "body-copy", BrandText { text: content::copy::MUSIC.paragraphs[0] } }
                }
                div { class: "tracks", for (index, track) in content::TRACKS.iter().enumerate() { TrackEntry { track: *track, number: index + 1 } } }
                div { class: "more-releases",
                    p { class: "eyebrow", BrandText { text: content::copy::MORE_RELEASES } }
                    ul {
                        for release in content::OTHER_RELEASES {
                            li { strong { "{release.title}" } ExternalLink { href: release.apple_music, label: "APPLE MUSIC ↗" } }
                        }
                    }
                    ExternalLink { href: content::APPLE_MUSIC_ARTIST, label: content::copy::MUSIC_ARTIST_LINK }
                }
                p { class: "music-note", "{content::copy::MUSIC_NOTE}" }
            }
        }
    }
}

#[component]
fn TrackEntry(track: Track, number: usize) -> Element {
    rsx! {
        article { class: "track-entry", id: track.id,
            div { class: "track-art", ResponsiveImage { image: track.cover, sizes: "(min-width: 900px) 200px, 120px" } }
            div { class: "track-copy", p { class: "eyebrow", "HER SONG / 0{number}" } h3 { "{track.title}" } p { BrandName {} }
                if let Some(audio) = track.audio { audio { controls: true, preload: "none", "aria-label": "Escuchar {track.title}", source { src: audio } "Tu navegador no admite audio. " a { href: audio, "Descargar la canción" } } }
                else if track.apple_music.is_none() && track.spotify.is_none() && track.youtube.is_none() { p { class: "availability", "{content::copy::AUDIO_UNAVAILABLE}" } }
                div { class: "track-platforms",
                    if let Some(href) = track.spotify { ExternalLink { href, label: "ESCUCHAR EN SPOTIFY ↗" } }
                    if let Some(href) = track.apple_music { ExternalLink { href, label: "ESCUCHAR EN APPLE MUSIC ↗" } }
                    if let Some(href) = track.youtube { ExternalLink { href, label: "VER EN YOUTUBE ↗" } }
                }
            }
            details { class: "track-story", summary { "LA HISTORIA" span { "aria-hidden": "true", "+" } }
                p { "{content::copy::TRACK_STORY_OPENING} {track.muse}. {content::copy::TRACK_STORY_CLOSING}" }
            }
        }
    }
}
