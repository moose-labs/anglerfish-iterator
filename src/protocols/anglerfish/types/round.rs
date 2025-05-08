use std::str::FromStr;

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use sui_sdk::{
    rpc_types::{SuiObjectData, SuiParsedData},
    types::{
        base_types::{ObjectID, SuiAddress},
        id::UID,
    },
};

use super::FieldsExtractor;

/// Rust representation of `anglerfish::round::Round`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Purchase {
    pub address: SuiAddress,
    pub ticket_count: u64,
    pub start_index: u64,
}

/// Rust representation of `anglerfish::round::Round`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Round {
    pub id: UID,
}

impl FieldsExtractor for Round {
    type T = Round;

    fn try_from_sui_obj_data(data: SuiObjectData) -> Result<Round> {
        let content_data = data
            .content
            .ok_or_else(|| anyhow!("Fetched object but no data was returned"))?;

        let parse_move_obj = match content_data {
            SuiParsedData::MoveObject(parse_move_obj) => parse_move_obj,
            _ => return Err(anyhow!("Fetched object is not a Move Object")),
        };

        let id = parse_move_obj
            .fields
            .field_value("value")
            .ok_or(anyhow!("Failed to extract 'id' field from Round"))?;

        Ok(Round {
            id: UID::new(ObjectID::from_str(&id.to_string()).unwrap()),
        })
    }
}
