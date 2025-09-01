# Backend Overview

Modern Rust backend built with Axum, SeaORM, Seaography (GraphQL), and a SeaORM Pro admin panel.

## Tech Stack
- HTTP: Axum
- Database: SeaORM + SQLite (local file `data.db`)
- GraphQL: async-graphql + Seaography (schema from entities)
- Auth: JWT (Bearer) with bcrypt hashing
- Admin: SeaORM Pro (static UI + TOML config)

## Project Structure
```
backend/
├── Cargo.toml
├── assets/
│   ├── admin/                # Prebuilt admin UI (static)
│   └── login.html            # Admin login template
├── pro_admin/                # Admin panel configuration
│   ├── config.toml
│   └── raw_tables/
├── src/
│   ├── admin/                # Admin endpoints & GraphQL playground
│   │   └── login.rs
│   ├── api/
│   │   ├── routes.rs         # Routes, CORS, static `/admin`
│   │   └── handlers/
│   │       ├── auth.rs       # /api/auth/login, /api/user/current
│   │       └── blogs.rs      # /api/blogs, /api/blogs/{slug}
│   ├── auth/                 # JWT helpers and types
│   ├── entities/             # SeaORM entities
│   ├── graphql/
│   │   └── schema.rs         # Seaography dynamic schema
│   ├── migration/            # DB migrations
│   ├── error.rs              # Error types/helpers
│   ├── lib.rs                # AppState, create_app(), run()
│   └── main.rs               # Entry point
├── tests/
│   ├── common/
│   └── integration_tests.rs
└── data.db                   # SQLite DB (created locally)
```

## Setup & Run
1) Prerequisites: Rust toolchain installed
2) Environment: create `backend/.env`
```env
DATABASE_URL=sqlite://./data.db
JWT_SECRET=your-secret-key
RUST_LOG=info

# Optional demo login for /api/auth/login
DEMO_LOGIN_EMAIL=demo@sea-ql.org
DEMO_LOGIN_PASSWORD=any-password
```
3) Start server
```bash
cargo run
```
4) Run tests
```bash
cargo test
```

Servers:
- API: http://localhost:3000
- Admin Panel: http://localhost:3000/admin
- GraphQL Playground: http://localhost:3000/api/graphql

## Key Endpoints
- Health
  - GET `/health`
  - GET `/api/v1/status`
- Auth
  - POST `/api/auth/login`
  - GET `/api/user/current` (Authorization: Bearer <token>)
- Blogs
  - GET `/api/blogs` (query: `page`, `per_page`, optional `status`)
  - GET `/api/blogs/{slug}` (records a view)
- GraphQL
  - GET `/api/graphql` (playground)
  - POST `/api/graphql`
- Admin
  - GET `/admin`
  - GET `/api/admin/config`
  - GET `/login`

## Tips
- Migrations run at startup; SQLite file is created locally.
- Clients must send `Authorization: Bearer <token>` for protected routes.

## More
- See `backend/README.md` for deeper details and maintenance tasks.
