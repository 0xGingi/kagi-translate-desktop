# Kagi Translate Desktop

A desktop application for [Kagi Translate](https://translate.kagi.com) built with Tauri.


## Install

### Binaries (Mac, Linux, Windows)

Download from https://github.com/0xGingi/kagi-translate-desktop/releases/latest

## Build

### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Bun](https://bun.sh/)

### Building

```bash
bun install

bun run tauri build --bundles
```

This will build to `src-tauri/target/release/bundle`.
