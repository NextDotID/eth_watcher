use deno_core::{op, error::AnyError};
use tracing::{info, debug, warn, error};

#[op]
fn op_console_log(content: String) -> Result<(), AnyError> {
    info!("{}", content);
    Ok(())
}

#[op]
fn op_console_debug(content: String) -> Result<(), AnyError> {
    debug!("{}", content);
    Ok(())
}

#[op]
fn op_console_warn(content: String) -> Result<(), AnyError> {
    warn!("{}", content);
    Ok(())
}

#[op]
fn op_console_error(content: String) -> Result<(), AnyError> {
    error!("{}", content);
    Ok(())
}
