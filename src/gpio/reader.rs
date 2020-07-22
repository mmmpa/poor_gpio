use crate::*;
use async_trait::async_trait;
use nix::sys::epoll::*;

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

    fn read_sync(&self) -> GpioResult<usize> {
        let o = match std::fs::read(self.value_path()) {
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
use std::sync::{Arc, RwLock};
#[async_trait]
pub trait GpioReaderIntoListener: GpioReader {
    async fn into_listener(self) -> GpioResult<tokio::sync::mpsc::Receiver<GpioReaderEvent>> {
        let pre = self.read().await?;
        let (mut sender, receiver) = tokio::sync::mpsc::channel(100);

        let stack = Arc::new(RwLock::new(None));

        let pass_stack = stack.clone();
        std::thread::spawn(move || {
            let value = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(self.value_path())
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

                let next = self.read_sync().unwrap();
                if pre != next {
                    *pass_stack.write().unwrap() = Some(GpioReaderEvent::FromTo((pre, next)));
                }
                pre = next;
            }
        });

        tokio::spawn(async move {
            loop {
                tokio::time::delay_for(tokio::time::Duration::from_millis(50)).await;
                let t = stack.write().unwrap().take();
                match t {
                    None => {}
                    Some(event) => {
                        sender.send(event).await.unwrap();
                    }
                }
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
