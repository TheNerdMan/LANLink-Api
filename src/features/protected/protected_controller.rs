use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::post;
use deadpool_diesel::postgres::Pool;
use crate::features::auth::key_creation_and_retrieval::claims::Claims;
use crate::features::user::repos::user_repo;

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/protected", post(protected))
}

#[axum::debug_handler]
async fn protected(State(_pool): State<Pool>, claims: Claims) -> impl IntoResponse {
    
    print!("Claims: {:?}", claims);
    let full_user = user_repo::get_user_by_public_id(&_pool, claims.user_public_id).await;
    match full_user {
        Some(user) => {
            // Send the protected data to the user
            (StatusCode::OK,
             format!("Welcome to the protected area, \n Your name is: {} \n Your discord is: {} \n Your steam is: {}",
                     format!("{} {}", user.first_name, user.last_name),
                     user.discord_username,
                     user.steam_url)
            ).into_response()
        }
        None => {
            return (StatusCode::NO_CONTENT, "User not found").into_response();
        }
    }
}
