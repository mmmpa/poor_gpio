use poor_gpio::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test().await;

    Ok(())
}

async fn test() {
    let reader = GpioReaderClient::open(21).await;

    println!("{:?}", reader);

    let reader = reader.unwrap();

    let mut value = 0;

    for _ in 0..100 {
        tokio::time::delay_for(Duration::from_millis(50)).await;

        let next = reader.read().await;
        if let Ok(n) = next {
            if n != value {
                println!("read: {}", value)
            }
            value = n;
        } else {
            println!("read failure: {:?}", next)
        }
    }
}
