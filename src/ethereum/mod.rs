use anyhow::Result;
use web3::{transports, types::U64};

pub struct Client {
    pub rpc_url: String,
    web3: web3::api::Web3<transports::Http>,
}

impl Client {
    /// Create a new client
    pub fn new(rpc_url: String) -> Result<Self> {
        let client = transports::Http::new(&rpc_url)?;
        let web3 = web3::Web3::new(client);
        Ok(Self { rpc_url, web3 })
    }

    pub async fn get_block_height(&self) -> Result<U64> {
        let height = self.web3.eth().block_number().await?;
        Ok(height)
    }
}
