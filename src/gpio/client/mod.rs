use crate::*;

pub struct GpioWriterClient {
    n: usize,
}

impl Gpio for GpioWriterClient {
    fn new_with_n(n: usize) -> Self
    where
        Self: Sized,
    {
        Self { n }
    }
    fn gpio_n(&self) -> usize {
        self.n
    }
}

impl GpioWriter for GpioWriterClient {}

pub struct GpioReaderClient {
    n: usize,
}

impl Gpio for GpioReaderClient {
    fn new_with_n(n: usize) -> Self
    where
        Self: Sized,
    {
        Self { n }
    }
    fn gpio_n(&self) -> usize {
        self.n
    }
}

impl GpioReader for GpioReaderClient {}
