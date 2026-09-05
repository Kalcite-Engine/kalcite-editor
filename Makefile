PREFIX ?= /usr/local
DESTDIR ?=

.PHONY: build release test install
build:
	cargo build
release:
	cargo build --release
test:
	cargo fmt --all -- --check
	cargo test
install: release
	install -Dm755 target/release/kalcite-editor $(DESTDIR)$(PREFIX)/bin/kalcite-editor
	install -Dm644 resources/kalcite-editor.desktop $(DESTDIR)$(PREFIX)/share/applications/kalcite-editor.desktop
