use axum::{response::Html, routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/equipment/hire", get(hire()))
}

async fn hire() -> Html<&'static str> {
    Html("<h1>You hired equipment</h1>")
}