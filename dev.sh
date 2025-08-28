#!/bin/bash

# Development script for User Admin Panel

set -e

case "$1" in
  "dev")
    echo "Starting development environment..."
    echo "Backend will run on http://127.0.0.1:3000"
    echo "Frontend will run on http://127.0.0.1:8080"
    echo ""
    
    # Start backend in background
    echo "Starting backend..."
    cd backend
    RUST_LOG=debug cargo run &
    BACKEND_PID=$!
    cd ..
    
    # Wait a moment for backend to start
    sleep 3
    
    # Start frontend
    echo "Starting frontend..."
    cd frontend
    trunk serve --port 8080 &
    FRONTEND_PID=$!
    cd ..
    
    echo ""
    echo "Development environment started!"
    echo "Backend PID: $BACKEND_PID"  
    echo "Frontend PID: $FRONTEND_PID"
    echo "Press Ctrl+C to stop both services"
    
    # Wait for interrupt
    trap "echo 'Stopping services...'; kill $BACKEND_PID $FRONTEND_PID 2>/dev/null; exit" INT
    wait
    ;;
    
  "build")
    echo "Building for production..."
    
    # Build backend
    echo "Building backend..."
    cd backend
    cargo build --release
    cd ..
    
    # Build frontend
    echo "Building frontend..."
    cd frontend
    trunk build --release
    cd ..
    
    echo "Production build complete!"
    echo "Backend binary: backend/target/release/backend"
    echo "Frontend dist: frontend/dist/"
    ;;
    
  "test")
    echo "Running tests..."
    
    # Test backend
    echo "Testing backend..."
    cd backend
    cargo test
    cd ..
    
    # Test frontend  
    echo "Testing frontend..."
    cd frontend
    cargo test
    cd ..
    
    echo "All tests completed!"
    ;;
    
  "clean")
    echo "Cleaning build artifacts..."
    cd backend
    cargo clean
    cd ../frontend
    cargo clean
    rm -rf dist/
    cd ..
    echo "Clean complete!"
    ;;
    
  "deps")
    echo "Installing dependencies..."
    
    # Check for required tools
    if ! command -v trunk &> /dev/null; then
        echo "Installing trunk..."
        cargo install trunk
    fi
    
    if ! command -v wasm-pack &> /dev/null; then
        echo "Installing wasm-pack..."
        cargo install wasm-pack
    fi
    
    echo "Dependencies installed!"
    ;;
    
  *)
    echo "User Admin Panel Development Script"
    echo ""
    echo "Usage: $0 {dev|build|test|clean|deps}"
    echo ""
    echo "Commands:"
    echo "  dev     Start development environment (backend + frontend)"
    echo "  build   Build for production"
    echo "  test    Run all tests"
    echo "  clean   Clean build artifacts"
    echo "  deps    Install required development dependencies"
    echo ""
    exit 1
    ;;
esac
