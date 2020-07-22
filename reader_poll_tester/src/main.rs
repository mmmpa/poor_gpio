use poor_gpio::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test().await;

    Ok(())
}

async fn test() {
    pretty_env_logger::init();
    println!("start reader_poll_tester");

    let reader = GpioReaderClient::open(Config {
        gpio_n: 21,
        ..Default::default()
    })
    .await;

    let mut reader = reader.unwrap().into_receiver().await.unwrap();

    let mut count = 0u32;

    tokio::spawn(async {
        loop {
            tokio::time::delay_for(tokio::time::Duration::from_secs(1)).await;
            println!("MUST NOT block this tokio thread");
        }
    });

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
