use crate::*;
use async_trait::async_trait;
use nix::sys::epoll::*;
use std::os::unix::io::AsRawFd;
use std::sync::{Arc, RwLock};

#[derive(Debug, Eq, PartialEq)]
pub enum GpioReaderEvent {
    FromTo((usize, usize)),
    EpollError(nix::Error),
    Picker,
    MaybeIoClosed(GpioError),
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

#[async_trait]
pub trait IntoGpioReaderReceiver: GpioReader {
    async fn into_receiver(self) -> GpioResult<tokio::sync::mpsc::Receiver<GpioReaderEvent>> {
        let (mut sender, receiver) = tokio::sync::mpsc::channel(100);

        // to pass events from normal thread to tokio.
        let stack = Arc::new(RwLock::new(None));

        {
            let gpio = tokio::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(self.value_path())
                .await?;
            let epoll_fd = epoll_create().unwrap();
            let mut event = EpollEvent::new(EpollFlags::EPOLLPRI | EpollFlags::EPOLLET, 0);
            let mut epoll_events = vec![EpollEvent::empty(); 1];
            let pre = self.read().await?;

            let stack = stack.clone();
            let send = move |event: GpioReaderEvent| *stack.write().unwrap() = Some(event);

            // MUST use std::thread.
            // In tokio, epoll blocks all tokio::spawns.
            std::thread::spawn(move || {
                let mut pre = pre;

                // MUST get fd in this thread.
                // (Otherwise you will get EBADF - bad file descriptor.)
                let gpio_fd = gpio.as_raw_fd();

                if let Err(e) = epoll_ctl(epoll_fd, EpollOp::EpollCtlAdd, gpio_fd, &mut event) {
                    debug!("{:?}", e);
                    send(GpioReaderEvent::EpollError(e));
                    return;
                };

                loop {
                    if let Err(e) = epoll_wait(epoll_fd, &mut epoll_events, -1) {
                        debug!("{:?}", e);
                        send(GpioReaderEvent::EpollError(e));
                        continue;
                    };

                    let next = match self.read_sync() {
                        Ok(v) => v,
                        Err(e) => {
                            debug!("{:?}", e);
                            send(GpioReaderEvent::MaybeIoClosed(e));
                            return;
                        }
                    };
                    if pre != next {
                        send(GpioReaderEvent::FromTo((pre, next)));
                    }
                    pre = next;
                }
            });
        }

        tokio::spawn(async move {
            loop {
                tokio::time::delay_for(tokio::time::Duration::from_millis(50)).await;

                let taken = match stack.try_write() {
                    Ok(mut op) => op.take(),
                    Err(_) => continue,
                };
                match taken {
                    Some(event) => {
                        sender.send(event).await.unwrap();
                    }
                    _ => {}
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
