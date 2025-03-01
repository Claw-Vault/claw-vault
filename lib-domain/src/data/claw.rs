use lib_core::{enums::ValidDuration, AppError, AppResult, ErrType};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use super::{Datastore, DbThing};

#[derive(Serialize, Deserialize)]
pub struct Claw {
    pub id: RecordId,
    pub expiry_at: i64,

    pub data: String,
    pub pem: String,
    pub sha256: String,
    pub validity: ValidDuration,
}
impl DbThing for Claw {
    fn get_name() -> &'static str {
        "claw"
    }
}

impl Datastore {
    pub async fn save_claw(
        &self,
        data: String,
        pem: String,
        sha256: String,
        validity: ValidDuration,
    ) -> AppResult<Claw> {
        let id = Claw::rand_id();
        let expiry_at =
            chrono::Utc::now().timestamp_millis() + (validity.get_duration() as i64 * 1000);

        let claw: Option<Claw> = self
            .db
            .create(&id)
            .content(Claw { id, expiry_at, data, pem, sha256, validity })
            .await
            .map_err(|e| AppError::err(ErrType::DbError, e, "Failed to create claw"))?;

        claw.ok_or_else(|| AppError::new(ErrType::DbError, "Created claw is NONE"))
    }

    pub async fn get_claw(&self, id: &str) -> AppResult<Option<Claw>> {
        let id = Claw::id(id);
        let claw: Option<Claw> = self
            .db
            .select(&id)
            .await
            .map_err(|e| AppError::err(ErrType::DbError, e, "Failed to fetch claw by id"))?;

        Ok(claw)
    }

    pub async fn delete_claw(&self, id: RecordId) -> AppResult<()> {
        let _: Option<Claw> = self
            .db
            .select(&id)
            .await
            .map_err(|e| AppError::err(ErrType::DbError, e, "Failed to delete claw by id"))?;

        Ok(())
    }
}
