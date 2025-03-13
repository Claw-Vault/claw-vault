pub mod res {
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(Serialize, ToSchema)]
    pub struct EncryptResponse {
        pub id: String,
        pub key: String,
        pub valid_for: String,
    }

    #[derive(Serialize, ToSchema)]
    pub struct DecryptResponse {
        pub data: String,
    }
}

pub mod req {
    use lib_core::enums::ValidDuration;
    use serde::Deserialize;
    use utoipa::ToSchema;
    use validator::Validate;

    #[derive(Deserialize, ToSchema, Validate)]
    pub struct EncryptRequest {
        pub data: String,
        pub validity: ValidDuration,
    }

    #[derive(Deserialize, ToSchema, Validate)]
    pub struct DecryptRequest {
        pub id: String,
        pub key: String,
    }
}
