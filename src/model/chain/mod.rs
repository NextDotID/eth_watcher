#[cfg(test)]
mod tests;

use anyhow::Result;
use entities::chain::{Model as ChainModel, Entity as ChainEntity, Column as ChainColumn};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, DatabaseConnection};

pub async fn find_by_name(db: &DatabaseConnection, name: &str) -> Result<Option<ChainModel>> {
    ChainEntity::find().filter(ChainColumn::Name.eq(name)).one(db).await.map_err(Into::into)
}

pub async fn find_by_network_id(db: &DatabaseConnection, network_id: &str) -> Result<Option<ChainModel>> {
    ChainEntity::find().filter(ChainColumn::NetworkId.eq(network_id)).one(db).await.map_err(Into::into)
}
