use axum::{routing::get, Router};
use handlers::{blocks, blocks_remote};

mod config;
mod domain;
mod handlers;
mod util;

#[tokio::main]
async fn main() {
    env_logger::init();

    let db_pool = config::database_pool()
        .await
        .expect("Unable to connect to the database");

    let app = Router::new()
        .route("/blocks", get(blocks))
        .route("/blocks/remote/:address", get(blocks_remote))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
