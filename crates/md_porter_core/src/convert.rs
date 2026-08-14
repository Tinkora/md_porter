use std::collections::HashMap;

use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd, html};

use crate::error::CoreError;

pub const MAX_MARKDOWN_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_CUSTOM_CSS_BYTES: usize = 256 * 1024;

/// Convert Markdown string to safe HTML with the supported GFM extensions.
///
/// Supports tables, task lists, strikethrough, footnotes, autolinks,
/// and code blocks with language annotations. Raw HTML is rendered as text,
/// unsafe links are disabled, and image references are represented as text so
/// preview and export never fetch remote resources.
pub fn md_to_html(markdown: &str) -> Result<String, CoreError> {
    validate_markdown(markdown)?;

    let mut options = Options::empty();
    // Enable all GFM features
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(markdown, options);

    let mut html_output = String::with_capacity(markdown.len() * 2);
    html::push_html(&mut html_output, sanitize_events(parser).into_iter());

    Ok(html_output)
}

/// Strip all Markdown formatting and return plain text.
///
/// Removes links, images, code spans, emphasis markers, headings,
/// block quotes, lists, tables, and all other formatting.
pub fn md_to_plain_text(markdown: &str) -> Result<String, CoreError> {
    validate_markdown(markdown)?;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);

    let mut plain = String::with_capacity(markdown.len());
    let mut in_table = false;

    for event in parser {
        match event {
            Event::Text(text) | Event::Code(text) if !in_table => {
                plain.push_str(&text);
            }
            Event::SoftBreak | Event::HardBreak if !in_table => {
                plain.push('\n');
            }
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    if !plain.is_empty() && !plain.ends_with('\n') {
                        plain.push('\n');
                    }
                    for _ in 0..level as usize {
                        plain.push('#');
                    }
                    plain.push(' ');
                }
                Tag::Table(_) => {
                    in_table = true;
                }
                Tag::Item => {
                    if !plain.is_empty() && !plain.ends_with('\n') {
                        plain.push('\n');
                    }
                    plain.push_str("- ");
                }
                Tag::CodeBlock(_) if !plain.is_empty() && !plain.ends_with('\n') => {
                    plain.push('\n');
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Heading(_) => {
                    plain.push('\n');
                }
                TagEnd::Table => {
                    in_table = false;
                }
                TagEnd::Paragraph if !plain.is_empty() && !plain.ends_with('\n') => {
                    plain.push('\n');
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(plain.trim().to_string())
}

/// Extract YAML frontmatter from a Markdown string.
///
/// Returns `None` if no frontmatter is present.
/// Returns a HashMap of key-value pairs if frontmatter is found and valid YAML.
///
/// Frontmatter must be at the very start of the document,
/// delimited by `---` on its own lines.
///
/// # Example
///
/// ```markdown
/// ---
/// title: My Document
/// date: 2026-08-07
/// tags: [rust, wasm, markdown]
/// ---
///
/// # Content starts here
/// ```
pub fn extract_frontmatter(markdown: &str) -> Result<Option<HashMap<String, String>>, CoreError> {
    if markdown.len() > MAX_MARKDOWN_BYTES {
        return Err(CoreError::InputTooLarge {
            limit: MAX_MARKDOWN_BYTES,
        });
    }
    let trimmed = markdown.trim_start();
    if !trimmed.starts_with("---\n") && !trimmed.starts_with("---\r\n") {
        return Ok(None);
    }

    // Find the closing `---`
    let after_first = &trimmed[3..]; // skip leading "---"
    let rest = after_first.trim_start(); // skip the newline after ---

    // Find the closing ---
    let closing_pos = rest.find("\n---").or_else(|| rest.find("\r\n---"));

    let fm_end = match closing_pos {
        Some(pos) => pos,
        None => return Ok(None), // Malformed: no closing ---
    };

    let fm_content = &rest[..fm_end];

    if fm_content.trim().is_empty() {
        return Ok(None);
    }

    // Parse as YAML mapping
    let yaml_value: serde_yaml::Value =
        serde_yaml::from_str(fm_content).map_err(|e| CoreError::FrontmatterError(e.to_string()))?;

    let mapping = match yaml_value {
        serde_yaml::Value::Mapping(map) => map,
        _ => {
            return Err(CoreError::FrontmatterError(
                "Frontmatter must be a YAML mapping".to_string(),
            ));
        }
    };

    let mut result = HashMap::new();
    for (key, value) in mapping {
        let key_str = match key {
            serde_yaml::Value::String(s) => s,
            other => yaml_value_to_string(&other)?,
        };

        let value_str = match value {
            serde_yaml::Value::String(s) => s,
            serde_yaml::Value::Bool(b) => b.to_string(),
            serde_yaml::Value::Number(n) => n.to_string(),
            serde_yaml::Value::Sequence(seq) => {
                let items: Vec<String> = seq
                    .iter()
                    .map(|v| match v {
                        serde_yaml::Value::String(s) => Ok(s.clone()),
                        other => yaml_value_to_string(other),
                    })
                    .collect::<Result<_, _>>()?;
                items.join(", ")
            }
            other => yaml_value_to_string(&other)?,
        };

        result.insert(key_str, value_str);
    }

    Ok(Some(result))
}

fn yaml_value_to_string(value: &serde_yaml::Value) -> Result<String, CoreError> {
    serde_yaml::to_string(value)
        .map(|serialized| serialized.trim_end().to_owned())
        .map_err(|error| CoreError::FrontmatterError(error.to_string()))
}

/// Convert Markdown into a complete self-contained HTML document.
///
/// Includes embedded GitHub-like CSS, meta tags, and the given body content.
/// The resulting document can be saved and opened in any browser offline.
pub fn wrap_markdown_document(markdown: &str, title: &str, css: &str) -> Result<String, CoreError> {
    if css.len() > MAX_CUSTOM_CSS_BYTES {
        return Err(CoreError::InputTooLarge {
            limit: MAX_CUSTOM_CSS_BYTES,
        });
    }
    let html_body = md_to_html(markdown)?;
    Ok(wrap_rendered_html_document(&html_body, title, css))
}

fn validate_markdown(markdown: &str) -> Result<(), CoreError> {
    if markdown.trim().is_empty() {
        return Err(CoreError::EmptyInput);
    }
    if markdown.len() > MAX_MARKDOWN_BYTES {
        return Err(CoreError::InputTooLarge {
            limit: MAX_MARKDOWN_BYTES,
        });
    }
    Ok(())
}

fn wrap_rendered_html_document(html_body: &str, title: &str, css: &str) -> String {
    let effective_css = if css.trim().is_empty() {
        DEFAULT_CSS
    } else {
        &escape_style_end_tags(css)
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<meta name="generator" content="md_porter">
<title>{title}</title>
<style>
{effective_css}
</style>
</head>
<body>
<article class="markdown-body">
{html_body}
</article>
</body>
</html>"#,
        title = html_escape(title),
        effective_css = effective_css,
        html_body = html_body,
    )
}

fn sanitize_events<'a>(events: impl Iterator<Item = Event<'a>>) -> Vec<Event<'a>> {
    let mut output = Vec::new();
    let mut image_alt: Option<String> = None;

    for event in events {
        if let Some(alt) = image_alt.as_mut() {
            match event {
                Event::End(TagEnd::Image) => {
                    let label = alt.trim();
                    let placeholder = if label.is_empty() {
                        "[image omitted]".to_owned()
                    } else {
                        format!("[image omitted: {label}]")
                    };
                    output.push(Event::Text(CowStr::from(placeholder)));
                    image_alt = None;
                }
                Event::Text(text) | Event::Code(text) => alt.push_str(&text),
                Event::SoftBreak | Event::HardBreak => alt.push(' '),
                _ => {}
            }
            continue;
        }

        match event {
            Event::Html(raw) | Event::InlineHtml(raw) => output.push(Event::Text(raw)),
            Event::Start(Tag::Image { .. }) => image_alt = Some(String::new()),
            Event::End(TagEnd::Image) => {}
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                let safe_url = sanitize_link_url(&dest_url);
                let title = if safe_url == "#blocked-link" && title.is_empty() {
                    CowStr::from("Blocked unsafe URL")
                } else {
                    title
                };
                output.push(Event::Start(Tag::Link {
                    link_type,
                    dest_url: CowStr::from(safe_url),
                    title,
                    id,
                }));
            }
            other => output.push(other),
        }
    }

    if let Some(alt) = image_alt {
        let label = alt.trim();
        let placeholder = if label.is_empty() {
            "[image omitted]".to_owned()
        } else {
            format!("[image omitted: {label}]")
        };
        output.push(Event::Text(CowStr::from(placeholder)));
    }

    output
}

