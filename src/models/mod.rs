#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Image {
    pub stem: &'static str,
    pub alt: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Track {
    pub id: &'static str,
    pub title: &'static str,
    pub muse: &'static str,
    pub cover: Image,
    pub audio: Option<&'static str>,
    pub spotify: Option<&'static str>,
    pub apple_music: Option<&'static str>,
    pub youtube: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Muse {
    pub name: &'static str,
    pub image: Image,
    pub track_id: &'static str,
    pub track_title: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chapter {
    pub letter: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub text: &'static str,
    pub subjects: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NavLink {
    pub label: &'static str,
    pub href: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeadingLine {
    pub before: &'static str,
    pub emphasis: Option<&'static str>,
    pub after: &'static str,
}

impl HeadingLine {
    pub const fn plain(text: &'static str) -> Self {
        Self {
            before: text,
            emphasis: None,
            after: "",
        }
    }
    pub const fn emphasized(
        before: &'static str,
        emphasis: &'static str,
        after: &'static str,
    ) -> Self {
        Self {
            before,
            emphasis: Some(emphasis),
            after,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SectionCopy {
    pub label: &'static str,
    pub heading: &'static [HeadingLine],
    pub paragraphs: &'static [&'static str],
}
