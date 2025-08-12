use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use clap::Parser;
use serde_json::Value;
use tuf::key::{PrivateKey, PublicKey};

/// Extract the public key from a TUF-compatible private key JSON file.
#[derive(Parser, Debug)]
#[command(about = "Extract public key from a TUF Ed25519 private key JSON file.")]
struct Args {
    /// Path to the private key file
    #[arg(short, long)]
    key: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file = File::open(&args.key)?;
    let reader = BufReader::new(file);

    // Parse the private key
    let private_key: PrivateKey = serde_json::from_reader(reader)?;

    // Get public key
    let public_key: PublicKey = private_key.public();

    // Serialize and print
    let public_json = serde_json::to_string_pretty(&public_key)?;
    println!("{}", public_json);

    Ok(())
}

