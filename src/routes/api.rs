use crate::handlers::api;
use axum::routing::post;
use axum::Router;

pub fn bind_routes(router: Router) -> Router {
    router.route("/api/v1/encrypt", post(api::encrypt))
}
