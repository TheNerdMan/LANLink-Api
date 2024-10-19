use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::core::errors::error::{AppError, AppErrorEnum};
use crate::features::game_server::models::game_server_model::GameServerModel;

#[derive(Deserialize, Serialize)]
pub struct GameServerDto{
    pub id: Option<i32>,
    pub publicid: Option<Uuid>,
    pub game_server_title: String,
    pub game_type: String,
}

impl GameServerDto {
    pub fn from_model(model: &GameServerModel) -> Self {
        GameServerDto{
            id: Some(model.id.clone()),
            publicid: Some(model.publicid.clone()),
            game_server_title: model.game_server_title.clone(),
            game_type: model.game_type.clone(),
        }
    }

    pub fn validate(&self) -> Result<bool, AppError> {
        if self.game_server_title.is_empty() || self.game_type.is_empty() {
            return Err(AppError::new(AppErrorEnum::BadRequestError, String::from("server_name or game_type cannot be empty")));
        }
        Ok(true)
    }
}