use std::fs::{create_dir_all, File, OpenOptions};
use std::path::Path;
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