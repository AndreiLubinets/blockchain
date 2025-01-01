use alloy::primitives::U256;
use sqlx::FromRow;

use crate::{services::eth::ERC20::Transfer, util};

use super::sqlx_types::Uint256;

#[derive(FromRow, Debug)]
pub struct Block {
    #[allow(dead_code)]
    id: Option<u32>,
    from: String,
    to: String,
    value: Uint256,
    hash: Vec<u8>,
}

impl Block {
    pub fn from(&self) -> &str {
        &self.from
    }

    pub fn to(&self) -> &str {
        &self.to
    }

    pub fn value(&self) -> &Uint256 {
        &self.value
    }

    pub fn hash(&self) -> &[u8] {
        &self.hash
    }
}

impl Block {
    pub fn new(from: String, to: String, value: U256) -> Self {
        let hash = Block::calculate_hash(&from, &to, &value);
        Self {
            id: None,
            from,
            to,
            value: Uint256(value),
            hash,
        }
    }

    fn calculate_hash(from: &str, to: &str, value: &U256) -> Vec<u8> {
        let input = format!("{}{}{}", from, to, value);
        util::sha256_double(input)
    }
}

impl From<Transfer> for Block {
    fn from(value: Transfer) -> Self {
        Block::new(
            value.from.to_string(),
            value.to.to_string(),
            value.value.to(),
        )
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from
            && self.to == other.to
            && self.value == other.value
            && self.hash == other.hash
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::U256;

    use super::Block;

    #[test]
    fn block_test_same_blocks() {
        let from = "0x6e9Bdd4A0f5847ea7d490E88d7c764A47a960f76";
        let to = "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984";
        let value: U256 = U256::from(1909868389000000_u64);

        let first_block = Block::new(from.to_string(), to.to_string(), value);
        let second_block = Block::new(from.to_string(), to.to_string(), value);

        assert_eq!(first_block.hash, second_block.hash);
    }

    #[test]
    fn block_test_different_blocks() {
        let from = "0x6e9Bdd4A0f5847ea7d490E88d7c764A47a960f76";
        let to = "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984";
        let value = U256::from(190986838900000_u64);
        let second_value = U256::from(237325974862_u64);

        let first_block = Block::new(to.to_string(), from.to_string(), value);
        let second_block = Block::new(from.to_string(), to.to_string(), second_value);

        assert_ne!(first_block.hash, second_block.hash);
    }
}
