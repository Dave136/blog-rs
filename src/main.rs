mod assets;
mod context;
mod modules;

use axum::http::StatusCode;
use axum::routing::{get, Router};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::assets::{get_admin_dir, get_assets_dir, static_handler};
use crate::context::config::Config;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = Config::new();
    let app = Router::new()
        .route("/", get(root))
        .nest_service("/admin", get_admin_dir())
        .nest_service("/assets", get_assets_dir())
        .fallback(static_handler);

    let addr = SocketAddr::new(config.server_host, config.server_port);
    println!("API -> {}/api", addr);
    println!("Admin UI -> {}/admin", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    code: u16,
    message: String,
    data: T,
}

async fn root() -> Json<ApiResponse<Option<String>>> {
    let response = ApiResponse {
        code: StatusCode::NOT_FOUND.as_u16(),
        message: String::from("Not found"),
        data: None,
    };

    Json(response)
}
