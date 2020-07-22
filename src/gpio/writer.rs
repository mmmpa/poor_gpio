use crate::*;
use async_trait::async_trait;

#[async_trait]
pub trait GpioWriter: Gpio {
    async fn write(&self, value: usize) -> GpioResult<()> {
        tokio::fs::write(self.value_path(), value.to_string().as_bytes()).await?;

        Ok(())
    }
}

#[async_trait]
pub trait GpioWriterOpener: Gpio {
    async fn open(config: Config) -> GpioResult<Self>
    where
        Self: Sized,
    {
        Self::prepare(config, GpioDirection::Out).await
    }
}
