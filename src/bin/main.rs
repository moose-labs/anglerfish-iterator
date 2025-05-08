use std::path::PathBuf;
use std::time::SystemTime;

use anglerfish_iterator_rs::config::load_config;
use anglerfish_iterator_rs::helper::duration::duration_ms_to_minutes;
use anglerfish_iterator_rs::protocols::anglerfish::client::AnglerfishClient;
use anyhow::Result;

use anglerfish_iterator_rs::helper::wallet::retrieve_wallet;
use anglerfish_iterator_rs::protocols::anglerfish::types::phase_info::Phase;

use sui_sdk::SuiClientBuilder;

use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    let wallet = retrieve_wallet()?;

    // Load the configuration file
    let config = load_config(PathBuf::from("anglerfish_iterator_config.toml"))?;

    // Instantiate the Anglerfish client
    let sui_client = SuiClientBuilder::default().build_testnet().await?;
    let anglerfish_client = AnglerfishClient::new(sui_client, wallet, config);

    loop {
        let phase_info = anglerfish_client.get_phase_info().await;
        if phase_info.is_err() {
            sleep(std::time::Duration::from_secs(30)).await;
            continue;
        }

        let phase_info = phase_info.unwrap();
        let start = SystemTime::now();
        let current_timestamp_ms = start.duration_since(SystemTime::UNIX_EPOCH)?.as_millis() as u64;

        let current_phase = phase_info.current_phase.clone();
        match current_phase {
            Phase::Uninitialized => {}
            Phase::LiquidityProviding => {
                let expected_end_at =
                    phase_info.current_phase_at + phase_info.durations.liquidity_providing_duration;
                if current_timestamp_ms > expected_end_at {
                    println!("Liquidity providing phase is over. Executing next entry...");
                    let res = anglerfish_client.execute_next_entry().await;
                    match res {
                        Err(e) => {
                            println!("Error executing next entry: {}", e);
                        }
                        Ok(digest) => {
                            println!("Next entry transaction digest: {}", digest);
                        }
                    }
                } else {
                    let remaining_time = expected_end_at - current_timestamp_ms;
                    println!(
                        "Liquidity providing: Remaining time: ~{} m",
                        duration_ms_to_minutes(remaining_time)
                    );
                }
            }
            Phase::Ticketing => {
                let expected_end_at =
                    phase_info.current_phase_at + phase_info.durations.ticketing_duration;
                if current_timestamp_ms > expected_end_at {
                    println!("Ticketing phase is over. Executing next entry...");
                    let res = anglerfish_client.execute_next_entry().await;
                    match res {
                        Err(e) => {
                            println!("Error executing next entry: {}", e);
                        }
                        Ok(digest) => {
                            println!("Next entry transaction digest: {}", digest);
                        }
                    }
                } else {
                    let remaining_time = expected_end_at - current_timestamp_ms;
                    println!(
                        "Ticketing: Remaining time: ~{} m",
                        duration_ms_to_minutes(remaining_time)
                    );
                }
            }
            Phase::Drawing => {
                println!("Drawing: execute draw");
                let res = anglerfish_client.execute_draw().await;
                match res {
                    Err(e) => {
                        println!("Error drawing: {}", e);
                    }
                    Ok(digest) => {
                        println!("Drawing transaction digest: {}", digest);
                    }
                }
            }
            Phase::Distributing => {
                println!("Distributing: Execute distribute");
                let res = anglerfish_client.execute_distribute().await;
                match res {
                    Err(e) => {
                        println!("Error distributing: {}", e);
                    }
                    Ok(digest) => {
                        println!("Distributing transaction digest: {}", digest);
                    }
                }
            }
            Phase::Settling => {
                println!("Settling: Iterate to next round");
                let res = anglerfish_client.execute_start_new_round().await;
                match res {
                    Err(e) => {
                        println!("Error starting new round: {}", e);
                    }
                    Ok(digest) => {
                        println!("Starting new round transaction digest: {}", digest);
                    }
                }
            }
        }

        sleep(std::time::Duration::from_secs(30)).await;
    }
}
