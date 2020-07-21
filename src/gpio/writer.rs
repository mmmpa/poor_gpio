use crate::*;
use async_trait::async_trait;

#[async_trait]
pub trait GpioWriter: Gpio {
    async fn write(&self, value: usize) -> GpioResult<()> {
        just_run(format!("echo {} > {}", value, self.value_path())).await?;

        Ok(())
    }
}

#[async_trait]
pub trait GpioWriterOpener: Gpio {
    async fn open(config: Config) -> GpioResult<Self>
    where
        Self: Sized,
    {
        Self::prepare(config, "out").await
    }
}
