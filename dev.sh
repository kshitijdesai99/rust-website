#!/bin/bash

# =============================================================================
# Development Script for Rust Website Project
# =============================================================================
# 
# This script helps you manage common development tasks without memorizing
# long command-line instructions. Think of it as your development assistant!
#
# What this script does:
# - Starts both frontend and backend servers for development
# - Builds the project for production deployment
# - Runs all tests to make sure everything works
# - Cleans up build files when things get messy
# - Installs tools you need for development
#
# Usage: ./dev.sh [command]
# Example: ./dev.sh dev (starts development servers)
# =============================================================================

# Stop the script immediately if any command fails (safety feature)
set -e

# Check what command the user wants to run
case "$1" in
  "dev")
    echo "🚀 Starting development environment..."
    echo ""
    echo "📊 Your servers will be available at:"
    echo "   Backend API:  http://127.0.0.1:3000"
    echo "   Frontend Web: http://127.0.0.1:8080"
    echo ""
    
    # Load environment variables from .env file (if it exists)
    # Environment variables are like settings for your app
    if [ -f ".env" ]; then
        echo "📋 Loading settings from .env file..."
        export $(grep -v '^#' .env | xargs)
    else
        echo "💡 No .env file found - using default settings"
    fi
    
    echo ""
    
    # Start the backend server in the background
    # The & symbol means "run this but don't wait for it to finish"
    echo "🔧 Starting backend server..."
    cd backend
    # Load backend-specific environment variables
    if [ -f ".env" ]; then
        export $(grep -v '^#' .env | xargs)
    fi
    RUST_LOG=info cargo run &
    BACKEND_PID=$!  # Remember the process ID so we can stop it later
    cd ..
    
    # Give the backend a moment to start up
    # This prevents the frontend from trying to connect before backend is ready
    echo "⏳ Waiting for backend to start up..."
    sleep 3
    
    # Start the frontend development server
    echo "🎨 Starting frontend development server..."
    cd frontend
    RUST_LOG=warn trunk serve --port 8080 &
    FRONTEND_PID=$!  # Remember this process ID too
    cd ..
    
    echo ""
    echo "✅ Development environment is ready!"
    echo "🔍 Process IDs for monitoring:"
    echo "   Backend:  $BACKEND_PID"  
    echo "   Frontend: $FRONTEND_PID"
    echo ""
    echo "💻 Open your browser to http://127.0.0.1:8080 to see your website"
    echo "🛑 Press Ctrl+C to stop both servers"
    
    # Set up a trap to clean up when user presses Ctrl+C
    # This ensures both servers get stopped properly
    trap "echo ''; echo '🛑 Stopping development servers...'; kill $BACKEND_PID $FRONTEND_PID 2>/dev/null; echo '✅ All servers stopped. Happy coding!'; exit" INT
    
    # Wait for user to press Ctrl+C
    wait
    ;;
    
  "build")
    echo "🏗️  Building project for production deployment..."
    echo ""
    
    # Load environment variables (production might need different settings)
    if [ -f ".env" ]; then
        echo "📋 Loading environment settings..."
        export $(grep -v '^#' .env | xargs)
    fi
    
    # Build the backend with optimizations (release mode)
    # Release mode makes the code run faster but takes longer to compile
    echo "🔧 Building optimized backend..."
    cd backend
    cargo build --release
    cd ..
    
    # Build the frontend with optimizations
    echo "🎨 Building optimized frontend..."
    cd frontend
    RUST_LOG=warn trunk build --release
    cd ..
    
    echo ""
    echo "✅ Production build complete!"
    echo "📁 Your built files are located at:"
    echo "   Backend executable: backend/target/release/backend"
    echo "   Frontend website:   frontend/dist/"
    echo ""
    echo "💡 You can now deploy these files to your production server"
    ;;
    
  "test")
    echo "🧪 Running all tests to make sure everything works..."
    echo ""
    
    # Run backend tests
    # Tests are like automated quality checks for your code
    echo "🔧 Testing backend functionality..."
    cd backend
    cargo test
    cd ..
    
    echo ""
    
    # Run frontend tests  
    echo "🎨 Testing frontend functionality..."
    cd frontend
    cargo test
    cd ..
    
    echo ""
    echo "✅ All tests passed! Your code is working correctly."
    ;;
    
  "clean")
    echo "🧹 Cleaning up build files and temporary data..."
    echo ""
    
    # Remove backend build artifacts
    echo "🔧 Cleaning backend build files..."
    cd backend
    cargo clean
    
    # Remove frontend build artifacts
    echo "🎨 Cleaning frontend build files..."
    cd ../frontend
    cargo clean
    rm -rf dist/  # Remove the built website files
    cd ..
    
    echo ""
    echo "✅ Cleanup complete! All build files have been removed."
    echo "💡 Run './dev.sh build' when you want to build again"
    ;;
    
  "setup")
    echo "⚙️  Installing development tools and dependencies..."
    echo ""
    
    # Check if trunk is installed (frontend build tool)
    if ! command -v trunk &> /dev/null; then
        echo "📦 Installing Trunk (frontend build tool)..."
        cargo install trunk
    else
        echo "✅ Trunk is already installed"
    fi
    
    # Check if wasm-pack is installed (WebAssembly build tool)
    if ! command -v wasm-pack &> /dev/null; then
        echo "📦 Installing wasm-pack (WebAssembly tool)..."
        cargo install wasm-pack
    else
        echo "✅ wasm-pack is already installed"
    fi
    
    # Add WebAssembly target for Rust compiler
    echo "🎯 Adding WebAssembly compilation target..."
    rustup target add wasm32-unknown-unknown
    
    echo ""
    echo "✅ Development environment setup complete!"
    echo "💡 You can now run './dev.sh dev' to start developing"
    ;;
    
  "help"|"--help"|"-h")
    echo "🦀 Rust Website Development Script"
    echo ""
    echo "This script helps you manage your full-stack Rust web application."
    echo "It handles both the backend API server and the frontend web interface."
    echo ""
    echo "📋 Available Commands:"
    echo ""
    echo "  🚀 dev      Start development servers (backend + frontend)"
    echo "             Backend runs on http://127.0.0.1:3000"
    echo "             Frontend runs on http://127.0.0.1:8080"
    echo ""
    echo "  🏗️  build    Build optimized version for production deployment"
    echo "             Creates fast, small files ready for your web server"
    echo ""
    echo "  🧪 test     Run all automated tests to verify code quality"
    echo "             Checks both backend and frontend functionality"
    echo ""
    echo "  🧹 clean    Remove all build files and temporary data"
    echo "             Use this when builds are acting weird"
    echo ""
    echo "  ⚙️  setup    Install required development tools (first-time setup)"
    echo "             Installs Trunk, wasm-pack, and WebAssembly targets"
    echo ""
    echo "  ❓ help     Show this help message"
    echo ""
    echo "💡 Examples:"
    echo "   ./dev.sh setup    (run this first time)"
    echo "   ./dev.sh dev      (start coding!)"
    echo "   ./dev.sh test     (make sure it works)"
    echo "   ./dev.sh build    (prepare for deployment)"
    echo ""
    ;;
    
  *)
    echo "❌ Unknown command: '$1'"
    echo ""
    echo "💡 Run './dev.sh help' to see available commands"
    echo "🚀 Quick start: './dev.sh setup' then './dev.sh dev'"
    exit 1
    ;;
esac