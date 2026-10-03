//! Editorial copy is independent of the components and the visual layouts.
use crate::models::{HeadingLine as Line, SectionCopy};

pub const HERO_KICKER: &str = "AGENCIA CREATIVA · VENEZUELA";
pub const HERO_NOTE: &str =
    "PSIKOPAPA es una agencia creativa para mujeres bellas que quieren cambiar su vida.";
pub const HERO_GOAL_LABEL: &str = "PRIMERA META";
pub const HERO_GOAL_PREFIX: &str = "Hasta";
pub const HERO_GOAL_AMOUNT: &str = "US$250";
pub const HERO_GOAL_SUFFIX: &str = "dedicando 3 horas al día.";
pub const HERO_GOAL_NEXT: &str = "Es solo el primer paso. Después, vamos por más.";
pub const HERO_DISCOVER: &str = "DESCUBRIR PSIKOPAPA";
pub const HERO_SCROLL: &str = "EL DESEO ES SOLO EL COMIENZO";
pub const BEGINNING: &str = "BEAUTY IS ONLY THE BEGINNING";

pub const UNIVERSE: SectionCopy = SectionCopy {
    label: "EL UNIVERSO",
    heading: &[
        Line::emphasized("Te hacemos ", "mirar.", ""),
        Line::plain("Te enseñamos"),
        Line::emphasized("a ", "pensar.", ""),
    ],
    paragraphs: &[
        "Belleza que provoca. Música que lleva tu nombre. Tecnología para crear. Conocimiento para construir algo tuyo.",
        "PSIKOPAPA es una agencia creativa y un artista. Dos formas de entrar al mismo universo. Tú eres parte de la historia.",
    ],
};
pub const MUSES: SectionCopy = SectionCopy {
    label: "ELLAS, EN EL CENTRO",
    heading: &[Line::plain("Muses.")],
    paragraphs: &["Una presencia. Una historia.", "Un universo propio."],
};
pub const MUSES_CLOSING: [&str; 2] = ["Bonita en el feed. ", "Humana en la reunión."];
pub const MUSE_CAPTION: &str = "SU HISTORIA TIENE UNA CANCIÓN";
pub const MUSIC: SectionCopy = SectionCopy {
    label: "PSIKOPAPA MUSIC",
    heading: &[
        Line::plain("Una mujer."),
        Line::plain("Una historia."),
        Line::emphasized("", "Una canción.", ""),
    ],
    paragraphs: &["PSIKOPAPA crea música para ellas."],
};
pub const MUSIC_NOTE: &str = "ESTA CANCIÓN FUE CREADA PARA ELLA.";
pub const AUDIO_UNAVAILABLE: &str = "Audio no disponible en el sitio.";
pub const TRACK_STORY_OPENING: &str = "Una canción lleva su nombre:";
pub const TRACK_STORY_CLOSING: &str = "Belleza, identidad y música dentro del mismo universo.";
pub const ALTER_EGO: SectionCopy = SectionCopy {
    label: "ALTER EGO",
    heading: &[
        Line::plain("¿Quién eres"),
        Line::emphasized("cuando ", "nadie", ""),
        Line::plain("te está mirando?"),
    ],
    paragraphs: &["No tienes que convertirte en otra persona. Solo descubrir otra parte de ti."],
};
pub const ALTER_EGO_FOOTNOTE: &str = "TU IDENTIDAD. TU EXPLORACIÓN. TUS REGLAS.";
pub const CREATION: SectionCopy = SectionCopy {
    label: "BEAUTY / AI LAB",
    heading: &[
        Line::plain("Tu imagen."),
        Line::emphasized("", "Sin límites.", ""),
    ],
    paragraphs: &[
        "Creamos imágenes que todavía no existen. Tu identidad es el punto de partida. La tecnología abre las posibilidades.",
    ],
};
pub const CREATION_HEADLINE: [&str; 3] = ["La próxima", "versión", "es tuya."];
pub const CREATION_PRINCIPLE: &str =
    "Creamos contigo. Tu imagen y tu voz se publican con tu autorización.";
