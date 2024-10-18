use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::post;
use deadpool_diesel::postgres::Pool;
use crate::features::auth::key_creation_and_retrieval::claims::Claims;
use crate::features::user::repos::user_repo;
use crate::core::permissions::permission_manager::{FeaturePermissions, PermissionsManager};
use crate::core::permissions::permission_constants::admin_permissions::*;
use crate::core::permissions::permission_constants::user_permissions::*;
use crate::core::permissions::permission_constants::equipment_permissions::*;
use crate::features::user::models::user_model::UserModel;

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/protected", post(protected))
        .route("/api/v1/protected/test", post(test))
}

async fn test(State(_pool): State<Pool>, permissions: PermissionsManager) -> impl IntoResponse {

    let mut perm = permissions;

    perm.admin_permissions.set_permission(ADMIN_READ_PERMISSION);
    perm.user_permissions.set_permission(USER_FULL_ACCESS);
    perm.equipment_permissions.set_permission(USER_FULL_ACCESS);

    let mut output = String::new();

    if perm.user_permissions.has_permission(USER_READ_PERMISSION){
        output.push_str("You have user read permission\n");
    }else{
        output.push_str("You do not have user read permission\n");
    }
    if perm.user_permissions.has_permission(USER_WRITE_PERMISSION){
        output.push_str("You have user write permission\n");
    }else{
        output.push_str("You do not have user read permission\n");
    }
    if perm.user_permissions.has_permission(USER_EXECUTE_PERMISSION){
        output.push_str("You have user execute permission\n");
    }else{
        output.push_str("You do not have user read permission\n");
    }
    if perm.user_permissions.has_permission(USER_DELETE_PERMISSION){
        output.push_str("You have user delete permission\n");
    }else{
        output.push_str("You do not have user read permission\n");
    }

    output.push_str("\n");

    if perm.admin_permissions.has_permission(ADMIN_READ_PERMISSION){
        output.push_str("You have admin read permission\n");
    }else{
        output.push_str("You do not have admin read permission\n");
    }
    if perm.admin_permissions.has_permission(ADMIN_WRITE_PERMISSION){
        output.push_str("You have admin write permission\n");
    }else{
        output.push_str("You do not have admin read permission\n");
    }
    if perm.admin_permissions.has_permission(ADMIN_EXECUTE_PERMISSION){
        output.push_str("You have admin execute permission\n");
    }else{
        output.push_str("You do not have admin read permission\n");
    }
    if perm.admin_permissions.has_permission(ADMIN_DELETE_PERMISSION){
        output.push_str("You have admin delete permission\n");
    }else{
        output.push_str("You do not have admin read permission\n");
    }

    output.push_str("\n");

    if perm.equipment_permissions.has_permission(EQUIP_READ_PERMISSION){
        output.push_str("You have equip read permission\n");
    }else{
        output.push_str("You do not have equip read permission\n");
    }
    if perm.equipment_permissions.has_permission(EQUIP_WRITE_PERMISSION){
        output.push_str("You have equip write permission\n");
    }else{
        output.push_str("You do not have equip read permission\n");
    }
    if perm.equipment_permissions.has_permission(EQUIP_EXECUTE_PERMISSION){
        output.push_str("You have equip execute permission\n");
    }else{
        output.push_str("You do not have equip read permission\n");
    }
    if perm.equipment_permissions.has_permission(EQUIP_DELETE_PERMISSION){
        output.push_str("You have equip delete permission\n");
    }else{
        output.push_str("You do not have equip read permission\n");
    }
    output.push_str(format!("\nYour permission string is: {}", perm.to_permissions_bitwise()).as_str());

    (StatusCode::OK, output).into_response()
}

#[axum::debug_handler]
async fn protected(State(_pool): State<Pool>, permissions: PermissionsManager) -> impl IntoResponse {

    print!("Permissions: {:?}", permissions);
    let full_user = user_repo::get_user_by_public_id(&_pool, permissions.claims.user_public_id).await;
    match full_user {
        Some(user) => {
            // Send the protected data to the user
            let str = protected_success(&permissions, &user);
            return (StatusCode::OK, str).into_response();
        }
        None => {
            return (StatusCode::NO_CONTENT, "User not found").into_response();
        }
    }
}

fn protected_success(permissions: &PermissionsManager, user: &UserModel) -> String {
    let mut output_str = format!("Welcome to the protected area {}, \n Your name is: {} \n Your discord is: {} \n Your steam is: {}",
                                 user.username,
                                 format!("{} {}", user.first_name, user.last_name),
                                 user.discord_username,
                                 user.steam_url
    );

    output_str.push_str("\n");

    output_str.push_str(format!("Your permission string is: {}", permissions.to_permissions_bitwise()).as_str());

    output_str
}