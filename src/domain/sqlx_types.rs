use std::ops::Deref;

use alloy::primitives::U256;
use sqlx::{Decode, Encode, Sqlite, Type};

#[derive(Clone)]
pub struct Uint256(pub U256);

impl Deref for Uint256 {
    type Target = U256;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'r> Encode<'r, Sqlite> for Uint256 {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'r>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let bytes: [u8; 32] = self.to_be_bytes();
        <Vec<u8> as Encode<Sqlite>>::encode(bytes.to_vec(), buf)
    }
}

impl<'r> Decode<'r, Sqlite> for Uint256 {
    fn decode(
        value: <Sqlite as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes: Vec<u8> = <Vec<u8> as Decode<Sqlite>>::decode(value)?;
        let be_bytes: [u8; 32] = bytes.try_into().unwrap();
        Ok(Uint256(U256::from_be_bytes(be_bytes)))
    }
}

impl Type<Sqlite> for Uint256 {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <Vec<u8> as Type<Sqlite>>::type_info()
    }
}
