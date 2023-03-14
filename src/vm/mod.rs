use anyhow::Result;
use std::{io::Write, rc::Rc};

#[cfg(test)]
mod tests;

pub(crate) mod block_call;

/// Run given JS code in VM.
async fn run_js_code(code: &str) -> Result<()> {
    let tmpdir = tempfile::tempdir()?;
    let tmpfile_path = tmpdir
        .path()
        .join(format!("{}.ts", uuid::Uuid::new_v4().to_string()));
    let mut tmpfile = std::fs::File::create(tmpfile_path.clone())?;
    write!(tmpfile, "{}", code).unwrap();

    run_js_file(tmpfile_path.to_str().unwrap()).await
}

/// Run a specific JS file in VM.
async fn run_js_file(file_path: &str) -> Result<()> {
    let main_module = deno_core::resolve_path(file_path)?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        ..Default::default()
    });
    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}
