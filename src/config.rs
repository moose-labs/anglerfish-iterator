use std::{fs, path::Path};

use anyhow::Result;
use serde::Deserialize;

// Define a struct to hold your configuration data.  Use `serde` attributes
// to specify how to deserialize the TOML data into this struct.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub package_id: String,
    pub objects: Objects,
    pub pool: Pool,
    pub iterator: Iterator,
}

#[derive(Debug, Deserialize)]
pub struct Objects {
    pub phase_info_id: String,
    pub round_registry_id: String,
    pub pool_registry_id: String,
    pub prize_pool_id: String,
    pub lounge_registry_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Pool {
    pub coin_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Iterator {
    pub cap_id: String,
}

// Function to load the configuration from a file.  This function now handles
// more error scenarios, providing more informative error messages.
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config> {
    let path = path.as_ref();

    let config_string = fs::read_to_string(path)?;

    let config: Config = toml::from_str(&config_string)?;

    Ok(config)
}
