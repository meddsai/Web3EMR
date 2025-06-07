# Web3EMR dApp

[![Rust](https://img.shields.io/badge/Rust-1.70.0-orange)](https://www.rust-lang.org/)
[![Substrate](https://img.shields.io/badge/Substrate-4.0.0-5929FF)](https://substrate.dev/)
[![StorageHub](https://img.shields.io/badge/StorageHub-0.9.0-FF6B6B)](https://github.com/Moonsong-Labs/storage-hub)
[![KILT](https://img.shields.io/badge/KILT_SDK-0.27.0-FF6B6B)](https://www.kilt.io/)
[![Node.js](https://img.shields.io/badge/Node.js-18.x-green)](https://nodejs.org/)
[![Polkadot.js](https://img.shields.io/badge/Polkadot.js-10.9.1-E6007A)](https://polkadot.js.org/)
[![Chainlink](https://img.shields.io/badge/Chainlink-2.0.0-375BD2)](https://chain.link/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
![Status](https://img.shields.io/badge/Status-Experimental-orange)

## Table of Contents

- [Description](#description)
- [Features](#features)
- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
- [Deployment](#deployment)
- [Security Considerations](#security-considerations)
- [Real-World Integration](#real-world-integration)
- [Compliance](#compliance)

## Description
Web3EMR is a decentralized Electronic Medical Record (EMR) with embedded AI/ML system built on a Substrate-based blockchain, transforming EMR into a Web3-native application. It empowers patients to control their medical records, ensures secure access for verified healthcare providers, and validates credentials (e.g., medical licenses, hospital accreditations) through real-world organizations using KILT Protocol for Decentralized Identifiers (DIDs). 

Leveraging a custom Substrate chain or a Substrate-based parachain in the Polkadot ecosystem, Web3EMR uses StorageHub for off-chain storage, aligning with Web3 principles of decentralization, transparency, and interoperability while meeting healthcare regulatory requirements (e.g., HIPAA, GDPR).

## Features

- **Patient-Centric Control**: Patients manage EMR access using cryptographic wallets (e.g., Polkadot.js) and KILT DIDs.
- **Secure Data Storage**: EMRs and credentials are encrypted and stored on StorageHub, with hashes recorded on-chain for integrity.
- **Attestation System**: Verifies patients, healthcare personnel, and entities (hospitals, clinics) through a Substrate-based attestation pallet, ensuring only authorized participants interact with EMRs.
- **Credential Validation**: Links medical licenses, nursing certifications, and hospital accreditations to KILT DIDs, validated by real-world organizations (e.g., medical boards, accreditation bodies).
- **Access Control**: Fine-grained permissions allow patients to grant/revoke provider access, with checks for valid credentials.
- **Revocation Support**: Handles credential revocation (e.g., expired or suspended licenses) with on-chain updates.
- **Interoperability**: Integrates with Polkadot's XCM for cross-chain data sharing, enabling multi-institutional ecosystems.
- **Regulatory Compliance**: Ensures HIPAA/GDPR compliance through AES-256 encryption, zero-knowledge proofs (ZKPs), and immutable audit trails.
- **Immutable Audit Trail**: Logs all EMR and attestation interactions on-chain for transparency and compliance.
- **Data Analytics**: Privacy-preserving analytics on medical data with patient consent, enabling population health insights while maintaining individual privacy.
- **AI/ML Integration**: Machine learning models for predictive diagnostics, treatment recommendations, and clinical decision support based on anonymized or consented patient data.

## Architecture

- **Blockchain Layer**: Custom Substrate pallets (`emr_pallet`, `attestation_pallet`) for EMR management, access control, attestation, and credential validation, running on a Substrate-based blockchain.
- **Off-Chain Storage**: Encrypted EMRs and credential documents (e.g., license PDFs) stored on StorageHub, with hashes on-chain.
- **Identity System**: Integrates with KILT Protocol for secure, verified DIDs, supporting credential attestation.
- **Analytics & AI Layer**: Privacy-preserving data analytics and machine learning models for medical insights, predictive diagnostics, and clinical decision support.
- **Frontend**: React-based UI with Polkadot.js for wallet integration and blockchain interaction.
- **Real-World Integration**: Oracles (e.g., Chainlink on Polkadot) or a healthcare DAO validate credentials from trusted organizations (e.g., AMA, Joint Commission).

## Prerequisites

- [Rust](https://www.rust-lang.org/): Stable toolchain (`rustup update stable`).
- [Substrate](https://substrate.dev/): Substrate Node Template ([repository](https://github.com/substrate-developer-hub/substrate-node-template)).
- [StorageHub](https://github.com/Moonsong-Labs/storage-hub): For off-chain storage.
- [Node.js](https://nodejs.org/): For the React frontend (>=16.0.0).
- [Polkadot.js](https://polkadot.js.org/extension/): Browser extension for wallet management.
- [KILT SDK](https://github.com/KILTprotocol/kilt-sdk-js): For DID management.
- Docker (optional): For running StorageHub or blockchain nodes.
- Chainlink (optional): For oracle-based credential validation.

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/meddsai/Web3EMR.git
cd Web3EMR
```

### 2. Set Up the Substrate Blockchain

Use the Substrate Node Template to create a custom chain:

```bash
git clone https://github.com/substrate-developer-hub/substrate-node-template
cd substrate-node-template
cargo build --release
./target/release/node-template --dev
```

Alternatively, connect to an existing Substrate-based parachain in the Polkadot ecosystem (e.g., a testnet or mainnet).

### 3. Set Up StorageHub

Clone and run StorageHub:

```bash
git clone https://github.com/Moonsong-Labs/storage-hub
cd storage-hub
cargo build --release
./target/release/storage-hub-node --dev
```

Follow StorageHub's documentation for configuration and network connection.

### 4. Set Up KILT Protocol

Install the KILT SDK for DID management:

```bash
cd backend
npm install @kiltprotocol/sdk-js
```

### 5. Set Up the Backend

Install Rust dependencies and build the pallets:

```bash
cd backend
cargo build --release
```

Add `emr_pallet.rs` and `attestation_pallet.rs` to the Substrate node's pallets directory. Update the runtime configuration (`runtime/src/lib.rs`):

```rust
impl emr_pallet::Config for Runtime {
    type Event = Event;
}

impl attestation_pallet::Config for Runtime {
    type Event = Event;
    type AttestationExpiry = AttestationExpiry;
    type MaxAttestationsPerEntity = MaxAttestationsPerEntity;
}
```

### 6. Set Up the Frontend

Install Node.js dependencies and start the React app:

```bash
cd frontend
npm install
npm start
```

### 7. Configure Polkadot.js and KILT

Install the [Polkadot.js browser extension](https://polkadot.js.org/extension/) and create/import accounts for testing.

Create KILT DIDs for testing entities, patients, and personnel using the KILT SDK. (see https://docs.kilt.io).

## Usage

### 1. Run the Blockchain

Start the Substrate node:

```bash
./target/release/node-template --dev
```

Connect to the node via WebSocket (e.g., `ws://127.0.0.1:9944`).

### 2. Register Entities

Healthcare entities (e.g., hospitals, clinics) register on-chain:

1. Create a KILT DID for the entity
2. Submit the entity's DID, name, and accreditation hash (e.g., Joint Commission certification) via `register_entity`
3. Accreditation is validated off-chain by real-world organizations or oracles

### 3. Attest Personnel and Patients

Healthcare entities attest patients and personnel:

- **Patients**: Link patient KILT DID to entity DID using `attest_patient`
- **Personnel**: Submit personnel KILT DID, entity DID, credential hash (e.g., medical license), and expiry timestamp via `attest_personnel`

Credentials are validated off-chain by real-world organizations (e.g., medical boards) and stored on StorageHub.

### 4. Store an EMR

Patients upload EMRs:

1. Log in with a Polkadot.js wallet and KILT DID
2. Upload an EMR (JSON format), which is encrypted and stored on StorageHub
3. Submit the EMR hash to the blockchain using `store_emr`

### 5. Manage Access

Patients control provider access:

- Grant access with `grant_access`, specifying the provider's KILT DID and EMR hash
- The system verifies the provider's attestation status before allowing access
- Revoke access with `revoke_access`

Providers retrieve EMRs from StorageHub using the on-chain hash, decrypting with authorized keys.

### 6. Revoke Credentials

Authorities revoke credentials (e.g., expired licenses) using `revoke_credential`, updating on-chain status.

### 7. View Audit Trail

Check the blockchain's event log (via Polkadot.js Apps) for EMR and attestation events (e.g., `EMRStored`, `PersonnelAttested`, `CredentialRevoked`).

## Example Workflow

### Healthcare Entity Registration
1. A medical board creates a KILT DID for a hospital
2. The board registers the hospital with an accreditation hash on-chain
3. The hospital's identity is now verifiable on the blockchain

### Credential Attestation
1. The hospital attests a doctor's medical license
2. This links the doctor's KILT DID to their credential hash and expiry date
3. The credential can be verified by any authorized party

### Patient Attestation
1. The hospital attests a patient's registration
2. This links the patient's KILT DID to the hospital
3. The patient's identity is now verifiable within the system

### EMR Management
1. The patient uploads an EMR
2. The EMR is encrypted and stored on StorageHub
3. Only the hash of the EMR is recorded on-chain
4. The patient grants access to the doctor
5. The doctor retrieves the EMR after credential validation

### Audit and Compliance
- All actions are logged on-chain
- This ensures transparency and regulatory compliance
- Audit trails are immutable and verifiable

## Development

### Key Components

- **EMR Pallet**: Manages EMR storage and access control (`backend/pallets/emr_pallet.rs`)
- **Attestation Pallet**: Handles entity, patient, and personnel attestation with credential validation (`backend/pallets/attestation_pallet.rs`)
- **StorageHub Integration**: Encrypts and stores EMRs/credentials off-chain (`backend/storagehub.rs`)
- **Analytics Engine**: Privacy-preserving data analytics with differential privacy guarantees (`backend/analytics/`)
- **AI/ML Models**: Machine learning pipeline for medical data analysis and predictive healthcare (`backend/ml/`)
- **Frontend**: React app with Polkadot.js and KILT SDK for blockchain and DID interaction (`frontend/src`)

### Adding New Features

- Extend pallets for clinical trial data or insurance integration
- Implement ZKPs for private credential verification using KILT's capabilities
- Add multi-language support to the frontend
- Integrate a healthcare DAO for governance using Substrate's democracy pallet
- Develop specialized AI models for disease prediction and early diagnosis
- Implement federated learning for collaborative model training across healthcare entities
- Create privacy-preserving analytics dashboards for population health monitoring
- Build secure APIs for third-party research integration with anonymized data

## Testing

### Local Testing

1. Run a local Substrate node with `--dev` flag
2. Use [Polkadot.js Apps](https://polkadot.js.org/apps) to interact with pallets
3. Test StorageHub storage/retrieval with sample EMRs and credentials
4. Test KILT DID creation and attestation using the KILT SDK

### Testnet

- Deploy to a Substrate-based testnet (e.g., a custom chain or Polkadot parachain)
- Connect to KILT's Peregrine testnet (https://docs.kilt.io)
- Simulate attestation workflows with multiple KILT DIDs

## Deployment

### Mainnet

1. Deploy to a custom Substrate-based mainnet or a Polkadot parachain slot
2. Configure production-grade StorageHub nodes
3. Host the frontend on a decentralized platform (e.g., Fleek)

### Compliance

- Use AES-256 encryption for EMRs and credentials
- Implement ZKPs for private transactions (supported by KILT)
- Maintain audit logs for HIPAA/GDPR compliance

### Real-World Integration

- Partner with medical boards (e.g., AMA) or accreditation bodies (e.g., Joint Commission) for credential validation
- Use oracles (e.g., Chainlink) or a healthcare DAO for off-chain verification

## Security Considerations

- **Encryption**: AES-256 for off-chain data on StorageHub
- **Key Management**: Secure patient/provider keys via Polkadot.js wallets and KILT DIDs
- **Credential Validation**: Ensure real-world organizations attest credentials to prevent fraud
- **Auditability**: Immutable on-chain logs for all interactions
- **Privacy**: Use KILT's ZKPs for sensitive data (e.g., license details)

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/xyz`)
3. Commit changes (`git commit -m "Add XYZ feature"`)
4. Push to the branch (`git push origin feature/xyz`)
5. Open a pull request

Please follow the Code of Conduct and report issues via GitHub.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Resources

- [Product Requirements Document (PRD)](./.taskmaster/templates/PRD.txt)
- [Substrate Developer Hub](https://substrate.dev)
- [KILT Protocol](https://www.kilt.io)
- [StorageHub](https://github.com/Moonsong-Labs/storage-hub)
- [Polkadot.js](https://polkadot.js.org)
- [Chainlink on Polkadot](https://docs.chain.link)

## Contact

For questions or support, open an issue or contact the maintainers at meddsai@example.com.

---

_Empowering decentralized healthcare with trust and transparency._
