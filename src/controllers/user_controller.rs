use axum::{extract::Path, http::StatusCode, response::Json, routing::get, routing::post, Router};

use crate::objects::user::User;
use crate::objects::file_manager;

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/user/:username", get(get_user))
        .route("/api/v1/user/:username", post(create_user ))
        .route("/api/v1/user/:username/edit/:discord", post(set_user_discord))

}

async fn set_user_discord(Path((username, discord)): Path<(String, String)>) -> (StatusCode, Json<User>) {
    let user = User{
        username,
        discord,
        steam: String::new(),
        strike_count: 0

    };
    (StatusCode::OK, Json(user))
}


async fn get_user(Path(username): Path<String>) -> (StatusCode, Json<User>) {
    // Do db call to find user
    let user = file_manager::load_user(&username);
    if user.is_empty(){
        return (StatusCode::NO_CONTENT, Json(user));
    }else{
        return (StatusCode::OK, Json(user));
    }
}

async fn create_user(Path(username): Path<String>) -> (StatusCode, Json<User>) {
    if username.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(User::new()));
    }

    let user = file_manager::load_user(&username);
    if !user.is_empty() {
        return (StatusCode::FORBIDDEN, Json(User::new()));
    }
    
    let user = User{
        username,
        discord: String::new(),
        steam: String::new(),
        strike_count: 0
    };

    // ======================== SAVE USER ===================================
    let code: StatusCode = file_manager::save_user(&user);

    let result: (StatusCode, Json<User>) = if code == StatusCode::OK{
        (StatusCode::OK, Json(user))
    }else{
        (StatusCode::IM_A_TEAPOT, Json(User::new()))
    };
    // ======================== END SAVE USER ===================================

    result
}