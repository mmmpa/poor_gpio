use crate::*;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct GpioWriterTestClient {
    n: usize,
}

impl Gpio for GpioWriterTestClient {
    fn new_with_n(n: usize) -> Self {
        Self { n }
    }

    fn gpio_n(&self) -> usize {
        self.n
    }
}

#[async_trait]
impl GpioWriter for GpioWriterTestClient {
    async fn write(&self, value: usize) -> GpioResult<()> {
        std::fs::write(format!("./tmp/{}", self.gpio_n()), value.to_string()).unwrap();
        info!("written: {} -> {}", self.gpio_n(), value);

        Ok(())
    }
}

#[async_trait]
impl GpioWriterOpener for GpioWriterTestClient {
    async fn open(n: usize) -> GpioResult<Self>
    where
        Self: Sized,
    {
        std::fs::create_dir_all("./tmp").unwrap();
        Ok(Self::new_with_n(n))
    }
}

#[async_trait]
impl GpioReader for GpioWriterTestClient {
    async fn read(&self) -> GpioResult<usize> {
        let n = std::fs::read_to_string(format!("./tmp/{}", self.gpio_n())).unwrap();

        Ok(n.parse().unwrap())
    }
}

pub async fn create_test_writer(n: usize) -> impl GpioWriter {
    GpioWriterTestClient::open(n).await.unwrap()
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
