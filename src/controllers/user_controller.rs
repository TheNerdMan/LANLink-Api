use axum::{extract::Path, http::StatusCode, response::Json, routing::get, routing::post, Router};

use crate::objects::user::User;
use crate::objects::file_manager;

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/user/:username", get(get_user))
        .route("/api/v1/user/:username", post(create_user ))
        .route("/api/v1/user/:username/edit/:type/:steam", post(set_user_test))

}

async fn set_user_test(Path((username, var, value)): Path<(String, String, String)>) -> (StatusCode, Json<User>){
    
    if username.is_empty() || var.is_empty() || value.is_empty(){
        return (StatusCode::BAD_REQUEST, Json(User::new()))
    }

    let user = get_user(Path(username.to_string())).await.1.0;
    if user.is_empty(){
        return (StatusCode::NO_CONTENT, Json(user));
    }
    let mut new_user = User{
        ..user
    };

    if var == "discord" {
        new_user.discord = value;
    }else if var == "steam"{
        new_user.steam = value;
    }else{
        return (StatusCode::BAD_REQUEST, Json(User::new()))
    }

    let code = file_manager::save_user(&new_user);
    (code, Json(new_user))
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