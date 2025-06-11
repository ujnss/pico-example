#![no_main]
pico_sdk::entrypoint!(main);

use pico_sdk::io::{commit, read_as};
use zktls_att_verification::attestation_data::AttestationData;

pub fn main() {
    let attestation_data: String = read_as();
    println!("attestation_data {:?}", attestation_data);

    let attestation_data: AttestationData = serde_json::from_str(&attestation_data).unwrap();
    let (messages, records) = attestation_data.verify().unwrap();

    let mut json_paths = vec![];
    json_paths.push("$.data.spotVol");
    json_paths.push("$.data.spotNeed");
    let json_value = messages[0].get_json_values(&json_paths);
    println!("json value:{:?}", json_value);
    println!("records: {}", records);

    commit(&records);
}
