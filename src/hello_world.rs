use axum::response::Html;

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World! MOTHER FUCKKERRRR</h1>")
}