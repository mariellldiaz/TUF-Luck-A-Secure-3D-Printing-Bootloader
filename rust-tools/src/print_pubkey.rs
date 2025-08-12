use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::Deserialize;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Path to the private key file
    #[arg(short, long)]
    key: PathBuf,
}

#[derive(Debug, Deserialize)]
struct KeyVal {
    public: String,
    private: String,
}

#[derive(Debug, Deserialize)]
struct KeyFile {
    keytype: String,
    scheme: String,
    keyval: KeyVal,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file = File::open(args.key)?;
    let reader = BufReader::new(file);
    let key_data: KeyFile = serde_json::from_reader(reader)?;

    println!("Public Key: {}", key_data.keyval.public);
    Ok(())
}
