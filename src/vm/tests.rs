use super::*;
use anyhow::Result;

#[tokio::test]
async fn test_run_js_code_console_success() -> Result<()> {
    let code = r#"
console.log("Test");
console.warn("Warning!");
console.debug("Debug!");
console.error("Error!");
"#;
    run_js_code(code).await
}
