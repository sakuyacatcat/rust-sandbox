PKG_NAME ?= default
PKG_DIR = pkg/$(PKG_NAME)

create:
	@echo "Creating package: $(PKG_NAME)"
	cargo new $(PKG_DIR)

remove:
	@echo "Removing package: $(PKG_NAME)"
	rm -rf $(PKG_DIR)

check:
	@echo "Checking package: $(PKG_NAME)"
	@cd $(PKG_DIR) && cargo check

build:
	@echo "Building package: $(PKG_NAME)"
	@cd $(PKG_DIR) && cargo build

rbuild:
	@echo "Building release package: $(PKG_NAME)"
	@cd $(PKG_DIR) && cargo build --release

run:
	@echo "Running package: $(PKG_NAME)"
	@cd $(PKG_DIR) && cargo run --quiet $(if $(ARGS), -- $(ARGS))

rrun:
	@echo "Running release package: $(PKG_NAME)"
	@cd $(PKG_DIR) && cargo run --release --quiet $(if $(ARGS), -- $(ARGS))

clean:
	@echo "Cleaning package: $(PKG_NAME)"
	@cd $(PKG_DIR) && cargo clean

test:
	@echo "Testing package: $(PKG_NAME)"
	@cd $(PKG_DIR) && cargo test

.PHONY: create remove run clean test
