use std::{ops::Deref, sync::Arc};

use axum::{http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;

use crate::core::dto::JsonExpand;
use crate::core::models::{claw, claw_keys};
use crate::{app::App, core::dto};

#[utoipa::path(
    post,
    path = "/api/v1/encrypt",
    request_body = EncryptQuery,
    responses(
        (status=200, description="Encrypt the given data", body = EncryptResponse),
        (status=400, description="Error", body = ErrorMessage),
    ),
    tag = "Api",
)]
pub async fn encrypt(
    Extension(app): Extension<Arc<App>>,
    Json(encrypt_query): Json<dto::EncryptQueryBody>,
) -> Result<Json<dto::EncryptResponse>, Json<dto::ErrorMessage>> {
    let (data, validity) = encrypt_query.expand();
    let (cipher, db) = app.deref().clone().expand();

    let uuid = Uuid::new_v4();
    let (id, md5hash) = cipher.generate_id_hash(&data);

    let (encrypted, pem) = match cipher.encrypt(data) {
        Ok((enc, pem)) => (enc, pem),
        Err(err) => {
            return Err(Json(dto::ErrorMessage::new(
                StatusCode::BAD_REQUEST,
                err.as_str(),
            )))
        }
    };

    let pem = match cipher.encrypt_pem(&uuid, pem) {
        Ok(pem) => pem,
        Err(err) => {
            return Err(Json(dto::ErrorMessage::new(
                StatusCode::BAD_REQUEST,
                err.as_str(),
            )))
        }
    };

    let vault = claw::ActiveModel {
        id: Set(id.clone()),
        data: Set(encrypted),
        md5hash: Set(md5hash),
        validity: Set(validity),
    };

    let key = claw_keys::ActiveModel {
        id: Set(id),
        pem: Set(pem),
    };

    let vault = match vault.insert(&db).await {
        Ok(v) => v,
        Err(_) => {
            return Err(Json(dto::ErrorMessage::new(
                StatusCode::EXPECTATION_FAILED,
                "Failed to save data to db",
            )))
        }
    };

    let _ = match key.insert(&db).await {
        Ok(key) => key,
        Err(_) => {
            return Err(Json(dto::ErrorMessage::new(
                StatusCode::EXPECTATION_FAILED,
                "Failed to save key to db",
            )))
        }
    };

    Ok(Json(dto::EncryptResponse::new(vault.id, uuid)))
}
