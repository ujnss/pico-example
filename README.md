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

RUST_LOG=info cargo pico prove --input "0x0A000000" --fast --elf ../app/elf/riscv32im-pico-zkvm-elf

RUST_LOG=info cargo pico prove --input "0x0A000000" --fast # input n = 10
--elf --evm --setup --input --output

RUST_LOG=info cargo pico prove --elf ../app/elf/riscv32im-pico-zkvm-elf \
--input ""  
