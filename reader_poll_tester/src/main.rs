use poor_gpio::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test().await;

    Ok(())
}

async fn test() {
    pretty_env_logger::init();

    let reader = GpioReaderClient::open(Config {
        gpio_n: 21,
        ..Default::default()
    })
    .await;

    let mut reader = reader.unwrap().into_listener().await.unwrap();

    let mut count = 0u32;

    loop {
        match reader.recv().await {
            None => count += 1,
            Some(n) => {
                count += 1;
                println!("{:?}", n);
            }
        }
        if count > 10 {
            break;
        }
    }
}
