# Lattice-based-Schnorr
# Post-Quantum Token Extension for Solana

This repository implements a post-quantum token extension for Solana's SPL Token 2022 program. It adds support for post-quantum signature verification (e.g lattice-based Schnorr signatures) to SPL tokens and NFTs.

## Features
- Post-quantum key registration.
- Hybrid Ed25519 and post-quantum signature verification.
- Custom token extension for SPL Token 2022.

## Setup
1. Install the Solana Tool Suite.
2. Clone this repository.
3. Build and deploy the program.

## Usage
- Register a post-quantum public key with your token account.
- Sign transactions with both Ed25519 and post-quantum keys.
- The token extension verifies post-quantum signatures during transfers.

## License
MIT
