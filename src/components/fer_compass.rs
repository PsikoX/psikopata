use dioxus::prelude::*;

/// A photographed compass reconstruction gives the FER triangle a physical
/// surface. The fine paths remain ordinary SVG, so they work without scripts.
#[component]
pub fn FerCompass() -> Element {
    rsx! {
        img {
            class: "fer-compass-image",
            src: "/assets/images/fer-compass.webp",
            width: "1254",
            height: "1254",
            alt: "Recreación artística de una brújula náutica antigua en latón; flor de lis al norte y cruz al este",
            loading: "eager",
            decoding: "async",
            "fetchpriority": "high",
        }
        svg {
            class: "fer-compass-threads",
            view_box: "0 0 600 600",
            fill: "none",
            "aria-hidden": "true",
            path { class: "fer-compass-triangle-shadow", d: "M300 72L102 502H498Z" }
            path { class: "fer-compass-triangle-edge", d: "M300 72L102 502H498Z" }
            path { class: "fer-compass-triangle-inset", d: "M300 82L113 493H487Z" }
            path { class: "fer-compass-route fer-compass-route-f", d: "M300 102V300" }
            path { class: "fer-compass-route fer-compass-route-e", d: "M118 482L300 300" }
            path { class: "fer-compass-route fer-compass-route-r", d: "M482 482L300 300" }
            circle { class: "fer-compass-jewel-rim", cx: "300", cy: "300", r: "30" }
            circle { class: "fer-compass-jewel-spark", cx: "300", cy: "300", r: "4" }
        }
    }
}
