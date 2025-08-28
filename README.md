# Full-Stack Rust Web Application

A modern web application built with Rust, featuring an Axum backend with SQLite database and Yew WebAssembly frontend with admin interface.

## Architecture

- **Backend**: Axum HTTP server with REST API and SQLx ORM
- **Database**: SQLite with automatic schema creation
- **Frontend**: Yew framework WebAssembly SPA with admin panel
- **Build System**: Cargo workspace with Trunk

## Features

- **CRUD Operations**: Full Create, Read, Update, Delete for users
- **Admin Interface**: Web-based admin panel similar to Django admin
- **Database ORM**: SQLx for type-safe database operations
- **Auto Schema**: Database schema automatically initialized on startup

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
Database automatically created and seeded

2. **Start Frontend** (in new terminal):
```bash
cd frontend
trunk serve --open
```
Frontend serves at: http://127.0.0.1:8080

## API Endpoints

### General
- `GET /api/hello` - Returns JSON greeting message

### User Management
- `GET /api/users` - List all users (supports ?limit=X&offset=Y)
- `GET /api/users/:id` - Get specific user
- `POST /api/users` - Create new user
- `PUT /api/users/:id` - Update user
- `DELETE /api/users/:id` - Delete user

### Request/Response Examples

**Create User:**
```json
POST /api/users
{
  "name": "John Doe",
  "email": "john@example.com"
}
```

**Update User:**
```json
PUT /api/users/:id
{
  "name": "Jane Doe",
  "email": "jane@example.com"
}
```

## Technology Stack

### Backend
- **Axum** - Web framework
- **SQLx** - Async SQL toolkit and ORM
- **SQLite** - Database
- **Tokio** - Async runtime
- **Serde** - JSON serialization
- **Tower-HTTP** - CORS middleware
- **UUID** - Unique identifiers
- **Chrono** - Date/time handling

### Frontend
- **Yew** - WebAssembly framework
- **Gloo-net** - HTTP client
- **Web-sys** - Web API bindings
- **Trunk** - Build tool and dev server

## Project Structure

```
website1/
├── backend/
│   ├── src/main.rs       # API server with CRUD operations
│   └── Cargo.toml
├── frontend/
│   ├── src/main.rs       # Admin interface SPA
│   ├── dist/             # Built assets
│   └── Cargo.toml
├── backend.db             # SQLite database (auto-created)
└── Cargo.toml            # Workspace config
```

## Database Schema

### Users Table
- `id`: TEXT (UUID primary key)
- `name`: TEXT (user's name)
- `email`: TEXT (unique email)
- `created_at`: TEXT (ISO 8601 timestamp)
- `updated_at`: TEXT (ISO 8601 timestamp)

## Admin Interface

The frontend provides a complete admin interface with:

- **User List**: View all users in a table
- **Create User**: Form to add new users
- **Edit User**: Inline editing of user details
- **Delete User**: Remove users with confirmation
- **Error Handling**: Display API errors to user
- **Real-time Updates**: Interface updates immediately after operations
