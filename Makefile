.PHONY: help build run test clean dev fmt lint docker-build docker-run

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build the project in release mode
	cargo build --release

run: ## Run the project in development mode
	cargo run

dev: ## Run with auto-reload using cargo-watch
	cargo watch -x run

test: ## Run tests
	cargo test

test-verbose: ## Run tests with output
	cargo test -- --nocapture

fmt: ## Format code
	cargo fmt

lint: ## Run clippy linter
	cargo clippy -- -D warnings

check: fmt lint test ## Run format, lint, and tests

clean: ## Clean build artifacts
	cargo clean
	rm -rf uploads/images/* uploads/excels/*

docker-build: ## Build Docker image
	docker build -t clinic-api-rust:latest .

docker-run: ## Run Docker container
	docker-compose up -d

docker-stop: ## Stop Docker containers
	docker-compose down

docker-logs: ## View Docker logs
	docker-compose logs -f clinic-api

install-tools: ## Install development tools
	cargo install cargo-watch
	cargo install cargo-tarpaulin

coverage: ## Generate test coverage report
	cargo tarpaulin --out Html

setup: ## Initial setup (copy .env, create directories)
	cp .env.example .env
	mkdir -p uploads/images uploads/excels
	@echo "Setup complete! Edit .env with your configuration."
