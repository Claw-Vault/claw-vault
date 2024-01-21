use axum::routing::Router;
use utoipa::OpenApi;

pub mod api;
pub mod health;

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health,
        crate::handlers::api::encrypt,
        crate::handlers::api::decrypt
    ),
    components(schemas(
        crate::core::dto::EncryptQueryBody,
        crate::core::dto::EncryptResponse,
        crate::core::dto::DecryptQueryBody,
        crate::core::dto::DecryptResponse,
        crate::core::dto::ErrorMessage
    ))
)]
pub struct ApiDoc;

pub fn bind_routes(router: Router) -> Router {
    let router = health::bind_routes(router);
    let router = api::bind_routes(router);
    router
}
