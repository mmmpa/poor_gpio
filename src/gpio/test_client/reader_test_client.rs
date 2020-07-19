use crate::*;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct GpioReaderTestClient {
    n: usize,
}

impl Gpio for GpioReaderTestClient {
    fn new_with_n(n: usize) -> Self {
        Self { n }
    }

    fn gpio_n(&self) -> usize {
        self.n
    }
}

#[async_trait]
impl GpioReaderOpener for GpioReaderTestClient {
    async fn open(n: usize) -> GpioResult<Self>
    where
        Self: Sized,
    {
        std::fs::create_dir_all("./tmp").unwrap();
        Ok(Self::new_with_n(n))
    }
}

#[async_trait]
impl GpioReader for GpioReaderTestClient {
    async fn read(&self) -> GpioResult<usize> {
        let n = match std::fs::read_to_string(format!("./tmp/{}", self.gpio_n())) {
            Ok(n) => n,
            Err(_) => return Ok(0),
        };

        match n.parse() {
            Ok(n) => Ok(n),
            Err(_) => Ok(0),
        }
    }
}

impl GpioReaderIntoListener for GpioReaderTestClient {}

pub async fn create_test_reader(n: usize) -> impl GpioReader {
    GpioReaderTestClient::open(n).await.unwrap()
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
