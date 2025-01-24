.PHONY: build build-release test clean run fmt-check fmt lint check

# Default target
all: build

# Development build
build:
	cargo build

# Release build with optimizations
build-release:
	cargo build --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Install release version locally
install: build-release
	cargo install --path .

# Check code formatting
fmt-check:
	cargo fmt --all -- --check

# Format code
fmt:
	cargo fmt --all

# Run clippy lints
lint:
	cargo clippy -- -D warnings

# Run all checks (format, lint, test)
check: fmt-check lint test

# Run the program
run: build
	./target/debug/passgen

# Run with release build
run-release: build-release
	./target/release/passgen 
