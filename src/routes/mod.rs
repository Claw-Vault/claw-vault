use axum::routing::Router;
use utoipa::OpenApi;

mod api;
mod health;

#[derive(OpenApi)]
#[openapi(
    paths(health::health, api::encrypt),
    components(schemas(api::EncryptQuery))
)]
pub struct ApiDoc;

pub fn bind_routes(router: Router) -> Router {
    let router = health::bind_routes(router);
    let router = api::bind_routes(router);
    router
}
