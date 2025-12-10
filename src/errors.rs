//! Error types for richrs.
//!
//! This module provides a comprehensive error handling system using the `thiserror` crate.
//! All errors are fully typed and provide detailed context for debugging.

use thiserror::Error;

/// The main error type for richrs operations.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error parsing a style string.
    #[error("invalid style: {message}")]
    StyleParse {
        /// Description of the parse error.
        message: String,
    },

    /// Error parsing a color specification.
    #[error("invalid color: {message}")]
    ColorParse {
        /// Description of the color parse error.
        message: String,
    },

    /// Error parsing markup syntax.
    #[error("invalid markup: {message}")]
    MarkupParse {
        /// Description of the markup parse error.
        message: String,
    },

    /// Error with console I/O operations.
    #[error("console I/O error: {source}")]
    ConsoleIo {
        /// The underlying I/O error.
        #[from]
        source: std::io::Error,
    },

    /// Error when a required feature is not available.
    #[error("feature not available: {feature}")]
    FeatureNotAvailable {
        /// The name of the unavailable feature.
        feature: String,
    },

    /// Error when a value is out of the expected range.
    #[error("value out of range: {message}")]
    OutOfRange {
        /// Description of the range violation.
        message: String,
    },

    /// Error when table column count doesn't match row data.
    #[error("table error: {message}")]
    TableError {
        /// Description of the table error.
        message: String,
    },

    /// Error when theme contains invalid style definitions.
    #[error("theme error: {message}")]
    ThemeError {
        /// Description of the theme error.
        message: String,
    },

    /// Error when regex compilation fails.
    #[error("regex error: {source}")]
    RegexError {
        /// The underlying regex error.
        #[from]
        source: regex::Error,
    },

    /// Error when JSON serialization/deserialization fails.
    #[error("JSON error: {source}")]
    JsonError {
        /// The underlying JSON error.
        #[from]
        source: serde_json::Error,
    },

    /// Error when integer conversion fails.
    #[error("integer conversion error: {message}")]
    IntegerConversion {
        /// Description of the conversion error.
        message: String,
    },

    /// Error when an emoji name is not found.
    #[error("no emoji called '{0}'")]
    NoEmoji(String),

    /// Error when a spinner name is not found.
    #[error("no spinner called '{0}'")]
    NoSpinner(String),
}

/// A specialized Result type for richrs operations.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_parse_error_display() {
        let err = Error::StyleParse {
            message: "unknown attribute 'foobar'".to_owned(),
        };
        assert_eq!(err.to_string(), "invalid style: unknown attribute 'foobar'");
    }

    #[test]
    fn test_color_parse_error_display() {
        let err = Error::ColorParse {
            message: "invalid hex color '#gggggg'".to_owned(),
        };
        assert_eq!(
            err.to_string(),
            "invalid color: invalid hex color '#gggggg'"
        );
    }

    #[test]
    fn test_markup_parse_error_display() {
        let err = Error::MarkupParse {
            message: "unclosed tag at position 5".to_owned(),
        };
        assert_eq!(
            err.to_string(),
            "invalid markup: unclosed tag at position 5"
        );
    }

    #[test]
    fn test_feature_not_available_display() {
        let err = Error::FeatureNotAvailable {
            feature: "syntax_highlighting".to_owned(),
        };
        assert_eq!(
            err.to_string(),
            "feature not available: syntax_highlighting"
        );
    }

    #[test]
    fn test_out_of_range_display() {
        let err = Error::OutOfRange {
            message: "color index 256 exceeds maximum 255".to_owned(),
        };
        assert_eq!(
            err.to_string(),
            "value out of range: color index 256 exceeds maximum 255"
        );
    }

    #[test]
    fn test_table_error_display() {
        let err = Error::TableError {
            message: "row has 3 columns but table has 4".to_owned(),
        };
        assert_eq!(
            err.to_string(),
            "table error: row has 3 columns but table has 4"
        );
    }

    #[test]
    fn test_theme_error_display() {
        let err = Error::ThemeError {
            message: "style name must start with a letter".to_owned(),
        };
        assert_eq!(
            err.to_string(),
            "theme error: style name must start with a letter"
        );
    }

    #[test]
    fn test_integer_conversion_display() {
        let err = Error::IntegerConversion {
            message: "value too large for u8".to_owned(),
        };
        assert_eq!(
            err.to_string(),
            "integer conversion error: value too large for u8"
        );
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }

    #[test]
    fn test_no_emoji_display() {
        let err = Error::NoEmoji("nonexistent".to_owned());
        assert_eq!(err.to_string(), "no emoji called 'nonexistent'");
    }
}
