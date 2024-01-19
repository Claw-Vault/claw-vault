use axum::routing::Router;

mod health;

pub fn bind_routes(router: Router) -> Router {
    let router = health::bind_routes(router);
    router
}
