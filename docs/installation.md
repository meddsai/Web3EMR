---
---

# Installation

This guide covers setting up RustEMR locally for development and production environments.

## Prerequisites

- **Rust** (v1.50+): Install from https://www.rust-lang.org/tools/install
- **Node.js** (v14+): Install from https://nodejs.org/
- **PostgreSQL** (v12+): Install from https://www.postgresql.org/
- **cargo-make** (optional): For task automation (`cargo install cargo-make`).

## Local Development Setup

1. **Clone repository**

   ```bash
   git clone https://github.com/UbudCare/rustemr.git
   cd rustemr
   ```

2. **Database setup**

   ```bash
   psql -U postgres -c "CREATE DATABASE rustemr;"
   psql -U postgres -c "CREATE USER rustemr_user WITH PASSWORD 'password';"
   psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE rustemr TO rustemr_user;"
   ```

3. **Backend configuration**

   ```bash
   cd backend
   cp .env.example .env
   # Edit .env with DB credentials
   cargo make build
   cargo make start
   ```

4. **Frontend configuration**

   ```bash
   cd ../frontend
   npm install
   npm run dev
   ```

5. **Access services**

   - Backend API: http://localhost:8080
   - Frontend UI: http://localhost:3000

## Production Deployment

For production, consider Docker or Kubernetes orchestrations inspired by OpenEMR Docker configurations.

1. **Docker Compose**

   ```bash
   docker-compose up -d
   ```

2. **Environment**

   - Use secure credentials, TLS certificates, and load balancers.
   - Configure PostgreSQL with backups and replication.

3. **Monitoring & Logging**

   - Integrate Prometheus/Grafana for metrics.
   - Centralize logs with ELK or similar stacks.
