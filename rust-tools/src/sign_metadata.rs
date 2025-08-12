use std::fs;
use std::path::PathBuf;
use std::io::Write;

use clap::Parser;
use ed25519_dalek::{Signer, SigningKey, Signature, pkcs8::DecodePrivateKey};

/// Sign metadata with Ed25519 key.
#[derive(Parser, Debug)]
#[command(name = "sign_metadata")]
struct Args {
    /// Path to PKCS#8 private key (.pk8)
    #[arg(short = 'p', long = "key-path")]
    key_path: PathBuf,

    /// Path to firmware or metadata file to sign
    #[arg(short = 'f', long = "file")]
    file: PathBuf,

    /// Path to write the detached signature
    #[arg(short = 'o', long = "out")]
    out: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Load the private key from PKCS#8 file
    let key_data = fs::read(&args.key_path)?;
    let signing_key = SigningKey::from_pkcs8_der(&key_data)
        .map_err(|e| format!("Failed to load signing key: {}", e))?;

    // Load firmware or metadata to sign
    let data = fs::read(&args.file)?;

    // Create signature
    let signature: Signature = signing_key.sign(&data);

    // Write signature to output file
    let mut sig_file = fs::File::create(&args.out)?;
    sig_file.write_all(&signature.to_bytes())?;

    println!("✅ Signature saved to {:?}", args.out);
    Ok(())
}

