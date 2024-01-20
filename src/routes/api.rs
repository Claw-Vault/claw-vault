use axum::{Json, Router};
use axum::routing::post;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub fn bind_routes(router: Router) -> Router {
    router.route("/api/v1/encrypt", post(encrypt))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncryptQuery {
    value: String,
    validity: u64,
}

#[utoipa::path(
    post,
    path = "/api/v1/encrypt",
    responses(
        (status=200, description="Encrypt the given data"),
        (status=400, description="Invalid data"),
    ),
)]
pub async fn encrypt(Json(encrypt_query): Json<EncryptQuery>) -> String {
    encrypt_query.value
}
