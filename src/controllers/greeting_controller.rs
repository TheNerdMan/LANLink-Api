use axum::{response::Html, routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/greeting/hello", get(hello))
        .route("/api/v1/greeting/good-bye", get(good_bye))
}

async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World! MOTHER FUCKKERRRR</h1><img src='https://placecats.com/4000/4000' />")
}

async fn good_bye() -> Html<&'static str> {
    Html("<h1>👋 Good bye!</h1>")
}