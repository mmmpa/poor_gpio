#!/bin/sh

CROSS_COMPILE=arm-linux-gnueabihf- cargo build --release --target arm-unknown-linux-gnueabihf
