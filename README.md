# leafpad3
Leafpad rewritten in Rust.

# Dependencies

## From rustup

* cargo
* rustc

## Platform specific

* Gtk+3
* GtkSourceView

# Compiling

As root, anywhere:

```bash
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
```

## Building

The commands for building should be executed in the project root directory.

### Debug

```bash
cargo build
```

The binary will be located at `target/debug/leafpad3`

### Release

```bash
cargo build --release
```

The binary will be located at `target/release/leafpad3`

## Installing

This command has to be executed in the project root directory.

```bash
cargo install
```

## Uninstalling

This command is directory independent.

```bash
cargo uninstall leafpad3
```

