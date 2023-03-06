#[cfg(test)]
pub mod tests;

use anyhow::Result;
use entities::chain::{Model as ChainModel, Entity as ChainEntity, Column as ChainColumn, ActiveModel};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, DatabaseConnection, ActiveValue, ActiveModelTrait};
use web3::types::U256;
use crate::ethereum::Client;

impl TryFrom<&ChainModel> for Client {
    type Error = anyhow::Error;

    fn try_from(model: &ChainModel) -> Result<Self> {
        Client::new(model.name.clone(), model.rpc.clone())
    }
}

pub async fn find_by_name(db: &DatabaseConnection, name: &str) -> Result<Option<ChainModel>> {
    ChainEntity::find().filter(ChainColumn::Name.eq(name)).one(db).await.map_err(Into::into)
}

pub async fn find_by_chain_id(db: &DatabaseConnection, chain_id: &str) -> Result<Option<ChainModel>> {
    ChainEntity::find().filter(ChainColumn::ChainId.eq(chain_id)).one(db).await.map_err(Into::into)
}

pub async fn update_chain_id(db: &DatabaseConnection, model: ChainModel) -> Result<ChainModel> {
    let client: Client = (&model).try_into()?;
    let chain_id: U256 = client.get_chain_id().await?;

    let mut chain_active: ActiveModel = model.into();
    chain_active.chain_id = ActiveValue::Set(Some(chain_id.to_string()));
    let result = chain_active.update(db).await?;

    Ok(result)
}
