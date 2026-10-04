use crate::{
    components::{MouseSmoke, SmokeAtmosphere},
    layouts::Contact,
    sections::{
        alter_ego::AlterEgo, community::Community, creation::Creation, education::Education,
        fer::FerReveal, hero::Hero, muses::Muses, music::Music, universe::Universe,
    },
};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "cinematic-universe",
            SmokeAtmosphere {}
            Hero {} FerReveal {} Universe {} Muses {} Music {} AlterEgo {}
            Creation {} Education {} Community {} Contact {}
            MouseSmoke {}
        }
    }
}
