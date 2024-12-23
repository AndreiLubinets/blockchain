use core::str;

use crate::{config::database::DatabaseConnection, domain::block::Block, util::decode_url};
use axum::{
    extract::{Path, Query},
    Json,
};
use error::ApiError;
use reqwest::Client;
use serde::Deserialize;
use sqlx::QueryBuilder;

mod error;

#[derive(Debug, Deserialize)]
pub struct Params {
    start_hash: Option<String>,
}

pub async fn blocks(
    DatabaseConnection(mut conn): DatabaseConnection,
    Query(params): Query<Params>,
) -> Result<Json<Vec<Block>>, ApiError> {
    let mut builder = QueryBuilder::new("select * from blocks");

    if let Some(hash) = params.start_hash {
        builder
            .push(" where id >= (select id from blocks where hash = ")
            .push_bind(hash)
            .push(")");
    };

    let result = builder
        .build_query_as::<Block>()
        .fetch_all(&mut *conn)
        .await
        .map(Json)?;

    Ok(result)
}

pub async fn blocks_remote(
    Query(params): Query<Params>,
    Path(address): Path<String>,
) -> Result<Json<Vec<Block>>, ApiError> {
    let address_decoded =
        decode_url(&address).map_err(|err| ApiError::BadRequest(err.to_string()))?;
    let mut request = Client::new().get(address_decoded);

    if let Some(hex_hash) = params.start_hash {
        request = request.query(&[("start_hash", &hex_hash)]);
    };

    let response = request.send().await?.json::<Vec<Block>>().await.map(Json)?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use alloy::hex::encode;
    use axum::{routing::get, Json, Router};
    use axum_test::TestServer;
    use reqwest::StatusCode;

    use super::blocks_remote;
    use super::Block;

    fn generate_blocks() -> Vec<Block> {
        let first_block = Block::new(
            "0x6e9Bdd4A0f5847ea7d490E88d7c764A47a960f76".to_owned(),
            "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984".to_owned(),
            "190986838900000000000".to_owned(),
        );
        let second_block = Block::new(
            "0x6e9Bdd4A0f5847ea7d490E88d7c764A47a960f76".to_owned(),
            "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984".to_owned(),
            "190986838900000000000".to_owned(),
        );

        vec![first_block, second_block]
    }

    async fn blocks_mock() -> Json<Vec<Block>> {
        Json(generate_blocks())
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

        response.assert_json(&generate_blocks());
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

        response.assert_json(&generate_blocks());
    }
}
