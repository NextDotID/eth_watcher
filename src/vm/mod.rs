pub(crate) mod block_call;
mod ops;
#[cfg(test)]
mod tests;

use anyhow::Result;
use deno_core::{
    v8::{Global, Value},
    Extension, JsRuntime,
};
use serde::Serialize;
use std::rc::Rc;
use tracing::{span, Level};

/// Run given JS code in VM.
async fn run_js_code(js_runtime: &mut JsRuntime, code: &str) -> Result<Global<Value>> {
    let span = span!(Level::INFO, "Running JS code");
    let _guard = span.enter();

    js_runtime.execute_script("[ethwatcher:main.js]", code)
}

/// Run a specific JS file in VM.
async fn run_js_file(file_path: &str) -> Result<()> {
    let span = span!(Level::INFO, "Running JS file", file_path);
    let _guard = span.enter();

    let mut js_runtime = new_js_runtime()?;
    let main_module = deno_core::resolve_path(file_path)?;
    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;

    result.await?;
    Ok(())
}

/// Create a new JS runtime, with `ops` injected.
fn new_js_runtime() -> Result<JsRuntime> {
    let extension_console = Extension::builder("console")
        .ops(vec![
            ops::op_console_log::decl(),
            ops::op_console_warn::decl(),
            ops::op_console_error::decl(),
            ops::op_console_debug::decl(),
        ])
        .build();

    let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![extension_console],
        ..Default::default()
    });
    js_runtime.execute_script("[ethwatcher:runtime.js]", include_str!("./js/runtime.js"))?;

    Ok(js_runtime)
}

fn inject_into_vm<T>(runtime: &mut JsRuntime, var_name: &str, data: &T) -> Result<Global<Value>>
where
    T: Serialize + ?Sized,
{
    let value = serde_json::to_string(data)?;
    let inject_code = format!(
        r#"((globalThis)=>{{ globalThis.ew.{} = {}; }})(globalThis);"#,
        var_name, value
    );
    let result = runtime.execute_script("[ethwatcher:inject_value.js]", &inject_code)?;
    Ok(result)
}

fn eval_and_return(context: &mut JsRuntime, code: &str) -> Result<serde_json::Value> {
    let res = context.execute_script("<anon>", code);
    match res {
        Ok(global) => {
            let scope = &mut context.handle_scope();
            let local = deno_core::v8::Local::new(scope, global);
            // Deserialize a `v8` object into a Rust type using `serde_v8`,
            // in this case deserialize to a JSON `Value`.
            let deserialized_value = serde_v8::from_v8::<serde_json::Value>(scope, local);

            match deserialized_value {
                Ok(value) => Ok(value),
                Err(err) => Err(err.into()),
            }
        }
        Err(err) => Err(err.into()),
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
enum Event {
    #[serde(rename = "new_block")]
    NewBlock,
}
