use psikopapa::{
    app::{Route, SiteConfig, prefix_local_urls, render_page},
    content, styles,
};
use scraper::{Html, Selector};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

fn select(css: &str) -> Selector {
    Selector::parse(css).expect("valid test selector")
}

fn pages() -> HashMap<&'static str, Html> {
    let config = SiteConfig::new("https://psikopapa.test").unwrap();
    Route::ALL
        .into_iter()
        .map(|route| {
            (
                route.path(),
                Html::parse_document(&render_page(route, config.clone())),
            )
        })
        .collect()
}

#[test]
fn every_page_is_readable_without_javascript_and_has_distinct_seo() {
    let config = SiteConfig::new("https://psikopapa.test").unwrap();
    let mut titles = HashSet::new();
    for route in Route::ALL {
        let html = render_page(route, config.clone());
        let doc = Html::parse_document(&html);
        assert!(html.starts_with("<!doctype html>"));
        assert_eq!(
            doc.select(&select("html"))
                .next()
                .unwrap()
                .value()
                .attr("lang"),
            Some("es-VE")
        );
        assert_eq!(doc.select(&select("h1")).count(), 1, "one h1 for {route:?}");
        assert_eq!(doc.select(&select("main")).count(), 1);
        assert_eq!(doc.select(&select("iframe")).count(), 0);
        let scripts: Vec<_> = doc.select(&select("script")).collect();
        if route == Route::Home {
            assert_eq!(scripts.len(), 1);
            assert!(
                scripts[0]
                    .value()
                    .attr("src")
                    .is_some_and(|src| src.starts_with("/assets/motion/pointer-smoke.js?v="))
            );
            assert!(scripts[0].inner_html().is_empty(), "no inline script");
        } else {
            assert!(scripts.is_empty());
        }
        assert!(!html.contains("javascript:") && !html.contains("onclick="));
        assert!(titles.insert(doc.select(&select("title")).next().unwrap().inner_html()));
        for field in [
            "meta[name='description']",
            "meta[property='og:title']",
            "meta[property='og:description']",
            "meta[name='twitter:title']",
        ] {
            assert!(
                !doc.select(&select(field))
                    .next()
                    .unwrap()
                    .value()
                    .attr("content")
                    .unwrap()
                    .is_empty()
            );
        }
        assert_eq!(
            doc.select(&select("link[rel='canonical']"))
                .next()
                .unwrap()
                .value()
                .attr("href"),
            config.canonical(route).as_deref()
        );
        for element in doc.select(&select("*")) {
            assert!(
                element
                    .value()
                    .attrs()
                    .all(|(key, _)| !key.starts_with("on")),
                "no event-handler attributes"
            );
        }
    }
}

#[test]
fn navigation_resolves_to_real_routes_and_unique_targets() {
    let pages = pages();
    let targets: HashMap<_, HashSet<_>> = pages
        .iter()
        .map(|(path, doc)| {
            let ids: Vec<_> = doc
                .select(&select("[id]"))
                .map(|node| node.value().attr("id").unwrap().to_owned())
                .collect();
            let set: HashSet<_> = ids.iter().cloned().collect();
            assert_eq!(ids.len(), set.len(), "duplicate ID on {path}");
            (*path, set)
        })
        .collect();
    for (path, doc) in &pages {
        for a in doc.select(&select("a[href]")) {
            let href = a.value().attr("href").unwrap();
            assert!(!href.is_empty() && href != "#", "no empty action");
            if href.starts_with("https://") || href.starts_with("mailto:") {
                continue;
            }
            let (target_path, fragment) = href
                .split_once('#')
                .map_or((href, None), |(p, f)| (p, Some(f)));
            let target_path = if target_path.is_empty() {
                *path
            } else {
                target_path
            };
            let target = targets
                .get(target_path)
                .unwrap_or_else(|| panic!("unresolved route {href}"));
            if let Some(id) = fragment {
                assert!(target.contains(id), "unresolved fragment {href} on {path}");
            }
            assert!(
                !a.text().collect::<String>().trim().is_empty()
                    || a.value().attr("aria-label").is_some(),
                "links need accessible names"
            );
        }
        for element in doc.select(&select("[aria-labelledby]")) {
            for id in element
                .value()
                .attr("aria-labelledby")
                .unwrap()
                .split_whitespace()
            {
                assert!(
                    targets[path].contains(id),
                    "missing accessible section title {id}"
                );
            }
        }
    }
}

