#!/bin/bash
curdir=$(pwd)

# http server
cd ${curdir}/zktls/app
RUST_LOG=info cargo pico build

cd ${curdir}/zktls/prover
RUST_LOG=info cargo build --release
