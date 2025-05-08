use anyhow::Result;
use sui_sdk::types::{
    programmable_transaction_builder::ProgrammableTransactionBuilder,
    transaction::{Command, ProgrammableMoveCall},
};

use crate::helper::{sui::SuiObjectBuilder, type_input::ToTypeInputs};

use super::{client::AnglerfishClient, ids, types::round::Round};

/// AnglerfishProgramableTransaction trait
/// This trait defines the programmable transaction of iterator capability methods for the Anglerfish protocol.

#[async_trait::async_trait]
pub trait AnglerfishProgramableTransaction {
    async fn build_next_entry(&self, ptb: &mut ProgrammableTransactionBuilder) -> Result<()>;
    async fn build_start_new_round(&self, ptb: &mut ProgrammableTransactionBuilder) -> Result<()>;
    async fn build_draw(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        round: Round,
    ) -> Result<()>;
    async fn build_distribute(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        round: Round,
    ) -> Result<()>;
}

#[async_trait::async_trait]
impl AnglerfishProgramableTransaction for AnglerfishClient {
    async fn build_next_entry(&self, ptb: &mut ProgrammableTransactionBuilder) -> Result<()> {
        let sui_client = self.sui_client();
        let iter_cap_id = self.iterator_cap_id();

        let iter_cap = ptb.obj(sui_client.owned_obj(&iter_cap_id).await?)?;

        let phase_info = ptb.obj(
            sui_client
                .shared_obj_mut(&ids::anglerfish_phase_info_id())
                .await?,
        )?;

        let clock = ptb.obj(sui_client.clock().await?)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: ids::anglerfish_package_obj_id(),
            module: "phase".to_string(),
            function: "next_entry".to_string(),
            type_arguments: vec![],
            arguments: vec![iter_cap, phase_info, clock],
        })));

        Ok(())
    }

    async fn build_start_new_round(&self, ptb: &mut ProgrammableTransactionBuilder) -> Result<()> {
        let sui_client = self.sui_client();
        let iter_cap_id = self.iterator_cap_id();

        let iter_cap = ptb.obj(sui_client.owned_obj(&iter_cap_id).await?)?;
        let phase_info = ptb.obj(
            sui_client
                .shared_obj_mut(&ids::anglerfish_phase_info_id())
                .await?,
        )?;
        let round_registry = ptb.obj(
            sui_client
                .shared_obj_mut(&ids::anglerfish_round_registry_id())
                .await?,
        )?;
        let prize_pool = ptb.obj(
            sui_client
                .shared_obj(&ids::anglerfish_prize_pool_id())
                .await?,
        )?;
        let clock = ptb.obj(sui_client.clock().await?)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: ids::anglerfish_package_obj_id(),
            module: "prize_pool".to_string(),
            function: "start_new_round".to_string(),
            type_arguments: vec![],
            arguments: vec![iter_cap, phase_info, round_registry, prize_pool, clock],
        })));
        Ok(())
    }

    async fn build_draw(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        round: Round,
    ) -> Result<()> {
        let sui_client = self.sui_client();
        let iter_cap_id = self.iterator_cap_id();
        let pool_coin_type = self.pool_coin_type();

        let iter_cap = ptb.obj(sui_client.owned_obj(&iter_cap_id).await?)?;
        let prize_pool = ptb.obj(
            sui_client
                .shared_obj(&ids::anglerfish_prize_pool_id())
                .await?,
        )?;
        let phase_info = ptb.obj(
            sui_client
                .shared_obj_mut(&ids::anglerfish_phase_info_id())
                .await?,
        )?;
        let pool_registry = ptb.obj(
            sui_client
                .shared_obj(&ids::anglerfish_pool_registry_id())
                .await?,
        )?;
        let round_registry = ptb.obj(
            sui_client
                .shared_obj(&ids::anglerfish_round_registry_id())
                .await?,
        )?;
        let round = ptb.obj(
            sui_client
                .shared_obj_mut(&round.id.id.bytes.to_string())
                .await?,
        )?;
        let randomness = ptb.obj(sui_client.randomness().await?)?;
        let clock = ptb.obj(sui_client.clock().await?)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: ids::anglerfish_package_obj_id(),
            module: "prize_pool".to_string(),
            function: "draw".to_string(),
            type_arguments: vec![pool_coin_type].to_type_inputs(),
            arguments: vec![
                iter_cap,
                prize_pool,
                phase_info,
                pool_registry,
                round_registry,
                round,
                randomness,
                clock,
            ],
        })));
        Ok(())
    }

    async fn build_distribute(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        round: Round,
    ) -> Result<()> {
        let sui_client = self.sui_client();
        let iter_cap_id = self.iterator_cap_id();
        let pool_coin_type = self.pool_coin_type();

        let iter_cap = ptb.obj(sui_client.owned_obj(&iter_cap_id).await?)?;
        let phase_info = ptb.obj(
            sui_client
                .shared_obj_mut(&ids::anglerfish_phase_info_id())
                .await?,
        )?;
        let prize_pool = ptb.obj(
            sui_client
                .shared_obj_mut(&ids::anglerfish_prize_pool_id())
                .await?,
        )?;
        let pool_registry = ptb.obj(
            sui_client
                .shared_obj_mut(&ids::anglerfish_pool_registry_id())
                .await?,
        )?;
        let lounge_registry = ptb.obj(
            sui_client
                .shared_obj_mut(&ids::anglerfish_lounge_registry_id())
                .await?,
        )?;
        let round_registry = ptb.obj(
            sui_client
                .shared_obj(&ids::anglerfish_round_registry_id())
                .await?,
        )?;
        let round = ptb.obj(
            sui_client
                .shared_obj_mut(&round.id.id.bytes.to_string())
                .await?,
        )?;
        let clock = ptb.obj(sui_client.clock().await?)?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: ids::anglerfish_package_obj_id(),
            module: "prize_pool".to_string(),
            function: "distribute".to_string(),
            type_arguments: vec![pool_coin_type].to_type_inputs(),
            arguments: vec![
                iter_cap,
                phase_info,
                prize_pool,
                pool_registry,
                lounge_registry,
                round_registry,
                round,
                clock,
            ],
        })));
        Ok(())
    }
}
