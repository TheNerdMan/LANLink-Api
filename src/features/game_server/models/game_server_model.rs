use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;
use crate::features::game_server::dtos::game_server_dto::GameServerDto;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::game_servers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GameServerModel {
    pub id: i32,
    pub publicid: Uuid,
    pub game_server_title: String,
    pub game_type: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::game_servers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGameServerModel{
    pub publicid: Uuid,
    pub game_server_title: String,
    pub game_type: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::game_servers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateGameServerModel{
    pub id: i32,
    pub publicid: Uuid,
    pub game_server_title: String,
    pub game_type: String,
}

impl GameServerModel{
    pub fn new() -> GameServerModel{
        GameServerModel{
            id: 0,
            publicid: Uuid::new_v4(),
            game_server_title: String::new(),
            game_type: String::new(),
        }
    }

    pub fn new_from_dto(dto: GameServerDto) -> Self{
        GameServerModel{
            id: 0,
            publicid: Uuid::new_v4(),
            game_server_title: dto.game_server_title.clone(),
            game_type: dto.game_type.clone(),
        }
    }

    pub fn is_empty(&self) -> bool{
        if self.game_server_title == String::new()
            && self.game_type == String::new(){
            true
        }else {
            false
        }
    }

    pub fn create_new_game_server_for_db(&self) -> NewGameServerModel{
        NewGameServerModel{
            publicid: Uuid::new_v4(),
            game_server_title: self.game_server_title.clone(),
            game_type: self.game_type.clone(),
        }
    }

    pub fn create_update_game_server_for_db(&self) -> UpdateGameServerModel{
        UpdateGameServerModel{
            id: self.id.clone(),
            publicid: Uuid::new_v4(),
            game_server_title: self.game_server_title.clone(),
            game_type: self.game_type.clone(),
        }
    }
}