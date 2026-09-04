# Kalcite Editor

`kalcite-editor` is the native graphical editor for Kalcite project directories.
It edits `.kscn` scene sources directly and reads project metadata from the
versioned Kalcite core crates.

## Features

- scene hierarchy and typed inspector;
- resource browser and node palette;
- 320×240 2D viewport with grid, zoom, and snapping;
- script, signals, resources, profiler, and tilemap tabs;
- undo/redo, immediate validation, and diagnostics console.

## KLC execution in the editor

The editor is currently a native Rust host, but its viewport snap policy is
implemented in `src/editor_core.klc` and compiled to Rust during the Cargo
build. This is an executable migration boundary: KLC owns the deterministic
integer snap decision while the host owns platform windows, file I/O, and
eframe integration. Future editor subsystems can move across this boundary
incrementally; the project does not claim a 70% KLC implementation yet.

## Install and run

Rust 1.88 or newer is required.

For the recommended full-toolchain setup, install
[Kallyup](https://github.com/Kalcite-Engine/kallyup) and run `kallyup install full`.
Manual installation remains available:

```bash
cargo install --path .
kalcite-editor /path/to/kalcite-project
```

For development:

```bash
cargo run -- /path/to/kalcite-project
```

When no path is supplied, the editor opens the current directory. A valid
project contains `kalcite.toml`; use the main Kalcite CLI to create one:

```bash
kalcite init MyGame --name MyGame
```

## Core compatibility

The editor is an independent product. Its runtime `kalcite-*` dependencies are
pinned to one tagged Kalcite core release. The KLC build pipeline is pinned to
one explicit compiler revision until that library emitter is included in the
next tag. Update each group together, regenerate `Cargo.lock`, and run the
full test suite.

## Development

```bash
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets
```

See [the development guide](docs/DEVELOPMENT.md) for contribution and release
rules.

## Related projects

- [Kalcite core](https://github.com/Kalcite-Engine/kalcite)
- [Kalcite LSP](https://github.com/Kalcite-Engine/kalcite-lsp)
- [Kalcite documentation](https://kalcite-engine.github.io/kalcite-docs/)
