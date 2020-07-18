use crate::*;
use std::process::Command;

pub trait GpioWriter: Gpio {
    fn open(n: usize) -> GpioResult<Self>
    where
        Self: Sized,
    {
        Self::prepare(n, "out")?;

        Ok(Self::new_with_n(n))
    }

    fn write(&self, value: usize) -> GpioResult<()> {
        Command::new("sh")
            .arg("-c")
            .arg("echo")
            .arg(value.to_string())
            .arg(">")
            .arg(self.value_path())
            .output()?;

        Ok(())
    }
}
