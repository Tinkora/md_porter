# Contributing to md_porter

Thanks for your interest in md_porter! Here's how to contribute.

## Development Environment

- Rust 1.95+ (stable)
- wasm-pack 0.15+
- wasm32-unknown-unknown target (`rustup target add wasm32-unknown-unknown`)

## Project Structure

```
md_porter/
├── crates/
│   ├── md_porter_core/       # Markdown parser, HTML export
│   └── md_porter_web/        # WASM bridge + HTML editor
├── docs/                      # Product specification
├── skills/                    # Agent Skill definitions
└── index.html                 # Landing page
```

## Local Development

```bash
# Run tests
cargo test --workspace

# Format & lint
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings

# Build Web WASM
wasm-pack build --target web crates/md_porter_web

# Start local editor
cp crates/md_porter_web/pkg/* crates/md_porter_web/static/pkg/
cd crates/md_porter_web/static && python3 -m http.server 8080
```

## Commit Convention

- Prefix: `feat:` / `fix:` / `docs:` / `refactor:` / `test:` / `chore:`
- Each commit should contain one logically complete change

## Pull Request Process

1. Fork the repo
2. Create a feature branch (`git checkout -b feat/your-feature`)
3. Commit your changes
4. Ensure `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` pass
5. Push to your fork (`git push origin feat/your-feature`)
6. Create a Pull Request

## Code of Conduct

Please read [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).
