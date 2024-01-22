use axum::routing::Router;
use utoipa::OpenApi;

mod api;
mod health;
mod web;

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
    let router = web::bind_routes(router);
    router
}
