use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{services::eth::ERC20::Transfer, util};

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Block {
    pub id: Option<u32>,
    pub from: String,
    pub to: String,
    pub value: String,
    pub hash: String,
}

impl Block {
    pub fn new(from: String, to: String, value: String) -> Self {
        let hash = hex::encode(Block::calculate_hash(&from, &to, &value));
        Self {
            id: None,
            from,
            to,
            value,
            hash,
        }
    }

    fn calculate_hash(from: &str, to: &str, value: &str) -> Vec<u8> {
        let input = format!("{}{}{}", from, to, value);
        util::sha256_double(input)
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
