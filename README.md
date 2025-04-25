- [Documentation](https://ubudcare.github.io/rustemr/)

# RustEMR

RustEMR is an open-source electronic medical record (EMR) system designed to provide a secure, scalable, and interoperable solution for healthcare providers. Built with a modern tech stack—Rust for the backend, TypeScript for the frontend, and PostgreSQL as the database—it supports FHIR and REST APIs for seamless integration with other healthcare systems.

Inspired by OpenEMR and OpenMRS, RustEMR combines the comprehensive feature set and usability of OpenEMR with the flexibility and global scalability of OpenMRS. Our goal is to create a robust, community-driven EMR system that meets the needs of clinics, hospitals, and healthcare organizations worldwide.

## Table of Contents

- [Architecture](#architecture)
- [Features](#features)
  - [Current Features](#current-features)
- [Roadmap](#roadmap)
- [Installation](#installation)
  - [Prerequisites](#prerequisites)
  - [Setup](#setup)
- [Development with Docker](#development-with-docker)
- [Task Automation with cargo-make](#task-automation-with-cargo-make)
- [CI/CD](#cicd)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)
- [Acknowledgments](#acknowledgments)

## Architecture

RustEMR follows a modular, service-oriented architecture inspired by OpenEMR and OpenMRS, ensuring scalability, maintainability, and ease of development. The system is divided into the following components:

- **Backend**: Written in Rust, the backend handles business logic, data processing, and exposes RESTful APIs for communication with the frontend and external systems. Rust’s performance, memory safety, and concurrency features make it ideal for handling sensitive medical data securely and efficiently.
- **Frontend**: Built with TypeScript and React, the frontend provides an intuitive, responsive user interface for healthcare professionals to manage patient records, schedule appointments, and track clinical workflows.
- **Database**: PostgreSQL stores all medical records, user data, and system configurations. Its reliability and support for complex queries make it a perfect fit for handling large-scale healthcare data.
- **APIs**:
  - **REST APIs**: Used for internal communication between the frontend and backend.
  - **FHIR-compliant APIs**: Ensure interoperability with other healthcare systems, following the Fast Healthcare Interoperability Resources (FHIR) standard.

The backend and frontend are decoupled, communicating exclusively via REST APIs. This separation allows for independent development, testing, and scaling of each component.

## Features

RustEMR is being developed with a focus on core EMR functionalities, drawing from the extensive feature sets of OpenEMR and OpenMRS.

### Current Features

- **Basic Patient Management**: Create, read, update, and delete (CRUD) patient records, including essential details like name, date of birth, and contact information.
- **Encounter Tracking**: Record patient visits (encounters) and capture observations such as vital signs, diagnoses, and treatments.

## Roadmap

RustEMR’s development roadmap includes the following planned features, prioritized based on community needs and healthcare workflows:

- **Advanced Patient Management**:

  - Demographics (age, gender, ethnicity)
  - Medical history (conditions, surgeries, allergies)
  - Family and social history

- **Clinical Decision Support**:

  - Alerts for drug interactions and allergies
  - Clinical guidelines and best practice reminders

- **Billing and Insurance Integration**:

  - Generate invoices and manage payments
  - Support for insurance claims and reimbursements

- **Reporting and Analytics**:

  - Customizable reports for patient outcomes, clinic performance, and population health
  - Data visualization tools for healthcare insights

- **Mobile Application Support**:

  - Responsive design for mobile devices
  - Native mobile apps for iOS and Android (future consideration)

- **Interoperability Enhancements**:
  - Full FHIR resource support for seamless data exchange
  - Integration with laboratory systems, pharmacies, and other external services

## Installation

Follow these steps to set up RustEMR locally for development or testing purposes.

### Prerequisites

- Rust (v1.50 or higher)
- Node.js (v14 or higher)
- PostgreSQL (v12 or higher)

### Setup

1. **Clone the repository**:

```bash
git clone https://github.com/yourusername/rustemr.git
cd rustemr
```

2. **Set up the database**:

```bash
# If running locally (outside Docker):
CREATE DATABASE rustemr;
CREATE USER rustemr_user WITH PASSWORD 'password';
GRANT ALL PRIVILEGES ON DATABASE rustemr TO rustemr_user;
# If using Docker Compose, the database will be created automatically on port 55432.
```

3. **Configure the backend**:

```bash
cd backend
cp .env.example .env
# Edit .env to include your PostgreSQL credentials
# Default PostgreSQL port: The database now runs on host port `55432` (not the default `5432`). If you run multiple projects or encounter connection issues, ensure your `.env` and client tools use:
DATABASE_URL=postgres://rustemr:rustemr@localhost:55432/rustemr_dev
cargo build
cargo run
```

4. **Configure the frontend**:

```bash
cd ../frontend
npm install
npm start
```

**Access the application**:

- Backend API: `http://localhost:8080`
- Frontend: `http://localhost:3000`

## Development with Docker

1. Make sure you have Docker and Docker Compose installed.
2. Copy the example .env files or create your own in the root, backend, and frontend directories.
3. Start all services:

```bash
cargo make start
```

- This will run `docker-compose up` and start the database, backend, and frontend.
- The backend will be available at http://localhost:8080
- The frontend will be available at http://localhost:3000
- The backend service in Docker Compose uses the service name `db` and port `5432` internally, so no changes are needed there unless you customize further.

To stop all services:

```bash
cargo make stop
```

## Task Automation with cargo-make

Install cargo-make if you haven't already:

```bash
cargo install cargo-make
```

Common tasks:

- `cargo make start` - Start Docker Compose services
- `cargo make stop` - Stop services
- `cargo make migrate` - Run DB migrations (placeholder)
- `cargo make backend-build` - Build backend
- `cargo make frontend-build` - Build frontend
- `cargo make test` - Run backend tests
- `cargo make lint` - Lint frontend and backend
- `cargo make format` - Format frontend and backend

## CI/CD

Basic CI is configured in `.github/workflows/ci.yml` to check build and lint for backend and frontend.

## Contributing

RustEMR is an open-source project, and we welcome contributions from developers, healthcare professionals, and the broader community. To contribute:

1. Fork the repository on GitHub.
2. Create a feature branch:

```bash
git checkout -b feature/your-feature-name
```

3. Follow coding standards outlined in `CONTRIBUTING.md`.
4. Submit a pull request with a clear description of your changes and any relevant issue numbers.

We use Git for version control and follow a feature-branch workflow to manage contributions.

## License

RustEMR is licensed under the GNU General Public License v3.0, the same license used by OpenEMR and OpenMRS. This ensures that the software remains free and open-source, while encouraging community collaboration and improvement.

## Contact

For questions, suggestions, or to get involved in the RustEMR community discussion:

- GitHub Discussions: [Discussions](https://github.com/UbudCare/rustemr/discussions)

## Acknowledgments

RustEMR is inspired by the architectures and feature sets of OpenEMR and OpenMRS, two pioneering open-source EMR systems. We extend our gratitude to their communities for their contributions to open-source healthcare software, which have informed and guided the development of RustEMR.
