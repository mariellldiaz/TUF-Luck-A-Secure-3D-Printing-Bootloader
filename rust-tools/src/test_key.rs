use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde_json::Value;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    key: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file = File::open(args.key)?;
    let reader = BufReader::new(file);

    let key_json: Value = serde_json::from_reader(reader)?;
    println!("{:#}", key_json);

    Ok(())
}
