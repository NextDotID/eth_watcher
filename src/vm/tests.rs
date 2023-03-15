use super::*;
use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn test_run_js_code_console_success() -> Result<()> {
    let code = r#"
console.log("Test");
console.warn("Warning!");
console.debug("Debug!");
console.error("Error!");
"#;
    let mut runtime = new_js_runtime()?;
    run_js_code(&mut runtime, code).await?;
    Ok(())
}

#[tokio::test]
async fn test_inject_and_get() -> Result<()> {
    let code = r#"
let result = ew.FOO;
result += 1;
actions.push({type: "set", key: "FOO", value: result});
"#;

    let mut runtime = new_js_runtime()?;
    let foo = json!(10);
    inject_into_vm(&mut runtime, "FOO", &foo)?;
    // Run code modification tasks
    let result = run_js_code(&mut runtime, code).await?;
    // Get result
    let get_actions_code = r#"globalThis.actions[0]"#;
    let action = eval_and_return(&mut runtime, get_actions_code)?;
    assert!(action.is_object());
    assert_eq!(json!("FOO"), action.get("key").unwrap().to_owned());
    assert_eq!(json!(11), action.get("value").unwrap().to_owned());

    Ok(())
}
