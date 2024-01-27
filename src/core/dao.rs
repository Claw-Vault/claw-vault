use std::time::Duration;

use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ConnectOptions, Database, DatabaseConnection, DbErr, EntityTrait, Set,
};

use super::models::{claw, claw_keys};

/// Function that connects db using `DATABASE_URL` from env
///
/// Panics when fails
pub async fn connect_db() -> DatabaseConnection {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut opts = ConnectOptions::new(db_url.to_owned());
    opts.max_connections(255)
        .min_connections(10)
        .connect_timeout(Duration::from_secs(15))
        .sqlx_logging(false);

    let connection = Database::connect(opts)
        .await
        .expect("Failed to connect database");

    tracing::info!("Connected to database");
    connection
}

/// Function to save the [`claw`]
pub async fn save_claw(
    id: String,
    data: String,
    hash: String,
    validity: i64,
    db: &DatabaseConnection,
) -> Result<claw::Model, DbErr> {
    let model = claw::ActiveModel {
        id: Set(id),
        data: Set(data),
        sha256: Set(hash),
        validity: Set(claw::ValidDuration::from_i64(validity)),
        created_at: Set(Utc::now().naive_utc()),
    };
    match model.insert(db).await {
        Ok(v) => Ok(v),
        Err(err) => Err(err),
    }
}

/// Function to save the [`claw_key`]
pub async fn save_claw_key(
    id: String,
    pem: String,
    db: &DatabaseConnection,
) -> Result<claw_keys::Model, DbErr> {
    let model = claw_keys::ActiveModel {
        id: Set(id),
        pem: Set(pem),
    };
    match model.insert(db).await {
        Ok(v) => Ok(v),
        Err(err) => Err(err),
    }
}

/// Function to get [`claw`] by `id`
pub async fn get_claw_by_id(id: String, db: &DatabaseConnection) -> Result<claw::Model, DbErr> {
    match claw::Entity::find_by_id(id).one(db).await {
        Ok(model) => match model {
            Some(model) => Ok(model),
            None => Err(DbErr::RecordNotFound(String::from("No data for id"))),
        },
        Err(err) => Err(err),
    }
}

/// Function to get all [`claw`]
pub async fn get_all_claws(db: &DatabaseConnection) -> Result<Vec<claw::Model>, DbErr> {
    match claw::Entity::find().all(db).await {
        Ok(claws) => Ok(claws),
        Err(err) => Err(err),
    }
}

/// Function to get [`claw_key`] by `id`
pub async fn get_claw_key_by_id(
    id: String,
    db: &DatabaseConnection,
) -> Result<claw_keys::Model, DbErr> {
    match claw_keys::Entity::find_by_id(id).one(db).await {
        Ok(model) => match model {
            Some(model) => Ok(model),
            None => Err(DbErr::RecordNotFound(String::from("No key for id"))),
        },
        Err(err) => Err(err),
    }
}

/// Function to delete the [`claw`]
///
/// This automatically deletes the [`claw_key`] (foreign key constraint)
pub async fn delete_claw(id: String, db: &DatabaseConnection) -> Result<bool, DbErr> {
    match claw::Entity::delete_by_id(id).exec(db).await {
        Ok(model) => Ok(model.rows_affected == 1),
        Err(err) => Err(err),
    }
}
