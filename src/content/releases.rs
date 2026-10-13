use crate::{content, models::Image};

pub struct MusicRelease {
    pub title: &'static str,
    pub cover: Image,
    pub apple_music: &'static str,
    pub spotify_search: &'static str,
}

pub const LATEST_DATE: &str = "2026-10-12";
pub const LATEST_DATE_LABEL: &str = "12 OCTUBRE 2026";
pub const LATEST: MusicRelease = MusicRelease {
    title: "Flaquita Pepita",
    cover: Image {
        stem: "flaquita-pepita",
        alt: "Portada oficial de Flaquita Pepita: una mujer, PSIKOPAPA y una pepita dorada entre humo crimson.",
    },
    apple_music: "https://music.apple.com/ve/album/flaquita-pepita/6822107939?i=6822107941",
    spotify_search: "https://open.spotify.com/search/PSIKOPAPA%20Flaquita%20Pepita",
};

// Editorial selection; no playback counts or popularity ranking are asserted.
pub const FEATURED: &[MusicRelease] = &[
    MusicRelease {
        title: "PsikoPalo",
        cover: Image {
            stem: "psikopalo",
            alt: "Portada oficial de PsikoPalo, con PSIKOPAPA y una mujer entre humo rojo.",
        },
        apple_music: "https://music.apple.com/ve/album/psikopalo/6819259421?i=6819259422",
        spotify_search: "https://open.spotify.com/search/PSIKOPAPA%20PsikoPalo",
    },
    MusicRelease {
        title: "Karen Mi Amor",
        cover: content::KAREN,
        apple_music: "https://music.apple.com/ve/album/karen-mi-amor/6819114968?i=6819114969",
        spotify_search: "https://open.spotify.com/search/PSIKOPAPA%20Karen%20Mi%20Amor",
    },
    MusicRelease {
        title: "Zoe (La Cortada)",
        cover: content::ZOE,
        apple_music: "https://music.apple.com/ve/album/zoe-la-cortada/6818815810?i=6818815811",
        spotify_search: content::SPOTIFY_SEARCH_ZOE,
    },
];
