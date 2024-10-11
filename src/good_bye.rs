use axum::response::Html;

pub async fn handler() -> Html<&'static str> {
    Html("<h1>👋 Good bye!</h1>")
}