use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{post};
use deadpool_diesel::postgres::Pool;
use crate::core::crypto::crypto_manager::generate_hash;
use crate::core::errors::error::{AppError, AppErrorEnum};
use crate::core::errors::error_handler::throw_error;
use crate::core::permissions::permission_manager::PermissionsManager;
use crate::features::auth::models::auth_user_model::AuthUserModel;
use crate::features::auth::repos::auth_repo::create_or_update_auth_user;
use crate::features::user::models::user_model::UserModel;
use crate::features::user::repos::user_repo;
use crate::features::user::services::user_service;

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/sign_up/username", post(username_sign_up))

}

#[derive(serde::Deserialize)]
struct UserSignUpDto {
    username: String,
    password: String,
}

#[axum::debug_handler]
async fn username_sign_up(
    State(_pool): State<Pool>,
    Json(_user_name_sign_up): Json<UserSignUpDto>,
) -> impl IntoResponse {
    let mut user_model = UserModel::new();
    user_model.username = _user_name_sign_up.username.clone();

    // check if user exists
    let user_exists = user_service::get_by_username(&_pool,  &_user_name_sign_up.username).await;

    if user_exists.is_ok() {
        throw_error(AppError::new(AppErrorEnum::BadRequestError, String::from("User already exists")));
        return StatusCode::BAD_REQUEST
    }

    let model = user_repo::create_or_update_user(&_pool, user_model).await;

    match model {
        None => {
            throw_error(AppError::new(AppErrorEnum::InternalServerError,String::from("Could not create user")));
            return StatusCode::INTERNAL_SERVER_ERROR
        },
        _ => {}
    }

    let password_hash_result = generate_hash(& _user_name_sign_up.password).await;

    match password_hash_result {
        Err(_) => {
            throw_error(AppError::new(AppErrorEnum::InternalServerError, String::from("Could not generate hash")));
            return StatusCode::INTERNAL_SERVER_ERROR
        },
        _ => {}
    }

    let new_auth_model = AuthUserModel {
        id: 0,
        user_id: model.unwrap().id,
        username: _user_name_sign_up.username.clone(),
        password_hash: password_hash_result.unwrap(),
        permissions_bitwise: PermissionsManager::get_default_permissions_bitwise(),
        created_at: Default::default(),
        updated_at: Default::default(),
    };
    let auth_model = create_or_update_auth_user(&_pool, new_auth_model).await;

    match auth_model {
        None => {
            throw_error(AppError::new(AppErrorEnum::InternalServerError, String::from("Could not create auth user")));
            StatusCode::INTERNAL_SERVER_ERROR
        },
        Some(_) => StatusCode::OK
    }
}