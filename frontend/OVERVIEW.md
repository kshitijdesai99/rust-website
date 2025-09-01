# Frontend Overview

Rust + Yew single-page app compiled to WebAssembly, served via Trunk. Integrates with the backend API.

## Tech Stack
- Framework: Yew
- Build/Dev: Trunk
- Target: wasm32-unknown-unknown
- Styling: CSS (Tailwind optional)

## Project Structure
```
frontend/
├── Cargo.toml
├── Trunk.toml
├── index.html                 # App HTML shell
├── styles.css                 # Global styles
├── input.css                  # Tailwind input (if used)
├── tailwind.config.js         # Tailwind config
├── src/
│   ├── lib.rs                 # Yew app bootstrap
│   ├── config.rs              # App constants (e.g., API_BASE_URL)
│   ├── app_routes.rs          # Route enum + switch()
│   ├── routes.rs              # Alternative routes/pages (legacy/demo)
│   ├── assets/                # Static assets (images, etc.)
│   ├── pages/                 # Page-level components
│   │   ├── landing.rs
│   │   ├── health.rs
│   │   ├── blogs_list.rs
│   │   ├── blog_detail.rs
│   │   ├── not_found.rs
│   │   └── mod.rs
│   ├── components/            # Reusable UI
│   │   ├── mod.rs
│   │   ├── header.rs
│   │   ├── footer.rs
│   │   ├── button.rs
│   │   └── form.rs
│   └── services/              # API + utilities
│       ├── mod.rs
│       ├── api.rs             # HTTP client/types
│       ├── blogs.rs           # Blogs API
│       ├── health.rs          # Health API
│       └── storage.rs         # LocalStorage helpers
├── dist/                      # Build output (Trunk)
└── README.md
```

## Setup
1) Install toolchain
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

2) Configure base URL
- `src/config.rs` defines constants such as `API_BASE_URL` (typically `http://localhost:3000`).

## Develop & Build
- Dev server
```bash
trunk serve --open
```
- Production build
```bash
trunk build --release
```

## App Structure
- Routing: `app_routes.rs` (primary), `routes.rs` (legacy/demo)
- Pages: landing, health, blogs list, blog detail, 404
- Services: `api.rs` (client), `blogs.rs`, `health.rs`, `storage.rs`
- Assets: static content under `src/assets/`

## More
- See `frontend/README.md` for more details on structure and commands.
