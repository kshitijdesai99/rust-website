# User Admin Panel

A full-stack Rust web application for user management with a clean, modern architecture following best practices.

## Architecture

### Backend (Axum + SQLite)
```
backend/src/
├── main.rs           # Application entry point
├── lib.rs            # Library root
├── config.rs         # Configuration management
├── errors.rs         # Error types and handling
├── utils.rs          # Utility functions
├── models/           # Data models
│   ├── mod.rs
│   └── user.rs       # User model and DTOs
├── database/         # Database layer
│   ├── mod.rs
│   └── connection.rs # Database connection and migrations
├── repositories/     # Data access layer
│   ├── mod.rs
│   └── user_repository.rs
├── services/         # Business logic layer
│   ├── mod.rs
│   └── user_service.rs
└── handlers/         # HTTP handlers
    ├── mod.rs
    ├── health.rs     # Health check endpoints
    └── user.rs       # User API endpoints
```

### Frontend (Yew + WebAssembly)
```
frontend/src/
├── main.rs           # Application entry point
├── lib.rs            # Library root
├── utils.rs          # Utility functions
├── models/           # Data models
│   ├── mod.rs
│   └── user.rs       # User model and form data
├── services/         # API services
│   ├── mod.rs
│   └── api.rs        # HTTP client for backend API
├── hooks/            # Custom hooks for state management
│   ├── mod.rs
│   └── use_users.rs  # User management hooks
└── components/       # UI components
    ├── mod.rs
    ├── user_form.rs  # User creation form
    ├── user_list.rs  # User table display
    └── user_row.rs   # Individual user row
```

## Features

### Backend
- **Clean Architecture**: Layered architecture with clear separation of concerns
- **Error Handling**: Comprehensive error handling with structured error types
- **Input Validation**: Request validation using the `validator` crate
- **Logging**: Structured logging with `tracing`
- **Configuration**: Environment-based configuration
- **Database**: SQLite with automatic migrations and seeding
- **CORS**: Properly configured CORS for frontend communication

### Frontend
- **Component Architecture**: Modular components with clear responsibilities  
- **State Management**: Custom hooks for state management with useReducer
- **Type Safety**: Full type safety with shared models
- **Responsive Design**: Mobile-first responsive CSS
- **Error Handling**: User-friendly error display and management
- **Modern UI**: Clean, accessible user interface

## API Endpoints

- `GET /api/health` - Health check
- `GET /api/users` - List all users (with pagination support)
- `GET /api/users/:id` - Get user by ID
- `POST /api/users` - Create new user
- `PUT /api/users/:id` - Update user
- `DELETE /api/users/:id` - Delete user

## Development

### Prerequisites
- Rust 1.70+ 
- `trunk` for frontend development: `cargo install trunk`
- `wasm-pack` for WebAssembly: `cargo install wasm-pack`

### Running the Application

1. **Start the backend server:**
   ```bash
   cd backend
   cargo run
   ```
   Backend will run on `http://127.0.0.1:3000`

2. **Start the frontend development server:**
   ```bash
   cd frontend
   trunk serve
   ```
   Frontend will run on `http://127.0.0.1:8080`

### Environment Variables

Backend supports the following environment variables:
- `DATABASE_URL`: SQLite database file path (default: `backend.db`)
- `HOST`: Server host (default: `127.0.0.1`)
- `PORT`: Server port (default: `3000`)

### Building for Production

1. **Backend:**
   ```bash
   cd backend
   cargo build --release
   ```

2. **Frontend:**
   ```bash
   cd frontend
   trunk build --release
   ```

## Code Quality Features

### Backend Best Practices
- **Modular Structure**: Clear separation between handlers, services, repositories
- **Error Handling**: Custom error types with proper HTTP status mapping
- **Input Validation**: Comprehensive request validation
- **Database Layer**: Repository pattern with proper error handling
- **Logging**: Structured logging throughout the application
- **Type Safety**: Strong typing for all data structures

### Frontend Best Practices  
- **Component Architecture**: Single-responsibility components
- **State Management**: Reducer pattern for complex state
- **Custom Hooks**: Reusable logic encapsulation
- **Type Safety**: Shared types between frontend and backend
- **Error Boundaries**: Proper error handling and display
- **Responsive Design**: Mobile-first approach

## Testing

To run tests:
```bash
# Backend tests
cd backend
cargo test

# Frontend tests
cd frontend  
cargo test
```

## Project Structure Benefits

1. **Maintainability**: Clear module boundaries make the code easy to understand and modify
2. **Testability**: Each layer can be tested independently
3. **Scalability**: Easy to add new features without affecting existing code
4. **Code Reuse**: Shared types and utilities across the application
5. **Performance**: Efficient state management and minimal re-renders
6. **Developer Experience**: Good error messages and debugging capabilities
