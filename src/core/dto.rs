use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// ----- REQUEST

/// Trait that all request DTO's should implement
pub trait RequestStruct<T> {
    /// Function to return the objects stored in the struct
    fn expand(self) -> T;
}

/// DTO used for receiving body in [`crate::handlers::api::encrypt`]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncryptQueryBody {
    value: String,
    validity: i64,
}

impl RequestStruct<(String, i64)> for EncryptQueryBody {
    fn expand(self) -> (String, i64) {
        (self.value, self.validity)
    }
}

/// DTO used for receiving body in [`crate::handlers::api::decrypt`]
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

// ----- RESPONSE

/// DTO used for response body in [`crate::handlers::api::encrypt`]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncryptResponse {
    data_id: String,
    key_id: String,
    valid_for: String,
}

impl EncryptResponse {
    pub fn new(data_id: String, key_id: Uuid, valid_for: String) -> Self {
        EncryptResponse {
            data_id,
            key_id: key_id.to_string(),
            valid_for,
        }
    }
}

/// DTO used for response body in [`crate::handlers::api::decrypt`]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DecryptResponse {
    data: String,
}

impl DecryptResponse {
    pub fn new(data: String) -> Self {
        DecryptResponse { data }
    }
}

/// Error DTO used for response body in all APIs
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ErrorMessage {
    code: u16,
    message: String,
}

impl ErrorMessage {
    pub fn new(code: StatusCode, message: String) -> Self {
        ErrorMessage {
            code: code.as_u16(),
            message,
        }
    }
}
