use axum::{extract::Path, response::Json, routing::post, Router};
use axum::extract::{State};
use axum::http::StatusCode;
use deadpool_diesel::postgres::Pool;
use uuid::Uuid;

use crate::core::errors::error::AppError;
use crate::features::game_server::dtos::game_server_dto::GameServerDto;
use crate::features::game_server::models::game_server_model::GameServerModel;
use crate::features::game_server::repos::game_server_repo;
use crate::features::game_server::repos::game_server_repo::{create_or_update_game_server, get_game_server_by_public_id};

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/game_server/register/:server_name/:game_type", post(register_server))
        .route("/api/v1/game_server/unregister/:server_public_id", post(unregister_server))
}
#[axum::debug_handler]
async fn register_server(
    State(_pool): State<Pool>,
    Json(payload): Json<GameServerDto>,
) -> Result<Json<String>, AppError>{
    if payload.game_server_title.is_empty() || payload.game_type.is_empty() {
        return Err(AppError::BadRequestError(String::from("server_name or game_type cannot be empty")));
    }

    let model = GameServerModel::new_from_dto(payload);

    let result: Option<GameServerModel> = create_or_update_game_server(&_pool, model).await;

    if result.is_some(){
        Ok(Json(result.unwrap().publicid.to_string()))
    }else{
        Err(AppError::DatabaseQueryError(String::from("We were unable to create your server record")))
    }
}

#[axum::debug_handler]
async fn unregister_server(State(_pool): State<Pool>, Path(server_public_id): Path<String>) -> Result<StatusCode, AppError>{
    if server_public_id.is_empty(){
        return Err(AppError::BadRequestError(String::from("public_id cannot be empty")));
    }

    let id = Uuid::parse_str(server_public_id.as_str());
    if id.is_err(){
        return Err(AppError::BadRequestError(String::from("invalid public_id")));
    }

    let id = id.unwrap();
    let model: Option<GameServerModel> = get_game_server_by_public_id(&_pool, id).await;

    if model.is_none(){
        return Err(AppError::DatabaseQueryError(String::from("We were unable to find your server record")));
    }

    let result = game_server_repo::delete_game_server(&_pool, model.unwrap().id).await;
    if result.is_err(){
        return Err(AppError::DatabaseQueryError(String::from("We were unable to delete your server record")));
    }
    if result? > 0{
        Ok(StatusCode::OK)
    }else{
        Ok(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

