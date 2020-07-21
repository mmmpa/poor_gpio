use crate::*;
use async_trait::async_trait;
use tokio::prelude::*;

#[async_trait]
pub trait GpioWriter: Gpio {
    async fn write(&mut self, value: usize) -> GpioResult<()> {
        self.config_mut()
            .file
            .as_mut()
            .unwrap()
            .write(value.to_string().as_bytes())
            .await?;

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
