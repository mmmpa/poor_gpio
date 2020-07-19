use crate::*;
use async_trait::async_trait;

#[async_trait]
pub trait GpioReader: Gpio {
    async fn read(&self) -> GpioResult<usize> {
        let o = just_run(format!("cat {}", self.value_path())).await?;
        let out = String::from_utf8(o.stdout)?;
        let mut out = out.as_str();

        if out.len() > 0 && &out[out.len() - 1..] == "\n" {
            out = &out[..out.len() - 1]
        }

        match out.parse() {
            Ok(n) => Ok(n),
            Err(_) => Ok(0),
        }
    }
}

#[async_trait]
pub trait GpioReaderOpener: Gpio {
    async fn open(n: usize) -> GpioResult<Self>
    where
        Self: Sized,
    {
        Self::prepare(n, "in").await
    }
}
