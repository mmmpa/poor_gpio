use crate::*;

#[derive(Debug, Clone)]
pub struct GpioWriterClient {
    config: NormalizedConfig,
}

impl Gpio for GpioWriterClient {
    fn new_with(config: NormalizedConfig) -> Self {
        Self { config }
    }

    fn config(&self) -> &NormalizedConfig {
        &self.config
    }
}

impl GpioWriterOpener for GpioWriterClient {}
impl GpioWriter for GpioWriterClient {}
impl GpioReader for GpioWriterClient {}

impl Drop for GpioWriterClient {
    fn drop(&mut self) {
        if !self.config.close {
            return;
        }

        match std::fs::write("/sys/class/gpio/unexport", self.n()) {
            Ok(_) => debug!("closed: {}", self.n()),
            Err(_) => debug!("failed to close: {}", self.n()),
        };
    }
}

#[derive(Debug, Clone)]
pub struct GpioReaderClient {
    config: NormalizedConfig,
}

impl Gpio for GpioReaderClient {
    fn new_with(config: NormalizedConfig) -> Self {
        Self { config }
    }

    fn config(&self) -> &NormalizedConfig {
        &self.config
    }
}

impl GpioReaderOpener for GpioReaderClient {}
impl GpioReader for GpioReaderClient {}
impl IntoGpioReaderReceiver for GpioReaderClient {}

impl Drop for GpioReaderClient {
    fn drop(&mut self) {
        if !self.config.close {
            return;
        }

        match std::fs::write("/sys/class/gpio/unexport", self.n()) {
            Ok(_) => debug!("closed: {}", self.n()),
            Err(_) => debug!("failed to close: {}", self.n()),
        };
    }
}
