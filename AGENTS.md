# Repository Guide for AI Agents

## Project Overview

md_porter is a browser-native Markdown inspector and safe HTML exporter with live preview. It uses a Rust Markdown parser compiled to WASM. The supported GFM subset includes tables, task lists, strikethrough, footnotes, autolinks, and code blocks. All processing happens in-browser — no server, no uploads.

## Architecture

```
md_porter/
├── crates/
│   ├── md_porter_core/        # Markdown parser, GFM extensions, HTML export
│   └── md_porter_web/         # WASM bridge + HTML split-pane editor
├── docs/                       # Product specification
├── skills/                     # Agent Skill definitions (MCP tools)
└── index.html                  # Product landing page
```

## Key Files for AI Context

| File | Purpose |
|------|---------|
| `crates/md_porter_core/src/convert.rs` | `md_to_html`, `md_to_plain_text`, `extract_frontmatter`, `wrap_markdown_document` |
| `crates/md_porter_core/src/error.rs` | `CoreError` enum with stable machine-readable codes |
| `crates/md_porter_web/src/lib.rs` | WASM bindings (5 JS exports + init hook) |
| `crates/md_porter_web/static/index.html` | Full-featured split-pane editor with live preview |
| `skills/md_porter.md` | Agent usage workflow |
| `skills/mcp-tools.json` | MCP tool definitions |
| `docs/product_spec.zh-CN.md` | Product specification |

## Build & Test Commands

```bash
# Run all tests
cargo test --workspace

# Format check
cargo fmt --all -- --check

# Lint (strict)
cargo clippy --workspace --all-targets -- -D warnings

# WASM compilation check
cargo check -p md_porter_web --target wasm32-unknown-unknown

# Build Web WASM for deployment
wasm-pack build --target web crates/md_porter_web
```

## Design Principles

1. **Browser-first**: All Markdown parsing and HTML generation happens in-browser via WASM
2. **No uploads ever**: Content never leaves the user's browser; no server component
3. **Bounded GFM subset**: Tables, task lists, strikethrough, footnotes, autolinks, and fenced code blocks via pulldown-cmark
4. **Zero-config export**: Download self-contained HTML with embedded CSS — no external dependencies
5. **Live preview**: Real-time WASM rendering as you type

## Core API

### convert.rs

- `md_to_html(markdown: &str) -> Result<String, CoreError>` — safe HTML with the supported GFM extensions
- `md_to_plain_text(markdown: &str) -> Result<String, CoreError>` — strip all formatting to plain text
- `extract_frontmatter(markdown: &str) -> Result<Option<HashMap<String, String>>, CoreError>` — YAML frontmatter extraction
- `wrap_markdown_document(markdown: &str, title: &str, css: &str) -> Result<String, CoreError>` — render and wrap into self-contained HTML

### error.rs

| Code | Meaning |
|------|---------|
| `EMPTY_INPUT` | Markdown string is empty |
| `PARSE_ERROR` | Markdown parsing failure |
| `CONVERSION_ERROR` | HTML/plain-text conversion failure |
| `FRONTMATTER_ERROR` | YAML frontmatter deserialization failure |

## Markdown Features Supported

pulldown-cmark features used:
- Tables (GFM)
- Task lists (`- [ ]` / `- [x]`)
- Strikethrough (`~~text~~`)
- Footnotes
- Autolinks
- Fenced code blocks with language classes

Raw HTML is escaped as text, images are represented by labels, and unsafe link
schemes are blocked. The parser is intentionally not a complete GitHub renderer.

The core rejects Markdown over 4 MiB and custom CSS over 256 KiB. Unsafe link
schemes are disabled and Markdown images are represented as text to prevent
external resource fetches.

## Commit Language

- Write commit subjects and bodies in English and follow Conventional Commits.
- This repository-level rule overrides any global preference for another commit-message language.

## Frontend Design Requirement

- Before creating, modifying, reviewing, or debugging any HTML page or user-facing frontend, invoke the `ui-ux-pro-max` skill.
- Run the skill's required `--design-system` search before editing, followed by relevant stack and UX searches.
- If `ui-ux-pro-max` is unavailable, stop frontend work and report the missing prerequisite.
- Verify the rendered result in a real browser at 375, 768, 1024, and 1440 pixel widths, including console, keyboard, accessibility, and overflow checks.
