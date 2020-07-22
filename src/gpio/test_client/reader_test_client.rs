use crate::*;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct GpioReaderTestClient {
    config: Config,
}

impl Gpio for GpioReaderTestClient {
    fn new_with(config: Config) -> Self {
        Self { config }
    }

    fn config(&self) -> &Config {
        &self.config
    }
}

#[async_trait]
impl GpioReaderOpener for GpioReaderTestClient {
    async fn open(config: Config) -> GpioResult<Self>
    where
        Self: Sized,
    {
        std::fs::create_dir_all("./tmp").unwrap();
        let _ = std::fs::remove_file(format!("./tmp/{}", config.gpio_n));
        Ok(Self::new_with(config))
    }
}

#[async_trait]
impl GpioReader for GpioReaderTestClient {
    async fn read(&self) -> GpioResult<usize> {
        let out = match std::fs::read_to_string(format!("./tmp/{}", self.config().gpio_n)) {
            Ok(n) => n,
            Err(_) => return Ok(0),
        };

        match chomp(&out).parse() {
            Ok(n) => Ok(n),
            Err(_) => Ok(0),
        }
    }
}

impl GpioReaderIntoListener for GpioReaderTestClient {}

pub async fn create_test_reader(gpio_n: usize) -> impl GpioReader + GpioReaderIntoListener {
    GpioReaderTestClient::open(Config {
        gpio_n,
        ..Default::default()
    })
    .await
    .unwrap()
}

pub fn test_write_value(gpio_n: usize, value: usize) {
    std::fs::write(format!("./tmp/{}", gpio_n), value.to_string()).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test() {
        let cli = create_test_reader(42).await;

        assert_eq!(0, cli.read().await.unwrap());

        test_write_value(42, 1);
        assert_eq!(1, cli.read().await.unwrap());
    }
}
