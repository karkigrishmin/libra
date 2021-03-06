// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! This module defines physical storage schema for the transaction accumulator.
//!
//! A hash value is stored on each position.
//! See `storage/accumulator/lib.rs` for details.
//! ```text
//! |<---key--->|<-value->|
//! | position  |  hash   |
//! ```

use crate::schema::{ensure_slice_len_eq, TRANSACTION_ACCUMULATOR_CF_NAME};
use byteorder::{BigEndian, ReadBytesExt};
use crypto::HashValue;
use failure::prelude::*;
use schemadb::{
    define_schema,
    schema::{KeyCodec, ValueCodec},
};
use std::mem::size_of;
use types::proof::position::Position;

define_schema!(
    TransactionAccumulatorSchema,
    Position,
    HashValue,
    TRANSACTION_ACCUMULATOR_CF_NAME
);

impl KeyCodec<TransactionAccumulatorSchema> for Position {
    fn encode_key(&self) -> Result<Vec<u8>> {
        Ok(self.to_inorder_index().to_be_bytes().to_vec())
    }

    fn decode_key(data: &[u8]) -> Result<Self> {
        ensure_slice_len_eq(data, size_of::<u64>())?;
        Ok(Position::from_inorder_index(
            (&data[..]).read_u64::<BigEndian>()?,
        ))
    }
}

impl ValueCodec<TransactionAccumulatorSchema> for HashValue {
    fn encode_value(&self) -> Result<Vec<u8>> {
        Ok(self.to_vec())
    }

    fn decode_value(data: &[u8]) -> Result<Self> {
        Self::from_slice(data)
    }
}

#[cfg(test)]
mod test;
