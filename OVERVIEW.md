# 📋 Project Overview

```
🦀 Full-Stack Rust Web Application
├── 🔧 Backend (API Server)
│   ├── Language: Rust
│   ├── Framework: Axum
│   ├── Database: SeaORM
│   ├── Architecture: Clean Architecture
│   └── Features: Type-safe APIs, async/await, migrations
│   └── Features: SeaORM Pro admin panel
│
├── 🎨 Frontend (Web App)
│   ├── Language: Rust → WebAssembly
│   ├── Framework: Yew
│   ├── Build Tool: Trunk
│   ├── Architecture: Component-based
│   └── Features: Zero JavaScript, near-native performance
│   └── Features: Tailwind CSS for styling
│
└── 🛠️ Development Tools
    ├── dev.sh - Single script for all tasks
    ├── Hot reload - Automatic browser refresh
    └── Backend integration tests

## 🚀 Quick Start
```bash
# First-time setup
chmod +x dev.sh
./dev.sh setup

# Start full-stack dev (backend + frontend)
./dev.sh dev
```
- Frontend: http://127.0.0.1:8080
- Backend API: http://127.0.0.1:3000
- Health: GET http://127.0.0.1:3000/health
- Admin: http://127.0.0.1:3000/admin (login at /login)

## 🧩 Architecture
```
┌─────────────────┐    HTTP/JSON/GraphQL   ┌──────────────────┐
│   Yew Frontend  │ ◄────────────────────► │   Axum Backend   │
│ (WebAssembly)   │                        │  (API + GraphQL) │
└─────────────────┘                        └──────────────────┘
             │                                          │
             │                                          │
        Browser APIs                              SQLite via SeaORM
```
- GraphQL schema generated via Seaography from SeaORM entities.
- Admin panel (SeaORM Pro) served as static assets under `/admin`.
- JWT-based auth, CORS enabled for local development.

## 📂 Repository Layout
```
.
├── backend/     # Axum API server, GraphQL, admin, migrations, entities
├── frontend/    # Yew SPA, Trunk config, pages, components, services
├── dev.sh       # Helper script: dev, build, test, clean, setup
├── README.md    # Root workflows, contribution, deployment
└── OVERVIEW.md  # This high-level overview
```

## ✨ Core Features
- Type-safe REST + GraphQL built on Axum, async-graphql, SeaORM, Seaography
- Admin interface via SeaORM Pro with TOML-based configuration
- JWT authentication for protected endpoints
- Yew-based SPA with WASM, router, and API services
- Simple local SQLite setup and auto-applied migrations

## 🔌 API Surface (summary)
- Health: `GET /health`, `GET /api/v1/status`
- Auth: `POST /api/auth/login`, `GET /api/user/current`
- Blogs: `GET /api/blogs`, `GET /api/blogs/{slug}`
- GraphQL: `GET/POST /api/graphql`
- Admin: `GET /admin`, `GET /api/admin/config`, `GET /login`

## 🌎 Environments
- Development defaults:
  - Frontend: `http://127.0.0.1:8080`
  - Backend: `http://127.0.0.1:3000`
  - Frontend `API_BASE_URL` in `frontend/src/config.rs`.
- Backend env vars (see `backend/OVERVIEW.md` for details):
  - `DATABASE_URL`, `JWT_SECRET`, `RUST_LOG`, optional demo login.

## 🔗 Further Reading
- Backend details: `backend/OVERVIEW.md`, `backend/README.md`
- Frontend details: `frontend/OVERVIEW.md`, `frontend/README.md`
- Helper script usage: `./dev.sh help`
