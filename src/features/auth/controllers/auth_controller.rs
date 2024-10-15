use axum::{Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{post};
use chrono::Utc;
use deadpool_diesel::postgres::Pool;
use jsonwebtoken::{encode, Header};
use serde::Deserialize;
use uuid::Uuid;
use crate::core::crypto::crypto_manager;
use crate::features::auth::dtos::auth_body_dto::AuthBodyDto;
use crate::core::errors::auth_errors::AuthError;
use crate::core::errors::error::AppError;
use crate::core::errors::error_handler::throw_error;
use crate::features::auth::key_creation_and_retrieval::claims::Claims;
use crate::features::auth::key_creation_and_retrieval::keys::KEYS;
use crate::features::auth::repos::auth_repo;

#[derive(Debug, Deserialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
}

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/login", post(authorize))
}

async fn authorize(State(_pool): State<Pool>, Json(payload): Json<AuthPayload>) -> Result<Json<AuthBodyDto>, AuthError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    
    let maybe_auth_user = auth_repo::get_auth_user_by_username(&_pool, &payload.client_id).await;
    
    if maybe_auth_user.is_none() {
        return Err(AuthError::WrongCredentials);
    }

    let submitted_password_hash = crypto_manager::generate_hash(&payload.client_secret).await;

    match submitted_password_hash {
        Err(_) => {
            throw_error(AppError::InternalServerError("Could not generate hash".parse().unwrap()));
            return Err(AuthError::WrongCredentials)
        }, 
        _ => {}
    }
    
    if &submitted_password_hash.unwrap() != &maybe_auth_user.unwrap().password_hash {
        return Err(AuthError::WrongCredentials);
    }

    // create the timestamp for the expiry time - here the expiry time is 1 day
    // TODO: in production you may not want to have such a long JWT life
    let exp = (Utc::now().naive_utc() + chrono::naive::Days::new(1)).timestamp() as usize;
    let claims = Claims {
        user_public_id: Uuid::new_v4(),
        username: payload.client_id,
        exp,
    };
    
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(
        |_| AuthError::TokenCreation
    )?;

    // Send the authorized token
    Ok(Json(AuthBodyDto::new(token)))
}