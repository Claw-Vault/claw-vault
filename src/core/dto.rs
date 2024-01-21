use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// REQUEST

pub trait RequestStruct<T> {
    fn expand(self) -> T;
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncryptQueryBody {
    value: String,
    validity: i32,
}

impl RequestStruct<(String, i32)> for EncryptQueryBody {
    fn expand(self) -> (String, i32) {
        (self.value, self.validity)
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DecryptQueryBody {
    id: String,
    key: String,
}

impl RequestStruct<(String, String)> for DecryptQueryBody {
    fn expand(self) -> (String, String) {
        (self.id, self.key)
    }
}

/// RESPONSE

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncryptResponse {
    data_id: String,
    key_id: String,
}

impl EncryptResponse {
    pub fn new(data_id: String, key_id: Uuid) -> Self {
        EncryptResponse {
            data_id,
            key_id: key_id.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DecryptResponse {
    data: String,
}

impl DecryptResponse {
    pub fn new(data: String) -> Self {
        DecryptResponse { data }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ErrorMessage {
    code: u16,
    message: String,
}

impl ErrorMessage {
    pub fn bad_request(message: String) -> Self {
        ErrorMessage {
            code: StatusCode::BAD_REQUEST.as_u16(),
            message,
        }
    }

    pub fn server_error(message: String) -> Self {
        ErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message,
        }
    }
}
