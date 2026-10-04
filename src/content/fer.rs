//! Short explanations lead the FER story. Extra context is disclosed on demand.

pub struct Force {
    pub letter: &'static str,
    pub name: &'static str,
    pub role: &'static str,
    pub declaration: &'static str,
    pub meaning: &'static str,
}

pub const OPENING_SUMMARY: &str = "El deseo da energía. El conocimiento da capacidad. La fe da dirección. Cuando se encuentran, nace el FER.";

pub const FORCES: &[Force] = &[
    Force {
        letter: "F",
        name: "FETICHE",
        role: "ENERGÍA",
        declaration: "Yo quiero.",
        meaning: "Tu deseo. Lo que te atrae y te mueve.",
    },
    Force {
        letter: "E",
        name: "EDUCACIÓN",
        role: "CAPACIDAD",
        declaration: "Yo puedo.",
        meaning: "Lo que aprendes para hacerlo realidad.",
    },
    Force {
        letter: "R",
        name: "RELIGIÓN",
        role: "DIRECCIÓN",
        declaration: "Sé por qué.",
        meaning: "Tu fe y los valores que te dan dirección.",
    },
];

pub const OUTCOMES: &[&str] = &[
    "Creación",
    "Relaciones",
    "Conocimiento",
    "Negocios",
    "Libertad",
    "Abundancia",
];

/// Prosperity belongs to the common nucleus, never to a peripheral station.
pub const CYCLE: &[&str] = &["Deseo", "Creación", "Libertad", "Nuevo deseo"];

#[derive(Clone, Copy, PartialEq)]
pub struct Artwork {
    pub kind: &'static str,
    pub stem: &'static str,
    pub description: &'static str,
    pub sizes: &'static str,
    pub paths: [&'static str; 3],
}

pub const HERO_ART_SIZES: &str =
    "(min-width: 900px) 520px, (min-width: 600px) 560px, calc(100vw - 40px)";
pub const INTERLACE: Artwork = Artwork {
    kind: "interlace",
    stem: "fer-editorial-interlace",
    sizes: HERO_ART_SIZES,
    description: "Entrelazado de rubí, oro y marfil: tres fuerzas conducen su luz al núcleo del Tridente.",
    paths: [
        "M 285 695 Q 325 595 500 535",
        "M 765 705 Q 675 580 500 535",
        "M 500 215 Q 565 350 500 535",
    ],
};
pub const SEAL: Artwork = Artwork {
    kind: "seal",
    stem: "fer-editorial-seal",
    sizes: "(min-width: 1100px) 580px, (min-width: 900px) 46vw, (min-width: 620px) 580px, calc(100vw - 40px)",
    description: "Tres medallones del sello triangular conducen su luz hasta una piedra central.",
    paths: [
        "M 500 230 Q 500 370 500 495",
        "M 235 670 Q 355 600 500 495",
        "M 765 670 Q 645 600 500 495",
    ],
};
pub const ROSE: Artwork = Artwork {
    kind: "rose",
    stem: "fer-editorial-rose",
    sizes: "(min-width: 660px) 620px, calc(100vw - 40px)",
    description: "El Tridente une Fetiche, Educación y Religión alrededor de una rosa crimson. Su centro representa la prosperidad.",
    paths: [
        "M 500 195 Q 500 340 500 465",
        "M 245 645 Q 350 550 500 465",
        "M 775 645 Q 660 550 500 465",
    ],
};

pub const FAITH_CONTEXT: &str = "Fe, Biblia, Cristo, valores y propósito. También preguntas sobre las relaciones, el matrimonio y las decisiones de cada día. En el FER, esta fuerza da significado a lo que deseas y construyes.";
pub const EDUCATION_CONTEXT: &str = "Inteligencia artificial, creación de contenido, vídeo, música, tecnología y emprendimiento. Aprender herramientas para convertir tus ideas en proyectos propios.";
pub const PROSPERITY_CONTEXT: &str = "Ingresos, conocimientos, relaciones, proyectos y libertad para elegir. Energía + capacidad + dirección = prosperidad es una metáfora filosófica del FER, no una garantía de resultados.";