#[test]
fn all_images_have_real_assets_dimensions_and_text_alternatives() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for doc in pages().values() {
        for image in doc.select(&select("img")) {
            let value = image.value();
            let decorative = image.ancestors().any(|node| {
                node.value()
                    .as_element()
                    .is_some_and(|element| element.attr("aria-hidden") == Some("true"))
            });
            let composite_art = image.ancestors().any(|node| {
                node.value().as_element().is_some_and(|element| {
                    element.attr("role") == Some("img")
                        && element
                            .attr("aria-label")
                            .is_some_and(|label| !label.trim().is_empty())
                })
            });
            if decorative {
                assert_eq!(value.attr("alt"), Some(""));
                assert!(value.attr("tabindex").is_none());
                assert_eq!(value.attr("width"), Some("1536"));
                assert_eq!(value.attr("height"), Some("1024"));
                assert_eq!(value.attr("fetchpriority"), Some("low"));
            } else {
                if composite_art {
                    assert_eq!(value.attr("alt"), Some(""));
                } else {
                    assert!(!value.attr("alt").unwrap().trim().is_empty());
                }
                assert!(
                    value
                        .attr("width")
                        .and_then(|n| n.parse::<u32>().ok())
                        .is_some_and(|n| n > 0)
                );
                assert!(
                    value
                        .attr("height")
                        .and_then(|n| n.parse::<u32>().ok())
                        .is_some_and(|n| n > 0)
                );
                assert!(matches!(value.attr("loading"), Some("eager" | "lazy")));
            }
            let path = value.attr("src").unwrap().trim_start_matches('/');
            assert!(root.join(path).is_file(), "missing {path}");
        }
        for source in doc.select(&select("source[srcset]")) {
            for candidate in source.value().attr("srcset").unwrap().split(',') {
                let path = candidate
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .trim_start_matches('/');
                assert!(
                    root.join(path).is_file(),
                    "missing responsive source {path}"
                );
            }
        }
    }
}

