use crate::*;
use std::process::Command;

pub trait Gpio {
    fn new_with_n(n: usize) -> Self
    where
        Self: Sized;

    fn gpio_n(&self) -> usize;

    fn prepare(n: usize, direction: &'static str) -> GpioResult<()>
    where
        Self: Sized,
    {
        Command::new("sh")
            .arg("-c")
            .arg("echo")
            .arg(n.to_string())
            .arg(">")
            .arg("/sys/class/gpio/export")
            .output()?;
        Command::new("sh")
            .arg("-c")
            .arg("echo")
            .arg(direction)
            .arg(">")
            .arg(format!("/sys/class/gpio/gpio{}/direction", n))
            .output()?;
        Ok(())
    }

    fn close(&self) -> GpioResult<()> {
        Command::new("sh")
            .arg("-c")
            .arg("echo")
            .arg(self.gpio_n().to_string())
            .arg(">")
            .arg("/sys/class/gpio/unexport")
            .output()?;

        Ok(())
    }

    fn value_path(&self) -> String {
        format!("/sys/class/gpio/gpio{}/value", self.gpio_n())
    }
}
