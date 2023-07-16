# format-bom

utf-8 BOM formatter written in Rust

## Installation

This tool provides with `cargo install` command.

```bash
cargo install format-bom
```

## Usage

```bash
format-bom 
```

### Format Mode

```bash
format-bom --add
```

There is `remove` `add` `add-strict` mode for format-bom.
It uses `remove` mode by default.

`remove` mode removes BOM from UTF-8 files.
`add` mode adds BOM to UTF-8 files except for files that has following extensions.

- html
- css
- svg
- js
- ts
- md
- json
- toml
- yaml
- csv
- xml
- ini
- conf
- cfg
- sh
- bat
- ps1

`add-strict` mode adds BOM to all UTF-8 files.
