use crate::models::{HeadingLine, Image};
use dioxus::prelude::*;

mod fer_artifact;
mod motion;
pub use fer_artifact::{FerInterlace, FerSeal};
pub use motion::{MotionControl, MouseSmoke, SmokeAtmosphere};

pub fn image_srcset(image: Image) -> String {
    let stem = image.stem;
    format!(
        "/assets/images/{stem}-160.webp 160w, /assets/images/{stem}-320.webp 320w, /assets/images/{stem}-480.webp 480w, /assets/images/{stem}-800.webp 800w, /assets/images/{stem}-1280.webp 1280w"
    )
}

#[component]
pub fn ResponsiveImage(
    image: Image,
    #[props(default = "")] class: &'static str,
    #[props(default = "100vw")] sizes: &'static str,
    #[props(default)] eager: bool,
) -> Element {
    let stem = image.stem;
    let srcset = image_srcset(image);
    rsx! {
        picture { class,
            source { r#type: "image/webp", "srcset": srcset, "sizes": sizes }
            img {
                src: "/assets/images/{stem}.jpg", width: "1280", height: "1280",
                alt: image.alt, loading: if eager { "eager" } else { "lazy" },
                decoding: "async", "fetchpriority": if eager { "high" } else { "low" },
            }
        }
    }
}

#[component]
pub fn SectionLabel(number: &'static str, label: &'static str) -> Element {
    rsx! { p { class: "section-label", span { class: "chapter-number", "{number}" } span { class: "label-rule", "aria-hidden": "true" } span { class: "section-label-text", BrandText { text: label } } } }
}

#[component]
pub fn ButtonLink(
    href: &'static str,
    label: &'static str,
    #[props(default)] secondary: bool,
) -> Element {
    rsx! { a { href, class: if secondary { "button button-outline" } else { "button button-crimson" }, BrandText { text: label } } }
}

#[component]
pub fn Brand(#[props(default = "")] class: &'static str) -> Element {
    rsx! { a { class: "brand {class}", href: "/", "aria-label": "PSIKOPAPA, inicio", BrandName {} } }
}

#[component]
pub fn BrandName() -> Element {
    rsx! { span { class: "brand-name", "PSIKOPAPA" } }
}

/// Keeps the brand treatment consistent inside editorial copy and actions.
#[component]
pub fn BrandText(text: &'static str) -> Element {
    rsx! {
        for (index, part) in text.split("PSIKOPAPA").enumerate() {
            if index > 0 { BrandName {} }
            "{part}"
        }
    }
}

#[component]
pub fn ExternalLink(href: &'static str, label: &'static str) -> Element {
    rsx! { a { href, target: "_blank", rel: "noopener noreferrer", "{label}" span { class: "sr-only", " (abre una nueva pestaña)" } } }
}

#[component]
pub fn SpotifyLink(
    href: &'static str,
    label: &'static str,
    #[props(default = "")] class: &'static str,
) -> Element {
    rsx! {
        a { class: "spotify-link {class}", href, target: "_blank", rel: "noopener noreferrer",
            span { class: "spotify-mark", "aria-hidden": "true" }
            span { "{label}" }
            span { class: "sr-only", " (abre una nueva pestaña)" }
        }
    }
}

#[component]
pub fn SectionHeading(id: &'static str, lines: &'static [HeadingLine]) -> Element {
    rsx! { h2 { id, class: "editorial-heading", for line in lines { span { class: "editorial-line", "{line.before}" if let Some(text) = line.emphasis { em { "{text}" } } "{line.after}" } } } }
}
