use crate::features::auth::repos::auth_repo::create_or_update_auth_user;
use axum::{extract::Path, response::Json, routing::post, Router};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use deadpool_diesel::postgres::Pool;
use uuid::Uuid;
use crate::core::crypto::crypto_manager::generate_hash;
use crate::core::errors::error::AppError;
use crate::core::errors::error_handler::throw_error;
use crate::features::auth::models::auth_user_model::AuthUserModel;
use crate::features::user::repos::user_repo::get_user_by_public_id;
use crate::features::user::models::user_model::UserModel;
use crate::features::user::repos::user_repo;
use crate::features::user::repos::user_repo::get_all_users;
use crate::features::user::dtos::user_dto::UserDto;

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/user", get(get_all_users_handler))
        .route("/api/v1/user/:public_id", get(get_user_handler))
        .route("/api/v1/user/create/:username", post(create_user))
}


#[axum::debug_handler]
async fn get_all_users_handler(State(_pool): State<Pool>) -> impl IntoResponse {
    let equipment_models = get_all_users(&_pool);

    match equipment_models.await {
        Some(vec) => {
            let dtos: Vec<UserDto> = vec.iter().map(|e| UserDto::from_model(e)).collect();
            (StatusCode::OK, Json(dtos)).into_response()
        },
        None => (StatusCode::NO_CONTENT).into_response(),
    }
}

#[axum::debug_handler]
async fn get_user_handler(
    State(_pool): State<Pool>, Path(public_id): Path<Uuid>) -> impl IntoResponse {
    let user_model = get_user_by_public_id(&_pool, public_id);

    match user_model.await {
        Some(item) => {
            let dtos = UserDto::from_model(&item);
            (StatusCode::OK, Json(dtos)).into_response()
        },
        None => (StatusCode::NO_CONTENT).into_response(),
    }
}


#[axum::debug_handler]
async fn create_user(
    State(_pool): State<Pool>, Path(username): Path<String>) -> impl IntoResponse {
    let mut user_model = UserModel::new();
    user_model.username = username.clone();

    let model = user_repo::create_or_update_user(&_pool, user_model).await;

    (StatusCode::OK, Json(model)).into_response()
}