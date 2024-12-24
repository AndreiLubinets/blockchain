use core::str;

use crate::{
    config::database::DatabaseConnection, domain::block::Block, dto::block::BlockDto,
    util::decode_url,
};
use alloy::hex::decode;
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
    pub start_hash: Option<String>,
}

pub async fn blocks(
    DatabaseConnection(mut conn): DatabaseConnection,
    Query(params): Query<Params>,
) -> Result<Json<Vec<BlockDto>>, ApiError> {
    let mut builder = QueryBuilder::new("select * from blocks");

    if let Some(hash) = params.start_hash {
        let decoded_hash = decode(hash).map_err(|err| ApiError::BadRequest(err.to_string()))?;
        builder
            .push(" where id >= (select id from blocks where hash = ")
            .push_bind(decoded_hash)
            .push(")");
    };

    let result = builder
        .build_query_as::<Block>()
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .map(|block| block.into())
        .collect::<Vec<BlockDto>>();

    Ok(Json(result))
}

pub async fn blocks_remote(
    Query(params): Query<Params>,
    Path(address): Path<String>,
) -> Result<Json<Vec<BlockDto>>, ApiError> {
    let address_decoded =
        decode_url(&address).map_err(|err| ApiError::BadRequest(err.to_string()))?;
    let mut request = Client::new().get(address_decoded);

    if let Some(hex_hash) = params.start_hash {
        request = request.query(&[("start_hash", &hex_hash)]);
    };

    let response = request
        .send()
        .await?
        .json::<Vec<BlockDto>>()
        .await
        .map(Json)?;

    Ok(response)
}
