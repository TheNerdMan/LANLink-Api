use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::post;
use deadpool_diesel::postgres::Pool;
use crate::features::auth::key_creation_and_retrieval::claims::Claims;

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/protected", post(protected))
}

#[axum::debug_handler]
async fn protected(State(_pool): State<Pool>, claims: Claims) -> impl IntoResponse {
    if claims.username.is_empty() {
        return (StatusCode::UNAUTHORIZED, "no").into_response();
    }
    // Send the protected data to the user
    (StatusCode::OK, format!("Welcome to the protected area, {}!", claims.username)).into_response()
}
