#!/bin/sh

CROSS_COMPILE=arm-linux-musleabihf-gcc cargo build --release --target arm-unknown-linux-musleabihf
