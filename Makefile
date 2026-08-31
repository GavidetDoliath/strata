.PHONY: start-dev run-dev build install-local

PREFIX ?= $(HOME)/.local
INSTALL_DIR ?= $(PREFIX)/opt/strata
BIN_DIR ?= $(PREFIX)/bin

start-dev:
	./scripts/dev.sh

run-dev:
	cargo build --bins
	./target/debug/strata

build:
	cargo build --release --bins

install-local: build
	pkill -x strata 2>/dev/null || true
	install -Dm755 target/release/strata "$(INSTALL_DIR)/strata"
	install -Dm755 target/release/strata-preview-helper "$(INSTALL_DIR)/strata-preview-helper"
	mkdir -p "$(BIN_DIR)"
	ln -sfn "$(INSTALL_DIR)/strata" "$(BIN_DIR)/strata"
