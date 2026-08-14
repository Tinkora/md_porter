# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x (current) | ✅ |

## Reporting a Vulnerability

If you discover a security vulnerability, please **do not** open a public issue.

Instead, email the project maintainer directly. You should receive a response
within 48 hours. We will work with you to understand the scope and coordinate
a fix and disclosure timeline.

### Scope

The following areas are within scope:

- WASM sandbox escapes
- Malicious Markdown input causing unbounded memory growth
- YAML frontmatter parsing vulnerabilities
- HTML injection vectors in rendered output
- Self-contained HTML export injection

### Out of Scope

- Issues already documented as known limitations
- Theoretical attacks requiring physical access
- Issues in dependencies (please report upstream)

## Security Model

The md_porter project follows these security principles:

1. **Fully browser-local**: All Markdown parsing, HTML generation, and file export happens in-browser via WASM. No data ever leaves the user's machine.

2. **No server component**: There is no backend, no API, no database. The application is purely static HTML + WASM.

3. **Safe HTML output**: Raw HTML events are escaped as text. Link destinations are limited to `http`, `https`, `mailto`, relative, and fragment URLs; unsafe schemes are replaced with a local blocked-link marker. Markdown images are represented as text so preview and exported files do not fetch remote resources.

4. **Content Security Policy ready**: The static editor generates no inline scripts from user content. All interactions are event-driven.

5. **No user-generated file execution**: Downloaded `.html` files are static documents with embedded CSS. They contain no JavaScript.

6. **Resource bounds**: Markdown input is limited to 4 MiB and custom export CSS to 256 KiB before parsing or allocation.
