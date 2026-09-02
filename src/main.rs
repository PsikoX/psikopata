use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::time::Instant;

const SITE_NAME: &str = "PSIKOPATA";

struct Page {
    slug: String,
    meta: HashMap<String, String>,
    body: String,
}

fn parse_front_matter(src: &str) -> (HashMap<String, String>, String) {
    let mut meta = HashMap::new();
    if let Some(rest) = src.strip_prefix("---") {
        if let Some(end) = rest.find("\n---") {
            let head = &rest[..end];
            let body = rest[end + 4..].trim_start_matches('\n').to_string();
            for line in head.lines() {
                if let Some((k, v)) = line.split_once(':') {
                    meta.insert(k.trim().to_string(), v.trim().to_string());
                }
            }
            return (meta, body);
        }
    }
    (meta, src.to_string())
}

fn minify_css(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut in_comment = false;
    let mut prev = '\0';
    let mut ws_pending = false;
    for c in css.chars() {
        if in_comment {
            if prev == '*' && c == '/' {
                in_comment = false;
            }
            prev = c;
            continue;
        }
        if prev == '/' && c == '*' {
            in_comment = true;
            out.pop();
            prev = '\0';
            continue;
        }
        if c.is_whitespace() {
            ws_pending = true;
            prev = c;
            continue;
        }
        if ws_pending {
            if !matches!(c, '{' | '}' | ';' | ':' | ',' | '>')
                && !matches!(prev, '{' | '}' | ';' | ':' | ',' | '>')
            {
                out.push(' ');
            }
            ws_pending = false;
        }
        out.push(c);
        prev = c;
    }
    out
}

fn layout(page: &Page, css: &str, nav: &[(&str, &str)]) -> String {
    let title = page.meta.get("title").map(|s| s.as_str()).unwrap_or(SITE_NAME);
    let desc = page
        .meta
        .get("description")
        .map(|s| s.as_str())
        .unwrap_or("Psikopata começa no som e termina na mente.");
    let canonical = format!("https://psikopata.com/{}", page.slug);

    let nav_html: String = nav
        .iter()
        .map(|(href, label)| {
            let active = if page.slug == href.trim_start_matches('/') {
                " aria-current=\"page\""
            } else {
                ""
            };
            format!("<a href=\"{href}\"{active}>{label}</a>")
        })
        .collect::<Vec<_>>()
        .join("");

    format!(
        r##"<!doctype html>
<html lang="pt">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>{title} — PSIKOPATA</title>
<meta name="description" content="{desc}">
<link rel="canonical" href="{canonical}">
<meta property="og:type" content="website">
<meta property="og:title" content="{title} — PSIKOPATA">
<meta property="og:description" content="{desc}">
<meta property="og:url" content="{canonical}">
<meta property="og:image" content="https://psikopata.com/art/og.png">
<meta property="og:locale" content="pt_PT">
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:image" content="https://psikopata.com/art/og.png">
<meta name="theme-color" content="#0d0a09">
<link rel="icon" type="image/svg+xml" href="/art/sigil.svg">
<style>{css}</style>
</head>
<body>
<div class="glow" aria-hidden="true"></div>
<a class="skip" href="#main">Saltar para o conteúdo</a>
<header class="hdr">
<a class="logo" href="/">PSIKOPATA</a>
<input type="checkbox" id="menu" class="menu-check">
<label for="menu" class="menu-btn" aria-label="Abrir menu"><i></i><i></i><i></i></label>
<nav class="nav" aria-label="principal">{nav_html}</nav>
</header>
<main id="main">
{body}
</main>
<footer class="ftr">
<p><a href="/manifesto">Manifesto</a> · <a href="/producoes">Produções</a> · <a href="/lab">Lab</a></p>
<p><span aria-hidden="true">©</span> PSIKOPATA — começa no som, termina na mente.</p>
</footer>
</body>
</html>"##,
        body = page.body
    )
}

