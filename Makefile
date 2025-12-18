PKG ?= default
PKG_DIR = pkg/$(PKG)
OPTS ?= ""
ARGS ?= ""

create:
	@echo "Creating package: $(PKG)"
	cargo new $(PKG_DIR)

lib:
	@echo "Adding module: $(PKG)"
	cargo new --lib $(PKG_DIR)

remove:
	@echo "Removing package: $(PKG)"
	rm -rf $(PKG_DIR)

check:
	@echo "Checking package: $(PKG)"
	@cd $(PKG_DIR) && cargo check

fmt:
	@echo "Formatting package: $(PKG)"
	@cd $(PKG_DIR) && cargo fmt

build:
	@echo "Building package: $(PKG)"
	@cd $(PKG_DIR) && cargo build

rbuild:
	@echo "Building release package: $(PKG)"
	@cd $(PKG_DIR) && cargo build --release

run:
	@echo "Running package: $(PKG)"
	@cd $(PKG_DIR) && cargo run --quiet $(ARGS) $(if $(OPTS), -- $(OPTS))

rrun:
	@echo "Running release package: $(PKG)"
	@cd $(PKG_DIR) && cargo run --release --quiet $(ARGS) $(if $(OPTS), -- $(OPTS))

doc:
	@echo "Documenting package: $(PKG)"
	@cd $(PKG_DIR) && cargo doc --open

add:
	@echo "Adding package: $(LIB)"
	@cd $(PKG_DIR) && cargo add $(LIB)

update:
	@echo "Updating package: $(PKG)"
	@cd $(PKG_DIR) && cargo update

clean:
	@echo "Cleaning package: $(PKG)"
	@cd $(PKG_DIR) && cargo clean

test:
	@echo "Testing package: $(PKG)"
	@cd $(PKG_DIR) && cargo test $(ARGS)

.PHONY: create remove run clean test