#[test]
fn fer_is_intact_and_follows_the_opening() {
    assert_eq!(
        content::FER_CHAPTERS
            .iter()
            .map(|c| (c.letter, c.title))
            .collect::<Vec<_>>(),
        vec![("F", "FETICHE"), ("E", "EDUCACIÓN"), ("R", "RELIGIÓN")]
    );
    let html = render_page(Route::Home, SiteConfig::new("").unwrap());
    assert!(!html.contains("LAS 7"));
    assert!(!html.contains("las-7"));
    assert!(html.find("id=\"hero-title\"").unwrap() < html.find("id=\"fer\"").unwrap());
    let mut previous = 0;
    for id in [
        "fer",
        "universo",
        "muses",
        "music",
        "alter-ego",
        "creation",
        "education",
        "community",
        "contact",
    ] {
        let position = html.find(&format!("id=\"{id}\"")).unwrap();
        assert!(position > previous, "narrative order at {id}");
        previous = position;
    }
    let fer = render_page(Route::Fer, SiteConfig::new("").unwrap());
    for term in [
        "FETICHE",
        "EDUCACIÓN",
        "RELIGIÓN",
        "Biblia",
        "Cristo",
        "matrimonio",
    ] {
        assert!(fer.contains(term));
    }
    let fer_doc = Html::parse_document(&fer);
    assert_eq!(fer_doc.select(&select(".fer-artifact-seal")).count(), 1);
    assert_eq!(
        fer_doc.select(&select(".fer-artifact-interlace")).count(),
        1
    );
    assert!(
        fer.find("fer-artifact-interlace").unwrap() < fer.find("fer-artifact-seal").unwrap(),
        "the interlace must explain the forces before the seal converges"
    );
    let opening = fer_doc
        .select(&select(".fer-experience > section"))
        .next()
        .unwrap();
    assert!(
        opening
            .value()
            .classes()
            .any(|class| class == "fer-opening")
    );
    let opening_text = opening.text().collect::<String>();
    assert!(opening_text.contains("EL TRIDENTE"));
    assert!(opening_text.contains("PROSPERIDAD"));
    assert!(opening_text.contains(content::fer::OPENING_SUMMARY));
    assert_eq!(
        fer_doc
            .select(&select(".fer-opening-letters .fer-glyph"))
            .count(),
        3
    );
    assert_eq!(
        fer_doc
            .select(&select(".fer-closing-letters .fer-glyph"))
            .count(),
        3
    );
    assert_eq!(
        fer_doc.select(&select(".fer-artifact-legend span")).count(),
        3
    );
    for force in content::fer::FORCES {
        assert!(fer.contains(force.name));
        assert!(fer.contains(force.role));
        assert!(fer.contains(force.meaning));
    }
    assert_eq!(
        opening.select(&select(".fer-artifact-interlace")).count(),
        1
    );
    assert_eq!(fer_doc.select(&select(".fer-forces img")).count(), 0);
    assert_eq!(fer_doc.select(&select(".fer-artifact-complete")).count(), 3);
    assert_eq!(fer_doc.select(&select(".fer-artifact-rose")).count(), 1);
    assert_eq!(fer_doc.select(&select(".fer-energy")).count(), 9);
    assert_eq!(fer_doc.select(&select(".fer-thread")).count(), 9);
    assert_eq!(fer_doc.select(&select(".fer-band")).count(), 0);
    assert_eq!(fer_doc.select(&select(".fer-outcomes li")).count(), 6);
    assert_eq!(fer_doc.select(&select(".fer-cycle-route li")).count(), 4);
    let prosperity = fer_doc
        .select(&select(
            ".fer-cycle-art .fer-rose-center #fer-prosperity-title",
        ))
        .next()
        .expect("prosperity belongs in the heart of the rose and its trident");
    assert_eq!(prosperity.text().collect::<String>(), "Prosperidad.");
    let art_sources: Vec<_> = fer_doc
        .select(&select(".fer-artifact img"))
        .map(|image| image.value().attr("src").unwrap())
        .collect();
    assert_eq!(art_sources.iter().collect::<HashSet<_>>().len(), 3);
    assert!(art_sources.iter().all(|src| src.contains("fer-editorial-")));
    assert!(!fer.contains("fer-rose-cycle-"));
    assert_eq!(fer_doc.select(&select(".fer-woman-art img")).count(), 1);
    assert_eq!(fer_doc.select(&select(".fer-question[open]")).count(), 0);
    assert!(!fer.contains("fer-current-halo"));
    assert!(!fer.contains("fer-bloom-ray"));
    assert!(!fer.contains("fer-woman-orbit"));
    assert!(fer.contains("FER no es una nueva religión"));
    assert!(fer.contains("metáfora filosófica"));
}

#[test]
fn opening_introduces_fer_agency_and_first_goal() {
    let doc = Html::parse_document(&render_page(Route::Home, SiteConfig::new("").unwrap()));
    let hero = doc.select(&select(".hero")).next().unwrap();
    let fer = hero.select(&select(".hero-fer")).next().unwrap();
    assert_eq!(fer.value().attr("href"), Some("#fer"));
    let terms = fer
        .select(&select(".hero-fer-terms span"))
        .map(|node| node.text().collect::<String>())
        .collect::<Vec<_>>();
    assert_eq!(terms, ["FETICHE", "EDUCACIÓN", "RELIGIÓN"]);
    let note = hero.select(&select(".hero-note")).next().unwrap();
    assert!(
        note.text()
            .collect::<String>()
            .contains("agencia creativa para mujeres bellas")
    );
    assert_eq!(note.select(&select(".brand-name")).count(), 1);
    let goal = hero.select(&select(".hero-goal")).next().unwrap();
    let goal_text = goal.text().collect::<String>();
    assert!(goal_text.contains("PRIMERA META"));
    assert!(goal_text.contains("US$250"));
    assert!(goal_text.contains("3 horas al día"));
    assert!(goal_text.contains("primer paso"));
}

