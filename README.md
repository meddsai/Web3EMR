Web3EMR dApp
============

**Web3EMR** is a decentralized Electronic Medical Record (EMR) system built on a Substrate-based blockchain, transforming EMR into a Web3-native application. It empowers patients to control their medical records, ensures secure access for verified healthcare providers, and validates credentials (e.g., medical licenses, hospital accreditations) through real-world organizations. Leveraging any custom Substrate chain, Web3EMR aligns with Web3 principles of decentralization, transparency, and interoperability, while meeting healthcare regulatory requirements (e.g., HIPAA, GDPR).

Features
--------

*   **Patient-Centric Control**: Patients manage EMR access using cryptographic wallets (e.g., Polkadot.js) and Decentralized Identifiers (DIDs).
    
*   **Secure Data Storage**: EMRs are encrypted and stored on IPFS, with hashes recorded on-chain for integrity.
    
*   **Attestation System**: Verifies patients, healthcare personnel, and entities (hospitals, clinics) through a Substrate-based attestation pallet, ensuring only authorized participants interact with EMRs.
    
*   **Credential Validation**: Links medical licenses, nursing certifications, and hospital accreditations to DIDs, validated by real-world organizations (e.g., medical boards, accreditation bodies).
    
*   **Access Control**: Fine-grained permissions allow patients to grant/revoke provider access, with checks for valid credentials.
    
*   **Revocation Support**: Handles credential revocation (e.g., expired or suspended licenses) with on-chain updates.
    
*   **Interoperability**: Integrates with Polkadot’s XCM for cross-chain data sharing, enabling multi-institutional ecosystems.
    
*   **Regulatory Compliance**: Ensures HIPAA/GDPR compliance through AES-256 encryption, zero-knowledge proofs (ZKPs), and immutable audit trails.
    
*   **Immutable Audit Trail**: Logs all EMR and attestation interactions on-chain for transparency and compliance.
    

Architecture
------------

*   **Blockchain Layer**: Custom Substrate pallets (emr\_pallet, attestation\_pallet) for EMR management, access control, attestation, and credential validation, running on any custom Substrate chain.
    
*   **Off-Chain Storage**: Encrypted EMRs and credential documents (e.g., license PDFs) stored on IPFS, with hashes on-chain.
    
*   **Identity System**: Integrates with any Substrate-based DID framework for secure, verified identities.
    
*   **Frontend**: React-based UI with Polkadot.js for wallet integration and blockchain interaction.
    
*   **Real-World Integration**: Oracles (e.g., Chainlink on Polkadot) or a healthcare DAO validate credentials from trusted organizations (e.g., AMA, Joint Commission).
    

Prerequisites
-------------

*   **Rust**: Stable toolchain (rustup update stable).
    
*   **Substrate**: Substrate Node Template or any custom Substrate chain.
    
