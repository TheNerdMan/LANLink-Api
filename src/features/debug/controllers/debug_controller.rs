use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::Router;
use axum::routing::get;
use deadpool_diesel::postgres::Pool;

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/api/v1/debug", get(debug))
        .route("/", get(|| async { Redirect::temporary("http://localhost:5000/") }))
}

async fn debug(State(_pool): State<Pool>) -> impl IntoResponse {
    // return 200 with info about the machine and application
    let mut output = String::new();
    output.push_str("Debug info:\n");

    // std::env:consts
    output.push_str(&format!("OS: {}\n", std::env::consts::OS));
    output.push_str(&format!("ARCH: {}\n", std::env::consts::ARCH));
    output.push_str(&format!("FAMILY: {}\n", std::env::consts::FAMILY));

    // server time
    output.push_str(&format!("Server time: {}\n", chrono::Utc::now().to_rfc3339()));


    (StatusCode::OK, output).into_response()
}
