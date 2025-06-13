# pico-example



## Build

```sh
cd app
RUST_LOG=info cargo pico build
```

## Run

```sh
cd prover
RUST_LOG=info cargo run --release
```

[OR] local test:

```sh
RUST_LOG=info ./target/release/zktls-prover \
  --elf ./zktls/app/elf/riscv32im-pico-zkvm-elf \
  --input ./zktls/prover/data/attestation_data.json \
  --config ./zktls/prover/data/attestation_config.json \
  --output-dir pico_out
```
