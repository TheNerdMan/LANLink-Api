use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::features::game_server::models::game_server_model::GameServerModel;

#[derive(Deserialize, Serialize)]
pub struct GameServerDto{
    pub id: i32,
    pub publicid: Uuid,
    pub game_server_title: String,
    pub game_type: String,
}

impl GameServerDto {
    pub fn from_model(model: &GameServerModel) -> Self {
        GameServerDto{
            id: model.id.clone(),
            publicid: model.publicid.clone(),
            game_server_title: model.game_server_title.clone(),
            game_type: model.game_type.clone(),
        }
    }
}