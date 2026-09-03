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

The editor is an independent product. Its `kalcite-*` dependencies are pinned
to one tagged Kalcite core release. Update them together when supporting a new
core version, then regenerate `Cargo.lock` and run the full test suite.

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
