# 🦀 Rust Full-Stack Web Application

> **Strategy:** Build a modern, type-safe web application using Rust across the entire stack for maximum performance, safety, and maintainability.

> For high-level architecture, features, and endpoint summary, see `OVERVIEW.md`.

## Quick Start
```bash
chmod +x dev.sh      # first time
./dev.sh setup       # install tools (Trunk, wasm target)
./dev.sh dev         # run backend (3000) + frontend (8080)
```

## Development Workflows
- Run both apps with hot reload: `./dev.sh dev`
- Backend only:
  ```bash
  cd backend
  RUST_LOG=info cargo run
  ```
- Frontend only:
  ```bash
  cd frontend
  trunk serve --open
  ```

## Common Tasks
- Test all: `./dev.sh test`
- Clean builds: `./dev.sh clean`
- Production build: `./dev.sh build`
- Tooling setup (idempotent): `./dev.sh setup`

## Configuration
- Backend env in `backend/.env` (see `backend/OVERVIEW.md`).
- Frontend base URL in `frontend/src/config.rs` (`API_BASE_URL`).
- CORS defaults are permissive in dev; restrict in production.

## Deployment
- Run `./dev.sh build` to produce:
  - Backend binary: `backend/target/release/backend`
  - Frontend assets: `frontend/dist/`
- Serve backend behind a reverse proxy; serve frontend as static files.
- Provide production environment via process manager or `.env`.

## Troubleshooting
- Trunk missing: `cargo install trunk`
- WASM target missing: `rustup target add wasm32-unknown-unknown`
- Ports in use: stop existing services on 3000/8080 or change ports
- Frontend cannot reach API: verify `API_BASE_URL` and CORS

## Contributing
- Format with `rustfmt`; keep modules focused and documented.
- Prefer small, cohesive changes; include rationale in PR description.
- Backend: add routes in `backend/src/api/routes.rs`, handlers under `backend/src/api/handlers/`.
- Frontend: add pages via `frontend/src/app_routes.rs`, services under `frontend/src/services/`.

## Links
- High-level overview: `OVERVIEW.md`
- Backend docs: `backend/OVERVIEW.md`, `backend/README.md`
- Frontend docs: `frontend/OVERVIEW.md`, `frontend/README.md`
- Helper script: `./dev.sh help`
