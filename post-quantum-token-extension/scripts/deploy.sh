#!/bin/bash

# Build the program
cargo build-bpf

# Deploy the program
solana program deploy target/deploy/post_quantum_extension.so
