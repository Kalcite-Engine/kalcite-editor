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
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	install -m755 target/release/kalcite-editor $(DESTDIR)$(PREFIX)/bin/kalcite-editor
	install -m755 target/release/kalcite-editor-info $(DESTDIR)$(PREFIX)/bin/kalcite-editor-info
	target/release/kalcite-editor-info linux $(DESTDIR)$(PREFIX)
	@if test -z "$(DESTDIR)"; then \
		if command -v update-mime-database >/dev/null 2>&1; then update-mime-database $(PREFIX)/share/mime; fi; \
		if command -v update-desktop-database >/dev/null 2>&1; then update-desktop-database $(PREFIX)/share/applications; fi; \
	fi

bundle-macos: release
	target/release/kalcite-editor-info macos target/release/kalcite-editor "dist/Kalcite Editor.app"
