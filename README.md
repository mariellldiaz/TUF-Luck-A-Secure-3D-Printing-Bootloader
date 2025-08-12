# TUF-Luck-:-A-Secure-3D-Printing-Bootloader
REU 2025 Project at University of Delaware under Dr. Nektarios

This project implements a TUF firmware verification from Rust-TUF to embedded C-TUF. It demonstrates firmware signing and verification using TUF principles.

## Overview
- **Rust tools** for key generation, firmware signing, and TUF metadata creation.
- **C implementation** for verifying firmware authenticity.
- Uses **Ed25519** [Monocypher] (https://monocypher.org/) and **SHA-256** hashing.
- **Targets metadata parsing** using [cJSON] (https://github.com/DaveGamble/cJSON).
- Desgined for STM32 embedded use, tested first on Ubuntu.

## Structure
- 'rust-tools/' → All Rust-based for key generation, signing, and metadata creation.
- 'c-verification/' → C-based firmware verification logic, ready for embedded porting.
- 'docs/' → Diagrams, figures, and workflow documentation.

## Rust-tools
1. **Generate keys** in Rust ('generate_key.rs').
2. **Sign firmware** ('sign_firmware.rs' → '.sig' file).
3. **Generate metadata** ('sign_metadata.rs').
4. **Verify in C** with Monocypher and cJSON.

## Build
'''bash
make
# Generate key
cargo run --bin generate_key

# Sign firmware
cargo run --bin sign_firmware firmware.bin private_key.json

# Verify firmware in C
cd c-verification
make
./verify firmware.bin firmware.sig firmware.hash
