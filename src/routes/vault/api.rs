use axum::extract::State;
use axum::Extension;
use lib_core::interceptor::ReqId;
use lib_core::{ApiResponse, EmptyResponse, Json};
use lib_domain::dto::vault::req::{DecryptRequest, EncryptRequest};
use lib_domain::dto::vault::res::{DecryptResponse, EncryptResponse};

use crate::app::App;

/// Api to encrypt data
#[utoipa::path(
    post,
    path = "/api/v1/encrypt",
    request_body = EncryptRequest,
    responses(
        (status=200, description="Encrypt the given data", body = EncryptResponse),
        (status=400, description="Error", body = EmptyResponse),
    ),
    tag = "Api",
)]
pub async fn encrypt(
    State(app): State<App>,
    Extension(req_id): Extension<ReqId>,
    Json(enc_req): Json<EncryptRequest>,
) -> ApiResponse<EncryptResponse> {
    ApiResponse::map_res(app.service().encrypt_data(enc_req).await, req_id)
}

/// Api to decrypt data
#[utoipa::path(
    post,
    path = "/api/v1/decrypt",
    request_body = DecryptRequest,
    responses(
        (status=200, description="Decrypt the given data", body = DecryptResponse),
        (status=400, description="Error", body = EmptyResponse),
    ),
    tag = "Api",
)]
pub async fn decrypt(
    State(app): State<App>,
    Extension(req_id): Extension<ReqId>,
    Json(dto): Json<DecryptRequest>,
) -> ApiResponse<DecryptResponse> {
    ApiResponse::map_res(app.service().decrypt_data(dto).await, req_id)
}
