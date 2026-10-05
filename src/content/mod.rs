use crate::models::{Chapter, Image, Muse, NavLink, Release, Track};

pub mod copy;
pub mod fer;

pub const BRAND: &str = "PSIKOPAPA";
pub const LANG: &str = "es-VE";
pub const DESCRIPTION: &str = "Entra al universo PSIKOPAPA. Belleza, música, alter ego, inteligencia artificial y educación para mujeres venezolanas. Descubre FER: Fetiche, Educación y Religión.";
pub const HERO_TITLE: [&str; 2] = ["TU BELLEZA", "TIENE PODER."];
pub const HERO_SECONDARY: &str = "APRENDE A USARLO.";
pub const HERO_ALTERNATIVE: [&str; 2] = [
    "NO QUEREMOS QUE SEAS PERFECTA.",
    "QUEREMOS DESCUBRIR QUIÉN ERES.",
];

pub const KAREN: Image = Image {
    stem: "karen-mi-amor",
    alt: "Portada de Karen mi amor: una mujer junto a PSIKOPAPA, rodeados de luz crimson.",
};
pub const ZOE: Image = Image {
    stem: "zoe-la-cortada",
    alt: "Portada de Zoe la cortada: retrato femenino entre rosas, terciopelo rojo y ornamentos plateados.",
};
pub const ZOE_ALTERNATE: Image = Image {
    stem: "zoe-la-cortada-alternate",
    alt: "Otra imagen de la portada Zoe la cortada, entre rosas y reflejos rojos.",
};
pub const ARTIST: Image = Image {
    stem: "psikopapa-artist",
    alt: "PSIKOPAPA con traje negro, tatuajes y un gesto irreverente entre humo rojo.",
};

pub const SPOTIFY_SEARCH_ARTIST: &str = "https://open.spotify.com/search/PSIKOPAPA";
pub const SPOTIFY_SEARCH_KAREN: &str = "https://open.spotify.com/search/PSIKOPAPA%20Karen";
pub const SPOTIFY_SEARCH_ZOE: &str = "https://open.spotify.com/search/PSIKOPAPA%20Zoe";

// Artwork was supplied by the project owner. Names below are editorial labels
// from those covers, not invented biographies or additional participants.
pub const MUSES: &[Muse] = &[
    Muse {
        name: "Karen",
        image: KAREN,
        track_id: "karen-mi-amor",
        track_title: "Karen",
        spotify_search: SPOTIFY_SEARCH_KAREN,
    },
    Muse {
        name: "Zoe",
        image: ZOE_ALTERNATE,
        track_id: "zoe-la-cortada",
        track_title: "Zoe (La Cortada)",
        spotify_search: SPOTIFY_SEARCH_ZOE,
    },
];

// Add only verified sources. An absent source renders an honest availability
// message instead of a pretend player or guessed platform link.
pub const TRACKS: &[Track] = &[
    Track {
        id: "karen-mi-amor",
        title: "Karen",
        muse: "Karen",
        cover: KAREN,
        audio: None,
        spotify: None,
        spotify_search: SPOTIFY_SEARCH_KAREN,
        apple_music: Some("https://music.apple.com/ve/album/karen/6816570213?i=6816570214"),
        youtube: None,
    },
    Track {
        id: "zoe-la-cortada",
        title: "Zoe (La Cortada)",
        muse: "Zoe",
        cover: ZOE,
        audio: None,
        spotify: None,
        spotify_search: SPOTIFY_SEARCH_ZOE,
        apple_music: Some(
            "https://music.apple.com/ve/album/zoe-la-cortada/6818815810?i=6818815811",
        ),
        youtube: None,
    },
];

pub const OTHER_RELEASES: &[Release] = &[
    Release {
        title: "Gata 4x4",
        apple_music: "https://music.apple.com/ve/album/gata-4x4/6816570049?i=6816570050",
    },
    Release {
        title: "Flaquita (Casados por Error)",
        apple_music: "https://music.apple.com/ve/album/flaquita-casados-por-error/6816570112?i=6816570113",
    },
];

pub const APPLE_MUSIC_ARTIST: &str = "https://music.apple.com/ve/artist/psikopapa/6816504681";
pub const SEETA_GOOGLE_PLAY: &str =
    "https://play.google.com/store/apps/details?id=com.mztech.seeta&hl=es_VE";

pub const TOP_NAV: &[NavLink] = &[
    NavLink {
        label: "EL UNIVERSO",
        href: "/#universo",
    },
    NavLink {
        label: "MUSES",
        href: "/#muses",
    },
    NavLink {
        label: "MUSIC",
        href: "/#music",
    },
    NavLink {
        label: "ALTER EGO",
        href: "/#alter-ego",
    },
];

