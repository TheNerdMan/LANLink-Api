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
    
    pub fn create_new_user_for_db(&self) -> UserModel {
        let mut new_user = UserModel::new();
        new_user.publicid = Uuid::new_v4();
        new_user.first_name = self.first_name.clone();
        new_user.first_name = self.first_name.clone();
        new_user.last_name = self.last_name.clone();
        new_user.discord_username = self.discord_username.clone();
        new_user.steam_url = self.steam_url.clone();
        new_user
    }
    
    pub fn create_update_user_for_db(&self) -> UserModel {
        // we are making sure not to delete the user public ID as it should be static
        let mut update_user = UserModel::new();
        update_user.id = self.id.clone();
        update_user.first_name = self.first_name.clone();
        update_user.last_name = self.last_name.clone();
        update_user.discord_username = self.discord_username.clone();
        update_user.steam_url = self.steam_url.clone();
        update_user
    }
}