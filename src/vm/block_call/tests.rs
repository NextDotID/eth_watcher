use super::*;
use anyhow::Result;
use crate::{model::{chain::tests::generate_data as generate_chain, tests::init_test_db}, ethereum::Client};

#[test]
fn test_deserialize_response() -> Result<()> {
    let response_raw = r#"{"type": "webhook", "data": {"url": "test-url"}}"#;
    let response: Box<dyn BlockCallbackResponse> = serde_json::from_str(response_raw)?;
    response.action(); // Webhook: test-url

    Ok(())
}

#[tokio::test]
async fn test_block_callback_request_from_block() -> Result<()> {
    let db = init_test_db().await?;
    let chain_model = generate_chain(&db).await?;
    let eth_client: Client = (&chain_model).try_into()?;
    let block = eth_client.get_block(None).await?;
    let req = BlockCallbackRequest::from_block(&block, &chain_model)?;
    assert_eq!(req.meta.event, crate::vm::Event::NewBlock);

    Ok(())
}
