# Developing Kalcite Editor

## Scope

This repository owns the native editor user experience: scene authoring,
inspection, resource browsing, diagnostics presentation, and editor-specific
project workflows. The compiler, scene format, project model, and runtime stay
in the Kalcite core repository.

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

Changes to scene parsing, serialization, tilemaps, or inspector validation must
include a regression test in `src/main.rs`.

## Updating the core dependency

1. Start from a tagged Kalcite core release.
2. Update every `kalcite-*` Git dependency in `Cargo.toml` to the same tag.
3. Regenerate `Cargo.lock` with `cargo update`.
4. Verify that a representative project opens and saves correctly.
5. Run the local check set and CI.

## Release checklist

1. CI is green on `main`.
2. The core version is consistent across `Cargo.toml` and `Cargo.lock`.
3. Documentation accurately describes supported editor behavior.
4. Manual smoke testing covers opening a project, editing a scene, saving it,
   and reopening it.
