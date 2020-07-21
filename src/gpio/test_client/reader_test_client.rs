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
        let _ = std::fs::remove_file(format!("./tmp/{}", n));
        Ok(Self::new_with_n(n))
    }
}

#[async_trait]
impl GpioReader for GpioReaderTestClient {
    async fn read(&self) -> GpioResult<usize> {
        let out = match std::fs::read_to_string(format!("./tmp/{}", self.gpio_n())) {
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

pub async fn create_test_reader(n: usize) -> impl GpioReader + GpioReaderIntoListener {
    GpioReaderTestClient::open(n).await.unwrap()
}

pub fn test_write_value(gpio_n: usize, value: usize) {
    std::fs::write(format!("./tmp/{}", gpio_n), value.to_string()).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test() {
        let cli = create_test_reader(42).await;

        assert_eq!(0, cli.read().await.unwrap());

        test_write_value(42, 1);
        assert_eq!(1, cli.read().await.unwrap());
    }

    #[tokio::test]
    async fn test_listener() {
        let mut cli = create_test_reader(44).await.into_listener().await.unwrap();

        let r = tokio::spawn(async move {
            let v = cli.recv().await.unwrap();
            assert_eq!(GpioReaderEvent::FromTo((0, 1)), v);
            let v = cli.recv().await.unwrap();
            assert_eq!(GpioReaderEvent::FromTo((1, 0)), v);
            let v = cli.recv().await.unwrap();
            assert_eq!(GpioReaderEvent::FromTo((0, 1)), v);
            true
        });

        tokio::spawn(async {
            tokio::time::delay_for(Duration::from_millis(20)).await;
            test_write_value(44, 1);
            tokio::time::delay_for(Duration::from_millis(20)).await;
            test_write_value(44, 0);
            tokio::time::delay_for(Duration::from_millis(20)).await;
            test_write_value(44, 1);
        });

        let ended = r.await.unwrap();
        assert!(ended);
    }
}
