use axum::routing::post;
use axum::Router;

use crate::handlers::api;

/// Binds routes for [`api`]
///
/// Returns a [`Router`]
pub fn bind_routes(router: Router) -> Router {
    router
        .route("/api/v1/encrypt", post(api::encrypt))
        .route("/api/v1/decrypt", post(api::decrypt))
}