pub const FOOTER_NAV: &[NavLink] = &[
    NavLink {
        label: "MUSES",
        href: "/#muses",
    },
    NavLink {
        label: "MUSIC",
        href: "/#music",
    },
    NavLink {
        label: "ALTER EGO",
        href: "/#alter-ego",
    },
    NavLink {
        label: "CREATION",
        href: "/#creation",
    },
    NavLink {
        label: "EDUCATION",
        href: "/#education",
    },
    NavLink {
        label: "FER",
        href: "/fer/",
    },
    NavLink {
        label: "COMMUNITY",
        href: "/#community",
    },
    NavLink {
        label: "CONTACT",
        href: "/#contact",
    },
];

pub const ALTER_EGO_STAGES: &[&str] = &[
    "REAL",
    "ALTER EGO",
    "ARTIST",
    "CREATOR",
    "ENTREPRENEUR",
    "MUSE",
];
pub const CREATION_SKILLS: &[&str] = &[
    "AI BEAUTY",
    "AI PHOTOGRAPHY",
    "EDITORIAL",
    "VIDEO",
    "MUSIC",
    "ALTER EGO",
    "CAMPAIGNS",
    "SOCIAL CONTENT",
];
pub const LEARNING_AREAS: &[(&str, &str)] = &[
    ("01", "Inteligencia artificial"),
    ("02", "Content creation"),
    ("03", "Video & edición"),
    ("04", "Música & creatividad"),
    ("05", "Marketing & social media"),
    ("06", "Business & monetización"),
    ("07", "E-commerce & print on demand"),
    ("08", "Tecnología & programación"),
];

pub const FER_CHAPTERS: &[Chapter] = &[
    Chapter {
        letter: "F",
        title: "FETICHE",
        subtitle: "Lo que te hace mirar.",
        text: "Deseo. Atracción. Belleza. El poder de una imagen y la libertad de explorar tu fantasía, tu cuerpo y tu alter ego. La curiosidad también tiene piel.",
        subjects: &[
            "Deseo",
            "Sensualidad",
            "Fantasía",
            "Belleza",
            "Cuerpo",
            "Energía femenina",
            "Imagen",
            "Alter ego",
        ],
    },
    Chapter {
        letter: "E",
        title: "EDUCACIÓN",
        subtitle: "Lo que te enseña a pensar.",
        text: "Tu belleza puede abrir una puerta. Aprender te permite decidir qué hacer al otro lado. Inteligencia artificial, creación, tecnología y negocios: habilidades para construir algo tuyo.",
        subjects: &[
            "Inteligencia artificial",
            "Video",
            "Música",
            "Redes sociales",
            "Marketing",
            "E-commerce",
            "Print on demand",
            "Monetización",
            "Programación",
        ],
    },
    Chapter {
        letter: "R",
        title: "RELIGIÓN",
        subtitle: "Lo que te invita a preguntar.",
        text: "Fe. Biblia. Cristo. Preguntas sobre la verdad, el propósito, los valores, las relaciones y el matrimonio. Espacio para interpretar, conversar y practicar la disciplina de buscar respuestas.",
        subjects: &[
            "Fe",
            "Espiritualidad",
            "Biblia",
            "Cristo",
            "Valores",
            "Relaciones",
            "Matrimonio",
            "Propósito",
            "Disciplina",
            "Verdad",
        ],
    },
];

// Set the real contact and social URLs here, then regenerate the site.
pub const CONTACT_EMAIL: Option<&str> = None;
pub const CONTACT_WHATSAPP: Option<&str> = None;
pub const COMMUNITY_WHATSAPP: Option<&str> = None;
pub const SOCIAL_LINKS: &[(&str, Option<&str>)] = &[
    ("Instagram", None),
    ("TikTok", None),
    ("YouTube", None),
    ("Buscar en Spotify", Some(SPOTIFY_SEARCH_ARTIST)),
    ("Apple Music", Some(APPLE_MUSIC_ARTIST)),
];
pub const SMOKE_TEXTURE_DESKTOP: &str = "/assets/atmosphere/cover-smoke-desktop.webp";
pub const SMOKE_TEXTURE_MOBILE: &str = "/assets/atmosphere/cover-smoke-mobile.webp";
pub const SMOKE_FLOW_DESKTOP: &str = "/assets/media/cover-smoke-flow-desktop.mp4";
pub const SMOKE_FLOW_MOBILE: &str = "/assets/media/cover-smoke-flow-mobile.mp4";
pub const HERO_IMAGE_SIZES: &str = "(min-width: 900px) 75vw, 100vw";
