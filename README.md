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

[OR]

```sh
RUST_LOG=info ./target/release/zktls-prover --elf ./zktls/app/elf/riscv32im-pico-zkvm-
elf --input ./zktls/prover/data/attestation_data.json --output-dir pico_out
```
