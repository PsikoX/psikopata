use crate::{
    components::{
        BrandText, ButtonLink, FerInterlace, FerSeal, ResponsiveImage, SectionLabel,
        SmokeAtmosphere,
    },
    content,
    layouts::ClosingInvitation,
};
use dioxus::prelude::*;

#[component]
pub fn Fer() -> Element {
    rsx! {
        div { class: "fer-experience",
            SmokeAtmosphere {}
            Arrival {}
            Forces {}
            Convergence {}
            Expansion {}
            Circuit {}
            WomanAtTheCenter {}
            Declarations {}
            Closing {}
        }
    }
}

/// Visual 01: three tangible vertices assemble into the FER seal.
#[component]
fn Arrival() -> Element {
    rsx! {
        section { class: "fer-arrival", "aria-labelledby": "fer-page-title",
            div { class: "container fer-arrival-inner",
                a { class: "text-link fer-back", href: "/#fer", "← VOLVER AL UNIVERSO" }
                SectionLabel { number: "F / E / R", label: "UN SISTEMA EN MOVIMIENTO" }
                p { class: "fer-arrival-prelude", "TE HICIERON CREER QUE TENÍAS QUE ELEGIR." }
                h1 { id: "fer-page-title", span { "FER" } em { "El Tridente de la Prosperidad" } }
                p { class: "fer-arrival-lead", "Deseo. Conocimiento. Fe. Tres fuerzas humanas. Una pregunta: ¿qué pasa cuando dejas de separarlas?" }
                a { class: "fer-scroll-cue", href: "#fer-vertices", "DESCUBRE EL SISTEMA" span { "aria-hidden": "true", "↓" } }
            }
            figure { class: "fer-vertices container", id: "fer-vertices", "aria-label": "Fetiche, Educación y Religión forman un triángulo alrededor de un núcleo central",
                div { class: "fer-vertices-stage",
                    FerSeal {}
                }
                div { class: "fer-artifact-legend",
                    for force in content::fer::FORCES {
                        span { b { "{force.letter}" } strong { "{force.name}" } small { "{force.role}" } }
                    }
                }
                figcaption { "TRES FUERZAS. UN CENTRO POR REVELAR." }
                div { class: "fer-artifact-provenance",
                    p { "El esquema de tres vértices y un centro aparece en el Scutum Fidei medieval. Este sello es una creación artística del FER, no un objeto histórico." }
                    a {
                        href: "https://commons.wikimedia.org/wiki/File:PetrusPictaviensis_CottonFaustinaBVII-folio42v_ScutumFidei_early13thc.jpg",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "VER EL MANUSCRITO HISTÓRICO ↗"
                    }
                }
            }
        }
    }
}

/// Visual 02: each current has its own color, rhythm and editorial chapter.
#[component]
fn Forces() -> Element {
    rsx! {
        section { class: "fer-forces section-space", "aria-labelledby": "fer-forces-title",
            div { class: "container fer-section-intro",
                SectionLabel { number: "01 / 04", label: "LAS TRES FUERZAS" }
                h2 { id: "fer-forces-title", "Primero, la tensión." }
                p { "Separadas, pueden impulsarte. Pero ninguna cuenta toda la historia." }
            }
            for (index, force) in content::fer::FORCES.iter().enumerate() {
                article { class: "fer-force fer-force-{index}", id: force.name,
                    div { class: "container fer-force-inner",
                        div { class: "fer-force-current", "aria-hidden": "true",
                            span { class: "fer-current-halo" }
                            span { class: "fer-current-stem" }
                            span { class: "fer-current-letter", "{force.letter}" }
                            span { class: "fer-current-spark fer-spark-one" }
                            span { class: "fer-current-spark fer-spark-two" }
                        }
                        div { class: "fer-force-copy",
                            p { class: "fer-force-role", "{force.role} / {force.name}" }
                            h3 { "{force.opening}" }
                            p { class: "fer-force-declaration", "{force.declaration}" }
                            p { class: "fer-force-meaning", "{force.meaning}" }
                            ul { class: "fer-force-subjects", for subject in force.subjects { li { "{subject}" } } }
                        }
                    }
                }
            }
        }
    }
}

