# Twine Solana Consensus Proof Verifier

A Rust program for verifying Solana consensus proofs produced by SP1 (Succinct) ZKVM.

## Overview

This project provides a tool for verifying Solana consensus proofs using the SP1 zero-knowledge proof system. The verifier is designed to work with proofs generated by the Twine Solana consensus prover.

## Requirements

- Rust (latest stable version recommended)
- Cargo package manager
- SP1 SDK (version 4.1.3)

## Installation

1. Clone this repository:

   ```bash
   git clone <repository-url>
   cd twine-solana-consensus-proof-verifier
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

The program reads:

- A SP1 proof from `data/groth16_proof.json` (default)
- A verification key from `data/vkey.json` (default)

Run the verifier:

```bash
cargo run --release
```

Or specify custom paths:

```bash
cargo run --release -- --proof-path path/to/proof.json --vkey-path path/to/vkey.json
```

## Solana Consensus Data in the Proof

The SP1 proof contains the following Solana consensus data:

1. **Slot Data**: Contains information for each slot in the consensus window, including:

   - Parent bank hash
   - Account delta root (Merkle root of all account changes)
   - Number of signatures processed
   - Blockhash
   - Bank hash

2. **Account Delta Proofs**: Merkle proofs for account state changes, including:

   - Account public keys
   - Account data (program state)
   - Account owners
   - Merkle paths proving inclusion in the account delta root

3. **Vote Information**: Cryptographically signed votes from validators, including:

   - Validator public keys
   - Vote messages (slot, hash, etc.)
   - Ed25519 signatures

4. **Tower Sync Data**: Validator tower state synchronization information with:

   - Validator public keys
   - Tower state messages
   - Ed25519 signatures

5. **Deposit Messages**: Cross-chain deposit information embedded in account data

## Verified Constraints

The proof verifies these key constraints:

1. **Slot Data Completeness**: All slots from first_slot to last_slot have corresponding data.

2. **Bank Hash Chain Integrity**: The bank hash chain is valid, with each slot's parent bank hash matching the previous slot's calculated bank hash.

3. **Deposit Messages Presence**: Deposit messages exist in account data for cross-chain operations.

4. **Account Delta Proof Verification**: All account changes are verified against their Merkle roots.

5. **Signature Verification**: All validator votes and tower sync messages have valid ed25519 signatures.

## Implementation

The implementation uses the SP1 SDK's `ProverClient.verify()` method to cryptographically verify the proof against the verification key, ensuring all constraints are satisfied.

## License

MIT
