use axum::{extract::Path, response::Json, routing::post, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use deadpool_diesel::postgres::Pool;
use serde::Serialize;
use uuid::{Uuid};

use crate::features::user::models::user_model::UserModel;

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/user/:username", get(get_user))
        .route("/api/v1/user/:username", post(create_user))

}

#[axum::debug_handler]
async fn get_user(Path(username): Path<String>) -> (StatusCode, Json<UserModel>) {
    let dto = UserModel::new();
    (StatusCode::IM_A_TEAPOT, Json(dto))
}

#[axum::debug_handler]
async fn create_user(Path(username): Path<String>) -> (StatusCode, Json<UserModel>) {
    let dto = UserModel::new();
    if username.is_empty() {
       return (StatusCode::IM_A_TEAPOT, Json(dto))
    }

    (StatusCode::IM_A_TEAPOT, Json(dto))
    /*
    let user = file_manager::load_user(&username);
    if !user.is_empty() {
        return (StatusCode::FORBIDDEN, Json(UserModel::new()));
    }

    // If we get here, the user does not currently exist
    let user = UserModel{
        username,
        discord: String::new(),
        steam: String::new(),
        strike_count: 0
    };

    // ======================== SAVE USER ===================================
    let code: StatusCode = file_manager::save_user(&user);

    let result: (StatusCode, Json<UserModel>) = if code == StatusCode::OK{
        (StatusCode::OK, Json(user))
    }else{
        (StatusCode::IM_A_TEAPOT, Json(UserModel::new()))
    };
    // ======================== END SAVE USER ===================================

    result
    */
}