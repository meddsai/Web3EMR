# Architecture

RustEMR follows a modular, service-oriented architecture inspired by OpenEMR and OpenMRS to ensure scalability, maintainability, and ease of extension.

## Components

- **Backend (Rust)**: Business logic, data validation, and RESTful/FHIR APIs. Leverages Rust’s safety, performance, and concurrency.
- **Frontend (TypeScript & React)**: Responsive UI, form rendering, clinical workflows, and administrative dashboards.
- **Database (PostgreSQL)**: Relational storage of patient, encounter, and system data. Supports migrations and complex queries.
- **Modules & Plugins**: 
  - OpenEMR-style clinical forms, custom modules, and reporting plugins.
  - OpenMRS-like concept dictionary and module registry for extensions.

## Data Flow

1. User actions in the React UI send HTTP/HTTPS requests.
2. Rust backend routes requests, enforces ACLs, and processes business rules.
3. Data persists in PostgreSQL; FHIR resources are mapped to relational tables.
4. External systems integrate via FHIR-compliant endpoints or REST hooks.

## Inspiration from OpenEMR & OpenMRS

- **OpenEMR**: Modular form engine, patient dashboard, billing modules.
- **OpenMRS**: Concept dictionary, metadata-driven modules, global community plugins.

This architecture allows independent scaling of frontend and backend, hot-swappable modules, and robust integration with external healthcare systems.
