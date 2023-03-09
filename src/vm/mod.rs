#[cfg(test)]
mod tests;

use serde::{Serialize, Deserialize};
use web3::types::{Block, Transaction};

#[derive(Debug, Clone)]
pub struct BlockCallbackRequest {
    pub chain: BlockCallbackRequestChain
}

#[derive(Debug, Clone)]
pub struct BlockCallbackRequestChain {
    pub name: String,
    pub chain_id: String,
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

impl TryFrom<Block<Transaction>> for BlockCallbackRequest {
    type Error = anyhow::Error;

    fn try_from(value: Block<Transaction>) -> Result<Self, Self::Error> {
        todo!()
    }
}
