use crate::core::errors::error::{AppError, AppErrorEnum};
use crate::core::permissions::permission_constants::user_permissions::*;
use crate::core::permissions::permission_manager::PermissionsManager;
use crate::features::user::dtos::user_dto::UserDto;
use crate::features::user::dtos::edit_user_payload_dto::EditUserPayloadDto;
use crate::features::user::services::user_service::{get_by_discord_username, get_by_public_id, get_by_steam_url, get_by_username};
use crate::features::user::models::user_model::UserModel;
use crate::features::user::repos::user_repo;
use crate::features::user::repos::user_repo::{get_user_by_discord, get_user_by_public_id, get_user_by_steam, get_user_by_username};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{extract::Path, response::Json, routing::post, Router};
use deadpool_diesel::postgres::Pool;
use uuid::Uuid;

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/user/create/:username", post(create_user))
        .route("/api/v1/user/edit", post(edit_user))
        .route("/api/v1/user/get/:get_type/:value", post(get_user))
}

#[axum::debug_handler]
async fn create_user(
    State(_pool): State<Pool>, permissions: PermissionsManager, Path(username): Path<String>) -> impl IntoResponse {

    let permission = permissions.user_permissions.has_permission(USER_WRITE_PERMISSION);
    if !permission {
        return (StatusCode::UNAUTHORIZED, "InsufficientPermissions").into_response();
    }
    
    let mut user = get_user_by_username(&_pool, username.clone()).await;
    if user.is_some(){
        return (StatusCode::UNAUTHORIZED, "User already exists").into_response();
    }

    let mut user = UserModel::new();
    user.username = username.clone();

    let model = user_repo::create_or_update_user(&_pool, user).await;

    (StatusCode::OK, Json(model)).into_response()
}


async fn edit_user(State(_pool): State<Pool>, permissions: PermissionsManager, Json(payload): Json<EditUserPayloadDto>) -> Result<impl IntoResponse, AppError> {

    let permission = permissions.user_permissions.has_permission(USER_WRITE_PERMISSION);
    if !permission {
        return Err(AppError::new(AppErrorEnum::InsufficientPermissions, String::from("InsufficientPermissions")))
    }

    if payload.new_username.is_empty()
        || payload.new_first_name.is_empty()
        || payload.new_last_name.is_empty()
        || payload.new_discord_username.is_empty()
        || payload.new_steam_url.is_empty(){
        return Err(AppError::new(AppErrorEnum::BadRequestError, String::from("Missing Field")));
    }

    let mut user = get_user_by_public_id(&_pool, permissions.claims.user_public_id).await
        .ok_or(AppError::new(AppErrorEnum::BadRequestError, String::from("User not found")))?;

    let (username_check, discord_check, steam_check) = tokio::join!(
        get_user_by_username(&_pool, payload.new_username.clone()),
        get_user_by_discord(&_pool, payload.new_discord_username.clone()),
        get_user_by_steam(&_pool, payload.new_steam_url.clone())
    );

    for allowed_test in [username_check, discord_check, steam_check]{
        if let Some(test_user) = allowed_test{
            if test_user.publicid != user.publicid{
                return Err(AppError::new(AppErrorEnum::UserMismatch, String::from("User mismatch")));
            }
        }
    }

    user.username = payload.new_username;
    user.first_name = payload.new_first_name;
    user.last_name = payload.new_last_name;
    user.discord_username = payload.new_discord_username;
    user.steam_url = payload.new_steam_url;

    if user_repo::create_or_update_user(&_pool, user).await.is_none(){
        return Err(AppError::new(AppErrorEnum::InternalServerError, String::from("User update failed")));
    }

    Ok(Json("User updated").into_response())
}


async fn get_user(State(_pool): State<Pool>, permissions: PermissionsManager, Path((get_type, value)): Path<(String, String)>) -> Result<Json<UserDto>, AppError> {

    let permission_option = permissions.user_permissions.has_permission(USER_READ_PERMISSION);

    if !permission_option { // Yes im aware default includes the read permission
        return Err(AppError::new(AppErrorEnum::InsufficientPermissions, String::from("InsufficientPermissions")))
    }


    if get_type.is_empty() || value.is_empty() {
        return Err(AppError::new(AppErrorEnum::BadRequestError, String::from("MissingField")));
    }

    if get_type == "username" {
        let result = get_by_username(_pool, value).await;
        if result.is_ok(){
            let user = result?;
            return Ok(Json(user));
        }else{
            return Ok(Json(UserDto::from_model(&UserModel::new())))
        }
    }

    if get_type == "public_id" {
        let id_result = Uuid::parse_str(&value);
        if id_result.is_ok(){
            let result = get_by_public_id(_pool, id_result.unwrap()).await;
            if result.is_ok(){
                let user = result?;
                return Ok(Json(user));
            }else{
                return Ok(Json(UserDto::from_model(&UserModel::new())))
            }
        }

    }

    if get_type == "discord_username" {
        let result = get_by_discord_username(_pool, value).await;
        if result.is_ok(){
            let user = result?;
            return Ok(Json(user));
        }else{
            return Ok(Json(UserDto::from_model(&UserModel::new())))
        }
    }

    if get_type == "steam_url" {
        let result = get_by_steam_url(_pool, value).await;
        if result.is_ok(){
            let user = result?;
            return Ok(Json(user));
        }else{
            return Ok(Json(UserDto::from_model(&UserModel::new())))
        }
    }
    Ok(Json(UserDto::from_model(&UserModel::new())))
}