use hex;
use pico_sdk::{client::KoalaBearProverClient, init_logger};
use std::{fs, path::PathBuf};

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

    let attestation_data = fs::read_to_string("./data/attestation_data.json").unwrap();
    let bytes = bincode::serialize(&attestation_data).expect("failed to serialize");
    println!("data len: {} 0x{}", bytes.len(), hex::encode(&bytes));
    stdin_builder.write_slice(&bytes);

    // Generate proof
    let (riscv_proof, embed_proof) = client
        .prove(stdin_builder)
        .expect("Failed to generate proof");
    let output_dir = PathBuf::from(&"./outputs");

    client
        .write_onchain_data(output_dir, &riscv_proof, &embed_proof)
        .unwrap();

    // Decodes public values from the proof's public value stream.
    let public_buffer = riscv_proof.pv_stream.unwrap();
}

/// Loads an ELF file from the specified path.
pub fn load_elf(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        panic!("Failed to load ELF file from {}: {}", path, err);
    })
}
