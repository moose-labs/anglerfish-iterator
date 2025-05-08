use std::str::FromStr;

use sui_sdk::types::{TypeTag, type_input::TypeInput};

pub trait ToTypeInputs {
    fn to_type_inputs(&self) -> Vec<TypeInput>;
}

impl ToTypeInputs for Vec<String> {
    fn to_type_inputs(&self) -> Vec<TypeInput> {
        self.iter()
            .map(|type_tag| TypeInput::from(TypeTag::from_str(type_tag).unwrap()))
            .collect()
    }
}
