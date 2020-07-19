use crate::*;
use async_trait::async_trait;

#[async_trait]
pub trait Gpio: Sized + Sync + Send + 'static {
    fn new_with_n(n: usize) -> Self;
    fn gpio_n(&self) -> usize;

    async fn prepare(n: usize, direction: &'static str) -> GpioResult<Self>
    where
        Self: Sized,
    {
        use GpioError::*;

        let re = || async {
            just_run(format!("echo {} > /sys/class/gpio/export", n)).await?;
            just_run(format!(
                "echo {} > /sys/class/gpio/gpio{}/direction",
                direction, n
            ))
            .await?;

            Ok(())
        };

        match re().await {
            Err(RunCommandError(e)) => Err(PreparationError(e)),
            Err(e) => Err(e),
            Ok(_) => Ok(Self::new_with_n(n)),
        }
    }

    fn close(self) {}

    fn value_path(&self) -> String {
        format!("/sys/class/gpio/gpio{}/value", self.gpio_n())
    }
}
