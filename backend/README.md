# RustEMR Backend

This is the Rust backend for the RustEMR project.

## Setup

- **Requires:** Rust (https://rustup.rs)
- Install dependencies: `cargo build`
- Run development server: `cargo run`
- For hot-reloading: `cargo watch -x run` (install with `cargo install cargo-watch`)

## Project Structure

- `src/main.rs` — Entry point
- `src/routes/` — Route handlers
- `src/models/` — Data models
- `src/db/` — Database logic

## Environment

- Copy `.env.example` to `.env` and set your database credentials and secrets.
- Example `.env` (not included in repo):
  ```env
  DATABASE_URL=postgres://user:password@localhost/rustemr
  RUST_LOG=debug
  ```

## Development

- Use `cargo watch -x run` for automatic reloading on code changes.
- Run tests: `cargo test`

## Useful Commands

- `cargo build` — Build the project
- `cargo run` — Run the backend
- `cargo test` — Run tests
- `cargo fmt` — Format code
- `cargo clippy` — Lint code

## Contributing

- Follow Rust style guidelines (`cargo fmt`)
- All code changes should be tested
- Open issues or PRs for bugs and feature requests

## License

See [LICENSE](../LICENSE).