/// Visual 03: three sculptural bands join around a transforming nucleus.
#[component]
fn Convergence() -> Element {
    rsx! {
        section { class: "fer-convergence section-space", id: "convergencia", "aria-labelledby": "fer-convergence-title",
            div { class: "container",
                SectionLabel { number: "02 / 04", label: "LA CONVERGENCIA" }
                h2 { id: "fer-convergence-title", "No basta querer." br {} "No basta saber." br {} em { "No basta creer." } }
                p { class: "fer-convergence-intro", "Cuando la energía encuentra capacidad y dirección, empieza la transformación." }
                figure { class: "fer-interlace-figure", "aria-label": "Tres bandas se entrelazan para formar el Tridente de la Prosperidad",
                    FerInterlace {}
                    div { class: "fer-artifact-legend",
                        for force in content::fer::FORCES {
                            span { b { "{force.letter}" } strong { "{force.name}" } small { "{force.role}" } }
                        }
                    }
                    figcaption { "ENERGÍA + CAPACIDAD + DIRECCIÓN" }
                    a {
                        class: "fer-interlace-source",
                        href: "https://commons.wikimedia.org/wiki/File:Bok_Detail_77v.jpg",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "TRIQUETRA EN UN MANUSCRITO MEDIEVAL ↗"
                    }
                }
            }
        }
    }
}

/// Visual 04: the nucleus expands into many kinds of abundance.
#[component]
fn Expansion() -> Element {
    rsx! {
        section { class: "fer-expansion section-space", "aria-labelledby": "fer-expansion-title",
            div { class: "container fer-expansion-heading",
                SectionLabel { number: "03 / 04", label: "LA MANIFESTACIÓN" }
                h2 { id: "fer-expansion-title", "Y entonces, algo se abre." }
                p { "La prosperidad no cabe en una moneda. También es lo que aprendes, lo que construyes, a quién amas y la libertad que ganas." }
            }
            figure { class: "fer-bloom container", "aria-label": "La unión del FER se expande en creación, relaciones, conocimiento, negocios, libertad y abundancia",
                div { class: "fer-bloom-stage", "aria-hidden": "true",
                    span { class: "fer-bloom-ring fer-bloom-ring-one" }
                    span { class: "fer-bloom-ring fer-bloom-ring-two" }
                    span { class: "fer-bloom-ring fer-bloom-ring-three" }
                    for ray in 0..12 { span { class: "fer-bloom-ray fer-bloom-ray-{ray}" } }
                    span { class: "fer-bloom-core", "✦" }
                }
                ol { class: "fer-bloom-outcomes",
                    for (index, outcome) in content::fer::OUTCOMES.iter().enumerate() {
                        li { class: "fer-bloom-outcome fer-bloom-outcome-{index}", "{outcome}" }
                    }
                }
                figcaption { strong { "PROSPERIDAD" } span { "Abundancia creada, no solo acumulada." } }
            }
            p { class: "fer-metaphor container", "Así lo imagina el FER: energía + capacidad + dirección pueden abrir caminos hacia la prosperidad. Es una metáfora filosófica, no una fórmula garantizada." }
        }
    }
}

/// Visual 05: a closed circuit whose final node returns to the first.
#[component]
fn Circuit() -> Element {
    rsx! {
        section { class: "fer-circuit section-space", id: "circuito", "aria-labelledby": "fer-circuit-title",
            div { class: "container",
                SectionLabel { number: "04 / 04", label: "EL CIRCUITO" }
                h2 { id: "fer-circuit-title", "Prosperar no es el final." em { " Es el comienzo de otra vuelta." } }
                p { class: "fer-circuit-intro", "El deseo despierta atención. Aprender permite crear valor. La libertad deja espacio para un nuevo deseo." }
                figure { class: "fer-cycle", "aria-label": "Ciclo del FER: del deseo a la libertad y de vuelta a un nuevo deseo",
                    div { class: "fer-cycle-track", "aria-hidden": "true" }
                    ol { for (index, step) in content::fer::CYCLE.iter().enumerate() {
                        li { class: "fer-cycle-step fer-cycle-step-{index}", span { class: "fer-cycle-index", "{index}" } strong { "{step}" } }
                    } }
                    div { class: "fer-cycle-center", "aria-hidden": "true", span { "FER" } small { "SE MUEVE CONTIGO" } }
                    figcaption { "Y vuelve a empezar. Con más conciencia que antes." }
                }
            }
        }
    }
}

