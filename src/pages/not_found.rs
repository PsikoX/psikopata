use crate::{components::ButtonLink, content};
use dioxus::prelude::*;

#[component]
pub fn NotFound() -> Element {
    rsx! { section { class: "not-found section-space", div { class: "container", p { class: "eyebrow", "{content::copy::NOT_FOUND_KICKER}" } h1 { "{content::copy::NOT_FOUND_HEADING[0]}" br {} "{content::copy::NOT_FOUND_HEADING[1]}" em { "{content::copy::NOT_FOUND_HEADING[2]}" } } p { class: "body-copy", "{content::copy::NOT_FOUND_DESCRIPTION}" } ButtonLink { href: "/", label: "VOLVER A PSIKOPAPA" } } } }
}
