use thiserror::Error;

/// Stable error type for markdown conversion operations.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("Input markdown is empty")]
    EmptyInput,

    #[error("Input exceeds the maximum allowed size of {limit} bytes")]
    InputTooLarge { limit: usize },

    #[error("Markdown parse error: {0}")]
    ParseError(String),

    #[error("Conversion error: {0}")]
    ConversionError(String),

    #[error("Frontmatter YAML deserialization error: {0}")]
    FrontmatterError(String),
}

impl CoreError {
    /// Returns a stable machine-readable error code for Web, CLI, and Agent consumers.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::EmptyInput => "EMPTY_INPUT",
            Self::InputTooLarge { .. } => "INPUT_TOO_LARGE",
            Self::ParseError(_) => "PARSE_ERROR",
            Self::ConversionError(_) => "CONVERSION_ERROR",
            Self::FrontmatterError(_) => "FRONTMATTER_ERROR",
        }
    }
}
