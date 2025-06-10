use pico_sdk::{client::KoalaBearProverClient, init_logger};
use std::fs;

fn main() {
    // Initialize logger
    init_logger();

    // Load the ELF file
    let elf = load_elf("../app/elf/riscv32im-pico-zkvm-elf");

    println!("elf length: {}", elf.len());

    // Initialize a client for full proving with VK verification
    // using STARK on KoalaBear.
    let client = KoalaBearProverClient::new(&elf);

    // Initialize new stdin
    let mut stdin_builder = client.new_stdin_builder();

    let verifying_key = fs::read_to_string("./verifying_k256.key").unwrap();
    let bytes = bincode::serialize(&verifying_key).expect("failed to serialize");
    stdin_builder.write_slice(&bytes);
    println!("key length: {}", bytes.len());

    let verifying_data = fs::read_to_string("./data/bench16.json").unwrap();
    let bytes = bincode::serialize(&verifying_data).expect("failed to serialize");
    stdin_builder.write_slice(&bytes);
    println!("data length: {}", bytes.len());

    // Generate proof
    let proof = client
        .prove_fast(stdin_builder)
        .expect("Failed to generate proof");

    // Decodes public values from the proof's public value stream.
    let public_buffer = proof.pv_stream.unwrap();
    // let public_values = PublicValuesStruct::abi_decode(&public_buffer, true).unwrap();

    // Verify the public values
    // verify_public_values(&public_values);
}

/// Verifies that the computed Fibonacci values match the public values.
// fn verify_public_values(public_values: &PublicValuesStruct) {}

/// Loads an ELF file from the specified path.
pub fn load_elf(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        panic!("Failed to load ELF file from {}: {}", path, err);
    })
}
