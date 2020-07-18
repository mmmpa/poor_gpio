use crate::*;
use std::process::Command;

pub trait GpioReader: Gpio {
    fn open(n: usize) -> GpioResult<Self>
    where
        Self: Sized,
    {
        Self::prepare(n, "in")?;

        Ok(Self::new_with_n(n))
    }

    fn read(&self) -> GpioResult<usize> {
        let o = Command::new("sh")
            .arg("-c")
            .arg("cat")
            .arg(self.value_path())
            .output()?;

        Ok(String::from_utf8(o.stdout)?.parse()?)
    }
}
