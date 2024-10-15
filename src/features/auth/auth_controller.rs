use axum::{Json, Router};
use axum::routing::{post};
use chrono::Utc;
use deadpool_diesel::postgres::Pool;
use jsonwebtoken::{encode, Header};
use serde::Deserialize;
use uuid::Uuid;
use crate::features::auth::auth_body::AuthBody;
use crate::features::auth::auth_errors::AuthError;
use crate::features::auth::claims::Claims;
use crate::features::auth::keys::KEYS;

#[derive(Debug, Deserialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
}

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/login", post(authorize))
}

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    
    // TODO: Here, basic verification is used but normally you would use a database
    if &payload.client_id != "foo" || &payload.client_secret != "bar" {
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
    Ok(Json(AuthBody::new(token)))
}