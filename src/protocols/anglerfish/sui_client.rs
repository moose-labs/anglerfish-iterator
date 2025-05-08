use anyhow::{Result, anyhow};
use sui_sdk::{
    SUI_COIN_TYPE,
    rpc_types::SuiTransactionBlockResponseOptions,
    types::{
        digests::TransactionDigest,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{ProgrammableTransaction, TransactionData},
    },
};

use super::client::AnglerfishClient;

#[async_trait::async_trait]
pub trait AnglerfishSuiClient {
    async fn execute(&self, pt: ProgrammableTransaction) -> Result<TransactionDigest>;
}

#[async_trait::async_trait]
impl AnglerfishSuiClient for AnglerfishClient {
    async fn execute(&self, pt: ProgrammableTransaction) -> Result<TransactionDigest> {
        let wallet = self.wallet();
        let sender_address = wallet
            .get_addresses()
            .first()
            .ok_or_else(|| anyhow!("No address found in the wallet"))?
            .clone();

        // we need to find the coin we will use as gas
        let sui_coins = self
            .sui_client()
            .coin_read_api()
            .get_coins(
                sender_address,
                Option::Some(SUI_COIN_TYPE.to_string()),
                None,
                None,
            )
            .await?;
        let gas_coin = sui_coins.data.into_iter().next().unwrap();

        // using the PTB that we just constructed, create the transaction data
        // that we will submit to the network
        let max_gas_budget = 5_000_000; // 1 Hop swap can take 5m gas budget
        let gas_price = self
            .sui_client()
            .read_api()
            .get_reference_gas_price()
            .await?;
        let tx_data = TransactionData::new_programmable(
            sender_address,
            vec![gas_coin.object_ref()],
            pt,
            max_gas_budget,
            gas_price,
        );

        // check if the transaction is valid
        // this is a dry run, so it won't be submitted to the network
        let ret = self
            .sui_client()
            .read_api()
            .dry_run_transaction_block(tx_data.clone())
            .await?;

        if ret.execution_error_source.is_none() {
            let signed_tx = wallet.sign_transaction(&tx_data);
            let transaction_response = self
                .sui_client()
                .quorum_driver_api()
                .execute_transaction_block(
                    signed_tx,
                    SuiTransactionBlockResponseOptions::default(),
                    Some(ExecuteTransactionRequestType::WaitForLocalExecution),
                )
                .await?;

            return Ok(transaction_response.digest);
        } else {
            return Err(anyhow!(
                "Transaction is reverted: {:?}",
                ret.execution_error_source
            ));
        }
    }
}
