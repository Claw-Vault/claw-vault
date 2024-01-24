use axum::routing::{get, Router};
use tower_http::services::ServeDir;

use crate::handlers::web;

/// Binds routes for [`web`]
///
/// Returns a [`Router`]
pub fn bind_routes(router: Router) -> Router {
    router
        .route("/", get(web::index))
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/privacy", get(web::privacy))
        .route("/:id", get(web::vault))
}
