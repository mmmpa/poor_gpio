use std::process::Output;
use tokio::io::Error;

#[derive(Debug)]
pub enum GpioError {
    SomethingWrong(String),
    PreparationError(RunCommandError),
    RunCommandError(RunCommandError),
}

#[derive(Debug)]
pub struct RunCommandError {
    pub command: String,
    pub output: Output,
}

impl std::fmt::Display for GpioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for GpioError {}

impl From<std::io::Error> for GpioError {
    fn from(e: Error) -> Self {
        Self::SomethingWrong(e.to_string())
    }
}

impl From<std::num::ParseIntError> for GpioError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::SomethingWrong(e.to_string())
    }
}

impl From<std::string::FromUtf8Error> for GpioError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::SomethingWrong(e.to_string())
    }
}
