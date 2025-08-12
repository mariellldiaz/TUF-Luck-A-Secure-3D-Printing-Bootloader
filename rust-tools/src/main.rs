use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

use tuf::{
    crypto::{Ed25519PrivateKey, HashAlgorithm, KeyType, PrivateKey},
    interchange::Json,
    metadata::{MetadataPath, MetadataVersion, TargetPath},
    repo::{FileSystemRepository, RepoBuilder},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load root key
    let mut root_key_file = File::open("root_key.pk8")?;
    let mut root_key_bytes = Vec::new();
    root_key_file.read_to_end(&mut root_key_bytes)?;
    let root_key = Ed25519PrivateKey::from_pkcs8(&root_key_bytes)?;

    // Set up repo paths
    let metadata_path = PathBuf::from("metadata");
    let targets_path = PathBuf::from("targets");

    // Create repository objects
    let metadata_repo = FileSystemRepository::new(metadata_path.clone());
    let targets_repo = FileSystemRepository::new(targets_path.clone());

    // Create RepoBuilder
    RepoBuilder::create(Box::new(metadata_repo))
        .trusted_root_keys(&[&root_key])
        .initialize_with_expires(
            &targets_repo,
            chrono::Utc::now() + chrono::Duration::days(30),
        )?;

    println!("Initialized new TUF repository.");
    Ok(())
}

