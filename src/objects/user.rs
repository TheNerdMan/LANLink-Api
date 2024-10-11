use serde::Serialize;

#[derive(Serialize)]
pub struct User{
    pub username: String,
    pub discord: String,
    pub steam: String,
    
}