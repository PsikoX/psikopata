use crate::{
    components::{
        BrandText, ButtonLink, FerInterlace, FerSeal, ResponsiveImage, SectionLabel,
        SmokeAtmosphere,
    },
    content::{self, fer},
    layouts::ClosingInvitation,
};
use dioxus::prelude::*;

#[component]
pub fn Fer() -> Element {
    rsx! {
        div { class: "fer-experience",
            SmokeAtmosphere {}
            Opening {}
            Forces {}
            Union {}
            Cycle {}
            WomanAtTheCenter {}
            Questions {}
            Closing {}
        }
    }
}

#[component]
fn FerSum(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        p { class: "fer-sum {class}", "aria-label": "F más E más R",
            span { class: "fer-glyph fer-glyph-f", "aria-hidden": "true", "F" }
            span { class: "fer-plus", "aria-hidden": "true", "+" }
            span { class: "fer-glyph fer-glyph-e", "aria-hidden": "true", "E" }
            span { class: "fer-plus", "aria-hidden": "true", "+" }
            span { class: "fer-glyph fer-glyph-r", "aria-hidden": "true", "R" }
        }
    }
}

/// The former closing manifesto now explains the idea on the very first screen.
#[component]
fn Opening() -> Element {
    rsx! {
        section { class: "fer-opening", "aria-labelledby": "fer-page-title",
            div { class: "container fer-opening-inner",
                FerSum { class: "fer-opening-letters" }
                h1 { id: "fer-page-title", "EL TRIDENTE" br {} em { "DE LA" br {} "PROSPERIDAD." } }
                p { class: "fer-opening-summary", "{fer::OPENING_SUMMARY}" }
                p { class: "fer-opening-brand", "UNA IDEA DEL UNIVERSO " BrandText { text: "PSIKOPAPA" } }
                div { class: "fer-opening-actions",
                    ButtonLink { href: "#fer-vertices", label: "DESCUBRIR LAS TRES FUERZAS", secondary: true }
                    a { class: "fer-community-link", href: "/#community", "EXPLORAR LA COMUNIDAD" span { "aria-hidden": "true", " ↗" } }
                }
            }
        }
    }
}

