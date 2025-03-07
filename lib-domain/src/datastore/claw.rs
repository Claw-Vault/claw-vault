use lib_core::{AppError, AppResult, ErrType, enums::ValidDuration};
use nanoid::nanoid;
use sqlx::Row;

use super::Datastore;

pub struct Claw {
    pub id: String,
    pub expiry_at: i64,

    pub data: String,
    pub pem: String,
    pub sha256: String,
    pub validity: ValidDuration,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Claw {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let expiry_at: i64 = row.try_get("expiry_at")?;
        let data: String = row.try_get("data")?;
        let pem: String = row.try_get("pem")?;
        let sha256: String = row.try_get("sha256")?;
        let validity: i64 = row.try_get("validity")?;
        let validity: ValidDuration = validity.into();

        Ok(Claw { id, expiry_at, data, pem, sha256, validity })
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
        let id = nanoid!(20);

        let validity = validity.get_duration();
        let expiry_at = chrono::Utc::now().timestamp_millis() + (validity as i64 * 1000);

        let claw = sqlx::query_as(
            r#"INSERT INTO claw (id, expiry_at, data, pem, sha256, validity)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, expiry_at, data, pem, sha256, validity"#,
        )
        .bind(id)
        .bind(expiry_at)
        .bind(data)
        .bind(pem)
        .bind(sha256)
        .bind(validity)
        .fetch_one(&self.db)
        .await
        .map_err(|e| AppError::err(ErrType::DbError, e, "Failed to create claw"))?;

        Ok(claw)
    }

    pub async fn get_claw(&self, id: &str) -> AppResult<Option<Claw>> {
        let claw = match sqlx::query_as(r#"SELECT * FROM claw WHERE id = $1"#)
            .bind(id)
            .fetch_one(&self.db)
            .await
        {
            Ok(c) => c,
            Err(e) => match e {
                sqlx::Error::RowNotFound => return Ok(None),
                _ => return Err(AppError::err(ErrType::DbError, e, "Failed to create claw")),
            },
        };
        Ok(Some(claw))
    }

    pub async fn delete_claw(&self, id: &str) -> AppResult<()> {
        let _ = Self::__delete_claw(&self.db, id)
            .await
            .map_err(|e| AppError::err(ErrType::DbError, e, "Failed to delete claw by id"))?;
        Ok(())
    }

    pub(super) async fn __delete_claw(
        db: &sqlx::SqlitePool,
        id: &str,
    ) -> Result<Option<()>, sqlx::Error> {
        let res = sqlx::query(r#"DELETE FROM claw WHERE id = $1"#).bind(id).execute(db).await;
        match res {
            Ok(res) => {
                if res.rows_affected() > 0 {
                    Ok(Some(()))
                } else {
                    Ok(None)
                }
            }
            Err(err) => Err(err),
        }
    }
}
