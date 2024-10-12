use std::fs::{self, create_dir_all, File, OpenOptions};
use std::io::{BufReader, Read};
use std::path::Path;
use axum::extract::rejection::MatchedPathRejection;
use axum::http::StatusCode;
use serde_json::to_writer;

use crate::objects::user::User;


const DATA_FOLDER: &str = "./userdata";

pub fn save_user(user: &User) -> StatusCode{

    let accounts_path = format!("{}/accounts", DATA_FOLDER);
    if !Path::new(&accounts_path).exists() {
        create_dir_all(&accounts_path).expect("Failed to create accounts folder");
    }


    let file_path = format!("{0}/accounts/{1}.json", &DATA_FOLDER, &user.username);

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .expect("Cannot open or create file");

    if let Err(e) = to_writer(&file, user) {
        eprintln!("Failed to write JSON: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    
    StatusCode::OK
}

pub fn load_user(username: &String) -> User{
    let accounts_path = format!("{}/accounts", DATA_FOLDER);
    if !Path::new(&accounts_path).exists() {
        create_dir_all(&accounts_path).expect("Failed to create accounts folder");
    }


    let file_path = format!("{0}/accounts/{1}.json", &DATA_FOLDER, &username);

    if !Path::new(&file_path).exists(){
        return User::new();
    }
    
    let json = match fs::read_to_string(&file_path){
        Ok(j) => j,
        Err(_) => "".to_string(),
    };


    let user: User = serde_json::from_str(json.as_str()).unwrap();
    user

}