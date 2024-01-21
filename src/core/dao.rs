use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use super::{
    dto::ErrorMessage,
    models::{claw, claw_keys},
};

pub async fn save_claw(
    id: String,
    data: String,
    md5hash: String,
    validity: u64,
    db: &DatabaseConnection,
) -> Result<claw::Model, ErrorMessage> {
    let model = claw::ActiveModel {
        id: Set(id),
        data: Set(data),
        md5hash: Set(md5hash),
        validity: Set(validity),
    };
    match model.insert(db).await {
        Ok(v) => Ok(v),
        Err(err) => Err(ErrorMessage::server_error(err.to_string())),
    }
}

pub async fn save_claw_key(
    id: String,
    pem: String,
    db: &DatabaseConnection,
) -> Result<claw_keys::Model, ErrorMessage> {
    let model = claw_keys::ActiveModel {
        id: Set(id),
        pem: Set(pem),
    };
    match model.insert(db).await {
        Ok(v) => Ok(v),
        Err(err) => Err(ErrorMessage::server_error(err.to_string())),
    }
}

pub async fn get_claw_by_id(
    id: String,
    db: &DatabaseConnection,
) -> Result<claw::Model, ErrorMessage> {
    match claw::Entity::find_by_id(id).one(db).await {
        Ok(model) => match model {
            Some(model) => Ok(model),
            None => Err(ErrorMessage::bad_request(String::from("No data for id"))),
        },
        Err(err) => Err(ErrorMessage::server_error(err.to_string())),
    }
}

pub async fn get_claw_key_by_id(
    id: String,
    db: &DatabaseConnection,
) -> Result<claw_keys::Model, ErrorMessage> {
    match claw_keys::Entity::find_by_id(id).one(db).await {
        Ok(model) => match model {
            Some(model) => Ok(model),
            None => Err(ErrorMessage::bad_request(String::from("No key for id"))),
        },
        Err(err) => Err(ErrorMessage::server_error(err.to_string())),
    }
}

pub async fn delete_claw(id: String, db: &DatabaseConnection) -> Result<bool, ErrorMessage> {
    match claw::Entity::delete_by_id(id).exec(db).await {
        Ok(model) => Ok(model.rows_affected == 1),
        Err(err) => Err(ErrorMessage::server_error(err.to_string())),
    }
}
