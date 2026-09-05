PREFIX ?= /usr/local
DESTDIR ?=

.PHONY: build release test install bundle-macos
build:
	cargo build
release:
	cargo build --release
test:
	cargo fmt --all -- --check
	cargo test
install: release
	install -Dm755 target/release/kalcite-editor $(DESTDIR)$(PREFIX)/bin/kalcite-editor
	install -Dm755 target/release/kalcite-editor-info $(DESTDIR)$(PREFIX)/bin/kalcite-editor-info
	target/release/kalcite-editor-info linux $(DESTDIR)$(PREFIX)

bundle-macos: release
	target/release/kalcite-editor-info macos target/release/kalcite-editor "dist/Kalcite Editor.app"
