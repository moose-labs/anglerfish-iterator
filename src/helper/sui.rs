use std::str::FromStr;

use anyhow::{Result, anyhow};
use serde::de::DeserializeOwned;
use sui_sdk::{
    SuiClient,
    rpc_types::{SuiData, SuiObjectData, SuiObjectDataOptions, SuiObjectResponse},
    types::{
        SUI_CLOCK_OBJECT_ID, SUI_CLOCK_OBJECT_SHARED_VERSION, SUI_RANDOMNESS_STATE_OBJECT_ID,
        base_types::{ObjectID, ObjectRef},
        transaction::ObjectArg,
    },
};

// Object build helper trait for SuiClient

#[async_trait::async_trait]
pub trait SuiObjectBuilder {
    async fn fetch_obj<T>(&self, obj_id: &str) -> Result<T>
    where
        T: DeserializeOwned;

    async fn parse_obj_bcs<T>(&self, data: SuiObjectData) -> Result<T>
    where
        T: DeserializeOwned;

    async fn object_ref(&self, object_id: &str) -> Result<ObjectRef>;
    async fn owned_obj(&self, object_id: &str) -> Result<ObjectArg>;
    async fn shared_obj_mut(&self, object_id: &str) -> Result<ObjectArg>;
    async fn shared_obj(&self, object_id: &str) -> Result<ObjectArg>;
    async fn clock(&self) -> Result<ObjectArg>;
    async fn randomness(&self) -> Result<ObjectArg>;
}

#[async_trait::async_trait]
impl SuiObjectBuilder for SuiClient {
    async fn fetch_obj<T>(&self, obj_id: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let obj_id = ObjectID::from_str(obj_id)?;
        let resp = self
            .read_api()
            .get_object_with_options(obj_id, SuiObjectDataOptions::default().with_bcs())
            .await?;
        let data = resp
            .data
            .ok_or_else(|| anyhow!("Could not find object with ID: {}", obj_id))?;
        self.parse_obj_bcs(data).await
    }

    async fn parse_obj_bcs<T>(&self, data: SuiObjectData) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let bcs_data = data
            .bcs
            .ok_or_else(|| anyhow!("Fetched object but no data was returned"))?;
        let move_obj = bcs_data
            .try_as_move()
            .ok_or_else(|| anyhow!("Fetched object is not a Move Object"))?;
        let deserialized_data: T = move_obj.deserialize()?;
        Ok(deserialized_data)
    }

    async fn object_ref(&self, object_id: &str) -> Result<ObjectRef> {
        let object_id = ObjectID::from_str(object_id)?;
        let object: SuiObjectResponse = self
            .read_api()
            .get_object_with_options(object_id, SuiObjectDataOptions::default())
            .await?;

        if let Some(error) = object.error {
            return Err(anyhow!(error));
        }

        let data = object.data.ok_or_else(|| {
            anyhow!(
                "Fetched object but no data was returned for object {:?}",
                object_id
            )
        })?;

        Ok(data.object_ref())
    }

    async fn owned_obj(&self, object_id: &str) -> Result<ObjectArg> {
        Ok(ObjectArg::ImmOrOwnedObject(
            self.object_ref(object_id).await?,
        ))
    }

    async fn shared_obj_mut(&self, object_id: &str) -> Result<ObjectArg> {
        inner_shared_obj_mutably_arg(self, ObjectID::from_str(object_id)?, true).await
    }

    async fn shared_obj(&self, object_id: &str) -> Result<ObjectArg> {
        inner_shared_obj_mutably_arg(self, ObjectID::from_str(object_id)?, false).await
    }

    /// Returns the clock object as a shared object argument.
    /// Clock object is a shared object which initial version is always the same
    async fn clock(&self) -> Result<ObjectArg> {
        Ok(ObjectArg::SharedObject {
            id: SUI_CLOCK_OBJECT_ID,
            initial_shared_version: SUI_CLOCK_OBJECT_SHARED_VERSION,
            mutable: false,
        })
    }

    /// Returns the randomness state object as a shared object argument.
    /// Randomness state is a shared object which initial version is always changed
    async fn randomness(&self) -> Result<ObjectArg> {
        inner_shared_obj_mutably_arg(self, SUI_RANDOMNESS_STATE_OBJECT_ID, false).await
    }
}

/// Helper function to create a shared object argument for a mutable or immutable object.
/// This function fetches the initial shared version of the shared object and returns an `ObjectArg`
async fn inner_shared_obj_mutably_arg(
    sui_client: &SuiClient,
    obj_id: ObjectID,
    mutable: bool,
) -> Result<ObjectArg> {
    let obj = sui_client
        .read_api()
        .get_object_with_options(obj_id, SuiObjectDataOptions::default().with_owner())
        .await?;
    let initial_shared_version = obj
        .data
        .ok_or_else(|| anyhow!("Could not find object with ID: {}", obj_id))?
        .owner
        .ok_or_else(|| anyhow!("Object has no owner fields"))?
        .start_version()
        .ok_or_else(|| anyhow!("Shared object has no start version"))?;

    Ok(ObjectArg::SharedObject {
        id: obj_id,
        initial_shared_version,
        mutable,
    })
}
