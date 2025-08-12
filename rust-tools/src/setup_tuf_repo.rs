use tuf::crypto::{Ed25519PrivateKey, HashAlgorithm, KeyType, PrivateKey};
use tuf::interchange::Json;
use tuf::metadata::{MetadataPath, MetadataVersion, RootMetadataBuilder, Role, SnapshotMetadataBuilder, TargetDescription, TargetsMetadataBuilder, TimestampMetadataBuilder};
use tuf::repo_builder::RepoBuilder;
use tuf::repository::{FilesystemRepositoryBuilder, FileSystemRepository};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create repo structure
    let metadata_path = PathBuf::from("tuf-repo/metadata");
    let targets_path = PathBuf::from("tuf-repo/targets");
    fs::create_dir_all(&metadata_path)?;
    fs::create_dir_all(&targets_path)?;

    // Copy firmware into targets
    fs::copy("firmware/firmware.bin", targets_path.join("firmware.bin"))?;

    // Generate one key for each role
    let root_key = Ed25519PrivateKey::from_pkcs8(&Ed25519PrivateKey::gen_pkcs8())?;
    let snapshot_key = Ed25519PrivateKey::from_pkcs8(&Ed25519PrivateKey::gen_pkcs8())?;
    let targets_key = Ed25519PrivateKey::from_pkcs8(&Ed25519PrivateKey::gen_pkcs8())?;
    let timestamp_key = Ed25519PrivateKey::from_pkcs8(&Ed25519PrivateKey::gen_pkcs8())?;

    let root_keys = vec![root_key.public().clone()];
    let snapshot_keys = vec![snapshot_key.public().clone()];
    let targets_keys = vec![targets_key.public().clone()];
    let timestamp_keys = vec![timestamp_key.public().clone()];

    // Save public root key for STM32/C use
    fs::write("root-public-key.json", serde_json::to_string_pretty(&root_keys[0])?)?;

    // Create local repo
    let mut repo = RepoBuilder::create(
        FileSystemRepositoryBuilder::<Json>::new(metadata_path).build(),
        FileSystemRepositoryBuilder::<Json>::new(targets_path).build(),
    )
    .trusted_root_keys(root_keys.clone(), 1)?
    .trusted_targets_keys(targets_keys.clone(), 1)?
    .trusted_snapshot_keys(snapshot_keys.clone(), 1)?
    .trusted_timestamp_keys(timestamp_keys.clone(), 1)?
    .signing_root_keys(vec![&root_key])?
    .signing_targets_keys(vec![&targets_key])?
    .signing_snapshot_keys(vec![&snapshot_key])?
    .signing_timestamp_keys(vec![&timestamp_key])?;

    repo.commit()?;
    println!("✅ Metadata generated in tuf-repo/");
    Ok(())
}
