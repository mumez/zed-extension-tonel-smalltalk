# Tonel Smalltalk extension for Zed

Zed extension that adds Tonel Smalltalk support:

- Syntax highlighting via tree-sitter grammar
- Language server integration for Tonel Smalltalk

## Current features

- Syntax highlighting for `.st` files
- "Go to definition" for class names
- "Find references" for class names

## Language server

The language server (`tonel-smalltalk-language-server`) is automatically downloaded when the extension is installed — no manual setup or PATH configuration is required.

The extension resolves the language server in this order:

1. Use `tonel-smalltalk-language-server` from your `PATH` if present
2. Otherwise download the latest release from [`mumez/tonel-smalltalk-language-server`](https://github.com/mumez/tonel-smalltalk-language-server) automatically

Supported platforms and their expected release asset names:

- `tonel-smalltalk-language-server-aarch64-apple-darwin.tar.gz`
- `tonel-smalltalk-language-server-x86_64-apple-darwin.tar.gz`
- `tonel-smalltalk-language-server-aarch64-unknown-linux-gnu.tar.gz`
- `tonel-smalltalk-language-server-x86_64-unknown-linux-gnu.tar.gz`
- `tonel-smalltalk-language-server-aarch64-pc-windows-msvc.zip`
- `tonel-smalltalk-language-server-x86_64-pc-windows-msvc.zip`

macOS/Linux assets use `.tar.gz`; Windows assets use `.zip`.

If the asset for your platform is missing from the latest release, Zed will show an installation error.

## Development

Build extension wasm:

```bash
cd extension
cargo check
```

Install in Zed as a dev extension from this repository root.
