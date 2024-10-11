use axum::{extract::Path, http::StatusCode, response::Json, routing::get, routing::post, Router};

use crate::objects::user::{self, User};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/user/:username", get(get_user))
        .route("/api/v1/user/:username", post(create_user))
}

async fn get_user(Path(username): Path<String>) -> (StatusCode, Json<User>) {
    // Do db call to find user
    let user = User{
        username,
        discord: String::new(),
        steam: String::new()
    };
    (StatusCode::OK, Json(user))
}

async fn create_user(Path(username): Path<String>) -> (StatusCode, Json<User>) {
    // Check if username already exists return null if so
    let user = User{
        username,
        discord: String::new(),
        steam: String::new()
    };
    // Push this user to db
    (StatusCode::CREATED, Json(user))
}