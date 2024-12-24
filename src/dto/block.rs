use serde::{Deserialize, Serialize};

use crate::domain::block::Block;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BlockDto {
    pub from: String,
    pub to: String,
    pub value: String,
}

impl From<Block> for BlockDto {
    fn from(value: Block) -> Self {
        BlockDto {
            from: value.from().to_owned(),
            to: value.to().to_owned(),
            value: value.value().to_string(),
        }
    }
}
