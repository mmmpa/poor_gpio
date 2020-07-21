use crate::*;
use async_trait::async_trait;
use tokio::prelude::*;
use tokio::sync::mpsc::Receiver;
use tokio::time::Duration;

#[derive(Debug, Eq, PartialEq)]
pub enum GpioReaderEvent {
    FromTo((usize, usize)),
}

#[async_trait]
pub trait GpioReader: Gpio {
    async fn read(&mut self) -> GpioResult<usize> {
        let o = match tokio::fs::read(self.value_path()).await {
            Ok(o) => o,
            Err(e) => return Ok(0),
        };
        let out = String::from_utf8(o)?;

        info!("{}", out);

        match chomp(&out).parse() {
            Ok(n) => Ok(n),
            Err(_) => Ok(0),
        }
    }
}
#[async_trait]
pub trait GpioReaderIntoListener: GpioReader {
    async fn into_listener(self) -> GpioResult<Receiver<GpioReaderEvent>> {
        self.into_listener_with_interval(10).await
    }

    async fn into_listener_with_interval(
        mut self,
        msec: u64,
    ) -> GpioResult<Receiver<GpioReaderEvent>> {
        let interval = Duration::from_millis(msec);
        let pre = self.read().await?;
        let (mut sender, receiver) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            let mut pre = pre;
            loop {
                tokio::time::delay_for(interval).await;

                let next = self.read().await.unwrap();
                if pre != next {
                    sender
                        .send(GpioReaderEvent::FromTo((pre, next)))
                        .await
                        .unwrap();
                }
                pre = next;
            }
        });

        Ok(receiver)
    }
}

#[async_trait]
pub trait GpioReaderOpener: Gpio {
    async fn open(config: Config) -> GpioResult<Self>
    where
        Self: Sized,
    {
        Self::prepare(config, GpioDirection::In).await
    }
}
