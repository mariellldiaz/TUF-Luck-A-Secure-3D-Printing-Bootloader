# TUF Firmware Verification

A complete demonstration of firmware signing & verification using **The Update Framework (TUF)** principles.

## Features
- Rust-based tools for:
  - Ed25519 key generation ('generate_key.rs')
  - Public key extraction ('extract_pubkey.rs')
  - Firmware signing ('sign_firmware.rs')
  - Metadata signing ('sign_metadata.rs')
  - Public key printing ('print_pubkey.rs')
  - Test key validation ('test_key.rs')
  - TUF repository setup ('setup_tuf_repo.rs')

- C-based verification (Monocypher + cJSON):
  - SHA-256 hash check
  - Ed25519 signature verification
  - JSON metadata parsing ('targets.json', 'root.json')
  - Ready for STM32 porting

## Workflow
1. **Generate keys** with 'generate_key.rs'
2. **Sign firmware** to create '.sig' and '.hash'
3. **Create metadata** with 'sign_metadata.rs'
4. **Verify firmware in C** using Monocypher

## Quick Start

##Rust-tools
'''bash
cd rust-tools
cargo run --bin generate_key
cargo run --bin sign_firmware firmware.bin private_key.json

## C verification
cd c-verification
make
./verify test-data/firmware.bin test-data/firmware.sig test-data/firmware.hash
