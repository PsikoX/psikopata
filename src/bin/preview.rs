use axum::{
    Router,
    http::{HeaderValue, header},
    response::IntoResponse,
};
use std::{error::Error, net::SocketAddr};
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().with_target(false).init();
    let address: SocketAddr = std::env::var("PREVIEW_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_owned())
        .parse()?;
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("dist");
    if !directory.join("index.html").exists() {
        return Err("Run cargo run --bin build-site before starting preview".into());
    }
    let app = Router::new().fallback_service(ServeDir::new(&directory).not_found_service(ServeFile::new(directory.join("404.html")))).layer(SetResponseHeaderLayer::overriding(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static("default-src 'self'; script-src 'none'; style-src 'self'; img-src 'self'; font-src 'self'; media-src 'self' https:; object-src 'none'; base-uri 'self'"))).layer(SetResponseHeaderLayer::overriding(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"))).route("/health", axum::routing::get(|| async { "ok".into_response() }));
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(url = %format!("http://{address}"), "PSIKOPAPA preview ready");
    axum::serve(listener, app).await?;
    Ok(())
}
