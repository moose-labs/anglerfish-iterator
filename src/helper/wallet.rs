use core::panic;

use sui_config::Config;
use sui_config::{PersistedConfig, SUI_CLIENT_CONFIG, SUI_KEYSTORE_FILENAME, sui_config_dir};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{sui_client_config::SuiClientConfig, wallet_context::WalletContext};

pub fn retrieve_wallet() -> Result<WalletContext, anyhow::Error> {
    let wallet_conf = sui_config_dir()?.join(SUI_CLIENT_CONFIG);
    let keystore_path = sui_config_dir()?.join(SUI_KEYSTORE_FILENAME);

    println!("Wallet config path: {:?}", wallet_conf);
    println!("Keystore path: {:?}", keystore_path);

    // check if a wallet exists and if not, create a wallet and a sui client config
    if !keystore_path.exists() {
        panic!("No wallet found. Please create a new wallet using the command: `sui client new`");
    }

    if !wallet_conf.exists() {
        panic!(
            "No wallet config found. Please create a new wallet using the command: `sui client new`"
        );
    }

    let keystore = FileBasedKeystore::new(&keystore_path)?;
    let mut client_config: SuiClientConfig = PersistedConfig::read(&wallet_conf)?;

    let default_active_address = if let Some(address) = keystore.addresses().first() {
        *address
    } else {
        panic!("No address found in the keystore.");
    };

    client_config.active_address = Some(default_active_address);
    client_config.save(&wallet_conf)?;

    println!("Active address: {:?}", client_config.active_address);

    let wallet = WalletContext::new(&wallet_conf, Some(std::time::Duration::from_secs(60)), None)?;

    Ok(wallet)
}