/// Visual 06: the woman is the subject who chooses how to use all three forces.
#[component]
fn WomanAtTheCenter() -> Element {
    rsx! {
        section { class: "fer-woman section-space", "aria-labelledby": "fer-woman-title",
            div { class: "container fer-woman-grid",
                div { class: "fer-woman-copy",
                    SectionLabel { number: "EL CENTRO", label: "ERES TÚ" }
                    h2 { id: "fer-woman-title", "No tienes que elegir una sola versión de ti." }
                    p { "Puedes ser sensual, inteligente, creyente, ambiciosa y creadora. Tu deseo no cancela tu fe. Tu fe no limita tu capacidad. Tú decides qué construir con las tres." }
                    p { class: "fer-woman-assertion", "NO ERES EL OBJETO DE ESTA HISTORIA." br {} em { "ERES QUIEN LA MUEVE." } }
                }
                figure { class: "fer-woman-art",
                    span { class: "fer-woman-orbit fer-woman-orbit-one", "aria-hidden": "true" }
                    span { class: "fer-woman-orbit fer-woman-orbit-two", "aria-hidden": "true" }
                    ResponsiveImage { image: content::KAREN, class: "fer-woman-portrait", sizes: "(min-width: 900px) 42vw, 88vw" }
                    span { class: "fer-woman-token fer-woman-token-f", "F · DESEO" }
                    span { class: "fer-woman-token fer-woman-token-e", "E · SABER" }
                    span { class: "fer-woman-token fer-woman-token-r", "R · SENTIDO" }
                    figcaption { "Karen, en una portada del universo PSIKOPAPA." }
                }
            }
        }
    }
}

#[component]
fn Declarations() -> Element {
    rsx! {
        section { class: "fer-declarations section-space", "aria-labelledby": "fer-declarations-title",
            div { class: "container",
                SectionLabel { number: "EL MANIFIESTO", label: "DE LA IDEA A LA ACCIÓN" }
                h2 { id: "fer-declarations-title", "Tres maneras de decir: sigo." }
                ol { class: "fer-declaration-sequence",
                    for (index, force) in content::fer::FORCES.iter().enumerate() {
                        li { class: "fer-declaration fer-declaration-{index}",
                            span { class: "fer-declaration-letter", "{force.letter}" }
                            span { class: "fer-declaration-words", strong { "{force.declaration}" } small { "{force.name} / {force.role}" } }
                        }
                    }
                    li { class: "fer-declaration fer-declaration-result", span { class: "fer-declaration-letter", "✦" } span { class: "fer-declaration-words", strong { "YO PROSPERO." } small { "LA UNIÓN EN MOVIMIENTO" } } }
                }
                div { class: "fer-faith-note",
                    h3 { "FER no es una nueva religión." }
                    p { "Fetiche nombra lo que deseas. Educación, lo que aprendes para conseguirlo. Religión, lo que da significado y dirección a lo que haces. La fe es una de las tres dimensiones del concepto; no una etiqueta para las otras dos." }
                }
            }
        }
    }
}

#[component]
fn Closing() -> Element {
    rsx! {
        section { class: "fer-finale section-space", "aria-labelledby": "fer-finale-title",
            div { class: "container",
                p { class: "fer-finale-letters", "F" span { "+" } "E" span { "+" } "R" }
                h2 { id: "fer-finale-title", "EL TRIDENTE" br {} em { "DE LA PROSPERIDAD." } }
                p { "El deseo da energía. El conocimiento da capacidad. La fe da dirección. Cuando se encuentran, nace el FER." }
                p { class: "fer-finale-brand", "UNA IDEA DEL UNIVERSO " BrandText { text: "PSIKOPAPA" } }
                div { class: "fer-finale-actions", ClosingInvitation {} ButtonLink { href: "/", label: "VOLVER AL UNIVERSO", secondary: true } }
            }
        }
    }
}
