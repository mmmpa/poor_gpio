use crate::*;
use std::process::Command;

#[derive(Clone, Debug)]
pub struct GpioWriterClient {
    n: usize,
}

impl Gpio for GpioWriterClient {
    fn new_with_n(n: usize) -> Self {
        Self { n }
    }

    fn gpio_n(&self) -> usize {
        self.n
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
            .arg(format!("echo {} > /sys/class/gpio/unexport", self.gpio_n()))
            .output();
    }
}

#[derive(Clone, Debug)]
pub struct GpioReaderClient {
    n: usize,
}

impl Gpio for GpioReaderClient {
    fn new_with_n(n: usize) -> Self {
        Self { n }
    }

    fn gpio_n(&self) -> usize {
        self.n
    }
}

impl GpioReaderOpener for GpioReaderClient {}
impl GpioReader for GpioReaderClient {}

impl Drop for GpioReaderClient {
    fn drop(&mut self) {
        // must sync
        let _ = Command::new("sh")
            .arg("-c")
            .arg(format!("echo {} > /sys/class/gpio/unexport", self.gpio_n()))
            .output();
    }
}
