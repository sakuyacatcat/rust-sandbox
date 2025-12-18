# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust workspace sandbox containing multiple packages for exploring various Rust concepts. Each package in `pkg/` focuses on a specific Rust topic (closures, collections, concurrency, smart pointers, OOP, etc.).

## Build Commands

All commands use the Makefile and require specifying `PKG_NAME`:

```bash
# Build a package
make build PKG_NAME=minigrep

# Run a package
make run PKG_NAME=minigrep

# Run with arguments
make run PKG_NAME=minigrep OPTS="search_term filename.txt"

# Run tests
make test PKG_NAME=rust-test

# Run specific test
make test PKG_NAME=rust-test ARGS="test_name"

# Format code
make fmt PKG_NAME=minigrep

# Create new package
make create PKG_NAME=my-new-package

# Create library package
make lib PKG_NAME=my-lib
```

## Architecture

- **Workspace root**: `Cargo.toml` defines workspace members
- **Packages**: Each package in `pkg/` is independent with its own `Cargo.toml`
- **Tests**: Standard Rust test structure - unit tests in `src/`, integration tests in `tests/`
