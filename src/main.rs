use axum::{routing::get, Router};
use clap::Parser;
use config::{cli::Args, database::DatabaseConnection};
use handlers::{blocks, blocks_remote};
use services::block::save_eth_logs_as_blocks;
use tracing::{error, warn};

mod config;
mod domain;
mod handlers;
mod services;
mod util;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let db_pool = config::database::database_pool()
        .await
        .expect("Unable to connect to the database");
    let pool = db_pool.clone();

    let args = Args::parse();
    match args.contract_address {
        Some(addr) => {
            tokio::spawn(async move {
                if let Err(err) =
                    save_eth_logs_as_blocks(DatabaseConnection(pool.acquire().await.unwrap()), addr)
                        .await
                {
                    error!("Unable to save blocks: {}", err);
                };
            });
        }
        None => warn!("No contract address was provided. Transfer logs will not be downloaded"),
    };

    let app = Router::new()
        .route("/blocks", get(blocks))
        .route("/blocks/remote/:address", get(blocks_remote))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
