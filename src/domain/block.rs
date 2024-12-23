use alloy::hex::ToHexExt;
use sqlx::FromRow;

use crate::{services::eth::ERC20::Transfer, util};

#[derive(FromRow)]
pub struct Block {
    #[allow(dead_code)]
    pub id: Option<u32>,
    pub from: String,
    pub to: String,
    pub value: String,
    pub hash: String,
}

impl Block {
    pub fn new(from: String, to: String, value: String) -> Self {
        let hash = Block::calculate_hash(&from, &to, &value);
        Self {
            id: None,
            from,
            to,
            value,
            hash,
        }
    }

    fn calculate_hash(from: &str, to: &str, value: &str) -> String {
        let input = format!("{}{}{}", from, to, value);
        util::sha256_double(input).encode_hex()
    }
}

impl From<Transfer> for Block {
    fn from(value: Transfer) -> Self {
        Block::new(
            value.to.to_string(),
            value.from.to_string(),
            value.value.to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Block;

    #[test]
    fn block_test_same_blocks() {
        let from = "0x6e9Bdd4A0f5847ea7d490E88d7c764A47a960f76";
        let to = "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984";
        let value = "190986838900000000000";

        let first_block = Block::new(from.to_string(), to.to_string(), value.to_string());
        let second_block = Block::new(from.to_string(), to.to_string(), value.to_string());

        assert_eq!(first_block.hash, second_block.hash);
    }

    #[test]
    fn block_test_different_blocks() {
        let from = "0x6e9Bdd4A0f5847ea7d490E88d7c764A47a960f76";
        let to = "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984";
        let value = "190986838900000000000";
        let second_value = "23732597486233378816";

        let first_block = Block::new(from.to_string(), to.to_string(), value.to_string());
        let second_block = Block::new(from.to_string(), to.to_string(), second_value.to_string());

        assert_ne!(first_block.hash, second_block.hash);
    }
}
