use alloy::hex::encode;
use axum::{routing::get, Json, Router};
use axum_test::TestServer;
use reqwest::StatusCode;

use blockchain::{dto::block::BlockDto, handlers::blocks};

use blockchain::handlers::blocks_remote;
use sqlx::{QueryBuilder, SqliteConnection, SqlitePool};

use crate::generate_blocks;

fn generate_blockdtos() -> Vec<BlockDto> {
    let first_block = BlockDto {
        from: "0x6e9Bdd4A0f5847ea7d490E88d7c764A47a960f76".to_owned(),
        to: "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984".to_owned(),
        value: "1909868389000000".to_owned(),
    };
    let second_block = BlockDto {
        from: "0x6e9Bdd4A0f5847ea7d490E88d7c764A47a960f76".to_owned(),
        to: "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984".to_owned(),
        value: "23098683890000000".to_owned(),
    };
    vec![first_block, second_block]
}

async fn blocks_mock() -> Json<Vec<BlockDto>> {
    Json(generate_blockdtos())
}

async fn init_test_db(connection: &mut SqliteConnection) -> Result<(), anyhow::Error> {
    let _ = QueryBuilder::new("insert into blocks('from', 'to', value, hash)")
        .push_values(generate_blocks(), |mut binds, block| {
            binds
                .push_bind(block.from().to_owned())
                .push_bind(block.to().to_owned())
                .push_bind(block.value().to_owned())
                .push_bind(block.hash().to_owned());
        })
        .build()
        .execute(&mut *connection)
        .await?;
    Ok(())
}

#[tokio::test]
async fn blocks_remote_test() {
    let local_app = Router::new().route("/blocks/remote/:address", get(blocks_remote));
    let local_server = TestServer::new(local_app).unwrap();

    let remote_app = Router::new().route("/blocks", get(blocks_mock));
    let remote_server = TestServer::builder()
        .http_transport()
        .build(remote_app)
        .unwrap();

    let mut remote_address = remote_server.server_address().unwrap();
    remote_address.set_path("blocks");
    let encoded_address = encode(remote_address.to_string());

    let response = local_server
        .get(&format!("/blocks/remote/{}", encoded_address))
        .await;

    response.assert_json(&generate_blockdtos());
}

#[tokio::test]
async fn blocks_remote_test_invalid_url() {
    let local_app = Router::new().route("/blocks/remote/:address", get(blocks_remote));
    let local_server = TestServer::new(local_app).unwrap();

    let invalid_encoded_url = "invalid_encoded_url";

    let response = local_server
        .get(&format!("/blocks/remote/{}", invalid_encoded_url))
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn blocks_remote_test_with_start_hash() {
    let local_app = Router::new().route("/blocks/remote/:address", get(blocks_remote));
    let local_server = TestServer::new(local_app).unwrap();
    let start_hash = "c5ac3f42770a115c3bb274f6428b382b58656f2770468c0453b6494277f820d1";

    let remote_app = Router::new().route("/blocks", get(blocks_mock));
    let remote_server = TestServer::builder()
        .http_transport()
        .build(remote_app)
        .unwrap();
    let mut remote_address = remote_server.server_address().unwrap();
    remote_address.set_path("blocks");
    let encoded_address = encode(remote_address.to_string());

    let response = local_server
        .get(&format!(
            "/blocks/remote/{}?start_hash={}",
            encoded_address, start_hash
        ))
        .await;

    response.assert_json(&generate_blockdtos());
}

#[sqlx::test]
async fn blocks_test(pool: SqlitePool) {
    let mut connection = pool.acquire().await.unwrap();
    let app = Router::new().route("/blocks", get(blocks)).with_state(pool);
    let server = TestServer::new(app).unwrap();
    init_test_db(&mut connection).await.unwrap();

    let response = server.get("/blocks").await;

    dbg!(response.json::<Vec<BlockDto>>());

    response.assert_json(&generate_blockdtos());
}

#[sqlx::test]
async fn blocks_test_with_start_hash(pool: SqlitePool) {
    let mut connection = pool.acquire().await.unwrap();
    let app = Router::new().route("/blocks", get(blocks)).with_state(pool);
    let server = TestServer::new(app).unwrap();
    let start_hash = alloy::hex::encode(generate_blocks()[1].hash());
    let expected = vec![generate_blockdtos()[1].clone()];
    init_test_db(&mut connection).await.unwrap();

    let response = server
        .get(&format!("/blocks?start_hash={}", start_hash))
        .await;

    response.assert_json(&expected);
}

#[sqlx::test]
async fn blocks_test_with_invalid_hash(pool: SqlitePool) {
    let app = Router::new().route("/blocks", get(blocks)).with_state(pool);
    let server = TestServer::new(app).unwrap();
    let start_hash = "invalid_hash";

    let response = server
        .get(&format!("/blocks?start_hash={}", start_hash))
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);
}
