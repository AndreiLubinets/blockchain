use core::str;

use crate::{config::DatabaseConnection, domain::block::Block, util::decode_url};
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

    if let Some(hex_hash) = params.start_hash {
        let start_hash: Vec<u8> =
            hex::decode(hex_hash).map_err(|err| ApiError::BadRequest(err.to_string()))?;
        builder
            .push("where id >= (select id from blocks where hash = ?1)")
            .push_bind(start_hash);
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
        request = request.form(&[("start_hash", &hex_hash)]);
    };

    let response = request.send().await?.json::<Vec<Block>>().await.map(Json)?;

    Ok(response)
}
