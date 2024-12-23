use alloy::{primitives::Address, providers::ProviderBuilder, sol};
use tracing::info;

use crate::domain::block::Block;

sol!(
    #[sol(rpc)]
    ERC20,
    "abi/ERC20.json"
);

pub async fn query_transfer_logs(
    contract_addr: Address,
    starting_block: Option<u64>,
) -> anyhow::Result<Vec<Block>> {
    let rpc_url = "https://rpc.payload.de".parse()?;
    info!(
        "Getting logs from {} for address: {}",
        &rpc_url, &contract_addr
    );
    let provider = ProviderBuilder::new().on_http(rpc_url);
    let contract = ERC20::new(contract_addr, provider);

    let mut query = contract.Transfer_filter();

    if let Some(block) = starting_block {
        info!("Starting from block: {}", block);
        query = query.from_block(block);
    };

    let logs = query.query().await?;

    info!("Finished getting logs. Count: {}", logs.len());

    let blocks = logs
        .into_iter()
        .map(|(transfer, _)| transfer.into())
        .collect();

    Ok(blocks)
}
