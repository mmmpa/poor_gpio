use crate::*;
use async_trait::async_trait;

#[derive(Debug)]
pub enum GpioDirection {
    In,
    Out,
}

#[async_trait]
pub trait Gpio: Sized + Sync + Send + 'static {
    fn new_with(config: NormalizedConfig) -> Self;
    fn config(&self) -> &NormalizedConfig;
    fn n(&self) -> &str {
        &self.config().gpio_n_str
    }
    fn value_path(&self) -> &str {
        &self.config().value_path
    }

    async fn prepare(config: Config, direction: GpioDirection) -> GpioResult<Self>
    where
        Self: Sized,
    {
        use GpioError::*;
        debug!("start preparation: {}", config.gpio_n);

        let gpio_n_str = config.gpio_n.to_string();
        let value_path = format!("/sys/class/gpio/gpio{}/value", config.gpio_n);

        let opened = if config.open {
            match tokio::fs::write("/sys/class/gpio/export", &gpio_n_str).await {
                Err(e) if config.err_if_already_opened => {
                    debug!("already opened: {}", config.gpio_n);
                    return Err(SomethingWrong(e.to_string()));
                }
                Ok(_) => true,
                _ => false,
            }
        } else {
            false
        };

        let direction_path = format!("/sys/class/gpio/gpio{}/direction", config.gpio_n);

        match match direction {
            GpioDirection::In => tokio::fs::write(&direction_path, "in").await,
            GpioDirection::Out => tokio::fs::write(&direction_path, "out").await,
        } {
            Err(_) => return Err(DirectionNotMatch),
            _ => {}
        };

        tokio::fs::write(format!("/sys/class/gpio/gpio{}/edge", &gpio_n_str), "both")
            .await
            .unwrap();

        debug!("prepared: {}", gpio_n_str);

        Ok(Self::new_with(NormalizedConfig {
            close: opened && config.close_if_open_self,
            gpio_n: 0,
            gpio_n_str,
            value_path,
        }))
    }

    fn close(self) {}
}
