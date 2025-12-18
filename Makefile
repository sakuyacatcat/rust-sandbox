PKG =
PKG_DIR = pkg/$(PKG)
OPTS ?=
ARGS ?=

check-pkg:
	@if [ -z "$(PKG)" ]; then echo "Error: PKG is required"; exit 1; fi

create: check-pkg
	@echo "Creating package: $(PKG)"
	cargo new $(PKG_DIR)

lib: check-pkg
	@echo "Adding module: $(PKG)"
	cargo new --lib $(PKG_DIR)

remove:
	@echo "Removing package: $(PKG)"
	rm -rf $(PKG_DIR)

check:
	@echo "Checking package: $(PKG)"
	@cd $(PKG_DIR) && cargo check

fmt: check-pkg
	@echo "Formatting package: $(PKG)"
	@cd $(PKG_DIR) && cargo fmt

build: check-pkg
	@echo "Building package: $(PKG)"
	@cd $(PKG_DIR) && cargo build

rbuild: check-pkg
	@echo "Building release package: $(PKG)"
	@cd $(PKG_DIR) && cargo build --release

run: check-pkg
	@echo "Running package: $(PKG)"
	@cd $(PKG_DIR) && cargo run --quiet $(ARGS) $(if $(OPTS), -- $(OPTS))

rrun: check-pkg
	@echo "Running release package: $(PKG)"
	@cd $(PKG_DIR) && cargo run --release --quiet $(ARGS) $(if $(OPTS), -- $(OPTS))

doc: check-pkg
	@echo "Documenting package: $(PKG)"
	@cd $(PKG_DIR) && cargo doc --open

add: check-pkg
	@echo "Adding package: $(LIB)"
	@cd $(PKG_DIR) && cargo add $(LIB)

update: check-pkg
	@echo "Updating package: $(PKG)"
	@cd $(PKG_DIR) && cargo update

clean: check-pkg
	@echo "Cleaning package: $(PKG)"
	@cd $(PKG_DIR) && cargo clean

test: check-pkg
	@echo "Testing package: $(PKG)"
	@cd $(PKG_DIR) && cargo test $(ARGS)

.PHONY: create lib remove check fmt build rbuild run rrun doc add update clean test