*   **IPFS**: For off-chain storage ([https://ipfs.io](https://ipfs.io/)).
    
*   **Node.js**: For the React frontend (>=16.0.0).
    
*   **Polkadot.js**: Browser extension for wallet management ([https://polkadot.js.org/extension](https://polkadot.js.org/extension)).
    
*   **Docker** (optional): For running IPFS or blockchain nodes.
    
*   **Chainlink** (optional): For oracle-based credential validation.
    

Installation
------------

### 1\. Clone the Repository

Plain textANTLR4BashCC#CSSCoffeeScriptCMakeDartDjangoDockerEJSErlangGitGoGraphQLGroovyHTMLJavaJavaScriptJSONJSXKotlinLaTeXLessLuaMakefileMarkdownMATLABMarkupObjective-CPerlPHPPowerShell.propertiesProtocol BuffersPythonRRubySass (Sass)Sass (Scss)SchemeSQLShellSwiftSVGTSXTypeScriptWebAssemblyYAMLXML`   git clone https://github.com/meddsai/Web3EMR.git  cd Web3EMR   `

### 2\. Set Up the Substrate Blockchain

Use the Substrate Node Template:

- `git clone https://github.com/substrate-developer-hub/substrate-node-template`
- `cd substrate-node-template`
- `cargo build --release`
- `./target/release/node-template --dev`

### 3\. Install IPFS

Download and initialize IPFS:

Plain textANTLR4BashCC#CSSCoffeeScriptCMakeDartDjangoDockerEJSErlangGitGoGraphQLGroovyHTMLJavaJavaScriptJSONJSXKotlinLaTeXLessLuaMakefileMarkdownMATLABMarkupObjective-CPerlPHPPowerShell.propertiesProtocol BuffersPythonRRubySass (Sass)Sass (Scss)SchemeSQLShellSwiftSVGTSXTypeScriptWebAssemblyYAMLXML`   wget https://dist.ipfs.io/go-ipfs/v0.10.0/go-ipfs_v0.10.0_linux-amd64.tar.gz  tar -xvzf go-ipfs_v0.10.0_linux-amd64.tar.gz  cd go-ipfs  ./install.sh  ipfs init  ipfs daemon   `

### 4\. Set Up the Backend

Install Rust dependencies and build the pallets:

Plain textANTLR4BashCC#CSSCoffeeScriptCMakeDartDjangoDockerEJSErlangGitGoGraphQLGroovyHTMLJavaJavaScriptJSONJSXKotlinLaTeXLessLuaMakefileMarkdownMATLABMarkupObjective-CPerlPHPPowerShell.propertiesProtocol BuffersPythonRRubySass (Sass)Sass (Scss)SchemeSQLShellSwiftSVGTSXTypeScriptWebAssemblyYAMLXML`   cd backend  cargo build --release   `

Add emr\_pallet.rs and attestation\_pallet.rs to the Substrate node’s pallets directory. Update the runtime configuration (runtime/src/lib.rs) to include:

Plain textANTLR4BashCC#CSSCoffeeScriptCMakeDartDjangoDockerEJSErlangGitGoGraphQLGroovyHTMLJavaJavaScriptJSONJSXKotlinLaTeXLessLuaMakefileMarkdownMATLABMarkupObjective-CPerlPHPPowerShell.propertiesProtocol BuffersPythonRRubySass (Sass)Sass (Scss)SchemeSQLShellSwiftSVGTSXTypeScriptWebAssemblyYAMLXML`   impl emr_pallet::Config for Runtime {      type Event = Event;  }  impl attestation_pallet::Config for Runtime {      type Event = Event;  }   `

### 5\. Set Up the Frontend

Install Node.js dependencies and start the React app:

Plain textANTLR4BashCC#CSSCoffeeScriptCMakeDartDjangoDockerEJSErlangGitGoGraphQLGroovyHTMLJavaJavaScriptJSONJSXKotlinLaTeXLessLuaMakefileMarkdownMATLABMarkupObjective-CPerlPHPPowerShell.propertiesProtocol BuffersPythonRRubySass (Sass)Sass (Scss)SchemeSQLShellSwiftSVGTSXTypeScriptWebAssemblyYAMLXML`   cd frontend  npm install @polkadot/api @polkadot/extension-dapp  npm start   `

### 6\. Configure Polkadot.js

Install the Polkadot.js browser extension and create/import accounts (e.g., Alice, Bob) for testing.

Usage
-----

### 1\. Run the Blockchain

Start the Substrate node:

`./target/release/node-template --dev`

### 2\. Register Healthcare Entities

Trusted authorities (e.g., regulators, healthcare DAO) register entities:

*   Access the frontend at http://localhost:3000.
    
*   Connect a Polkadot.js wallet with authority privileges.
    
*   Submit entity DID (e.g., did:mandala:hospital123) and accreditation hash (from IPFS) via the register\_entity pallet call.
    

### 3\. Attest Patients and Personnel

Healthcare entities attest patients and personnel:

*   **Patients**: Link patient DID to entity DID using attest\_patient.
    
*   **Personnel**: Submit personnel DID, entity DID, credential hash (e.g., medical license), and expiry timestamp via attest\_personnel.
    
*   Credentials are validated off-chain by real-world organizations (e.g., medical boards) and stored on IPFS.
    

### 4\. Store an EMR

Patients upload EMRs:

*   Log in with a Polkadot.js wallet.
    
*   Upload an EMR (JSON format, e.g., {"name": "John Doe", "record": "..."}).
    
*   EMR is encrypted, stored on IPFS, and its hash is recorded on-chain via store\_emr.
    

### 5\. Manage Access

Patients grant/revoke provider access:

*   Grant access to a provider’s DID using grant\_access, ensuring provider credentials are valid.
    
*   Revoke access with revoke\_access.
    
*   Providers retrieve EMRs from IPFS using the on-chain hash, decrypting with authorized keys.
    

### 6\. Revoke Credentials

Authorities revoke credentials (e.g., expired licenses) using revoke\_credential, updating on-chain status.

### 7\. View Audit Trail

Check the blockchain’s event log (via Polkadot.js Apps) for EMR and attestation events (e.g., EMRStored, PersonnelAttested, CredentialRevoked).

Example Workflow
----------------

1.  **Healthcare Entity Registration**:
    
    *   A medical board registers a hospital’s DID and accreditation hash on-chain.
        
2.  **Credential Attestation**:
    
    *   The hospital attests a doctor’s medical license, linking their DID to a credential hash and expiry.
        
3.  **Patient Attestation**:
    
    *   The hospital attests a patient’s registration, linking their DID to the hospital.
        
4.  **EMR Management**:
    
    *   The patient uploads an EMR, stored on IPFS with a hash on-chain.
        
    *   The patient grants access to the doctor, who retrieves the EMR after credential validation.
        
5.  **Audit and Compliance**:
    
    *   All actions are logged on-chain, ensuring transparency and regulatory compliance.
        

Development
-----------

### Key Components

*   **EMR Pallet**: Manages EMR storage and access control (backend/pallets/emr\_pallet.rs).
    
*   **Attestation Pallet**: Handles entity, patient, and personnel attestation with credential validation (backend/pallets/attestation\_pallet.rs).
    
*   **IPFS Integration**: Encrypts and stores EMRs/credentials off-chain (backend/ipfs\_storage.rs).
    
*   **Frontend**: React app with Polkadot.js for blockchain interaction (frontend/src).
    

### Adding New Features

*   Extend pallets for clinical trial data or insurance integration.
    
*   Implement zero-knowledge proofs (ZKPs) for private credential verification.
    
*   Add multi-language support to the frontend.
    
*   Integrate a healthcare DAO for governance using Substrate’s democracy pallet.
    

Testing
-------

1.  **Local Testing**:
    
    *   Run a local Substrate node (--dev).
        
    *   Use Polkadot.js Apps ([https://polkadot.js.org/apps](https://polkadot.js.org/apps)) to interact with pallets.
        
    *   Test IPFS storage/retrieval with sample EMRs and credentials.
        
2.  **Testnet**:
    
    *   Deploy to any custom Substrate chain’s testnet.
        
    *   Simulate attestation workflows with multiple wallets (e.g., patient, provider, regulator).
        

Deployment
----------

1.  **Mainnet**:
    
    *   Deploy to any custom Substrate chain’s mainnet or a custom Polkadot parachain slot.
        
    *   Configure production-grade IPFS nodes.
        
    *   Host the frontend on a decentralized platform (e.g., Fleek, IPFS).
        
2.  **Compliance**:
    
    *   Use AES-256 encryption for EMRs and credentials.
        
    *   Implement ZKPs for private transactions.
        
    *   Maintain audit logs for HIPAA/GDPR compliance.
        
3.  **Real-World Integration**:
    
    *   Partner with medical boards (e.g., AMA) or accreditation bodies (e.g., Joint Commission) for credential validation.
        
    *   Use oracles (e.g., Chainlink) or a healthcare DAO for off-chain verification.
        

Security Considerations
-----------------------

*   **Encryption**: AES-256 for off-chain data.
    
*   **Key Management**: Secure patient/provider keys via Polkadot.js wallets.
    
*   **Credential Validation**: Ensure real-world organizations attest credentials to prevent fraud.
    
*   **Auditability**: Immutable on-chain logs for all interactions.
    
*   **Privacy**: Use ZKPs for sensitive data (e.g., `license` details).
    

Contributing
------------

1.  Fork the repository.
    
2.  Create a feature branch (`git checkout -b feature/xyz`).
    
3.  Commit changes (`git commit -m "Add XYZ feature"`).
    
4.  Push to the branch (`git push origin feature/xyz`).
    
5.  Open a pull request.
    

Follow the [Code of Conduct](./CODE_OF_CONDUCT.md) and report issues via GitHub.

License
-------

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

Resources
---------

*   **Substrate Developer Hub**: [https://substrate.dev](https://substrate.dev/)
    
*   **Polkadot.js**: [https://polkadot.js.org](https://polkadot.js.org/)
    
*   **IPFS**: [https://ipfs.io](https://ipfs.io/)
    
*   **Chainlink on Polkadot**: [https://docs.chain.link](https://docs.chain.link/)
    

Contact
-------

For questions or support, open an issue or contact [Kresna Sucandra](https://github.com/SHA888).

_Empowering decentralized healthcare with trust and transparency._