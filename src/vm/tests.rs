use super::*;
use anyhow::Result;

#[test]
fn test_deserialize_response() -> Result<()> {
    let response_raw = r#"{"type": "webhook", "data": {"url": "test-url"}}"#;
    let response: Box<dyn BlockCallbackResponse> = serde_json::from_str(response_raw)?;
    response.action();

    Ok(())
}
