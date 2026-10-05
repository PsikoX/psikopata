use crate::{
    components::{BrandName, BrandText, ButtonLink, ResponsiveImage},
    content,
};
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "hero", "aria-labelledby": "hero-title",
            div { class: "hero-media", ResponsiveImage { image: content::KAREN, class: "hero-picture", sizes: content::HERO_IMAGE_SIZES, eager: true } }
            div { class: "hero-shade", "aria-hidden": "true" }
            a { class: "hero-fer", href: "#fer", "aria-label": "FER: Fetiche, Educación y Religión",
                strong { "aria-label": "FER",
                    span { class: "fer-glyph fer-glyph-f", "aria-hidden": "true", "F" }
                    span { class: "fer-glyph fer-glyph-e", "aria-hidden": "true", "E" }
                    span { class: "fer-glyph fer-glyph-r", "aria-hidden": "true", "R" }
                }
                span { class: "hero-fer-terms",
                    for chapter in content::FER_CHAPTERS { span { "{chapter.title}" } }
                }
                span { class: "hero-fer-arrow", "aria-hidden": "true", "VER" }
            }
            div { class: "container hero-content",
                p { class: "hero-kicker", span { "aria-hidden": "true", "✦" } "{content::copy::HERO_KICKER}" }
                h1 { id: "hero-title", span { "{content::HERO_TITLE[0]}" } span { "{content::HERO_TITLE[1]}" } }
                p { class: "hero-secondary", "{content::HERO_SECONDARY}" }
                p { class: "hero-note", BrandText { text: content::copy::HERO_NOTE } }
                div { class: "hero-goal",
                    span { class: "hero-goal-label", "{content::copy::HERO_GOAL_LABEL}" }
                    p { "{content::copy::HERO_GOAL_PREFIX} " strong { "{content::copy::HERO_GOAL_AMOUNT}" } " {content::copy::HERO_GOAL_SUFFIX}" }
                    small { "{content::copy::HERO_GOAL_NEXT}" }
                }
                div { class: "hero-actions", ButtonLink { href: "#muses", label: "ENTRAR" } a { class: "text-link", href: "#universo", BrandText { text: content::copy::HERO_DISCOVER } span { class: "link-line", "aria-hidden": "true" } } }
            }
            div { class: "container hero-bottom",
                a { class: "scroll-cue", href: "#universo", span { class: "scroll-line", "aria-hidden": "true" } "{content::copy::HERO_SCROLL}" }
                a { class: "hero-song", href: "#music", ResponsiveImage { image: content::KAREN, class: "hero-song-cover", sizes: "64px" } div { span { class: "eyebrow", "HER SONG" } strong { "{content::TRACKS[0].title}" } span { class: "song-artist", BrandName {} } } span { class: "sound-mark", "aria-hidden": "true", i {} i {} i {} i {} i {} } }
            }
            span { class: "hero-edition", "aria-hidden": "true", BrandName {} " / VOL. 001" }
        }
        div { class: "manifesto-strip", p { "{content::copy::BEGINNING}" } span { "aria-hidden": "true", "✦" } p { "{content::copy::BEGINNING}" } span { "aria-hidden": "true", "✦" } p { "{content::copy::BEGINNING}" } }
        a { class: "mobile-music-teaser", href: "#music",
            ResponsiveImage { image: content::KAREN, sizes: "64px" }
            span { class: "mobile-music-copy", small { "HER SONG / PSIKOPAPA" } strong { "{content::TRACKS[0].title}" } }
            span { class: "mobile-music-action", "MÚSICA" }
        }
    }
}
