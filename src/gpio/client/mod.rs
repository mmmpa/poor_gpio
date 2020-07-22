use crate::*;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct GpioWriterClient {
    config: Config,
}

impl Gpio for GpioWriterClient {
    fn new_with(config: Config) -> Self {
        Self { config }
    }

    fn config(&self) -> &Config {
        &self.config
    }
}

impl GpioWriterOpener for GpioWriterClient {}
impl GpioWriter for GpioWriterClient {}
impl GpioReader for GpioWriterClient {}

impl Drop for GpioWriterClient {
    fn drop(&mut self) {
        // must sync
        let _ = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "echo {} > /sys/class/gpio/unexport",
                self.config.gpio_n
            ))
            .output();
    }
}

#[derive(Debug, Clone)]
pub struct GpioReaderClient {
    config: Config,
}

impl Gpio for GpioReaderClient {
    fn new_with(config: Config) -> Self {
        Self { config }
    }

    fn config(&self) -> &Config {
        &self.config
    }
}

impl GpioReaderOpener for GpioReaderClient {}
impl GpioReader for GpioReaderClient {}
impl IntoGpioReaderReceiver for GpioReaderClient {}

impl Drop for GpioReaderClient {
    fn drop(&mut self) {
        // must sync
        let _ = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "echo {} > /sys/class/gpio/unexport",
                self.config.gpio_n
            ))
            .output();
    }
}
