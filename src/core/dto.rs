use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

pub trait JsonExpand<T> {
    fn expand(self) -> T;
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncryptQueryBody {
    value: String,
    validity: u64,
}

impl JsonExpand<(String, u64)> for EncryptQueryBody {
    fn expand(self) -> (String, u64) {
        (self.value, self.validity)
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EncryptResponse {
    data_id: String,
    key_id: Uuid,
}

impl EncryptResponse {
    pub fn new(data_id: String, key_id: Uuid) -> Self {
        EncryptResponse { data_id, key_id }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ErrorMessage {
    code: u16,
    message: &'static str,
}

impl ErrorMessage {
    pub fn new(code: StatusCode, message: &'static str) -> Self {
        ErrorMessage {
            code: code.as_u16(),
            message,
        }
    }
}
