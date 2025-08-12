// sign_firmware.rs
use std::fs;
use ring::digest::{digest, SHA256};
use serde::{Deserialize};
use base64::{engine::general_purpose, Engine as _};
use ring::signature::{Ed25519KeyPair, KeyPair};

#[derive(Debug, Deserialize)]
struct KeyVal {
    public: String,
    private: String,
}

#[derive(Debug, Deserialize)]
struct KeyFile {
    keyval: KeyVal,
}

fn main() {
    let key_path = "keystore/ed25519_root.json";
    let firmware_path = "firmware.bin";

    let key_data = fs::read_to_string(key_path).unwrap();
    let key: KeyFile = serde_json::from_str(&key_data).unwrap();

    let priv_bytes = general_purpose::STANDARD.decode(&key.keyval.private).unwrap();
    let keypair = Ed25519KeyPair::from_seed_unchecked(&priv_bytes).unwrap();

    let firmware = fs::read(firmware_path).unwrap();
    let hash = digest(&SHA256, &firmware);

    let signature = keypair.sign(hash.as_ref());

    fs::write("firmware.sig", signature.as_ref()).unwrap();
    fs::write("firmware.hash", hex::encode(hash.as_ref())).unwrap();
    fs::write("firmware.pub", key.keyval.public).unwrap();

    println!("✓ Signed firmware.bin successfully");
}
