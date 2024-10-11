use axum::Router;
use listenfd::ListenFd;
use tokio::net::TcpListener;

mod objects;
mod controllers;
use crate::controllers::greeting_controller;
use crate::controllers::user_controller;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(greeting_controller::router())
        .merge(user_controller::router());

    // create listener
    let listener = create_listener().await;

    // run it
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn create_listener() -> TcpListener {
    let mut listenfd = ListenFd::from_env();
    match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    }
}