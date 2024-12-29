use std::str::FromStr;
use axum::Json;
use deadpool_diesel::postgres::Pool;
use uuid::Uuid;
use crate::core::errors::error::{AppError, AppErrorEnum};
use crate::features::user::dtos::user_dto::UserDto;
use crate::features::user::models::user_model::UserModel;
use crate::features::user::repos::user_repo::{get_user_by_discord, get_user_by_public_id, get_user_by_steam, get_user_by_username};


pub enum GetUserType {
    Username,
    PublicId,
    DiscordUsername,
    SteamUrl,
}

impl FromStr for GetUserType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "username" => Ok(GetUserType::Username),
            "public_id" => Ok(GetUserType::PublicId),
            "discord_username" => Ok(GetUserType::DiscordUsername),
            "steam_url" => Ok(GetUserType::SteamUrl),
            _ => Err(()),
        }
    }
}

pub async fn get_user_by_type(_pool: &Pool, get_type: &str, value: &String) -> Result<Json<UserDto>, AppError> {
    let user_type = GetUserType::from_str(get_type).map_err(|_| AppError::new(AppErrorEnum::BadRequestError, String::from("MissingField")))?;

    let result = match user_type {
        GetUserType::Username => get_by_username(_pool, value).await,
        GetUserType::PublicId => {
            let id = Uuid::parse_str(&value).map_err(|_| AppError::new(AppErrorEnum::BadRequestError, String::from("Invalid UUID")))?;
            get_by_public_id(_pool, &id).await
        },
        GetUserType::DiscordUsername => get_by_discord_username(_pool, value).await,
        GetUserType::SteamUrl => get_by_steam_url(_pool, value).await,
    };

    Ok(Json(result
        .map(|user| user)
        .unwrap_or_else(|_| UserDto::from_model(&UserModel::new()))))
}

pub async fn get_by_public_id(_pool: &Pool, public_id: &Uuid) -> Result<UserDto, AppError> {
    let option = get_user_by_public_id(_pool, public_id).await;
    if option.is_some(){
        Ok(UserDto::from_model(&option.unwrap()))
    }else {
        Err(AppError::new(AppErrorEnum::NotFoundError,String::from("User not found")))
    }
}

pub async fn get_by_username(_pool: &Pool, username: &String) -> Result<UserDto, AppError> {
    let option = get_user_by_username(_pool, username).await;
    if option.is_some(){
        Ok(UserDto::from_model(&option.unwrap()))
    }else {
        Err(AppError::new(AppErrorEnum::NotFoundError,String::from("User not found")))
    }
}

pub async fn get_by_discord_username(_pool: &Pool, discord_username: &String) -> Result<UserDto, AppError> {
    let option = get_user_by_discord(_pool, discord_username).await;
    if option.is_some(){
        Ok(UserDto::from_model(&option.unwrap()))
    }else {
        Err(AppError::new(AppErrorEnum::NotFoundError,String::from("User not found")))
    }
}

pub async fn get_by_steam_url(_pool: &Pool, steam_url: &String) -> Result<UserDto, AppError> {
    let option = get_user_by_steam(_pool, steam_url).await;
    if option.is_some(){
        Ok(UserDto::from_model(&option.unwrap()))
    }else {
        Err(AppError::new(AppErrorEnum::NotFoundError,String::from("User not found")))
    }
}