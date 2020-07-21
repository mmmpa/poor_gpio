use crate::*;
use async_trait::async_trait;
use tokio::fs::File;
use tokio::io::Error;

#[derive(Debug)]
pub enum GpioDirection {
    In,
    Out,
}

#[async_trait]
pub trait Gpio: Sized + Sync + Send + 'static {
    fn new_with(config: Config) -> Self;
    fn config(&self) -> &Config;
    fn config_mut(&mut self) -> &mut Config;
    fn n(&self) -> &str {
        self.config().gpio_n_str.as_ref().unwrap()
    }

    async fn prepare(mut config: Config, direction: GpioDirection) -> GpioResult<Self>
    where
        Self: Sized,
    {
        use GpioError::*;
        info!("gpio start preparation");

        config.gpio_n_str = Some(config.gpio_n.to_string());
        config.value_path = Some(format!("/sys/class/gpio/gpio{}/value", config.gpio_n));

        if config.open {
            match tokio::fs::write("/sys/class/gpio/export", config.gpio_n.to_string()).await {
                Ok(_) => info!("opened {}", config.gpio_n),
                Err(e) => {
                    error!("open error");
                    if config.err_if_already_opened {
                        return Err(SomethingWrong(e.to_string()));
                    }
                }
            }
        }

        // TODO: verify direction
        let direction_path = format!("/sys/class/gpio/gpio{}/direction", config.gpio_n);
        let mut open_option = tokio::fs::OpenOptions::new();
        let file = match direction {
            GpioDirection::In => {
                tokio::fs::write(&direction_path, "in").await.unwrap();
                open_option.read(true)
            }
            GpioDirection::Out => {
                tokio::fs::write(&direction_path, "out").await.unwrap();
                open_option.read(true).write(true)
            }
        }
        .open(config.value_path.as_ref().unwrap())
        .await;

        let file = match file {
            Ok(file) => {
                info!("detected direction: {} {:?}", direction_path, direction);
                file
            }
            Err(e) => {
                return {
                    error!("detect direction error: {}", e);
                    Err(SomethingWrong(e.to_string()))
                }
            }
        };
        config.file = Some(file);

        info!("gpio prepared");

        Ok(Self::new_with(config))
    }

    fn close(self) {}

    fn value_path(&self) -> &str {
        self.config().value_path.as_ref().unwrap()
    }
}
