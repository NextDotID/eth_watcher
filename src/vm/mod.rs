pub(crate) mod block_call;
mod ops;
#[cfg(test)]
mod tests;

use anyhow::Result;
use deno_core::Extension;
use std::rc::Rc;
use tracing::{span, Level};

/// Run given JS code in VM.
async fn run_js_code(code: &str) -> Result<()> {
    let span = span!(Level::INFO, "Running JS code");
    let _guard = span.enter();

    let mut js_runtime = new_js_runtime()?;
    js_runtime.execute_script("[ethwatcher:main.js]", code)?;

    Ok(())
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
    result.await?
}

/// Create a new JS runtime, with `ops` injected.
fn new_js_runtime() -> Result<deno_core::JsRuntime> {
    let extension_console = Extension::builder("console")
        .ops(vec![
            ops::op_console_log::decl(),
            ops::op_console_warn::decl(),
            ops::op_console_error::decl(),
            ops::op_console_debug::decl(),
        ])
        .build();

    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![extension_console],
        ..Default::default()
    });
    js_runtime.execute_script("[ethwatcher:runtime.js]", include_str!("./js/runtime.js"))?;

    Ok(js_runtime)
}
