use crate::*;
use async_trait::async_trait;

#[async_trait]
pub trait Gpio: Sized + Sync + Send + 'static {
    fn new_with(config: Config) -> Self;
    fn config(&self) -> &Config;

    async fn prepare(config: Config, direction: &'static str) -> GpioResult<Self>
    where
        Self: Sized,
    {
        use GpioError::*;

        let re = || async {
            just_run(format!("echo {} > /sys/class/gpio/export", config.gpio_n)).await?;
            just_run(format!(
                "echo {} > /sys/class/gpio/gpio{}/direction",
                direction, config.gpio_n
            ))
            .await?;

            Ok(())
        };

        match re().await {
            Err(RunCommandError(e)) => Err(PreparationError(e)),
            Err(e) => Err(e),
            Ok(_) => Ok(Self::new_with(config)),
        }
    }

    fn close(self) {}

    fn value_path(&self) -> String {
        format!("/sys/class/gpio/gpio{}/value", self.config().gpio_n)
    }
}
