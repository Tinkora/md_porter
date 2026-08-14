# Changelog

All notable changes to Tinkora Markdown Porter are documented here.

## [Unreleased]

### Added

- Safe local Markdown-to-HTML preview with tables, task lists, strikethrough,
  footnotes, autolinks, code blocks, and frontmatter extraction.
- Raw HTML escaping, unsafe-link blocking, text-only image handling, and bounded
  Markdown/CSS inputs.
- Self-contained HTML export from trusted Markdown input.
- Bilingual documentation and an explicit non-Agent-callable boundary.

### Fixed

- Frontmatter values now serialize correctly with the current `serde_yaml` API.

## [0.1.0-alpha.1] - 2026-08-14

The first public alpha release provides a bounded local Markdown workflow with
safe preview, frontmatter inspection, and self-contained HTML export.
