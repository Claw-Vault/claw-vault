use axum::routing::{get, Router};

pub fn bind_routes(router: Router) -> Router {
    router.route("/", get(health))
}

async fn health() -> &'static str {
    "Server is up and running 🚀🚀"
}
