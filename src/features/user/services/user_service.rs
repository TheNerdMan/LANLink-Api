use deadpool_diesel::postgres::Pool;
use uuid::Uuid;
use crate::core::errors::error::{AppError, AppErrorEnum};
use crate::features::user::dtos::user_dto::UserDto;
use crate::features::user::repos::user_repo::{get_user_by_discord, get_user_by_public_id, get_user_by_steam, get_user_by_username};

pub async fn get_by_public_id(_pool: Pool, public_id: Uuid) -> Result<UserDto, AppError> {
    let option = get_user_by_public_id(&_pool, public_id).await;
    if option.is_some(){
        Ok(UserDto::from_model(&option.unwrap()))
    }else {
        Err(AppError::new(AppErrorEnum::NotFoundError,String::from("User not found")))
    }
}

pub async fn get_by_username(_pool: Pool, username: String) -> Result<UserDto, AppError> {
    let option = get_user_by_username(&_pool, username).await;
    if option.is_some(){
        Ok(UserDto::from_model(&option.unwrap()))
    }else {
        Err(AppError::new(AppErrorEnum::NotFoundError,String::from("User not found")))
    }
}

pub async fn get_by_discord_username(_pool: Pool, discord_username: String) -> Result<UserDto, AppError> {
    let option = get_user_by_discord(&_pool, discord_username).await;
    if option.is_some(){
        Ok(UserDto::from_model(&option.unwrap()))
    }else {
        Err(AppError::new(AppErrorEnum::NotFoundError,String::from("User not found")))
    }
}

pub async fn get_by_steam_url(_pool: Pool, steam_url: String) -> Result<UserDto, AppError> {
    let option = get_user_by_steam(&_pool, steam_url).await;
    if option.is_some(){
        Ok(UserDto::from_model(&option.unwrap()))
    }else {
        Err(AppError::new(AppErrorEnum::NotFoundError,String::from("User not found")))
    }
}