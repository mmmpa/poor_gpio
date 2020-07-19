use poor_gpio::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test().await;

    Ok(())
}

async fn test() {
    let gp_green = GpioWriterClient::open(2).await;
    let gp_red = GpioWriterClient::open(3).await;

    println!("{:?} {:?}", gp_green, gp_red);

    let mut on = gp_green.unwrap();
    let mut off = gp_red.unwrap();

    for _ in 0..50 {
        tokio::time::delay_for(Duration::from_millis(50)).await;

        let a = on.write(1).await;
        let b = off.write(0).await;

        println!("{:?} {:?}", a, b);

        std::mem::swap(&mut on, &mut off)
    }
}
