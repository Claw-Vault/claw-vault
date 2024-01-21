use std::str::FromStr;
use std::{ops::Deref, sync::Arc};

use axum::{Extension, Json};
use uuid::Uuid;

use crate::core::dao;
use crate::core::dto::RequestStruct;
use crate::{app::App, core::dto};

#[utoipa::path(
    post,
    path = "/api/v1/encrypt",
    request_body = EncryptQueryBody,
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
        Err(err) => return Err(Json(dto::ErrorMessage::server_error(err.to_string()))),
    };

    let pem = match cipher.encrypt_pem(&uuid, pem) {
        Ok(pem) => pem,
        Err(err) => return Err(Json(dto::ErrorMessage::server_error(err.to_string()))),
    };

    let claw = match dao::save_claw(id.clone(), encrypted, md5hash, validity, &db).await {
        Ok(v) => v,
        Err(err) => return Err(Json(err)),
    };
    match dao::save_claw_key(id, pem, &db).await {
        Ok(_) => (),
        Err(err) => return Err(Json(err)),
    };

    Ok(Json(dto::EncryptResponse::new(claw.id, uuid)))
}

#[utoipa::path(
    post,
    path = "/api/v1/decrypt",
    request_body = DecryptQueryBody,
    responses(
        (status=200, description="Decrypt the given data", body = DecryptQueryBody),
        (status=400, description="Error", body = ErrorMessage),
    ),
    tag = "Api",
)]
pub async fn decrypt(
    Extension(app): Extension<Arc<App>>,
    Json(decrypt_query): Json<dto::DecryptQueryBody>,
) -> Result<Json<dto::DecryptResponse>, Json<dto::ErrorMessage>> {
    let (cipher, db) = app.deref().clone().expand();
    let (id, key) = decrypt_query.expand();

    let claw = match dao::get_claw_by_id(id.clone(), &db).await {
        Ok(model) => model,
        Err(err) => return Err(Json(err)),
    };

    let claw_key = match dao::get_claw_key_by_id(id, &db).await {
        Ok(model) => model,
        Err(err) => return Err(Json(err)),
    };

    let uuid = match Uuid::from_str(key.as_str()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err(Json(dto::ErrorMessage::bad_request(String::from(
                "Bad key",
            ))))
        }
    };

    let pem = match cipher.decrypt_pem(&uuid, claw_key.pem) {
        Ok(pem) => pem,
        Err(err) => return Err(Json(dto::ErrorMessage::bad_request(err.to_string()))),
    };

    let data = match cipher.decrypt(pem, claw.data) {
        Ok(data) => data,
        Err(err) => return Err(Json(dto::ErrorMessage::bad_request(err.to_string()))),
    };

    match dao::delete_claw(claw.id, &db).await {
        Ok(_) => (),
        Err(err) => return Err(Json(err)),
    }

    Ok(Json(dto::DecryptResponse::new(data)))
}
