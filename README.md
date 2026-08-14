# Tinkora Markdown Porter

Markdown Porter is a local Markdown inspector and safe HTML exporter. It runs
the Rust parser in WebAssembly, so private notes and documentation stay in the
browser while you review the result and download one self-contained HTML file.

This project is currently **Alpha**. The first public release is intentionally
narrow: it does not claim PDF generation, complete GitHub rendering, bundled
syntax highlighting, or an MCP transport.

## What It Solves

Online Markdown editors are convenient, but pasting private documentation into
an unknown service creates an unnecessary data boundary. Markdown Porter gives
developers and agents a small, auditable local workflow for:

- previewing common GFM structures;
- checking plain-text and YAML frontmatter output;
- exporting a self-contained HTML document without external resources.

## Safety Contract

- Markdown is processed locally; the application has no server or upload path.
- Raw HTML is displayed as text instead of being executed.
- `http`, `https`, `mailto`, relative, and fragment links are retained. Unsafe
  schemes are replaced by a local blocked-link marker.
- Images are shown as text labels, so preview and exported files do not fetch
  remote resources.
- Markdown input is limited to 4 MiB; custom export CSS is limited to 256 KiB.
- Exported documents contain embedded CSS and no JavaScript.

## Supported Markdown

The current parser enables tables, task lists, strikethrough, footnotes,
autolinks, headings, lists, block quotes, and fenced code blocks. Code language
classes are preserved for consumers that provide their own highlighting. Raw
HTML, remote images, and arbitrary URL schemes are deliberately outside the
contract.

## Try It Locally

```bash
git clone https://github.com/Tinkora/md_porter.git
cd md_porter
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
wasm-pack build --target web crates/md_porter_web --out-dir static/pkg
python3 -m http.server 8080 --directory crates/md_porter_web/static
```

Open `http://127.0.0.1:8080/` after the build completes. The browser page is
the product; there is no backend to configure.

## Development

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p md_porter_web --target wasm32-unknown-unknown
```

Frontend changes must also be checked in a real browser at 375, 768, 1024,
and 1440 pixels. See [AGENTS.md](AGENTS.md) for the UI workflow and security
boundaries.

## Agent Integration Status

The repository contains a draft schema describing local Markdown operations.
It is **not Agent-callable**: there is no runnable MCP server, registration
flow, permission model, or end-to-end transport test yet.

## Documentation

- [Chinese README](README.zh-CN.md)
- [Product specification](docs/product_spec.md)
- [Chinese product specification](docs/product_spec.zh-CN.md)
- [Security policy](SECURITY.md)
- [Contributing guide](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)

## Support

[Support Tinkora on Ko-fi](https://ko-fi.com/tinkora)

## License

MIT. See [LICENSE](LICENSE).
