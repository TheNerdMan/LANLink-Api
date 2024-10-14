use axum::{response::Html, routing::get, Router};
use deadpool_diesel::postgres::Pool;

pub fn router() -> Router<Pool>   {
    Router::new()
        .route("/api/v1/greeting/hello", get(hello))
        .route("/api/v1/greeting/good-bye", get(good_bye))
}

async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World! MOTHER FUCKKERRRR</h1>")
}

async fn good_bye() -> Html<&'static str> {
    Html("<h1>👋 Good bye!</h1>")
}