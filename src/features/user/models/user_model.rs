use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel{
    pub id: i32,
    pub publicid: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub discord_username: String,
    pub steam_url: String,
}

impl UserModel{
    pub fn new() -> UserModel{
        UserModel{
            id: 0,
            publicid: Uuid::new_v4(),
            username: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            discord_username: String::new(),
            steam_url: String::new(),
        }
    }
    pub fn is_empty(&self) -> bool{
        return if self.username == String::new()
            && self.id == 0
            && self.first_name == String::new()
            && self.last_name == String::new()
            && self.discord_username == String::new()
            && self.steam_url == String::new() {
            true
        } else {
            false
        }
    }
}