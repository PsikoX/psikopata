use dioxus::prelude::*;

/// Three photographed medallions assemble around a hexagonal seal.
/// The finished image is the static fallback when animation is unavailable.
#[component]
pub fn FerSeal() -> Element {
    rsx! {
        div {
            class: "fer-artifact fer-artifact-seal",
            role: "img",
            "aria-label": "Tres medallones se acercan, se unen mediante tres aristas doradas y revelan un núcleo hexagonal",
            div { class: "fer-artifact-shadow", "aria-hidden": "true" }
            svg {
                class: "fer-seal-geometry",
                view_box: "0 0 1000 1000",
                "aria-hidden": "true",
                path { class: "fer-seal-rail fer-seal-rail-heavy", d: "M 500 188 L 204 692 L 796 692 Z" }
                path { class: "fer-seal-rail fer-seal-rail-light", d: "M 500 188 L 204 692 L 796 692 Z" }
            }
            span { class: "fer-seal-piece fer-seal-f", "aria-hidden": "true" }
            span { class: "fer-seal-piece fer-seal-e", "aria-hidden": "true" }
            span { class: "fer-seal-piece fer-seal-r", "aria-hidden": "true" }
            span { class: "fer-seal-piece fer-seal-core", "aria-hidden": "true" }
            img {
                class: "fer-artifact-complete",
                src: "/assets/images/fer-seal.webp",
                width: "1254",
                height: "1254",
                alt: "",
                loading: "eager",
                decoding: "async",
                "fetchpriority": "high",
            }
            span { class: "fer-artifact-light", "aria-hidden": "true" }
        }
    }
}

/// Three alpha cutouts of the sculptural bands move into their final weave.
#[component]
pub fn FerInterlace() -> Element {
    rsx! {
        div {
            class: "fer-artifact fer-artifact-interlace",
            role: "img",
            "aria-label": "Tres bandas de esmalte rojo, marfil con una cruz y oro se entrelazan y revelan un centro luminoso",
            div { class: "fer-artifact-shadow", "aria-hidden": "true" }
            img { class: "fer-band fer-band-red", src: "/assets/images/fer-interlace-red.webp", width: "1254", height: "1254", alt: "", loading: "lazy", decoding: "async" }
            img { class: "fer-band fer-band-ivory", src: "/assets/images/fer-interlace-ivory.webp", width: "1254", height: "1254", alt: "", loading: "lazy", decoding: "async" }
            img { class: "fer-band fer-band-gold", src: "/assets/images/fer-interlace-gold.webp", width: "1254", height: "1254", alt: "", loading: "lazy", decoding: "async" }
            img { class: "fer-artifact-complete", src: "/assets/images/fer-interlace.webp", width: "1254", height: "1254", alt: "", loading: "lazy", decoding: "async" }
            span { class: "fer-artifact-light", "aria-hidden": "true" }
        }
    }
}
