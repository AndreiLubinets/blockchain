use core::str;

use crate::{config::DatabaseConnection, domain::block::Block, util::decode_url};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use log::error;
use reqwest::{Client, Url};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    start_hash: Option<String>,
}

pub async fn blocks(
    DatabaseConnection(mut conn): DatabaseConnection,
    Query(params): Query<Params>,
) -> Result<Json<Vec<Block>>, StatusCode> {
    match params.start_hash {
        Some(hex_hash) => {
            let start_hash = hex::decode(hex_hash)
                .inspect_err(|err| error!("{}", err))
                .map_err(|_| StatusCode::BAD_REQUEST)?;
            sqlx::query_as(
                r#"
            select * from blocks 
            where id >= (select id from blocks where hash = ?1)
            "#,
            )
            .bind(start_hash)
            .fetch_all(&mut *conn)
        }
        None => sqlx::query_as("select * from blocks").fetch_all(&mut *conn),
    }
    .await
    .inspect_err(|err| error!("{}", err))
    .map(|response| Json(response))
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn blocks_remote(
    Query(params): Query<Params>,
    Path(address): Path<String>,
) -> Result<Json<Vec<Block>>, StatusCode> {
    let address_decoded = decode_url(&address)
        .inspect_err(|err| error!("{}", err))
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut request = Client::new().get(address_decoded);

    if let Some(hex_hash) = params.start_hash {
        request = request.form(&[("start_hash", &hex_hash)]);
    };

    //TODO: Look into error handling
    request
        .send()
        .await
        .inspect_err(|err| error!("{}", err))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .json::<Vec<Block>>()
        .await
        .map(|response| Json(response))
        .inspect_err(|err| error!("{}", err))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