fn sanitize_link_url(url: &str) -> String {
    let trimmed = url.trim();
    let has_control = trimmed
        .chars()
        .any(|character| character.is_ascii_control());
    if trimmed.starts_with("//") || has_control {
        return "#blocked-link".to_owned();
    }

    if let Some((scheme, _)) = trimmed.split_once(':') {
        let is_scheme = !scheme.is_empty()
            && scheme.chars().enumerate().all(|(index, character)| {
                character.is_ascii_alphabetic()
                    || (index > 0 && (character == '+' || character == '-' || character == '.'))
            });
        if is_scheme
            && !matches!(
                scheme.to_ascii_lowercase().as_str(),
                "http" | "https" | "mailto"
            )
        {
            return "#blocked-link".to_owned();
        }
    }

    trimmed.to_owned()
}

fn escape_style_end_tags(css: &str) -> String {
    let lower = css.to_ascii_lowercase();
    let mut output = String::with_capacity(css.len());
    let mut cursor = 0;
    while let Some(relative) = lower[cursor..].find("</style") {
        let start = cursor + relative;
        output.push_str(&css[cursor..start]);
        output.push_str("<\\/style");
        cursor = start + "</style".len();
    }
    output.push_str(&css[cursor..]);
    output
}

/// Basic HTML entity escaping for the title attribute.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Default GitHub-like CSS embedded in exported HTML documents.
pub const DEFAULT_CSS: &str = r#"/* md_porter default GitHub-like stylesheet */
:root {
  --color-canvas-default: #ffffff;
  --color-canvas-subtle: #f6f8fa;
  --color-border-default: #d0d7de;
  --color-border-muted: #d8dee4;
  --color-fg-default: #24292f;
  --color-fg-muted: #57606a;
  --color-accent-fg: #0969da;
  --color-danger-fg: #cf222e;
}

