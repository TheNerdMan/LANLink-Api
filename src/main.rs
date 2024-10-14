use axum::Router;
use listenfd::ListenFd;
use tokio::net::TcpListener;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

mod features;
mod schema;
mod core;

use crate::features::greeting::greeting_controller;
use crate::features::equipment::controllers::equipment_controller;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() {
    // create db pools
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    // run the migrations on server startup
    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    let app = Router::new()
        .merge(greeting_controller::router())
        .merge(equipment_controller::router())
        .with_state(pool);


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