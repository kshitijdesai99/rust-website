# Frontend

Rust-based frontend using Yew framework.

> For a high-level introduction (stack, directory layout, and basics), see `frontend/OVERVIEW.md`.

## What this README covers
- Advanced development workflows
- Configuration details for API base URL and build targets
- API integration patterns and adding new endpoints
- Routing conventions and adding new pages
- Component patterns, state, and error UX
- Styling and asset pipeline
- Build, release, and deployment notes

## Development workflows
- Root helper script to run both apps:
  ```bash
  ./dev.sh dev
  ```
- Frontend only (hot reload):
  ```bash
  cd frontend
  trunk serve --open
  ```
- Target configuration: ensure wasm target is installed once
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

## Configuration details
- API base URL lives in `src/config.rs` as a compile-time constant:
  - `API_BASE_URL` currently: `http://127.0.0.1:3000`.
  - For production, update `src/config.rs` accordingly before building release.
- App identity: `APP_NAME`, `APP_VERSION` in `src/config.rs`.

## API integration patterns
- HTTP client: `src/services/api.rs` using `gloo_net::http::Request`.
- Blogs endpoints implemented:
  - `list_blogs(page, per_page)`
  - `get_blog_by_slug(slug)`
- Pattern to add new endpoints:
  1. Define response models with `serde::Deserialize`.
  2. Add request function on `ApiService` returning `Result<T, String>`.
  3. Use from pages/components via `wasm_bindgen_futures::spawn_local` + Yew hooks.

## Routing conventions
- Primary router in `src/app_routes.rs` (`Route` enum + `switch()` function).
- To add a route:
  1. Add a new variant to `Route`.
  2. Map it in `switch()` to a page component.
  3. Link using `yew_router::prelude::Link<Route>`.

## Components, state, and error UX
- Components live in `src/components/` and pages in `src/pages/`.
- Favor function components with props; lift state to parent when shared.
- Service calls return `Result<T, String>`; render friendly errors in UI.

## Styling and assets
- Global CSS: `styles.css`.
- Tailwind (optional): `input.css` + `tailwind.config.js`.
- Static assets live under `src/assets/`.

## Build, release, and deployment
- Production build:
  ```bash
  cd frontend
  trunk build --release
  ```
- Output is written to `frontend/dist/`.
- Ensure `API_BASE_URL` in `src/config.rs` targets your backend before building.
- Serve `dist/` via any static host or behind the backend server.

## Links
- High-level overview: `frontend/OVERVIEW.md`
- Backend guidance: `../backend/README.md`
- Helper script: `../dev.sh`
