mod assets;
mod context;
mod modules;

use axum::routing::Router;
use std::net::SocketAddr;

use crate::assets::{get_admin_dir, get_assets_dir, static_handler};
use crate::context::config::Config;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = Config::new();
    let app = Router::new()
        .nest_service("/admin", get_admin_dir())
        .nest_service("/assets", get_assets_dir())
        .fallback(static_handler);

    let addr = SocketAddr::new(config.server_host, config.server_port);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
