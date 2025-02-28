use axum::routing::{get, Router};
use lib_core::config::Config;
use tower_http::services::{ServeDir, ServeFile};

use crate::app::App;

pub mod api;

pub fn bind_routes(router: Router<App>) -> Router<App> {
    router
        .route("/", get(api::index))
        .nest_service("/assets", ServeDir::new(Config::get_assets_dir()))
        .route_service("/robots.txt", ServeFile::new(Config::get_assets_dir() + "/robots.txt"))
        .route("/privacy", get(api::privacy))
        .route("/vault/:id", get(api::vault))
}
