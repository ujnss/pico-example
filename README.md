# pico-example


fibonacci

# Build program in app folder
cd app
RUST_LOG=info cargo pico build    

# Prove in prover folder
cd prover
RUST_LOG=info cargo pico prove --input "0x0A000000" --fast # input n = 10

# Prove in prover folder
cd prover
RUST_LOG=info cargo run --release 

