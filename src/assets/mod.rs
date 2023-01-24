use std::io;

use axum::{
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{get_service, MethodRouter},
};

use rust_embed::RustEmbed;
use tower_http::services::ServeDir;

#[derive(RustEmbed)]
#[folder = "ui/dist"]
#[exclude = "*.html"]
#[include = "*.svg"]
struct Assets;

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

fn get_mime(path: &str) -> &str {
    if path.contains(".js") {
        return "text/javascript";
    } else if path.contains(".css") {
        return "text/css";
    } else if path.contains(".svg") {
        return "image/svg+xml";
    } else if path.contains(".png") {
        return "image/png";
    } else if path.contains(".jpg") {
        return "image/jpg";
    } else {
        return "octet-stream";
    }
}

async fn not_found() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(boxed(Full::from("404")))
        .unwrap()
}

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    match Assets::get(path) {
        Some(content) => {
            let body = boxed(Full::from(content.data));

            Response::builder()
                .header(header::CONTENT_TYPE, get_mime(path))
                .body(body)
                .unwrap()
        }
        None => not_found().await,
    }
}

pub fn get_admin_dir() -> MethodRouter {
    get_service(ServeDir::new("ui/dist")).handle_error(handle_error)
}

pub fn get_assets_dir() -> MethodRouter {
    get_service(ServeDir::new("ui/dist/assets")).handle_error(handle_error)
}
