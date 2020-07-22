use crate::*;
use async_trait::async_trait;
use nix::sys::epoll::*;
use tokio::sync::mpsc::Receiver;

#[derive(Debug, Eq, PartialEq)]
pub enum GpioReaderEvent {
    FromTo((usize, usize)),
}

#[async_trait]
pub trait GpioReader: Gpio {
    async fn read(&self) -> GpioResult<usize> {
        let o = match tokio::fs::read(self.value_path()).await {
            Ok(o) => o,
            Err(_) => return Ok(0),
        };
        let out = String::from_utf8(o)?;

        match chomp(&out).parse() {
            Ok(n) => Ok(n),
            Err(_) => Ok(0),
        }
    }
}
use std::os::unix::io::AsRawFd;
#[async_trait]
pub trait GpioReaderIntoListener: GpioReader {
    async fn into_listener(self) -> GpioResult<Receiver<GpioReaderEvent>> {
        let pre = self.read().await?;
        let (mut sender, receiver) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            let value = tokio::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(self.value_path())
                .await
                .unwrap();
            let value_fd = value.as_raw_fd();
            let epoll_fd = epoll_create().unwrap();
            let mut event =
                EpollEvent::new(EpollFlags::EPOLLPRI | EpollFlags::EPOLLET, value_fd as u64);
            epoll_ctl(epoll_fd, EpollOp::EpollCtlAdd, value_fd, &mut event).unwrap();

            let mut pre = pre;
            let mut epoll_events = vec![EpollEvent::empty(); 1];

            loop {
                epoll_wait(epoll_fd, &mut epoll_events, -1).unwrap();

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
