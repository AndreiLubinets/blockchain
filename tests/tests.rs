use alloy::primitives::U256;
use blockchain::domain::block::Block;

mod handlers;
mod services;

fn generate_blocks() -> Vec<Block> {
    let first_block = Block::new(
        "0x6e9Bdd4A0f5847ea7d490E88d7c764A47a960f76".to_owned(),
        "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984".to_owned(),
        U256::from(1909868389000000_u64),
    );
    let second_block = Block::new(
        "0x6e9Bdd4A0f5847ea7d490E88d7c764A47a960f76".to_owned(),
        "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984".to_owned(),
        U256::from(23098683890000000_u64),
    );
    vec![first_block, second_block]
}
