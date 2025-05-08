use std::str::FromStr;

use sui_sdk::types::base_types::ObjectID;

///

pub fn anglerfish_package_obj_id() -> ObjectID {
    ObjectID::from_str(&anglerfish_package_id()).unwrap()
}

pub fn anglerfish_phase_info_obj_id() -> ObjectID {
    ObjectID::from_str(&anglerfish_phase_info_id()).unwrap()
}

pub fn anglerfish_round_registry_obj_id() -> ObjectID {
    ObjectID::from_str(&anglerfish_round_registry_id()).unwrap()
}

pub fn anglerfish_pool_registry_obj_id() -> ObjectID {
    ObjectID::from_str(&anglerfish_pool_registry_id()).unwrap()
}

pub fn anglerfish_prize_pool_obj_id() -> ObjectID {
    ObjectID::from_str(&anglerfish_prize_pool_id()).unwrap()
}

pub fn anglerfish_lounge_registry_obj_id() -> ObjectID {
    ObjectID::from_str(&anglerfish_lounge_registry_id()).unwrap()
}

// ids

pub fn anglerfish_package_id() -> String {
    String::from("0x09c23dc75590103509b266b5f54fa38e73313b5ec9ddba480951efd9c70bec00")
}

pub fn anglerfish_phase_info_id() -> String {
    String::from("0x7b5e50c18c4fde1c342d6a2920e24d833e41ac1c497fa0ac06b6f2d46fc959ee")
}

pub fn anglerfish_round_registry_id() -> String {
    String::from("0x40601860c44486096f4a8704b07601fbfa9f4bbe3126a64fe192fec9ab29c8cd")
}

pub fn anglerfish_pool_registry_id() -> String {
    String::from("0x0e1e5527da787ffa1665e94e9eec5dbcd422aa5bcfd8c62f45cafd40c745bc98")
}

pub fn anglerfish_prize_pool_id() -> String {
    String::from("0x114580221e43cada6daba9c93b7370ea9ff0ddb83f9c7a8d6cd962add4212efd")
}

pub fn anglerfish_lounge_registry_id() -> String {
    String::from("0x7ed2df7a7ee900f4deec82211464babf2dc568185d332371bcabe63def09b7a5")
}