pub const EDUCATION: SectionCopy = SectionCopy {
    label: "EDUCACIÓN",
    heading: &[
        Line::plain("Beauty gets"),
        Line::plain("attention."),
        Line::emphasized("", "Knowledge", ""),
        Line::emphasized("", "builds power.", ""),
    ],
    paragraphs: &[
        "No queremos que dependas de tu belleza.",
        "Queremos que descubras todo lo que puedes hacer con ella — y mucho más allá de ella.",
    ],
};
pub const LEARNING_KICKER: &str = "APRENDE. CREA. HAZLO TUYO.";
pub const LEARNING_NOTE: [&str; 2] = ["Mucho glamour. Mucha educación.", "Y bastante palo."];
pub const ENERGY: SectionCopy = SectionCopy {
    label: "PRESENCIA. INTENCIÓN. DIRECCIÓN.",
    heading: &[
        Line::plain("Tu energía femenina"),
        Line::emphasized("no es una ", "debilidad.", ""),
    ],
    paragraphs: &[
        "Tu presencia. Tu creatividad. Tu sensualidad. Tu intuición.",
        "Tu capacidad de crear, de influenciar y de aprender.",
    ],
};
pub const ENERGY_CLOSING: &str = "QUEREMOS AYUDARTE A DIRIGIRLA.";
pub const COMMUNITY: SectionCopy = SectionCopy {
    label: "COMUNIDAD",
    heading: &[Line::emphasized("No estás ", "sola.", "")],
    paragraphs: &[
        "Un lugar para preguntar, crear, equivocarte, compartir lo que sabes y encontrar orientación.",
    ],
};
pub const COMMUNITY_MANIFESTO: [&str; 2] = ["QUIEN APRENDE,", "ENSEÑA."];
pub const COMMUNITY_ACTIONS: &[&str] =
    &["Aprender", "Preguntar", "Crear", "Colaborar", "Participar"];
pub const SEVEN: SectionCopy = SectionCopy {
    label: "EL PRIMER CAPÍTULO",
    heading: &[Line::emphasized("LAS ", "7", "")],
    paragraphs: &[
        "Aprender. Probar. Crear. Dar feedback. Mejorar el sistema y abrir el camino para las que vienen.",
    ],
};
pub const SEVEN_PROMISE: [&str; 3] = ["Siete mujeres.", "Siete historias.", "Un experimento."];
pub const SEVEN_CLOSING: &str = "El comienzo se escribe juntas.";
pub const FER_LABEL: &str = "AHORA, LA PREGUNTA";
pub const FER_QUESTION: &str = "¿Qué coño es este concepto?";
pub const FER_STATUS: &str = "UN PROYECTO EN DESARROLLO.";
pub const FER_HEADING: &[Line] = &[
    Line::plain("Entraste por"),
    Line::emphasized("la ", "belleza.", ""),
];
pub const FER_INTRO: [&str; 2] = [
    "Te quedaste por la curiosidad.",
    "Ahora, descubre lo que hay detrás.",
];
pub const FER_CLOSING: [&str; 2] = ["Tu belleza provoca.", "Tu mente transforma."];
pub const FER_DEVELOPMENT: &str = "FER sigue en desarrollo. La conversación apenas empieza.";
pub const CONTACT_KICKER: &str = "EL PRÓXIMO CAPÍTULO";
pub const CONTACT_HEADING: &str = "¿Y si empieza contigo?";
pub const CONTACT_UNAVAILABLE: &str = "El canal de contacto todavía no está disponible.";
pub const CONTACT_LOCATION: &str = "Venezuela. Para el mundo.";
pub const FOOTER_DESCRIPTION: &str = "Beauty. Music. Creation. Education.";
pub const NOT_FOUND_KICKER: &str = "404 / FUERA DE ESCENA";
pub const NOT_FOUND_HEADING: [&str; 3] = ["Esta puerta", "no lleva ", "aquí."];
pub const NOT_FOUND_DESCRIPTION: &str =
    "La página que buscas no existe. El universo sigue abierto.";
pub const MOTION_LABEL: &str = "Pausar efectos visuales";
pub const MOTION_PAUSE: &str = "PAUSAR EFECTOS";
pub const MOTION_RESUME: &str = "ACTIVAR EFECTOS";
