#[cfg(test)]
mod tests;

use anyhow::{anyhow, Result};
use chrono::naive::NaiveDateTime;
use entities::chain::Model as ChainModel;
use serde::{Deserialize, Serialize};
use web3::types::{Block, Transaction};

#[derive(Debug, Clone, Serialize)]
pub struct BlockCallChain {
    pub id: i32,
    pub name: String,
    pub chain_id: String,
    pub created_at: NaiveDateTime,
}

impl TryFrom<&ChainModel> for BlockCallChain {
    type Error = anyhow::Error;
    fn try_from(chain_model: &ChainModel) -> Result<Self> {
        Ok(Self {
            id: chain_model.id,
            name: chain_model.name.clone(),
            chain_id: chain_model.chain_id.clone().ok_or(anyhow!(
                "Chain #{:?} ({}): ChainID is required before script call",
                chain_model.id,
                chain_model.name,
            ))?,
            created_at: chain_model.created_at.clone(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BlockCallbackRequest {
    pub meta: BlockCallMeta,
    pub chain: BlockCallChain,
    pub block: Block<Transaction>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BlockCallMeta {
    pub event: super::Event,
    pub triggered_at: NaiveDateTime,
}

#[typetag::serde(tag = "type", content = "data")]
pub trait BlockCallbackResponse {
    fn action(&self);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BlockCallbackResponseWebhook {
    pub url: String,
}

#[typetag::serde(name = "webhook")]
impl BlockCallbackResponse for BlockCallbackResponseWebhook {
    fn action(&self) {
        println!("Webhook: {}", self.url); // TODO
    }
}

impl BlockCallbackRequest {
    /// Create a new block callback request from fetched block and database model.
    pub fn from_block(block: &Block<Transaction>, chain_model: &ChainModel) -> Result<Self> {
        Ok(Self {
            block: block.clone(),
            chain: chain_model.try_into()?,
        })
    }
}
