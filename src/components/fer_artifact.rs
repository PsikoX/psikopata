use crate::content::fer::{self, Artwork};
use dioxus::prelude::*;

pub fn fer_material_srcset(artwork: &Artwork) -> String {
    format!(
        "/assets/images/{}-600.webp 600w, /assets/images/{}-1200.webp 1200w",
        artwork.stem, artwork.stem
    )
}

#[component]
pub fn FerInterlace() -> Element {
    rsx! { MaterialArtwork { artwork: fer::INTERLACE, eager: true } }
}

#[component]
pub fn FerSeal() -> Element {
    rsx! { MaterialArtwork { artwork: fer::SEAL } }
}

#[component]
pub fn FerRose() -> Element {
    rsx! { MaterialArtwork { artwork: fer::ROSE } }
}

/// The whole scene, its smoke and its reflections move together.
#[component]
fn MaterialArtwork(artwork: Artwork, #[props(default)] eager: bool) -> Element {
    let srcset = fer_material_srcset(&artwork);
    rsx! {
        div {
            class: "fer-artifact fer-artifact-{artwork.kind}",
            role: "img",
            "aria-label": artwork.description,
            div { class: "fer-artifact-scene",
                picture { class: "fer-artifact-complete",
                    source { r#type: "image/webp", "srcset": srcset, "sizes": artwork.sizes }
                    img {
                        src: "/assets/images/{artwork.stem}-600.webp",
                        width: "1200", height: "1200", alt: "",
                        loading: if eager { "eager" } else { "lazy" },
                        decoding: "async",
                        "fetchpriority": if eager { "high" } else { "low" },
                    }
                }
                span { class: "fer-artifact-smoke", "aria-hidden": "true" }
                svg { class: "fer-artifact-threads", view_box: "0 0 1000 1000", "aria-hidden": "true",
                    path { class: "fer-thread fer-thread-f", d: artwork.paths[0] }
                    path { class: "fer-thread fer-thread-e", d: artwork.paths[1] }
                    path { class: "fer-thread fer-thread-r", d: artwork.paths[2] }
                }
                span { class: "fer-energy fer-energy-f", "aria-hidden": "true" }
                span { class: "fer-energy fer-energy-e", "aria-hidden": "true" }
                span { class: "fer-energy fer-energy-r", "aria-hidden": "true" }
                span { class: "fer-artifact-core", "aria-hidden": "true" }
                span { class: "fer-artifact-gleam", "aria-hidden": "true" }
            }
        }
    }
}