* { box-sizing: border-box; margin: 0; padding: 0; }

body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans",
    Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
  font-size: 16px;
  line-height: 1.6;
  color: var(--color-fg-default);
  background: var(--color-canvas-default);
  max-width: 900px;
  margin: 0 auto;
  padding: 32px 16px;
}

.markdown-body h1, .markdown-body h2, .markdown-body h3,
.markdown-body h4, .markdown-body h5, .markdown-body h6 {
  margin-top: 24px;
  margin-bottom: 16px;
  font-weight: 600;
  line-height: 1.25;
  border-bottom: 1px solid var(--color-border-muted);
  padding-bottom: .3em;
}

.markdown-body h1 { font-size: 2em; }
.markdown-body h2 { font-size: 1.5em; }
.markdown-body h3 { font-size: 1.25em; }
.markdown-body h4 { font-size: 1em; }

.markdown-body p { margin-bottom: 16px; }

.markdown-body a { color: var(--color-accent-fg); text-decoration: none; }
.markdown-body a:hover { text-decoration: underline; }

.markdown-body strong { font-weight: 600; }

.markdown-body del { text-decoration: line-through; color: var(--color-fg-muted); }

.markdown-body ul, .markdown-body ol { padding-left: 2em; margin-bottom: 16px; }
.markdown-body li { margin-bottom: 4px; }
.markdown-body li > ul, .markdown-body li > ol { margin-bottom: 0; }

