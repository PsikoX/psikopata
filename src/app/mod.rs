use crate::{
    components::{MotionControl, image_srcset},
    content,
    layouts::{Footer, Header},
    pages::{fer::Fer, home::Home, not_found::NotFound},
    styles,
};
use dioxus::prelude::*;
use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    Home,
    Fer,
    NotFound,
}

impl Route {
    pub const ALL: [Self; 3] = [Self::Home, Self::Fer, Self::NotFound];
    pub fn path(self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Fer => "/fer/",
            Self::NotFound => "/404.html",
        }
    }
    pub fn output_path(self) -> &'static str {
        match self {
            Self::Home => "index.html",
            Self::Fer => "fer/index.html",
            Self::NotFound => "404.html",
        }
    }
    pub fn title(self) -> &'static str {
        match self {
            Self::Home => "PSIKOPAPA — Tu belleza tiene poder",
            Self::Fer => "FER — Fetiche. Educación. Religión. | PSIKOPAPA",
            Self::NotFound => "Fuera de escena — PSIKOPAPA",
        }
    }
    pub fn description(self) -> &'static str {
        match self {
            Self::Home => content::DESCRIPTION,
            Self::Fer => {
                "Descubre FER: Fetiche, Educación y Religión. Deseo, conocimiento y fe dentro del universo PSIKOPAPA. Un proyecto en desarrollo."
            }
            Self::NotFound => "La página que buscas no existe. Vuelve al universo PSIKOPAPA.",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SiteConfig {
    pub origin: Option<String>,
    pub base_path: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidOrigin;
impl fmt::Display for InvalidOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SITE_ORIGIN must be an HTTPS origin, or HTTP localhost, with no path, query or credentials")
    }
}
impl Error for InvalidOrigin {}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidBasePath;
impl fmt::Display for InvalidBasePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SITE_BASE_PATH must be empty or an absolute path without a trailing slash")
    }
}
impl Error for InvalidBasePath {}

impl SiteConfig {
    pub fn new(value: &str) -> Result<Self, InvalidOrigin> {
        let value = value.trim().trim_end_matches('/');
        if value.is_empty() {
            return Ok(Self {
                origin: None,
                base_path: String::new(),
            });
        }
        let (secure, authority) = if let Some(host) = value.strip_prefix("https://") {
            (true, host)
        } else if let Some(host) = value.strip_prefix("http://") {
            (false, host)
        } else {
            return Err(InvalidOrigin);
        };
        let (host, port) = authority
            .split_once(':')
            .map_or((authority, None), |(host, port)| (host, Some(port)));
        if host.is_empty()
            || host.split('.').any(|label| {
                label.is_empty()
                    || label.starts_with('-')
                    || label.ends_with('-')
                    || !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
            })
            || port.is_some_and(|p| p.parse::<u16>().map_or(true, |p| p == 0))
            || (!secure && host != "localhost" && host != "127.0.0.1")
        {
            return Err(InvalidOrigin);
        }
        Ok(Self {
            origin: Some(value.to_owned()),
            base_path: String::new(),
        })
    }
    pub fn with_base_path(mut self, value: &str) -> Result<Self, InvalidBasePath> {
        if value.is_empty() {
            return Ok(self);
        }
        if !value.starts_with('/')
            || value.ends_with('/')
            || value.split('/').skip(1).any(|segment| {
                segment.is_empty()
                    || !segment
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
            })
        {
            return Err(InvalidBasePath);
        }
        self.base_path = value.to_owned();
        Ok(self)
    }
    pub fn from_environment() -> Result<Self, Box<dyn Error>> {
        let origin = match std::env::var("SITE_ORIGIN") {
            Ok(value) => value,
            Err(std::env::VarError::NotPresent) => include_str!("../../site-origin.txt").to_owned(),
            Err(error) => return Err(error.into()),
        };
        let base_path = match std::env::var("SITE_BASE_PATH") {
            Ok(value) => value,
            Err(std::env::VarError::NotPresent) => String::new(),
            Err(error) => return Err(error.into()),
        };
        Ok(Self::new(&origin)?.with_base_path(&base_path)?)
    }
    pub fn canonical(&self, route: Route) -> Option<String> {
        self.origin
            .as_ref()
            .map(|origin| format!("{origin}{}{}", self.base_path, route.path()))
    }
}

/// Prefixes local document and CSS URLs for a project hosted below a URL path.
pub fn prefix_local_urls(document: &str, base_path: &str) -> String {
    if base_path.is_empty() {
        return document.to_owned();
    }
    let mut output = document.to_owned();
    for marker in ["href=\"/", "src=\"/", "srcset=\"/", "url('/"] {
        output = output.replace(marker, &marker.replace('/', &format!("{base_path}/")));
    }
    output.replace(", /assets/", &format!(", {base_path}/assets/"))
}

pub fn render_page(route: Route, config: SiteConfig) -> String {
    let base_path = config.base_path.clone();
    let mut dom = VirtualDom::new_with_props(Document, DocumentProps { route, config });
    dom.rebuild_in_place();
    let html = format!(
        "<!doctype html><html lang=\"{}\" itemscope itemtype=\"https://schema.org/WebSite\">{}</html>",
        content::LANG,
        dioxus_ssr::render(&dom)
    );
    prefix_local_urls(&html, &base_path)
}

#[component]
fn Document(route: Route, config: SiteConfig) -> Element {
    let canonical = config.canonical(route);
    rsx! {
            head {
                meta { charset: "utf-8" }
                meta { name: "viewport", content: "width=device-width, initial-scale=1" }
                title { "{route.title()}" }
                meta { name: "description", content: route.description() }
                meta { name: "theme-color", content: "#080708" }
                meta { name: "color-scheme", content: "dark" }
                meta { name: "robots", content: if route == Route::NotFound { "noindex,follow" } else { "index,follow" } }
                meta { property: "og:title", content: route.title() }
                meta { property: "og:description", content: route.description() }
                meta { property: "og:type", content: "website" }
                meta { property: "og:site_name", content: content::BRAND }
                meta { property: "og:locale", content: "es_VE" }
                meta { name: "twitter:card", content: "summary" }
                meta { name: "twitter:title", content: route.title() }
                meta { name: "twitter:description", content: route.description() }
                meta { "itemprop": "name", content: content::BRAND }
                if let Some(url) = canonical { link { rel: "canonical", href: "{url}" } meta { property: "og:url", content: "{url}" } meta { "itemprop": "url", content: "{url}" } }
                link { rel: "icon", r#type: "image/svg+xml", href: "/assets/favicon.svg" }
                if route == Route::Home {
                    link { rel: "preload", r#as: "image", r#type: "image/webp", "imagesrcset": image_srcset(content::KAREN), "imagesizes": content::HERO_IMAGE_SIZES, "fetchpriority": "high" }
                }
                link { rel: "preload", href: "/assets/fonts/cormorant-garamond-latin.woff2", r#as: "font", r#type: "font/woff2", crossorigin: "anonymous" }
                link { rel: "stylesheet", href: styles::css_href() }
            }
            body { class: if route == Route::Home { "page-home" } else { "page-inner" },
                a { class: "skip-link", href: "#main", "Saltar al contenido" }
                if route != Route::NotFound { MotionControl {} }
                div { class: "site-experience",
                    Header {}
                    main { id: "main", tabindex: "-1", match route { Route::Home => rsx! { Home {} }, Route::Fer => rsx! { Fer {} }, Route::NotFound => rsx! { NotFound {} } } }
                    Footer {}
                }
            }
    }
}
