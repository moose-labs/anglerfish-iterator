use anyhow::{Result, anyhow};
use serde_json::Value;
use sui_sdk::{
    SuiClient,
    types::{
        TypeTag, base_types::ObjectID, digests::TransactionDigest, dynamic_field::DynamicFieldName,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
    },
    wallet_context::WalletContext,
};

use crate::{
    config::{self, Config},
    helper::sui::SuiObjectBuilder,
};

use super::{
    ids,
    ptb::AnglerfishProgramableTransaction,
    sui_client::AnglerfishSuiClient,
    types::{FieldsExtractor, phase_info::PhaseInfo, round::Round, round_registry::RoundRegistry},
};

pub struct AnglerfishClient {
    sui_client: SuiClient,
    wallet: WalletContext,
    config: Config,
}

impl AnglerfishClient {
    pub fn new(sui_client: SuiClient, wallet: WalletContext, config: Config) -> Self {
        AnglerfishClient {
            sui_client,
            wallet,
            config,
        }
    }

    pub fn sui_client(&self) -> &SuiClient {
        &self.sui_client
    }

    pub fn wallet(&self) -> &WalletContext {
        &self.wallet
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn objects(&self) -> &config::Objects {
        &self.config().objects
    }

    pub fn pool_coin_type(&self) -> String {
        self.config().pool.coin_type.to_owned()
    }

    pub fn iterator_cap_id(&self) -> String {
        self.config().iterator.cap_id.to_owned()
    }

    /// Fetch the Anglerfish objects

    pub async fn get_phase_info(&self) -> Result<PhaseInfo> {
        let phase_info = self
            .sui_client()
            .fetch_obj::<PhaseInfo>(&ids::anglerfish_phase_info_id())
            .await?;
        Ok(phase_info)
    }

    pub async fn get_round_registry(&self) -> Result<RoundRegistry> {
        let round_registry = self
            .sui_client()
            .fetch_obj::<RoundRegistry>(&ids::anglerfish_round_registry_id())
            .await?;
        Ok(round_registry)
    }

    pub async fn get_round_obj_id_from_table(
        &self,
        table_id: ObjectID,
        round_number: u64,
    ) -> Result<Round> {
        let round_fields = self
            .sui_client()
            .read_api()
            .get_dynamic_field_object(
                table_id,
                DynamicFieldName {
                    type_: TypeTag::U64,
                    value: Value::String(round_number.to_string()),
                },
            )
            .await?;
        let round = match round_fields.data {
            Some(obj) => Round::try_from_sui_obj_data(obj)?,
            _ => {
                return Err(anyhow!("Failed to fetch round fields"));
            }
        };
        Ok(round)
    }

    // Execute Anglerfish functions

    pub async fn execute_next_entry(&self) -> Result<TransactionDigest> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        self.build_next_entry(&mut ptb).await?;
        Ok(self.execute(ptb.finish()).await?)
    }

    pub async fn execute_draw(&self) -> Result<TransactionDigest> {
        let phase_info = self.get_phase_info().await?;
        let round_registry = self.get_round_registry().await?;
        let round = self
            .get_round_obj_id_from_table(round_registry.rounds.id, phase_info.current_round_number)
            .await?;
        let mut ptb = ProgrammableTransactionBuilder::new();
        self.build_draw(&mut ptb, &round).await?;
        Ok(self.execute(ptb.finish()).await?)
    }

    pub async fn execute_distribute(&self) -> Result<TransactionDigest> {
        let phase_info = self.get_phase_info().await?;
        let round_registry = self.get_round_registry().await?;
        let round = self
            .get_round_obj_id_from_table(round_registry.rounds.id, phase_info.current_round_number)
            .await?;
        let mut ptb = ProgrammableTransactionBuilder::new();
        self.build_distribute(&mut ptb, &round).await?;
        Ok(self.execute(ptb.finish()).await?)
    }

    pub async fn execute_start_new_round(&self) -> Result<TransactionDigest> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        self.build_start_new_round(&mut ptb).await?;
        Ok(self.execute(ptb.finish()).await?)
    }
}
