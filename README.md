# 🦀 Rust Full-Stack Web Application

> **Strategy:** Build a modern, type-safe web application using Rust across the entire stack for maximum performance, safety, and maintainability.

## ⚡ Quick Start — Basic Health Check Landing Page

- **Run dev env:** `bash dev.sh dev`
- **Frontend:** http://127.0.0.1:8080
- **Backend:**  http://127.0.0.1:3000
- **Health API:** `GET /health` returns `{ "status": "healthy", "timestamp": ... }`
- **UI:** A simple landing page with a "Check Health" button that calls the Health API and displays the result.

Troubleshooting:
- If frontend fails with `unresolved module or unlinked crate 'frontend'`, ensure `frontend/Cargo.toml` has:

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

## 🎯 Strategic Goals

### **Performance First**
- **Backend:** Async Rust with Axum for high-concurrency API handling
- **Frontend:** WebAssembly for near-native browser performance
- **Database:** SQLx for compile-time verified, zero-cost SQL queries

### **Type Safety Everywhere**
- End-to-end type safety from database to UI
- Compile-time error prevention
- Shared data models between frontend and backend

### **Developer Experience**
- Single command development workflow (`./dev.sh dev`)
- Hot reload for rapid iteration
- Comprehensive error messages and debugging

## 🏗️ Architecture Strategy

```
┌─────────────────┐    HTTP/JSON    ┌──────────────────┐
│   Yew Frontend  │ ◄──────────────►│  Axum Backend    │
│   (WebAssembly) │                 │  (API Server)    │
└─────────────────┘                 └──────────────────┘
         │                                     │
         │                                     │
    Browser APIs                        ┌─────────────┐
    Local Storage                       │   SQLite    │
                                        │  Database   │
                                        └─────────────┘
```

**Clean Architecture Principles:**
- **Domain-Driven Design:** Business logic isolated from infrastructure
- **Dependency Inversion:** High-level modules don't depend on low-level details
- **Separation of Concerns:** Each layer has a single responsibility

## 📋 Development Strategy

### **Best Practices Framework**

#### 🎯 **Code Quality Standards**
1. **Big Picture Comments** - Every file explains its purpose and role
2. **Beginner-Friendly Documentation** - Code reads like a story
3. **Single Responsibility** - Each function/module does one thing well
4. **Smart Modularization** - Balance between simplicity and organization
5. **Minimal Verbosity** - Elegant, concise, purposeful code
6. **Consistent Naming** - Use snake_case for functions and variables
7. **Maintainability** - Clear file organization and documentation with no file more than 500 lines of code

#### 🔧 **Technical Decisions**
- **Rust-First:** No JavaScript, minimal external dependencies
- **Type-Safe APIs:** Shared models between frontend/backend
- **Database-First:** Schema-driven development with migrations
- **Component Architecture:** Reusable, composable UI pieces
- **Error-First Design:** Comprehensive error handling and user feedback
- **Tailwind CSS:** For styling and layout
- **Theme:** see .env

#### 📦 **Dependency Strategy**
- **Proven Libraries:** Axum, Yew, SeaORM, SeaORM pro
- **Minimal Footprint:** Avoid dependencies for trivial functionality
- **Security-Focused:** Regular updates, vulnerability scanning
- **Performance-Oriented:** Zero-cost abstractions where possible

## 🚀 Quick Start Strategy

```bash
# Strategic workflow for maximum efficiency
./dev.sh setup    # One-time environment preparation
./dev.sh dev      # Start full development environment
./dev.sh test     # Comprehensive quality assurance
./dev.sh build    # Production-ready deployment
```

## 📊 Success Metrics

### **Performance Targets**
- API response times: < 100ms for basic operations
- Frontend bundle size: < 500KB optimized WebAssembly
- Database queries: < 10ms for indexed operations
- Build times: < 30s for incremental development builds

### **Code Quality Metrics**
- Test coverage: > 80% for business logic
- Documentation coverage: 100% for public APIs
- Clippy warnings: Zero tolerance policy
- Dependency vulnerabilities: Automated scanning and updates

### **Developer Experience Goals**
- Setup time: < 5 minutes for new developers
- Feedback loop: < 3 seconds for development changes
- Error clarity: Self-explanatory error messages
- Learning curve: Comprehensive beginner-friendly documentation

📚 **See [OVERVIEW.md](./OVERVIEW.md) for detailed setup and architecture information**