.markdown-body input[type="checkbox"] {
  margin-right: 6px;
  vertical-align: middle;
}

.markdown-body blockquote {
  padding: 0 1em;
  color: var(--color-fg-muted);
  border-left: .25em solid var(--color-border-default);
  margin-bottom: 16px;
}

.markdown-body code {
  padding: .2em .4em;
  margin: 0;
  font-size: 85%;
  white-space: break-spaces;
  background-color: var(--color-canvas-subtle);
  border-radius: 6px;
  font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas,
    "Liberation Mono", monospace;
}

.markdown-body pre {
  padding: 16px;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  background-color: var(--color-canvas-subtle);
  border-radius: 6px;
  margin-bottom: 16px;
  border: 1px solid var(--color-border-default);
}

.markdown-body pre code {
  display: inline;
  padding: 0;
  margin: 0;
  overflow: visible;
  line-height: inherit;
  word-wrap: normal;
  background-color: transparent;
  border: 0;
  border-radius: 0;
  font-size: 100%;
}

.markdown-body table {
  border-collapse: collapse;
  width: 100%;
  margin-bottom: 16px;
  display: block;
  overflow: auto;
}

.markdown-body table th, .markdown-body table td {
  padding: 6px 13px;
  border: 1px solid var(--color-border-default);
}

.markdown-body table th {
  font-weight: 600;
  background-color: var(--color-canvas-subtle);
}

.markdown-body table tr:nth-child(2n) {
  background-color: var(--color-canvas-subtle);
}

.markdown-body img {
  max-width: 100%;
  height: auto;
}

.markdown-body hr {
  height: .25em;
  padding: 0;
  margin: 24px 0;
  background-color: var(--color-border-default);
  border: 0;
}

.markdown-body sup { font-size: 75%; line-height: 0; position: relative; vertical-align: baseline; top: -0.5em; }

@media (prefers-color-scheme: dark) {
  :root {
    --color-canvas-default: #0d1117;
    --color-canvas-subtle: #161b22;
    --color-border-default: #30363d;
    --color-border-muted: #21262d;
    --color-fg-default: #c9d1d9;
    --color-fg-muted: #8b949e;
    --color-accent-fg: #58a6ff;
    --color-danger-fg: #f85149;
  }
  body { background: var(--color-canvas-default); color: var(--color-fg-default); }
}

