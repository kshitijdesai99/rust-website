# Backend API - Rust Web Application

Modern Rust backend with SeaORM, GraphQL, and admin panel.

## Quick Start

```bash
# Start development server
cd backend
cargo run
```

Servers:
- **API:** http://localhost:3000
- **Admin Panel:** http://localhost:3000/admin
- **GraphQL Playground:** http://localhost:3000/api/graphql

> For a high-level introduction (stack, endpoints, structure, and basics), see `backend/OVERVIEW.md`.

## What this README covers
- Advanced developer workflows
- Configuration details (CORS, env management)
- Database/migration lifecycle notes
- Testing strategy
- Extending routes, handlers, entities, and GraphQL
- Admin panel customization tips
- Release/build guidance and security checklist

## Development workflows
- Use the root helper script `./dev.sh dev` to run backend and frontend together (see `dev.sh help`).
- Backend only:
  ```bash
  cd backend
  RUST_LOG=info cargo run
  ```
- Adjust log verbosity via `RUST_LOG` (e.g., `debug`, `info`, `warn`).

## Configuration details
- Environment variables: see `backend/OVERVIEW.md` for the canonical list. Tips:
  - Place backend-specific values in `backend/.env`. The root `./dev.sh dev` loads both root `.env` and `backend/.env`.
  - Keep secrets (e.g., `JWT_SECRET`) out of version control.
- CORS: configured in `src/api/routes.rs` using `CorsLayer` with permissive defaults:
  - `allow_origin(Any)`, `allow_methods(Any)`, `allow_headers(Any)`.
  - For production, restrict origins/methods/headers as needed.

## Database and migrations
- SQLite database file is created locally (`backend/data.db`).
- Migrations execute during startup; migration source lives under `src/migration/`.
- To add a new table/entity:
  1. Add/adjust SeaORM entity in `src/entities/`.
  2. Create a matching migration in `src/migration/`.
  3. Start the app to apply migration; verify via admin panel or queries.

## Testing
- Run all tests: `cargo test`.
- Integration example: `tests/integration_tests.rs` spins up the Axum app and calls `/health`.
- Add shared test utilities under `tests/common/`.

## Extending the API
- Add a handler in `src/api/handlers/` (e.g., `foo.rs`).
- Register the route in `src/api/routes.rs` using Axum `get`/`post`.
- Update entities and migrations if persistence is needed.
- GraphQL: `src/graphql/schema.rs` builds from entities—new entities can be exposed via Seaography.

## Admin panel customization
- Configure via `pro_admin/config.toml` and `pro_admin/raw_tables/*`.
- Static admin UI is served from `assets/admin/` at `/admin`.
- Admin login page template: `assets/login.html` (served by `src/admin/login.rs`).

## Release and build
- Fast path (root script): `./dev.sh build` produces:
  - Backend binary: `backend/target/release/backend`
  - Frontend assets: `frontend/dist/`
- Backend only:
  ```bash
  cargo build --release
  ```
- Provide production env via `backend/.env` or your process manager.

## Security checklist
- Use a strong `JWT_SECRET` in production.
- Restrict CORS to trusted origins.
- Consider adding rate limiting and audit logging.
- Avoid leaking stack traces or internal errors to clients.

## Links
- High-level overview: `backend/OVERVIEW.md`
- Frontend workflows: `../frontend/README.md`
- Helper script: `../dev.sh`
