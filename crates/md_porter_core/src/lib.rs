pub mod convert;
pub mod error;

pub use convert::{
    DEFAULT_CSS, MAX_CUSTOM_CSS_BYTES, MAX_MARKDOWN_BYTES, extract_frontmatter, md_to_html,
    md_to_plain_text, wrap_markdown_document,
};
pub use error::CoreError;
