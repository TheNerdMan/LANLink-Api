use axum::{extract::Path, http::StatusCode, response::Json, routing::get, routing::post, Router};

use crate::objects::user::User;
use crate::objects::file_manager;

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
        steam: String::new(),
        strike_count: 0,

    };
    (StatusCode::OK, Json(user))
}

async fn create_user(Path(username): Path<String>) -> (StatusCode, Json<User>) {
    if username.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(User::new()));
    }
    
    let user = User{
        username,
        discord: String::new(),
        steam: String::new(),
        strike_count: 0
    };

    let code: StatusCode = file_manager::save_user(&user);

    let result: (StatusCode, Json<User>) = if code == StatusCode::OK{
        (StatusCode::OK, Json(user))
    }else{
        (StatusCode::IM_A_TEAPOT, Json(User::new()))
    };
    
    // Push this user to db
    result
}