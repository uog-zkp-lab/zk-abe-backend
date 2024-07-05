# zk-abe-backend

A backend server including Key Generating Server (KGS) for zk-ABE (Zero-Knowledge Attribute-Based Encryption) system.

## Overview

This project implements a backend server for a Zero-Knowledge Attribute-Based Encryption system. It includes a Key Generating Server (KGS) that handles the generation of keys required for the encryption and decryption processes.

Current structure:

```shell
> tree . -f -L 3
.
├── ./Cargo.lock
├── ./Cargo.toml
├── ./README.md
├── ./rust-toolchain.toml
├── ./src
│   ├── ./src/api               --> the apis we need for frontend to interact
│   │   ├── ./src/api/routes
│   │   └── ./src/api/utils
│   ├── ./src/main.rs           --> the main entry point of backend
│   └── ./src/zkvm              --> zkvm to generate proof 
│       ├── ./src/zkvm/host
│       └── ./src/zkvm/methods
```

## Features

- Key Generating Server (KGS) for zk-ABE
- REST API endpoints for key management
- Built with Rust for performance and safety
- Asynchronous handling using Tokio and Warp

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/zk-abe-backend.git
   cd zk-abe-backend

   ```
2. **Install Rust**:
   If you haven't already, install Rust using rustup
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env

   ```
3. **Build**:
   ```bash
   cargo build

   ```
4. **Run the server**:
   ```bash
   cargo run

   ```

## Dependencies

1. Warp: A super-easy, composable, web server framework for warp speeds.
2. Serde: A framework for serializing and deserializing Rust data structures efficiently and generically.
3. Tokio: An asynchronous runtime for the Rust programming language.
4. Rabe: A library for attribute-based encryption schemes.
