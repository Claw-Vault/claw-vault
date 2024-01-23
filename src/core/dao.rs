use std::time::Duration;

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait, Set};

use crate::app::AppError;

use super::models::{claw, claw_keys};

pub async fn connect_db() -> DatabaseConnection {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut opts = ConnectOptions::new(db_url.to_owned());
    opts.max_connections(255)
        .min_connections(10)
        .connect_timeout(Duration::from_secs(15))
        .sqlx_logging(false);

    Database::connect(opts)
        .await
        .expect("Failed to connect database")
}

pub async fn save_claw(
    id: String,
    data: String,
    md5hash: String,
    validity: i64,
    db: &DatabaseConnection,
) -> Result<claw::Model, AppError> {
    let model = claw::ActiveModel {
        id: Set(id),
        data: Set(data),
        md5hash: Set(md5hash),
        validity: Set(claw::ValidDuration::from_i64(validity)),
        created_at: Set(Utc::now().naive_utc()),
    };
    match model.insert(db).await {
        Ok(v) => Ok(v),
        Err(err) => Err(AppError::DbError(err.to_string())),
    }
}

pub async fn save_claw_key(
    id: String,
    pem: String,
    db: &DatabaseConnection,
) -> Result<claw_keys::Model, AppError> {
    let model = claw_keys::ActiveModel {
        id: Set(id),
        pem: Set(pem),
    };
    match model.insert(db).await {
        Ok(v) => Ok(v),
        Err(err) => Err(AppError::DbError(err.to_string())),
    }
}

pub async fn get_claw_by_id(id: String, db: &DatabaseConnection) -> Result<claw::Model, AppError> {
    match claw::Entity::find_by_id(id).one(db).await {
        Ok(model) => match model {
            Some(model) => Ok(model),
            None => Err(AppError::BadRequest(String::from("No data for id"))),
        },
        Err(err) => Err(AppError::DbError(err.to_string())),
    }
}

pub async fn get_all_claws(db: &DatabaseConnection) -> Result<Vec<claw::Model>, AppError> {
    match claw::Entity::find().all(db).await {
        Ok(claws) => Ok(claws),
        Err(err) => Err(AppError::DbError(err.to_string())),
    }
}

pub async fn get_claw_key_by_id(
    id: String,
    db: &DatabaseConnection,
) -> Result<claw_keys::Model, AppError> {
    match claw_keys::Entity::find_by_id(id).one(db).await {
        Ok(model) => match model {
            Some(model) => Ok(model),
            None => Err(AppError::BadRequest(String::from("No key for id"))),
        },
        Err(err) => Err(AppError::DbError(err.to_string())),
    }
}

pub async fn delete_claw(id: String, db: &DatabaseConnection) -> Result<bool, AppError> {
    match claw::Entity::delete_by_id(id).exec(db).await {
        Ok(model) => Ok(model.rows_affected == 1),
        Err(err) => Err(AppError::DbError(err.to_string())),
    }
}
