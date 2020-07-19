mod base;
mod client;
mod reader;
mod test_client;
mod writer;

pub use base::*;
pub use client::*;
pub use reader::*;
pub use test_client::*;
pub use writer::*;

use crate::*;
use std::process::Output;
use tokio::process::Command;

pub async fn just_run(command: String) -> GpioResult<Output> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command.as_str())
        .output()
        .await?;

    match output.status.code() {
        None => Err(GpioError::RunCommandError(RunCommandError {
            command,
            output,
        })),
        Some(code) if code != 0 => Err(GpioError::RunCommandError(RunCommandError {
            command,
            output,
        })),
        _ => Ok(output),
    }
}
