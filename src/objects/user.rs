use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User{
    pub username: String,
    pub discord: String,
    pub steam: String,
    pub strike_count: u8,

}

impl User{
    pub fn new() -> User{
        User{
            username: String::new(),
            discord: String::new(),
            steam: String::new(),
            strike_count: 0
        }
    }
    pub fn is_empty(&self) -> bool{
        if self.username == String::new()
        && self.discord == String::new()
        && self.steam == String::new()
        && self.strike_count == 0{
            return true;
        }else{
            return false;
        }
    }
}