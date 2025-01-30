# Post-Quantum Token Extension STILL IN PROGRESS

A Solana program implementing a lattice-based Schnorr signature scheme for post-quantum security.

## Overview

This project implements a post-quantum secure extension for Solana tokens using lattice-based cryptography. It provides a Schnorr signature scheme that is resistant to quantum computer attacks.

## Features

- Lattice-based cryptography implementation
- Post-quantum secure Schnorr signatures
- Solana program integration
- Compatible with existing token programs

## Prerequisites

- Rust 1.68.0 or later
- Solana CLI 1.14.17 or later
- Node.js 14.0.0 or later (for tests)

## Building

bash
cargo build-bpf

## Testing

bash
cargo test-bpf

## Security

This implementation is experimental and has not been audited. Use at your own risk.

## License

MIT License