/// The chosen interlace accompanies three short, concrete definitions.
#[component]
fn Forces() -> Element {
    rsx! {
        section { class: "fer-forces fer-chapter", id: "fer-vertices", "aria-labelledby": "fer-forces-title",
            div { class: "container",
                div { class: "fer-section-heading",
                    SectionLabel { number: "01", label: "QUÉ SIGNIFICA FER" }
                    h2 { id: "fer-forces-title", "Tres fuerzas." br {} em { "Una misma vida." } }
                }
                div { class: "fer-forces-grid",
                    figure { class: "fer-vertices", "aria-label": "Fetiche, Educación y Religión, tres bandas entrelazadas en un mismo símbolo",
                        FerInterlace {}
                        figcaption { "TRES FUERZAS. UN MISMO CENTRO." }
                    }
                    div { class: "fer-force-list",
                        for (index, force) in fer::FORCES.iter().enumerate() {
                            article { class: "fer-force fer-force-{index}", "aria-labelledby": "fer-force-title-{index}",
                                span { class: "fer-force-letter", "aria-hidden": "true", "{force.letter}" }
                                div { class: "fer-force-copy",
                                    h3 { id: "fer-force-title-{index}", "{force.name}" }
                                    p { class: "fer-force-role", "{force.role}" }
                                    p { class: "fer-force-meaning", "{force.meaning}" }
                                    p { class: "fer-force-declaration", "{force.declaration}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// The second chosen artifact shows the union, followed by its human meaning.
#[component]
fn Union() -> Element {
    rsx! {
        section { class: "fer-union fer-chapter", id: "convergencia", "aria-labelledby": "fer-union-title",
            div { class: "container",
                div { class: "fer-section-heading",
                    SectionLabel { number: "02", label: "CUANDO SE ENCUENTRAN" }
                    h2 { id: "fer-union-title", "Cuando se unen," br {} em { "algo cambia." } }
                    p { "Deseas algo. Aprendes a crearlo. Eliges para qué." }
                }
                div { class: "fer-union-grid",
                    figure { class: "fer-convergence-figure", "aria-label": "Tres vértices llevan su luz hasta un núcleo común: la prosperidad",
                        FerSeal {}
                        div { class: "fer-artifact-legend",
                            for force in fer::FORCES {
                                span { b { "{force.letter}" } strong { "{force.role}" } }
                            }
                        }
                        figcaption { "ENERGÍA + CAPACIDAD + DIRECCIÓN" }
                    }
                    div { class: "fer-union-result",
                        p { class: "fer-result-name", "PROSPERIDAD" }
                        h3 { "Poder crear." br {} "Poder amar." br {} em { "Poder elegir." } }
                        p { "Lo que construyes con las tres fuerzas puede abrir espacio para:" }
                        ul { class: "fer-outcomes", for outcome in fer::OUTCOMES { li { "{outcome}" } } }
                    }
                }
            }
        }
    }
}

/// A photographed brass ring replaces the generic digital circuit.
#[component]
fn Cycle() -> Element {
    rsx! {
        section { class: "fer-circuit fer-chapter", id: "circuito", "aria-labelledby": "fer-circuit-title",
            div { class: "container",
                div { class: "fer-section-heading",
                    SectionLabel { number: "03", label: "EL CICLO CONTINÚA" }
                    h2 { id: "fer-circuit-title", "Lo que creas" br {} em { "abre otro camino." } }
                }
                figure { class: "fer-cycle", "aria-label": "El ciclo del FER: deseo, educación, creación, prosperidad y libertad. La libertad abre espacio para un nuevo deseo.",
                    div { class: "fer-cycle-art",
                        picture { class: "fer-cycle-picture",
                            source { "media": "(min-width: 600px)", "srcset": "/assets/images/fer-rose-cycle-1200.webp", r#type: "image/webp" }
                            img { src: "/assets/images/fer-rose-cycle-600.webp", width: "1200", height: "800", alt: "Rosa crimson dentro de un aro de latón grabado, sobre terciopelo rojo.", loading: "lazy", decoding: "async", "fetchpriority": "low" }
                        }
                        span { class: "fer-cycle-reflection", "aria-hidden": "true" }
                        ol { for (index, step) in fer::CYCLE.iter().enumerate() {
                            li { class: "fer-cycle-step fer-cycle-step-{index}", strong { "{step}" } }
                        } }
                    }
                    figcaption { "Más libertad. Nuevos deseos." br {} em { "Y vuelves a empezar." } }
                }
            }
        }
    }
}

#[component]
fn WomanAtTheCenter() -> Element {
    rsx! {
        section { class: "fer-woman fer-chapter", "aria-labelledby": "fer-woman-title",
            div { class: "container fer-woman-grid",
                div { class: "fer-woman-copy",
                    SectionLabel { number: "EL CENTRO", label: "ERES TÚ" }
                    h2 { id: "fer-woman-title", "Tu deseo." br {} "Tu inteligencia." br {} em { "Tu fe." } }
                    p { "No tienes que elegir entre ser sensual, aprender, creer y prosperar." }
                    p { class: "fer-woman-assertion", "Tú decides qué construir." }
                }
                figure { class: "fer-woman-art",
                    ResponsiveImage { image: content::KAREN, class: "fer-woman-portrait", sizes: "(min-width: 900px) 480px, 90vw" }
                    figcaption { BrandText { text: "KAREN / UNIVERSO PSIKOPAPA" } }
                }
            }
        }
    }
}

/// Context remains available without turning the visual story into an essay.
#[component]
fn Questions() -> Element {
    rsx! {
        section { class: "fer-questions fer-chapter", "aria-labelledby": "fer-questions-title",
            div { class: "container fer-questions-inner",
                h2 { id: "fer-questions-title", "¿Y la religión?" }
                p { class: "fer-faith-note", "FER no es una nueva religión. Es una idea filosófica que une deseo, aprendizaje y fe." }
                details { class: "fer-question",
                    summary { "¿Qué significa Religión aquí?" span { "aria-hidden": "true", "+" } }
                    p { "{fer::FAITH_CONTEXT}" }
                }
                details { class: "fer-question",
                    summary { "¿Qué puedo aprender?" span { "aria-hidden": "true", "+" } }
                    p { "{fer::EDUCATION_CONTEXT}" }
                }
                details { class: "fer-question",
                    summary { "¿Qué significa prosperar?" span { "aria-hidden": "true", "+" } }
                    p { "{fer::PROSPERITY_CONTEXT}" }
                }
                details { class: "fer-question fer-references",
                    summary { "El origen de los símbolos" span { "aria-hidden": "true", "+" } }
                    p { "Estas piezas son creaciones artísticas del FER, inspiradas en estructuras que aparecen en el arte medieval:" }
                    ul {
                        li { a { href: "https://commons.wikimedia.org/wiki/File:Bok_Detail_77v.jpg", target: "_blank", rel: "noopener noreferrer", "Triquetra: tres arcos entrelazados ↗" } }
                        li { a { href: "https://commons.wikimedia.org/wiki/File:PetrusPictaviensis_CottonFaustinaBVII-folio42v_ScutumFidei_early13thc.jpg", target: "_blank", rel: "noopener noreferrer", "Scutum Fidei: tres vértices y un centro ↗" } }
                    }
                }
            }
        }
    }
}

#[component]
fn Closing() -> Element {
    rsx! {
        section { class: "fer-closing fer-chapter", "aria-labelledby": "fer-closing-title",
            div { class: "container",
                FerSum { class: "fer-closing-letters" }
                h2 { id: "fer-closing-title", "Tu vida." br {} em { "Tus tres fuerzas." } }
                p { "Desea. Aprende. Encuentra tu dirección." }
                div { class: "fer-closing-actions",
                    ClosingInvitation {}
                    ButtonLink { href: "/", label: "VOLVER AL UNIVERSO", secondary: true }
                }
            }
        }
    }
}
