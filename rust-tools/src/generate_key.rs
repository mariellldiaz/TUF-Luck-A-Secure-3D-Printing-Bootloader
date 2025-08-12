use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use ed25519_dalek::SigningKey;
use pkcs8::DecodePrivateKey;
use tuf::crypto::{HashAlgorithm, PrivateKey, SignatureScheme};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "sign_metadata")]
struct Args {
    /// Path to PKCS#8 private key
    #[arg(short, long)]
    pub p: PathBuf,

    /// Path to input file to sign
    #[arg(short, long)]
    pub f: PathBuf,

    /// Path to output signature file
    #[arg(short, long)]
    pub o: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Read key and parse PKCS#8
    let key_bytes = fs::read(&args.p).context("failed to read key file")?;
    let signing_key = SigningKey::from_pkcs8_der(&key_bytes)
        .context("Could not parse key as PKCS#8v2 Ed25519")?;

    // 2. Convert to TUF PrivateKey
    let private_key = PrivateKey::from_ed25519(signing_key.to_bytes().as_slice().into())?;

    // 3. Read firmware file
    let firmware_bytes = fs::read(&args.f).context("failed to read firmware file")?;

    // 4. Sign the firmware
    let signature = private_key.sign(&firmware_bytes, SignatureScheme::Ed25519)?;

    // 5. Output the signature as JSON
    let json = serde_json::to_string_pretty(&signature)?;
    fs::write(&args.o, json)?;

    println!("✅ Signature saved to {}", args.o.display());
    Ok(())
}

