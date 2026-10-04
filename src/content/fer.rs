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
        meaning: "Tu deseo, tu sensualidad, tu curiosidad. Lo que te atrae y te pone en movimiento.",
    },
    Force {
        letter: "E",
        name: "EDUCACIÓN",
        role: "CAPACIDAD",
        declaration: "Yo puedo.",
        meaning: "Lo que aprendes para hacer realidad una idea: creatividad, tecnología y negocios.",
    },
    Force {
        letter: "R",
        name: "RELIGIÓN",
        role: "DIRECCIÓN",
        declaration: "Sé por qué.",
        meaning: "Tu fe, tus valores y tu propósito. Lo que te ayuda a elegir hacia dónde ir.",
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

/// Five readable stations around the physical ring; freedom opens a new desire.
pub const CYCLE: &[&str] = &["Deseo", "Educación", "Creación", "Prosperidad", "Libertad"];

pub const FAITH_CONTEXT: &str = "Fe, Biblia, Cristo, valores y propósito. También preguntas sobre las relaciones, el matrimonio y las decisiones de cada día. En el FER, esta fuerza da significado a lo que deseas y construyes.";
pub const EDUCATION_CONTEXT: &str = "Inteligencia artificial, creación de contenido, vídeo, música, tecnología y emprendimiento. Aprender herramientas para convertir tus ideas en proyectos propios.";
pub const PROSPERITY_CONTEXT: &str = "Ingresos, conocimientos, relaciones, proyectos y libertad para elegir. Energía + capacidad + dirección = prosperidad es una metáfora filosófica del FER, no una garantía de resultados.";