#[test]
fn unknown_audio_and_social_urls_do_not_become_pretend_actions() {
    let doc = Html::parse_document(&render_page(Route::Home, SiteConfig::new("").unwrap()));
    assert_eq!(
        doc.select(&select("audio")).count(),
        content::TRACKS.iter().filter(|t| t.audio.is_some()).count()
    );
    assert_eq!(
        doc.select(&select(".availability")).count(),
        content::TRACKS
            .iter()
            .filter(|t| {
                t.audio.is_none()
                    && t.spotify.is_none()
                    && t.apple_music.is_none()
                    && t.youtube.is_none()
            })
            .count()
    );
    assert_eq!(
        doc.select(&select(".seven-section, .seven-slots")).count(),
        0
    );
    let community = doc.select(&select(".community-access")).next().unwrap();
    assert_eq!(community.select(&select(".whatsapp-mark")).count(), 1);
    assert_eq!(
        community
            .select(&select(".community-whatsapp-link"))
            .count(),
        usize::from(content::COMMUNITY_WHATSAPP.is_some())
    );
    assert_eq!(
        doc.select(&select(".track-platforms a")).count(),
        content::TRACKS
            .iter()
            .map(|track| 1
                + usize::from(track.apple_music.is_some())
                + usize::from(track.youtube.is_some()))
            .sum::<usize>()
    );
    assert_eq!(
        doc.select(&select(".spotify-mark")).count(),
        content::MUSES.len() + content::TRACKS.len()
    );
    for track in content::TRACKS {
        let entry = doc
            .select(&select(&format!("#{}", track.id)))
            .next()
            .unwrap();
        let spotify = entry.select(&select(".spotify-link")).next().unwrap();
        assert_eq!(
            spotify.value().attr("href"),
            Some(track.spotify.unwrap_or(track.spotify_search))
        );
        if track.spotify.is_none() {
            assert!(
                spotify
                    .text()
                    .collect::<String>()
                    .contains("BUSCAR EN SPOTIFY")
            );
            assert!(
                track
                    .spotify_search
                    .starts_with("https://open.spotify.com/search/")
            );
        }
    }
}

#[test]
fn ambient_motion_keeps_textures_decorative_and_has_a_keyboard_control() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let home = Html::parse_document(&render_page(Route::Home, SiteConfig::new("").unwrap()));
    let atmosphere = home.select(&select(".smoke-atmosphere")).next().unwrap();
    assert_eq!(atmosphere.value().attr("aria-hidden"), Some("true"));
    let videos: Vec<_> = home.select(&select("video")).collect();
    assert_eq!(videos.len(), 1, "one native decoder for the entire page");
    let video = videos[0];
    for attr in ["autoplay", "muted", "loop", "playsinline"] {
        assert!(video.value().attr(attr).is_some(), "native {attr}");
    }
    assert_eq!(video.value().attr("preload"), Some("none"));
    for source in video.select(&select("source")) {
        assert!(
            source
                .value()
                .attr("media")
                .unwrap()
                .contains("prefers-reduced-motion: no-preference")
        );
        assert_eq!(source.value().attr("type"), Some("video/mp4"));
        assert!(
            root.join(source.value().attr("src").unwrap().trim_start_matches('/'))
                .is_file()
        );
    }
    for source in atmosphere.select(&select("source")) {
        assert_eq!(source.value().attr("media"), Some("(max-width: 899px)"));
        assert!(
            root.join(
                source
                    .value()
                    .attr("srcset")
                    .unwrap()
                    .trim_start_matches('/')
            )
            .is_file()
        );
        assert_eq!(source.value().attr("type"), Some("image/webp"));
    }
    let control = home.select(&select("#pause-motion")).next().unwrap();
    assert_eq!(control.value().attr("type"), Some("checkbox"));
    assert!(!control.value().attr("aria-label").unwrap().is_empty());
    assert_eq!(home.select(&select("label[for='pause-motion']")).count(), 1);
    let fer = Html::parse_document(&render_page(Route::Fer, SiteConfig::new("").unwrap()));
    assert_eq!(fer.select(&select(".smoke-atmosphere")).count(), 1);
    assert_eq!(fer.select(&select("video")).count(), 1);
    let not_found =
        Html::parse_document(&render_page(Route::NotFound, SiteConfig::new("").unwrap()));
    assert_eq!(not_found.select(&select(".smoke-atmosphere")).count(), 0);
    assert_eq!(not_found.select(&select("video")).count(), 0);
}

