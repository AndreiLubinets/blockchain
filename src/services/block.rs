use alloy::primitives::Address;
use sqlx::QueryBuilder;
use tracing::info;

use crate::{config::database::DatabaseConnection, domain::block::Block};

use super::eth::query_transfer_logs;

pub async fn save_eth_logs_as_blocks(
    conn: DatabaseConnection,
    contract_address: Address,
) -> anyhow::Result<()> {
    let blocks = query_transfer_logs(contract_address).await?;
    save_blocks(conn, blocks).await?;

    Ok(())
}

pub async fn save_blocks(
    DatabaseConnection(mut conn): DatabaseConnection,
    blocks: Vec<Block>,
) -> anyhow::Result<()> {
    let len = blocks.len();
    if len == 0 {
        info!("No blocks to insert");
        return Ok(());
    }
    info!("Inserting {} blocks", len);

    let mut builder = QueryBuilder::new("insert into blocks('from', 'to', value, hash)");

    builder.push_values(blocks, |mut binds, block| {
        binds
            .push_bind(block.to)
            .push_bind(block.from)
            .push_bind(block.value)
            .push_bind(block.hash);
    });
    builder.push("on conflict(hash) do nothing");

    let query = builder.build();
    query.execute(&mut *conn).await?;

    info!("Finished inserting");

    Ok(())
}
