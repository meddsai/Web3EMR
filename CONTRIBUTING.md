# Contributing to Web3EMR

Thank you for your interest in contributing to Web3EMR! This document outlines the process for setting up your development environment, making contributions, and troubleshooting common issues.

## Table of Contents
- [Prerequisites](#prerequisites)
- [Development Environment Setup](#development-environment-setup)
- [Project Structure](#project-structure)
- [Building the Project](#building-the-project)
- [Running Tests](#running-tests)
- [Making Contributions](#making-contributions)
- [Code Style](#code-style)
- [Troubleshooting](#troubleshooting)
- [Getting Help](#getting-help)

## Prerequisites

Before you begin, ensure you have the following installed on your system:

- **Rust**: Latest stable version (1.68+)
- **Node.js**: Version 18+
- **Docker** (for running a local node)
- **Git**
- **Rust toolchain**: Includes `rustup`, `cargo`, and `rustc`
- **System dependencies**:
  ```bash
  # For Ubuntu/Debian
  sudo apt update
  sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    cmake \
    clang \
    libclang-dev \
    protobuf-compiler
  ```

## Development Environment Setup

### 1. Install Rust

If you don't have Rust installed, install it using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### 2. Install ink! Toolchain

Install the required Rust toolchains and components:

```bash
# Install stable toolchain
rustup toolchain install stable
rustup default stable

# Add wasm target
rustup target add wasm32-unknown-unknown --toolchain stable
```

### 3. Install cargo-contract

Install the `cargo-contract` CLI for ink! smart contract development:

```bash
cargo install --force --locked cargo-contract
```

### 4. Install Substrate Contracts Node

To deploy and test your contracts locally, install the Substrate Contracts Node:

```bash
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --tag v0.31.0 --force --locked
```

## Project Structure

```
web3emr/
├── contracts/          # Smart contracts
├── frontend/           # Frontend application
├── node/               # Substrate node
├── pallets/            # Custom pallets
├── scripts/            # Utility scripts
├── tests/              # Integration tests
└── CONTRIBUTING.md     # This file
```

## Building the Project

### Build Contracts

To build all contracts:

```bash
cd contracts/
cargo contract build
```

### Run a Local Node

Start a local development node:

```bash
substrate-contracts-node --dev
```

## Running Tests

### Unit Tests

```bash
cargo test
```

### Integration Tests

```bash
cargo test --features=try-runtime
```

## Making Contributions

1. **Fork** the repository
2. Create a **feature branch**: `git checkout -b feature/amazing-feature`
3. **Commit** your changes: `git commit -m 'Add some amazing feature'`
4. **Push** to the branch: `git push origin feature/amazing-feature`
5. Open a **Pull Request**

## Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for consistent formatting:
  ```bash
  cargo fmt --all
  ```
- Run `clippy` for linting:
  ```bash
  cargo clippy -- -D warnings
  ```

## Troubleshooting

### Common Issues

#### 1. Rust Toolchain Issues

**Problem**: Build fails with Rust version errors.
**Solution**: Ensure you're using the correct Rust version:
```bash
rustup default stable
rustup target add wasm32-unknown-unknown
```

#### 2. cargo-contract Installation Issues

**Problem**: `cargo-contract` fails to install.
**Solution**: Try installing with `--force` and `--locked` flags:
```bash
cargo install --force --locked cargo-contract
```

#### 3. Node.js Version Issues

**Problem**: Frontend build fails.
**Solution**: Ensure you're using Node.js 18+:
```bash
node --version  # Should be 18.x or higher
```

#### 4. Substrate Node Connection Issues

**Problem**: Cannot connect to local node.
**Solution**: Make sure the node is running:
```bash
substrate-contracts-node --dev
```

## Getting Help

- Check the [ink! documentation](https://use.ink/)
- Join our [Discord/Slack] channel
- Open an [issue](https://github.com/your-org/web3emr/issues)

## License

By contributing to Web3EMR, you agree that your contributions will be licensed under the [MIT License](LICENSE).
