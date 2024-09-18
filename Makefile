PKG_NAME ?= default
PKG_DIR = pkg/$(PKG_NAME)

create:
	@echo "Creating package: $(PKG_NAME)"
	cargo new $(PKG_DIR)

remove:
	@echo "Removing package: $(PKG_NAME)"
	rm -rf $(PKG_DIR)

run:
	@echo "Running package: $(PKG_NAME)"
	@cd $(PKG_DIR) && cargo run --quiet

.PHONY: create remove run
