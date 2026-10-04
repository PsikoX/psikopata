//! Editorial content for the FER experience. The three names and their roles
//! are kept together so the visual chapters cannot drift from the concept.

pub struct Force {
    pub letter: &'static str,
    pub name: &'static str,
    pub role: &'static str,
    pub declaration: &'static str,
    pub opening: &'static str,
    pub meaning: &'static str,
    pub subjects: &'static [&'static str],
}

pub const FORCES: &[Force] = &[
    Force {
        letter: "F",
        name: "FETICHE",
        role: "ENERGÍA",
        declaration: "YO QUIERO.",
        opening: "El deseo hace que mires.",
        meaning: "Atracción, belleza, sensualidad, placer, fantasía. La curiosidad que enciende el primer movimiento. No se trata de vivir para la mirada ajena: se trata de reconocer lo que te mueve a ti.",
        subjects: &["Deseo", "Magnetismo", "Imagen", "Alter ego"],
    },
    Force {
        letter: "E",
        name: "EDUCACIÓN",
        role: "CAPACIDAD",
        declaration: "YO PUEDO.",
        opening: "Aprender le da manos al deseo.",
        meaning: "Conocimiento, creatividad y competencias para transformar una idea en algo real. Inteligencia artificial, tecnología, contenido y negocios: herramientas para construir lo que imaginaste.",
        subjects: &["Conocimiento", "Tecnología", "Creación", "Negocios"],
    },
    Force {
        letter: "R",
        name: "RELIGIÓN",
        role: "DIRECCIÓN",
        declaration: "SÉ POR QUÉ.",
        opening: "La fe pregunta hacia dónde.",
        meaning: "Conciencia, propósito, valores y disciplina. También preguntas difíciles sobre la Biblia, Cristo, las relaciones y el matrimonio. Una dimensión para decidir qué merece tu energía y qué significado tiene lo que creas.",
        subjects: &["Fe", "Propósito", "Valores", "Matrimonio", "Conciencia"],
    },
];

pub const OUTCOMES: &[&str] = &[
    "CREACIÓN",
    "RELACIONES",
    "CONOCIMIENTO",
    "NEGOCIOS",
    "LIBERTAD",
    "ABUNDANCIA",
];

pub const CYCLE: &[&str] = &[
    "DESEO",
    "ATENCIÓN",
    "CURIOSIDAD",
    "EDUCACIÓN",
    "CAPACIDAD",
    "CREACIÓN",
    "VALOR",
    "PROSPERIDAD",
    "LIBERTAD",
    "NUEVO DESEO",
];
