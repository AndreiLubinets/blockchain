use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::util;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Block {
    id: u32,
    from: String,
    to: String,
    value: String,
    hash: Vec<u8>,
}

impl Block {
    pub fn new(id: u32, from: String, to: String, value: String) -> Self {
        let hash = Block::calculate_hash(id, &from, &to, &value);
        Self {
            id,
            from,
            to,
            value,
            hash,
        }
    }

    fn calculate_hash(id: u32, from: &str, to: &str, value: &str) -> Vec<u8> {
        let input = format!("{}{}{}{}", id, from, to, value);
        util::sha256_double(input)
    }
}

//pub struct Sha256Hash([u8; 32]);
