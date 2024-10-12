use serde::Serialize;

#[derive(Serialize)]
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
}