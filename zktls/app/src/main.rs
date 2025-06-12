#![no_main]
pico_sdk::entrypoint!(main);

use pico_sdk::io::{commit, read_as};
use zktls_att_verification::attestation_data::{AttestationConfig, AttestationData};

pub fn main() {
    let attestation_data: String = read_as();
    // println!("attestation_data {:?}", attestation_data);

    // Predefined config
    let attestation_config: AttestationConfig = AttestationConfig {
        attestor_addr: "0xe02bd7a6c8aa401189aebb5bad755c2610940a73".to_string(),
        url: vec!["https://www.bitget.com/v1/mix/vip/need".to_string()],
    };

    // Verify
    let attestation_data: AttestationData = serde_json::from_str(&attestation_data).unwrap();
    let messages = attestation_data.verify(&attestation_config).unwrap();

    // Do something
    let mut json_paths = vec![];
    json_paths.push("$.data.spotVol");
    let json_value = messages[0].get_json_values(&json_paths);
    println!("json value:{:?}", json_value);

    commit(&attestation_data.public_data);
}
