use psikopapa::{
    app::{Route, SiteConfig, prefix_local_urls, render_page},
    styles,
};
use std::{error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .init();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let output = root.join("dist");
    let config = SiteConfig::from_environment()?;
    if output.exists() {
        fs::remove_dir_all(&output)?;
    }
    fs::create_dir_all(&output)?;
    copy_tree(&root.join("assets"), &output.join("assets"))?;
    copy_tree(&root.join("docs/mockups/fer"), &output.join("mockups/fer"))?;
    fs::write(
        output.join("assets/site.css"),
        prefix_local_urls(styles::CSS, &config.base_path),
    )?;
    for route in Route::ALL {
        let html = render_page(route, config.clone());
        if html.contains("javascript:") || html.contains("<script>") {
            return Err("Inline JavaScript is not permitted".into());
        }
        let path = output.join(route.output_path());
        fs::create_dir_all(path.parent().ok_or("Output path has no parent")?)?;
        fs::write(&path, &html)?;
        tracing::info!(route = route.path(), bytes = html.len(), "Rendered page");
    }
    if let Some(origin) = &config.origin {
        let allowed_path = if config.base_path.is_empty() {
            "/".to_owned()
        } else {
            format!("{}/", config.base_path)
        };
        fs::write(
            output.join("robots.txt"),
            format!(
                "User-agent: *\nAllow: {allowed_path}\nSitemap: {origin}{}/sitemap.xml\n",
                config.base_path
            ),
        )?;
        fs::write(
            output.join("sitemap.xml"),
            format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\"><url><loc>{origin}{base}/</loc></url><url><loc>{origin}{base}/fer/</loc></url></urlset>\n",
                base = config.base_path
            ),
        )?;
    } else {
        fs::write(output.join("robots.txt"), "User-agent: *\nDisallow: /\n")?;
        let sitemap = output.join("sitemap.xml");
        if sitemap.exists() {
            fs::remove_file(sitemap)?;
        }
        tracing::warn!("No site origin set; local output is excluded from indexing");
    }
    fs::write(
        output.join("_headers"),
        "/*\n  Content-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self'; font-src 'self'; media-src 'self' https:; object-src 'none'; frame-src 'none'; base-uri 'self'; form-action 'self'; frame-ancestors 'self'\n  X-Content-Type-Options: nosniff\n  Referrer-Policy: strict-origin-when-cross-origin\n  Permissions-Policy: camera=(), microphone=(), geolocation=()\n/assets/*\n  Cache-Control: public, max-age=3600\n",
    )?;
    fs::write(output.join(".nojekyll"), "")?;
    tracing::info!(directory = %output.display(), "Static Rust build complete; one optional pointer script");
    Ok(())
}

fn copy_tree(from: &Path, to: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}