fn build(root: &Path) {
    let t0 = Instant::now();
    let dist = root.join("dist");
    if dist.exists() {
        fs::remove_dir_all(&dist).unwrap();
    }
    fs::create_dir_all(&dist).unwrap();

    let css_src = fs::read_to_string(root.join("css/site.css")).expect("css/site.css em falta");
    let css = minify_css(&css_src);

    let mut pages: Vec<Page> = Vec::new();
    for entry in fs::read_dir(root.join("content")).expect("pasta content/ em falta") {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "html") {
            let src = fs::read_to_string(&path).unwrap();
            let (meta, body) = parse_front_matter(&src);
            let slug = path.file_stem().unwrap().to_string_lossy().to_string();
            let slug = if slug == "index" { String::new() } else { slug };
            pages.push(Page { slug, meta, body });
        }
    }
    pages.sort_by(|a, b| a.slug.cmp(&b.slug));

    let nav: Vec<(&str, &str)> = vec![
        ("/", "SOM"),
        ("/psikotica-bipolar", "PSIKOTICA BIPOLAR"),
        ("/psikose-em-maputo", "PSIKOSE EM MAPUTO"),
        ("/universo", "UNIVERSO"),
        ("/entrar", "ENTRAR"),
    ];

    for page in &pages {
        let html = layout(page, &css, &nav);
        let out = if page.slug.is_empty() {
            dist.join("index.html")
        } else {
            fs::create_dir_all(dist.join(&page.slug)).unwrap();
            dist.join(&page.slug).join("index.html")
        };
        fs::write(&out, html).unwrap();
        println!("  → {}", out.strip_prefix(root).unwrap().display());
    }

    copy_images(&root.join("img"), &dist.join("img"));
    copy_images(&root.join("art"), &dist.join("art"));

    fs::write(
        dist.join("robots.txt"),
        "User-agent: *\nAllow: /\n\nSitemap: https://psikopata.com/sitemap.xml\n",
    )
    .unwrap();

    let urls: Vec<String> = pages
        .iter()
        .map(|p| {
            format!(
                "  <url><loc>https://psikopata.com/{}</loc></url>",
                p.slug
            )
        })
        .collect();
    fs::write(
        dist.join("sitemap.xml"),
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}
</urlset>
"#,
            urls.join("\n")
        ),
    )
    .unwrap();

    let total: u64 = walk_size(&dist);
    println!(
        "✔ build em {:.1}ms — {} páginas, {:.1} KB total, 0 bytes de JS",
        t0.elapsed().as_secs_f64() * 1000.0,
        pages.len(),
        total as f64 / 1024.0
    );
}

fn copy_images(src: &Path, dst: &Path) {
    if !src.exists() {
        return;
    }
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap().flatten() {
        let p = entry.path();
        if p.is_file() {
            fs::copy(&p, dst.join(p.file_name().unwrap())).unwrap();
        }
    }
}

fn walk_size(dir: &Path) -> u64 {
    let mut total = 0;
    for entry in fs::read_dir(dir).unwrap().flatten() {
        let p = entry.path();
        if p.is_dir() {
            total += walk_size(&p);
        } else {
            total += p.metadata().map(|m| m.len()).unwrap_or(0);
        }
    }
    total
}

fn mime(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "text/javascript",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "avif" => "image/avif",
        "mp3" => "audio/mpeg",
        "xml" => "application/xml",
        "txt" => "text/plain",
        "webmanifest" => "application/manifest+json",
        _ => "application/octet-stream",
    }
}

fn serve(dist: &Path, port: u16) {
    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
    println!("http://127.0.0.1:{port}  (Ctrl+C para parar)");
    for stream in listener.incoming().flatten() {
        let dist = dist.to_path_buf();
        std::thread::spawn(move || handle(stream, &dist));
    }
}

fn handle(mut stream: TcpStream, dist: &Path) {
    let mut buf = [0u8; 2048];
    let n = stream.read(&mut buf).unwrap_or(0);
    let req = String::from_utf8_lossy(&buf[..n]);
    let mut path = req
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
        .split('?')
        .next()
        .unwrap_or("/")
        .to_string();

    if path.ends_with('/') {
        path.push_str("index.html");
    }
    let file = dist.join(path.trim_start_matches('/'));
    let file = if file.is_dir() {
        file.join("index.html")
    } else {
        file
    };

    let (status, body) = match fs::read(&file) {
        Ok(b) => ("200 OK", b),
        Err(_) => ("404 Not Found", "404 — não existe".as_bytes().to_vec()),
    };
    let resp = format!(
        "HTTP/1.1 {status}\r\ncontent-type: {}\r\ncontent-length: {}\r\ncache-control: no-cache\r\nconnection: close\r\n\r\n",
        mime(&file),
        body.len()
    );
    let _ = stream.write_all(resp.as_bytes());
    let _ = stream.write_all(&body);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    match args.get(1).map(|s| s.as_str()) {
        Some("serve") => {
            build(&root);
            serve(&root.join("dist"), 8080);
        }
        _ => build(&root),
    }
}
