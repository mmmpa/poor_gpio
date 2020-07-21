use crate::*;
use async_trait::async_trait;

#[derive(Debug)]
pub struct GpioWriterTestClient {
    config: Config,
}

impl Gpio for GpioWriterTestClient {
    fn new_with(config: Config) -> Self {
        Self { config }
    }

    fn config(&self) -> &Config {
        &self.config
    }
}

#[async_trait]
impl GpioWriter for GpioWriterTestClient {
    async fn write(&self, value: usize) -> GpioResult<()> {
        std::fs::write(format!("./tmp/{}", self.config().gpio_n), value.to_string()).unwrap();
        info!("written: {} -> {}", self.config().gpio_n, value);

        Ok(())
    }
}

#[async_trait]
impl GpioWriterOpener for GpioWriterTestClient {
    async fn open(config: Config) -> GpioResult<Self>
    where
        Self: Sized,
    {
        std::fs::create_dir_all("./tmp").unwrap();
        Ok(Self::new_with(config))
    }
}

#[async_trait]
impl GpioReader for GpioWriterTestClient {
    async fn read(&self) -> GpioResult<usize> {
        let n = std::fs::read_to_string(format!("./tmp/{}", self.config().gpio_n)).unwrap();

        Ok(n.parse().unwrap())
    }
}

pub async fn create_test_writer(gpio_n: usize) -> impl GpioWriter {
    GpioWriterTestClient::open(Config {
        gpio_n,
        ..Default::default()
    })
    .await
    .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs::read_to_string;

    #[tokio::test]
    async fn test() {
        let cli = create_test_writer(24).await;

        cli.write(1).await.unwrap();
        assert_eq!(read_to_string("./tmp/24").unwrap(), "1");

        cli.write(2).await.unwrap();
        assert_eq!(read_to_string("./tmp/24").unwrap(), "2");
    }
}
