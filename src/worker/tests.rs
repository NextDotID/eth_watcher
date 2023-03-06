use crate::model::{chain::tests::generate_data, tests::init_test_db};

use super::*;
use anyhow::Result;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_watch_newest_blocks() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(10);
    let db = init_test_db().await?;
    let chain_model = generate_data(&db).await?;
    let worker = watch_newest_blocks(chain_model, tx.clone());
    let handler = tokio::spawn(worker);
    match rx.recv().await {
        Some(result) => {
            handler.abort();
            match result {
                BlockFetchResult::Ok {
                    name: _,
                    chain_id: _,
                    block_number,
                    block,
                } => {
                    assert!(block_number > U64::from(40024612));
                    assert!(block.transactions.len() > 0);
                }
                BlockFetchResult::Error(err) => {
                    println!("Error from worker: {:?}", err);
                    assert!(false, "Error happened in wroker.");
                }
            }
        }
        None => {
            handler.abort();
            assert!(false, "rx.recv() returns None.");
        }
    }

    Ok(())
}
