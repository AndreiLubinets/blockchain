use alloy::primitives::Address;
use sqlx::{QueryBuilder, SqlitePool};
use tracing::info;

use crate::domain::block::Block;

use super::eth::query_transfer_logs;

pub async fn save_eth_logs_as_blocks(
    pool: SqlitePool,
    contract_address: Address,
    starting_block: Option<u64>,
) -> anyhow::Result<()> {
    let blocks = query_transfer_logs(contract_address, starting_block).await?;
    save_blocks(pool, blocks).await?;

    Ok(())
}

pub async fn save_blocks(pool: SqlitePool, blocks: Vec<Block>) -> anyhow::Result<()> {
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
    let mut connection = pool.acquire().await?;
    query.execute(&mut *connection).await?;

    info!("Finished inserting");

    Ok(())
}
