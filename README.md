# Tonel Smalltalk extension for Zed

Zed extension that adds Tonel Smalltalk support:

- Syntax highlighting via tree-sitter grammar
- Language server integration for Tonel Smalltalk

## Current features

- Syntax highlighting for `.st` files
- Go-to-definition for class names (provided by the language server)

## Language server installation

The extension resolves `tonel-smalltalk-language-server` in this order:

1. Use `tonel-smalltalk-language-server` from your `PATH` if present
2. Otherwise download the latest GitHub release asset from:
   - `mumez/tonel-smalltalk-language-server`

Expected release asset names:

- `tonel-smalltalk-language-server-aarch64-apple-darwin.tar.gz`
- `tonel-smalltalk-language-server-x86_64-apple-darwin.tar.gz`
- `tonel-smalltalk-language-server-aarch64-unknown-linux-gnu.tar.gz`
- `tonel-smalltalk-language-server-x86_64-unknown-linux-gnu.tar.gz`
- `tonel-smalltalk-language-server-aarch64-pc-windows-msvc.zip`
- `tonel-smalltalk-language-server-x86_64-pc-windows-msvc.zip`

macOS/Linux assets are expected as `.tar.gz`; Windows assets are expected as `.zip`.

If your platform asset is missing in the latest release, Zed will show an installation error.

## Development

Build extension wasm:

```bash
cd extension
cargo check
```

Install in Zed as a dev extension from this repository root.
