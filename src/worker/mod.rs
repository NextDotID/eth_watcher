#[cfg(test)]
mod tests;

use tokio::{sync::mpsc::Sender, time::Duration};

use anyhow::Result;
use entities::chain::Model as ChainModel;
use web3::types::{Block, Transaction, U64};

use crate::ethereum::Client;

/// Fetch result yield from newest block watcher.
#[derive(Debug)]
pub enum BlockFetchResult {
    Ok {
        name: String,
        chain_id: Option<String>,
        block_number: U64,
        block: Block<Transaction>,
    },
    Error(anyhow::Error),
}

#[allow(unreachable_code)]
async fn watch_newest_blocks(chain: ChainModel, tx: Sender<BlockFetchResult>) -> Result<()> {
    let client: Client = (&chain).try_into()?;
    let mut current = client.get_block_height().await?; // TODO: Should fetch from a specific block height
    const SLEEP_DURATION: Duration = Duration::from_secs(5); // TODO: sleep second should be defined by ChainModel

    loop {
        match client.get_block(Some(current)).await {
            Ok(block) => {
                tx.send(BlockFetchResult::Ok {
                    block_number: current,
                    block,
                    chain_id: chain.chain_id.clone(),
                    name: chain.name.clone(),
                }).await?;
                current += U64::one();
            }
            Err(err) => {
                tx.send(BlockFetchResult::Error(err.into())).await?;
            }
        };
        tokio::time::sleep(SLEEP_DURATION).await;
    }

    Ok(())
}
