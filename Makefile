# Typing Practice - Makefile
# Simple commands to build, run, and manage the project

.PHONY: help build run release clean check test install dev fmt clippy all

# Default target - show help
help:
	@echo "Typing Practice - Available commands:"
	@echo ""
	@echo "  make run        - Run in debug mode"
	@echo "  make release    - Build and run optimized version"
	@echo "  make build      - Build debug version"
	@echo "  make build-rel  - Build release version"
	@echo "  make check      - Check code without building"
	@echo "  make clean      - Remove build artifacts"
	@echo "  make install    - Install to system"
	@echo "  make fmt        - Format code"
	@echo "  make clippy     - Run linter"
	@echo "  make all        - Build everything"
	@echo ""

# Run in debug mode
run:
	@echo "Running in debug mode..."
	cargo run

# Build and run release version
release:
	@echo "Building and running release version..."
	cargo run --release

# Build debug version
build:
	@echo "Building debug version..."
	cargo build

# Build release version
build-rel:
	@echo "Building release version..."
	cargo build --release

# Check code without building
check:
	@echo "Checking code..."
	cargo check

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	@echo "Clean complete!"

# Install to system
install:
	@echo "Installing to system..."
	cargo install --path .
	@echo "Installed! Run with: typing-practice"

# Development mode (watch for changes)
dev:
	@echo "Development mode - install cargo-watch first:"
	@echo "  cargo install cargo-watch"
	@echo ""
	cargo watch -x run

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Run clippy linter
clippy:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

# Build everything
all: fmt clippy build build-rel
	@echo "All builds complete!"

# Quick start (most common use)
start: release

# Development cycle
dev-check: fmt clippy check
	@echo "Development checks complete!"
