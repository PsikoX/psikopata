use crate::content;
use dioxus::prelude::*;

/// The cover's smoke continues behind every chapter. Only these decorative
/// textures move; original portraits and their identities remain untouched.
#[component]
pub fn SmokeAtmosphere() -> Element {
    rsx! {
        div { class: "smoke-atmosphere", "aria-hidden": "true",
            for layer in ["smoke-field smoke-field-far", "smoke-field smoke-field-near"] {
                picture { class: layer,
                    source {
                        "srcset": content::SMOKE_TEXTURE_MOBILE, r#type: "image/webp",
                        "media": "(max-width: 899px)",
                    }
                    img {
                        src: content::SMOKE_TEXTURE_DESKTOP, alt: "",
                        width: "1536", height: "1024", decoding: "async",
                        "fetchpriority": "low", draggable: "false",
                    }
                }
            }
        }
        div { class: "smoke-live", "aria-hidden": "true",
            video {
                class: "smoke-video", autoplay: true, muted: true, r#loop: true,
                playsinline: true, preload: "none", tabindex: "-1",
                "disablepictureinpicture": "",
                source {
                    src: content::SMOKE_FLOW_DESKTOP, r#type: "video/mp4",
                    "media": "(prefers-reduced-motion: no-preference) and (min-width: 900px)",
                }
                source {
                    src: content::SMOKE_FLOW_MOBILE, r#type: "video/mp4",
                    "media": "(prefers-reduced-motion: no-preference)",
                }
            }
        }
    }
}

/// A decorative smoke accent follows CSS coordinates set by the pointer script.
/// It never receives pointer events or covers links for hit testing.
#[component]
pub fn MouseSmoke() -> Element {
    rsx! { div { class: "mouse-smoke", "aria-hidden": "true" } }
}

/// A native checkbox pauses CSS motion without any client-side code.
#[component]
pub fn MotionControl() -> Element {
    rsx! {
        input {
            id: "pause-motion", class: "motion-toggle sr-only", r#type: "checkbox",
            "aria-label": content::copy::MOTION_LABEL,
        }
        label { class: "motion-control", r#for: "pause-motion",
            span { class: "motion-indicator", "aria-hidden": "true" }
            span { class: "motion-running", "{content::copy::MOTION_PAUSE}" }
            span { class: "motion-paused", "{content::copy::MOTION_RESUME}" }
        }
    }
}
