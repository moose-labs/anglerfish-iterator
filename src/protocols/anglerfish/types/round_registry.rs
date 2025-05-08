use serde::{Deserialize, Serialize};
use sui_sdk::types::{collection_types::Table, id::UID};

/// Rust representation of `anglerfish::round::RoundRegistry`.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoundRegistry {
    pub id: UID,
    pub rounds: Table,
}