#[test]
fn origin_configuration_rejects_credentials_paths_and_injection() {
    for value in [
        "javascript:alert(1)",
        "https://",
        "https://user:secret@site.test",
        "https://site.test/path",
        "https://site.test?q=1",
        "https://site.test#x",
        "https://site.test\nmalicious",
        "http://remote.test",
        "https://site.test:99999",
        "https://-bad.test",
        "https://site..test",
    ] {
        assert!(SiteConfig::new(value).is_err(), "reject {value}");
    }
    for value in [
        "https://site.test",
        "https://site.test/",
        "http://127.0.0.1:8080",
        "http://localhost:8080",
    ] {
        assert!(SiteConfig::new(value).is_ok(), "accept {value}");
    }
    assert!(SiteConfig::new("").unwrap().origin.is_none());
}

#[test]
fn project_pages_keep_navigation_and_assets_inside_the_repository_path() {
    let config = SiteConfig::new("https://psikox.github.io")
        .unwrap()
        .with_base_path("/psikopata/psikopapa")
        .unwrap();
    assert_eq!(
        config.canonical(Route::Fer).as_deref(),
        Some("https://psikox.github.io/psikopata/psikopapa/fer/")
    );
    for route in Route::ALL {
        let html = render_page(route, config.clone());
        let doc = Html::parse_document(&html);
        for element in doc.select(&select(
            "a[href], link[href], img[src], source[src], script[src]",
        )) {
            let path = element
                .value()
                .attr("href")
                .or_else(|| element.value().attr("src"))
                .unwrap();
            if path.starts_with('/') {
                assert!(
                    path.starts_with("/psikopata/psikopapa/"),
                    "{route:?}: {path}"
                );
            }
        }
        for element in doc.select(&select("source[srcset], link[imagesrcset]")) {
            let candidates = element
                .value()
                .attr("srcset")
                .or_else(|| element.value().attr("imagesrcset"))
                .unwrap();
            for candidate in candidates.split(',') {
                assert!(
                    candidate.trim().starts_with("/psikopata/psikopapa/assets/"),
                    "{route:?}: {candidate}"
                );
            }
        }
        assert!(!html.contains("javascript:"));
    }
    let css = prefix_local_urls(styles::CSS, &config.base_path);
    assert!(css.contains("url('/psikopata/psikopapa/assets/fonts/"));
    assert!(css.contains("url('/psikopata/psikopapa/assets/icons/whatsapp.svg')"));
    assert!(css.contains("url('/psikopata/psikopapa/assets/icons/spotify.svg')"));
    assert!(!css.contains("url('/assets/"));
    for invalid in [
        "psikopapa",
        "/",
        "/psikopapa/",
        "/../psikopapa",
        "/psikopapa?test",
        "/psikopapa//fer",
    ] {
        assert!(
            SiteConfig::new("")
                .unwrap()
                .with_base_path(invalid)
                .is_err(),
            "reject {invalid}"
        );
    }
}