@media print {
  body { max-width: 100%; padding: 0; }
  .markdown-body pre, .markdown-body blockquote { border: 1px solid #999; page-break-inside: avoid; }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let result = md_to_html("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "EMPTY_INPUT");

        let result = md_to_html("   \n  ");
        assert!(result.is_err());
    }

    #[test]
    fn test_oversized_markdown_is_rejected_before_parsing() {
        let markdown = "x".repeat(MAX_MARKDOWN_BYTES + 1);
        let error = md_to_html(&markdown).unwrap_err();
        assert_eq!(error.code(), "INPUT_TOO_LARGE");
    }

    #[test]
    fn test_basic_html_conversion() {
        let md = "# Hello\n\nThis is **bold** text.";
        let html = md_to_html(md).unwrap();
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn test_gfm_tables() {
        let md = "| A | B |\n|---|---|\n| 1 | 2 |";
        let html = md_to_html(md).unwrap();
        assert!(html.contains("<table>"));
        assert!(html.contains("<th>A</th>"));
        assert!(html.contains("<td>1</td>"));
    }

    #[test]
    fn test_task_lists() {
        let md = "- [x] Done\n- [ ] Todo";
        let html = md_to_html(md).unwrap();
        assert!(html.contains("checked"));
        assert!(html.contains("type=\"checkbox\""));
    }

    #[test]
    fn test_strikethrough() {
        let md = "~~deleted text~~";
        let html = md_to_html(md).unwrap();
        assert!(html.contains("<del>deleted text</del>"));
    }

    #[test]
    fn test_code_blocks() {
        let md = "```rust\nfn main() {}\n```";
        let html = md_to_html(md).unwrap();
        assert!(html.contains("<code"));
        assert!(html.contains("fn main()"));
    }

    #[test]
    fn test_raw_html_is_rendered_as_text() {
        let html = md_to_html("<script>alert('x')</script>").unwrap();
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;"));
    }

    #[test]
    fn test_unsafe_link_is_blocked_without_dropping_text() {
        let html = md_to_html("[click](javascript:alert(1))").unwrap();
        assert!(html.contains("click"));
        assert!(html.contains("href=\"#blocked-link\""));
        assert!(!html.contains("javascript:"));
    }

    #[test]
    fn test_images_are_text_only_to_prevent_external_fetches() {
        let html = md_to_html("![diagram](https://example.com/diagram.png)").unwrap();
        assert!(html.contains("[image omitted: diagram]"));
        assert!(!html.contains("<img"));
        assert!(!html.contains("example.com"));
    }

    #[test]
    fn test_plain_text_conversion() {
        let md = "# Hello\n\nThis is **bold** and ~~strikethrough~~.";
        let plain = md_to_plain_text(md).unwrap();
        assert!(plain.contains("# Hello"));
        assert!(plain.contains("This is bold and strikethrough."));
        // Plain text should not contain markdown syntax
        assert!(!plain.contains("**"));
        assert!(!plain.contains("~~"));
    }

    #[test]
    fn test_extract_frontmatter() {
        let md = "---\ntitle: Test Doc\ndate: 2026-08-07\ntags: [rust, wasm]\n---\n\n# Content";
        let fm = extract_frontmatter(md).unwrap().unwrap();
        assert_eq!(fm.get("title").unwrap(), "Test Doc");
        assert_eq!(fm.get("date").unwrap(), "2026-08-07");
        assert_eq!(fm.get("tags").unwrap(), "rust, wasm");
    }

    #[test]
    fn test_no_frontmatter() {
        let md = "# Just a heading\n\nSome content.";
        let fm = extract_frontmatter(md).unwrap();
        assert!(fm.is_none());
    }

    #[test]
    fn test_wrap_markdown_document() {
        let markdown = "# Hello\n\nWorld";
        let doc = wrap_markdown_document(markdown, "Test", "").unwrap();
        assert!(doc.contains("<!DOCTYPE html>"));
        assert!(doc.contains("<title>Test</title>"));
        assert!(doc.contains("<h1>Hello</h1>"));
        assert!(doc.contains("markdown-body"));
        assert!(doc.contains("md_porter"));
    }

    #[test]
    fn test_wrap_html_document_custom_css() {
        let body = "<p>Content</p>";
        let css = "body { background: red; }";
        let doc = wrap_markdown_document(body, "Custom", css).unwrap();
        assert!(doc.contains("background: red;"));
    }

    #[test]
    fn test_custom_css_cannot_close_the_style_element() {
        let doc = wrap_markdown_document("# Content", "Title", "</STYLE><script>alert(1)</script>")
            .unwrap();
        assert!(!doc.contains("</STYLE><script>"));
        assert!(doc.contains("<\\/style>"));
    }

    #[test]
    fn test_oversized_custom_css_is_rejected() {
        let css = "a".repeat(MAX_CUSTOM_CSS_BYTES + 1);
        let error = wrap_markdown_document("# Content", "Title", &css).unwrap_err();
        assert_eq!(error.code(), "INPUT_TOO_LARGE");
    }

    #[test]
    fn test_footnotes() {
        let md = "Text with footnote[^1].\n\n[^1]: Footnote content.";
        let html = md_to_html(md).unwrap();
        assert!(html.contains("footnote"));
    }
}
