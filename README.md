# poor_gpio

This sample is most simple L チカ with gpio2 and gpio3.

```rust
use poor_gpio::*;
use tokio::time::Duration;

async fn test() {
    let mut on =  GpioWriterClient::open(2).await.unwrap();
    let mut off = GpioWriterClient::open(3).await.unwrap();

    for _ in 0..50 {
        tokio::time::delay_for(Duration::from_millis(50)).await;

        let a = on.write(1).await;
        let b = off.write(0).await;

        std::mem::swap(&mut on, &mut off)
    }
}
```

# Sample steps to build for Raspberry pi zero

There are Build samples in `reader_tester` and `writer_tester`.

## Add target

```
rustup target add arm-unknown-linux-musleabihf
```

## Add Config

`.cargo/config`

```ini
[target.arm-unknown-linux-musleabihf]
linker = "arm-linux-gnueabihf-gcc"
```

## Compile

```sh
CROSS_COMPILE=arm-linux-musleabihf- cargo build --release --target arm-unknown-linux-musleabihf
```
