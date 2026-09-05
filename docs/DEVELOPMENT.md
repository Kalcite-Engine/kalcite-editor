# Developing Kalcite Editor

## Scope

This repository owns the native editor user experience: scene authoring,
inspection, resource browsing, diagnostics presentation, and editor-specific
project workflows. The compiler, scene format, project model, and runtime stay
in the Kalcite core repository.

`src/editor_core.klc` is compiled by `build.rs` using the Kalcite compiler
pipeline. Keep this module deterministic and bounded: it contains editor
policy, while the Rust host retains native windowing, filesystem access, and
eframe-specific rendering. Update the KLC regression tests in `src/main.rs`
when changing a generated policy function.

## Running the editor

```bash
cargo run -- /path/to/kalcite-project
```

Use a project with `kalcite.toml`, `scenes/`, and `scripts/`. The editor writes
scene sources directly, so test on a disposable project when changing
serialization or scene-editing behavior.

## Local checks

```bash
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets
```

## Distribution metadata

`kalcite-editor-info` is the source of truth for platform file associations.
`kalcite-editor-info linux PREFIX` writes the freedesktop desktop entry and
shared-MIME XML under `PREFIX/share`; `kalcite-editor-info macos BINARY APP`
creates an `.app` bundle with its `Info.plist`. Use `make bundle-macos` for the
release bundle. Keep its tests updated whenever a project, scene, or script
file type is added.

Changes to scene parsing, serialization, tilemaps, or inspector validation must
include a regression test in `src/main.rs`.

## Updating the core dependency

1. Start from a tagged Kalcite core release for runtime dependencies.
2. Update runtime `kalcite-*` Git dependencies in `Cargo.toml` to that tag.
3. Update all KLC build dependencies to one compiler revision that exports
   `emit_library`; move them back to the release tag when it contains that API.
4. Regenerate `Cargo.lock` with `cargo update`.
5. Verify that a representative project opens and saves correctly.
6. Run the local check set and CI.

## Release checklist

1. CI is green on `main`.
2. The core version is consistent across `Cargo.toml` and `Cargo.lock`.
3. Documentation accurately describes supported editor behavior.
4. Manual smoke testing covers opening a project, editing a scene, saving it,
   and reopening it.
