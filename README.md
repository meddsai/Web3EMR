# Web3EMR dApp

Web3EMR is a decentralized Electronic Medical Record (EMR) system built on a Substrate-based blockchain, transforming RustEMR into a Web3-native application. It empowers patients to control their medical records securely, enables providers to access records with permission, and ensures interoperability via Polkadot's ecosystem. The dApp uses custom Substrate pallets for on-chain logic, IPFS for off-chain storage, and a React frontend with Polkadot.js for user interaction.

## Features

- **Patient-Centric Control**: Patients manage EMR access using cryptographic wallets (e.g., Polkadot.js).
- **Secure Data Storage**: EMRs are encrypted and stored on IPFS, with hashes on-chain for integrity.
- **Access Control**: Fine-grained permissions via Substrate pallets, allowing patients to grant/revoke provider access.
- **Interoperability**: Built on a Substrate-based blockchain, supporting Polkadot's XCM for cross-chain data sharing.
- **Regulatory Compliance**: Designed for HIPAA/GDPR compliance through encryption and audit trails.
- **Immutable Audit Trail**: All EMR interactions are logged on-chain for transparency.

## Architecture

- **Blockchain Layer**: Custom Substrate pallets for EMR storage, access control, and identity management, running on a Substrate-based chain.
- **Off-Chain Storage**: Encrypted EMRs stored on IPFS, with metadata/hashes on-chain.
- **Frontend**: React-based UI with Polkadot.js for wallet integration and blockchain interaction.
- **Identity**: Integrates with DID or a custom decentralized identity system.

## Prerequisites

- [Rust](https://www.rust-lang.org/): Stable toolchain (`rustup update stable`).
- [Substrate](https://substrate.dev/): Substrate Node Template.
- [IPFS](https://ipfs.io/): For off-chain storage.
- [Node.js](https://nodejs.org/): For the React frontend (>=16.0.0).
- [Polkadot.js](https://polkadot.js.org/extension/): Browser extension for wallet management.
- Docker (optional): For running IPFS or blockchain nodes.

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/meddsai/Web3EMR.git
cd Web3EMR
```

### 2. Set Up the Substrate Blockchain

```bash
git clone https://github.com/substrate-developer-hub/substrate-node-template
cd substrate-node-template
cargo build --release
./target/release/node-template --dev
```

#### Option 2: Custom Substrate Chain

```bash
git clone https://github.com/substrate-developer-hub/substrate-node-template
cd substrate-node-template
cargo build --release
./target/release/node-template --dev
```

### 3. Install IPFS

```bash
wget https://dist.ipfs.io/go-ipfs/v0.10.0/go-ipfs_v0.10.0_linux-amd64.tar.gz
tar -xvzf go-ipfs_v0.10.0_linux-amd64.tar.gz
cd go-ipfs
./install.sh
ipfs init
ipfs daemon
```

### 4. Set Up the Backend

```bash
cd backend
cargo build --release
```

Copy the `emr_pallet.rs` to the Substrate node's pallets directory and update the runtime configuration (`runtime/src/lib.rs`).

### 5. Set Up the Frontend

```bash
cd frontend
npm install
npm start
```

### 6. Configure Polkadot.js

Install the [Polkadot.js browser extension](https://polkadot.js.org/extension/) and create/import accounts for testing (e.g., Alice, Bob).

## Usage

### Run the Blockchain:

Start the Substrate node:

```bash
./target/release/mandala-node --dev
```

Connect to the node via WebSocket (e.g., `ws://127.0.0.1:9944`).

### Store an EMR:

1. Access the frontend at http://localhost:3000
2. Connect your Polkadot.js wallet
3. Upload an EMR (JSON format), which is encrypted and stored on IPFS
4. Submit the EMR hash to the blockchain using the `store_emr` pallet call

### Manage Access:

- Grant/revoke provider access via the frontend, using the `grant_access`/`revoke_access` pallet calls
- Providers retrieve EMRs by fetching the IPFS hash from the blockchain and decrypting with authorized keys

### View Audit Trail:

Check the blockchain's event log for EMR interactions (e.g., `EMRStored`, `AccessGranted`).

## Example Workflow

### Patient:
1. Logs in with Polkadot.js wallet
2. Uploads EMR (e.g., `{"name": "John Doe", "record": "..."}`)
3. EMR is encrypted, stored on IPFS, and its hash is recorded on-chain

### Provider:
1. Requests access via the dApp
2. Patient grants access, triggering an on-chain `AccessGranted` event
3. Provider retrieves the EMR from IPFS using the on-chain hash

### Audit:
- All actions are logged on-chain, viewable via [Polkadot.js Apps](https://polkadot.js.org/apps)

## Development

### Key Components

- **EMR Pallet**: Handles EMR storage, access control, and audit logging (see `backend/pallets/emr_pallet.rs`)
- **IPFS Integration**: Encrypts and stores EMRs off-chain (see `backend/ipfs_storage.rs`)
- **Frontend**: React app with Polkadot.js for blockchain interaction (see `frontend/src/emr_dapp.js`)

### Adding New Features

- Extend the EMR pallet for additional functionality (e.g., clinical trial data, insurance integration)
- Enhance the frontend with visualizations or multi-language support
- Integrate with DID for advanced identity management

## Testing

### Local Testing:

1. Run a local Substrate node with `--dev` flag
2. Use [Polkadot.js Apps](https://polkadot.js.org/apps) to interact with the EMR pallet
3. Test IPFS storage/retrieval with sample EMRs

### Testnet:

- Deploy to Substrate's testnet (contact [Substrate](https://www.substrate.io) for access)
- Simulate patient/provider interactions with multiple wallets

## Deployment

### Mainnet:

1. Deploy the Substrate chain to Substrate's mainnet or a custom Polkadot parachain slot
2. Configure IPFS nodes for production-grade storage
3. Host the frontend on a decentralized platform (e.g., Fleek, IPFS)

### Compliance:

- Ensure AES-256 encryption for EMRs
- Implement zero-knowledge proofs (ZKPs) for private transactions
- Maintain audit logs for HIPAA/GDPR compliance

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/xyz`)
3. Commit your changes (`git commit -m 'Add XYZ feature'`)
4. Push to the branch (`git push origin feature/xyz`)
5. Open a pull request

Please follow the Code of Conduct and report issues via GitHub.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Resources

- [Substrate Developer Hub](https://substrate.dev)
- [Polkadot.js](https://polkadot.js.org)
- [IPFS](https://ipfs.io)
- [Original RustEMR](https://github.com/meddsai/RustEMR)

## Contact

For questions or support, open an issue or contact the maintainers at meddsai@example.com.

---

Built with ❤️ for decentralized healthcare.
