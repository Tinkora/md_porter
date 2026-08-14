# md_porter Agent Skill

A browser-native Markdown inspector and safe HTML exporter. It uses a Rust Markdown parser compiled to WASM. The supported GFM subset includes tables, task lists, strikethrough, footnotes, autolinks, and code blocks.

## Workflow

1. **Open Editor**: Agent directs user to open the md_porter editor URL.
2. **Author Content**: User writes or pastes Markdown in the left pane; right pane shows live HTML preview.
3. **Review & Edit**: User can use toolbar buttons or keyboard shortcuts for formatting.
4. **Export**: User clicks "Export HTML" to download a self-contained HTML file.
5. **Optional**: Agent can pre-fill content or provide Markdown for the user to paste.

## Tool Definitions

### `render_markdown`

Convert Markdown to safe HTML and extract plain text/frontmatter.

**Parameters:**
- `markdown` (string, required): The Markdown source text to convert.

**Returns:**
- `html`: Rendered HTML string with the supported GFM extensions applied.
- `plain_text`: Plain text with all formatting stripped.
- `frontmatter`: Parsed YAML frontmatter as key-value pairs (or null if none).
- Raw HTML is escaped, unsafe links are disabled, and images are represented as text.

### `export_html_document`

Convert Markdown into a complete self-contained HTML document.

**Parameters:**
- `markdown` (string, required): Markdown source text.
- `title` (string, optional): Document title. Defaults to "Untitled".
- `css` (string, optional): Custom CSS to embed. If empty, uses default GitHub-like stylesheet.

**Returns:**
- `html_document`: Complete self-contained HTML document string.

### `get_default_css`

Return the default GitHub-like CSS stylesheet embedded in exported documents.

**Parameters:** none

**Returns:**
- `css`: The default CSS string.

## Agent Rules

- Always process Markdown in-browser; never send content to a server.
- When user provides Markdown, convert and return the HTML preview.
- For export, use `export_html_document` so the Markdown is rendered inside the trusted Rust core.
- The default CSS provides GitHub-like styling with dark mode support.
- Frontmatter is optional; handle both cases gracefully.
- Code block language annotations are preserved as classes; syntax highlighting is not bundled.
- Task lists render as disabled checkboxes in preview; they are visual only.
- Tables require proper header separator (`|---|---|`) to render correctly.
- Markdown input is limited to 4 MiB and custom CSS to 256 KiB.

## Privacy & Security

- All processing happens in-browser via WASM. No server component.
- No data is ever uploaded or stored externally.
- The exported HTML file is fully self-contained with no external resource requests.
- The default CSS includes `prefers-color-scheme: dark` support for accessibility.
