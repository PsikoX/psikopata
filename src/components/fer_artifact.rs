use dioxus::prelude::*;

/// The photographed artifact stays intact while the three forces travel over it.
/// Keeping one image throughout the cycle prevents the parts from changing shape.
#[component]
pub fn FerSeal() -> Element {
    rsx! {
        div {
            class: "fer-artifact fer-artifact-seal",
            role: "img",
            "aria-label": "Tres puntos del sello triangular se iluminan y conducen la luz hasta el núcleo hexagonal",
            img {
                class: "fer-artifact-complete",
                src: "/assets/images/fer-seal.webp",
                width: "1254",
                height: "1254",
                alt: "",
                loading: "lazy",
                decoding: "async",
                "fetchpriority": "low",
            }
            ArtifactLight { interlace: false }
        }
    }
}

#[component]
pub fn FerInterlace() -> Element {
    rsx! {
        div {
            class: "fer-artifact fer-artifact-interlace",
            role: "img",
            "aria-label": "La luz recorre tres bandas de esmalte rojo, marfil y oro, y se reúne en el centro del tridente",
            img {
                class: "fer-artifact-complete",
                src: "/assets/images/fer-interlace.webp",
                width: "1254",
                height: "1254",
                alt: "",
                loading: "eager",
                decoding: "async",
                "fetchpriority": "high",
            }
            ArtifactLight { interlace: true }
        }
    }
}

#[component]
fn ArtifactLight(interlace: bool) -> Element {
    let (f_path, e_path, r_path) = if interlace {
        (
            "M 275 655 Q 345 580 500 510",
            "M 765 670 Q 660 565 500 510",
            "M 500 255 Q 515 380 500 510",
        )
    } else {
        (
            "M 500 195 Q 495 345 500 490",
            "M 220 690 Q 350 610 500 490",
            "M 790 690 Q 645 610 500 490",
        )
    };
    rsx! {
        svg { class: "fer-artifact-threads", view_box: "0 0 1000 1000", "aria-hidden": "true",
            path { class: "fer-thread fer-thread-f", d: f_path }
            path { class: "fer-thread fer-thread-e", d: e_path }
            path { class: "fer-thread fer-thread-r", d: r_path }
        }
        span { class: "fer-energy fer-energy-f", "aria-hidden": "true" }
        span { class: "fer-energy fer-energy-e", "aria-hidden": "true" }
        span { class: "fer-energy fer-energy-r", "aria-hidden": "true" }
        span { class: "fer-artifact-core", "aria-hidden": "true" }
        span { class: "fer-artifact-gleam", "aria-hidden": "true" }
    }
}
