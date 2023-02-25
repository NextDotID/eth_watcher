use anyhow::Result;
use web3::{
    transports,
    types::{Block, BlockNumber, U64, H256, BlockId, Transaction},
};

pub struct Client {
    pub name: String,
    pub rpc_url: String,
    web3: web3::api::Web3<transports::Http>,
}

impl Client {
    /// Create a new client
    pub fn new(name: String, rpc_url: String) -> Result<Self> {
        let client = transports::Http::new(&rpc_url)?;
        let web3 = web3::Web3::new(client);
        Ok(Self {
            name,
            rpc_url,
            web3,
        })
    }

    /// Get current target's block height
    pub async fn get_block_height(&self) -> Result<U64> {
        let height = self.web3.eth().block_number().await?;
        Ok(height)
    }

    pub async fn get_block(&self, block_number: Option<U64>) -> Result<Block<Transaction>> {
        let block = match block_number {
            None => {
                self.web3
                    .eth()
                    .block_with_txs(BlockId::Number(BlockNumber::Latest))
                    .await?
            }
            Some(height) => {
                self.web3
                    .eth()
                    .block_with_txs(BlockId::Number(BlockNumber::Number(height)))
                    .await?
            }
        };
        match block {
            Some(block) => Ok(block),
            None => {
                if block_number.is_some() {
                    Err(anyhow::Error::msg(format!(
                        "Block #{} not found.",
                        block_number.unwrap()
                    )))
                } else {
                    Err(anyhow::Error::msg("Latest block not found."))
                }
            }
        }
    }
}
