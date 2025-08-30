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
    ├── Makefiles - Project-specific commands
    ├── Hot reload - Automatic browser refresh
    └── Integrated testing - Backend + Frontend tests
```

## 🚀 Quick Start
```bash
# Make dev script executable (first time only)
chmod +x dev.sh

# Install tools and start developing
./dev.sh setup
./dev.sh dev
```

Visit http://127.0.0.1:8080 to see your website! 🌟
Visit http://127.0.0.1:3000 to see your API server! 🌟
Visit http://127.0.0.1:3000/health to see your health! 🌟
Visit http://127.0.0.1:3000/login to see admin panel! 🌟

