# md_porter Product Specification

## One-line product

Write Markdown in a browser, review safe HTML live, and export one
self-contained HTML file without external resources. All processing runs in
Rust WebAssembly in the browser.

## Users and problem

- Developers checking README files, changelogs, and technical documentation.
- People who do not want to paste private Markdown into an unknown online service.
- Agent-assisted documentation workflows where a person reviews and exports the draft.

## Core workflow

1. Open the editor or paste Markdown.
2. Edit Markdown in the left pane and inspect a safe preview in the right pane.
3. Insert common syntax with toolbar actions and keyboard shortcuts.
4. Inspect YAML frontmatter keys in the status bar.
5. Export a self-contained HTML document.
6. Keep preview and export free of automatic remote image or resource fetches.

## Technical boundaries

- Rust `pulldown-cmark` 0.11 compiled to WASM.
- Tables, task lists, strikethrough, footnotes, autolinks, and heading attributes.
- Plain HTML/CSS/JS frontend with no framework dependency.
- Blob download in the browser; no application server.

## Supported Markdown

The renderer supports common headings, emphasis, strikethrough, inline and
fenced code, block quotes, lists, thematic breaks, GFM tables, task lists,
footnotes, autolinks, and YAML frontmatter extraction. Fenced-code language
classes are preserved, but syntax highlighting is not bundled.

Raw HTML is escaped as text. Link destinations are limited to `http`, `https`,
`mailto`, relative, and fragment URLs. Unsafe schemes and protocol-relative
URLs are replaced with a local blocked-link marker. Images become text labels;
they are never rendered as network-fetching `<img>` elements.

## Limits and security

- Markdown input: 4 MiB maximum.
- Custom export CSS: 256 KiB maximum.
- Exported HTML contains embedded CSS and no JavaScript or external resources.
- The public API exports from Markdown, not caller-provided HTML, so untrusted
  HTML cannot be smuggled into the document wrapper.

## Non-goals

- Online hosting, sharing, or collaborative editing.
- Direct PDF generation.
- Image upload, proxying, or remote asset fetching.
- A promise of complete GitHub rendering or bundled syntax highlighting.
- A runnable MCP server or implicit Agent permissions.

## Acceptance criteria

- Empty input returns `EMPTY_INPUT`; over-limit input returns `INPUT_TOO_LARGE`.
- GFM tables, task lists, strikethrough, and footnotes render deterministically.
- Raw HTML is escaped, unsafe links are disabled, and images do not produce `<img>`.
- YAML frontmatter becomes stable key/value output.
- Titles and custom CSS cannot break the exported HTML document boundary.
- Native tests, Clippy, WASM compilation, documentation checks, and real-browser
  checks pass at 375, 768, 1024, and 1440 pixel widths.
