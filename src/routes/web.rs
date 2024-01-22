use axum::routing::{get, Router};

use crate::handlers::web;

pub fn bind_routes(router: Router) -> Router {
    router
        .route("/", get(web::index))
        .route("/store", get(web::store))
        .route("/:id", get(web::vault))
}
