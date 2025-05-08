use anyhow::Result;
use sui_sdk::rpc_types::SuiObjectData;

pub mod phase_info;
pub mod round;
pub mod round_registry;

pub trait FieldsExtractor {
    type T;

    fn try_from_sui_obj_data(data: SuiObjectData) -> Result<Self::T>;
}
