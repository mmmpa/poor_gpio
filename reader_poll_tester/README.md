# Example

```
# build
./build

# transfer
scp -i ~/.ssh/xxx target/arm-unknown-linux-musleabihf/release/reader_tester pi@192.168.0.x:/home/pi/reader_tester

# execute
ssh -i ~/.ssh/pi_zero pi@192.168.0.7 "sudo ./tester"
```
