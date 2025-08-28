# Full-Stack Rust Web Application

A modern web application built with Rust, featuring an Axum backend and Yew WebAssembly frontend.

## Architecture

- **Backend**: Axum HTTP server with REST API
- **Frontend**: Yew framework WebAssembly SPA
- **Build System**: Cargo workspace with Trunk

## Quick Start

### Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Running the Application

1. **Start Backend** (from project root):
```bash
cargo run -p backend
```
Backend serves at: http://127.0.0.1:3000

2. **Start Frontend** (in new terminal):
```bash
cd frontend
trunk serve --open
```
Frontend serves at: http://127.0.0.1:8080

## API Endpoints

- `GET /api/hello` - Returns JSON greeting message

## Technology Stack

### Backend
- **Axum** - Web framework
- **Tokio** - Async runtime
- **Serde** - JSON serialization
- **Tower-HTTP** - CORS middleware

### Frontend
- **Yew** - WebAssembly framework
- **Gloo-net** - HTTP client
- **Trunk** - Build tool and dev server

## Project Structure

```
website1/
├── backend/          # Axum server
│   ├── src/
│   └── Cargo.toml
├── frontend/         # Yew WebAssembly app
│   ├── src/
│   ├── dist/         # Built assets
│   └── Cargo.toml
└── Cargo.toml        # Workspace config
```
