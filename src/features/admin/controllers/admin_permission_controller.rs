use axum::extract::State;
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::post;
use deadpool_diesel::postgres::Pool;
use uuid::Uuid;
use crate::core::errors::error::{AppError, AppErrorEnum};
use crate::core::permissions::permission_constants::admin_permissions::ADMIN_READ_WRITE;
use crate::core::permissions::permission_manager::PermissionsManager;
use crate::features::auth::repos::auth_repo;

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/admin/permissions", post(update_auth_user_permissions))
}

#[derive(serde::Deserialize)]
struct PermissionUpdateDto {
    user_public_id: Uuid,
    permissions_bitwise: String
}

#[axum::debug_handler]
async fn update_auth_user_permissions(
    State(_pool): State<Pool>, 
    permissions: PermissionsManager,
    Json(dto): Json<PermissionUpdateDto>
) -> impl IntoResponse {
    let new_permissions_option = PermissionsManager::from_permissions_bitwise(&dto.permissions_bitwise);
    
    if new_permissions_option.is_none() {
        return Err(AppError::new(AppErrorEnum::BadRequestError, String::from("Invalid permissions")))
    }
    
    let new_permissions = new_permissions_option.unwrap();
    
    let valid = update_auth_user_permissions_validation(&permissions, &dto.user_public_id, &new_permissions).await;
    
    if valid.is_err() {
        return Err(valid.err().unwrap())
    }
    
    let auth_user_option = auth_repo::get_auth_user_by_user_public_id(&_pool, &dto.user_public_id).await;
    
    if auth_user_option.is_none() {
        return Err(AppError::new(AppErrorEnum::UserNotFound, String::from("User not found")))
    }
    
    let mut auth_user = auth_user_option.unwrap();
    
    auth_user.permissions_bitwise = new_permissions.to_permissions_bitwise();
    
    auth_repo::create_or_update_auth_user(&_pool, auth_user).await;
    
    Ok("Permissions updated")
}

async fn update_auth_user_permissions_validation(
    permissions: &PermissionsManager,
    dto_user_public_id: &Uuid,
    new_permissions: &PermissionsManager
) -> Result<bool, AppError> {
    let is_admin = permissions.admin_permissions.has_permission(ADMIN_READ_WRITE);
    if(*dto_user_public_id != permissions.claims.user_public_id && !is_admin) {
        return Err(AppError::new(AppErrorEnum::BadRequestError, String::from("Non admins can't edit other users permissions")))
    }
    
    if (new_permissions.admin_permissions.any_permission() && !is_admin) {
        return Err(AppError::new(AppErrorEnum::BadRequestError, String::from("Non admins can't add admin permissions")))
    }
    
    Ok(true)
}