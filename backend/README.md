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

## Features

- **Database:** SQLite with SeaORM and migrations
- **Authentication:** JWT-based user authentication  
- **GraphQL API:** Auto-generated from SeaORM entities
- **Admin Panel:** SeaORM Pro web interface
- **CORS:** Enabled for frontend integration

## API Endpoints

### Health & Status
- `GET /health` - Server health check
- `GET /api/v1/status` - API status

### Authentication
- `POST /api/auth/login` - User login
- `GET /api/user/current` - Current user info (requires auth)

### Admin Panel
- `GET /admin` - Admin panel interface
- `GET /api/admin/config` - Panel configuration
- `GET /login` - Admin login page

### GraphQL
- `GET /api/graphql` - GraphQL playground
- `POST /api/graphql` - GraphQL API endpoint

## Configuration

### Environment Variables
Create `.env` file:
```env
DATABASE_URL=sqlite://./data.db
JWT_SECRET=your-secret-key
RUST_LOG=info
```

### Admin Panel Configuration
The admin panel is configured via TOML files in `pro_admin/`:

```
pro_admin/
├── config.toml              # Site configuration
└── raw_tables/
    └── users.toml           # Table-specific settings
```

**Example:** `pro_admin/raw_tables/users.toml`
```toml
[create]
enable = true
hidden_columns = ["id", "created_at", "updated_at"]
```

## Development

### Database Migrations
```bash
# Run migrations
cargo run
```

### Testing
```bash
cargo test
```

### Demo Login
- **Username:** demo@sea-ql.org
- **Password:** (any password works)

## Architecture

- **Framework:** Axum for HTTP server
- **Database:** SeaORM with SQLite
- **GraphQL:** Seaography for auto-generated schema
- **Admin:** SeaORM Pro for management interface
- **Auth:** JWT tokens with bcrypt password hashing

## Dependencies

```toml
[dependencies]
axum = "0.8"
sea-orm = { version = "1.1", features = ["runtime-tokio-rustls", "sqlx-sqlite"] }
seaography = { version = "1.1", features = ["with-chrono", "with-uuid"] }
sea-orm-pro = "0.1"
async-graphql = "7.0"
jsonwebtoken = "9.3"
```

## File Structure

```
src/
├── admin/              # Admin panel handlers
├── api/                # API route definitions  
├── auth/               # Authentication logic
├── entities/           # SeaORM entity definitions
├── graphql/            # GraphQL schema configuration
├── migration/          # Database migrations
├── lib.rs              # Library exports
└── main.rs             # Application entry point
```
