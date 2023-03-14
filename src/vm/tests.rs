use super::*;
use anyhow::Result;

#[tokio::test]
async fn test_run_js_code_success() -> Result<()> {
    let code = r#"Deno.core.print("Test\n");"#;
    run_js_code(code).await
}
