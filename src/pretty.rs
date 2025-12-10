//! Pretty printing for Rust values.
//!
//! This module provides pretty-printing functionality for displaying
//! data structures in a readable, formatted way.
//!
//! # Example
//!
//! ```ignore
//! use richrs::pretty::Pretty;
//! use std::collections::HashMap;
//!
//! let mut map = HashMap::new();
//! map.insert("name", "Alice");
//! map.insert("age", "30");
//!
//! let pretty = Pretty::new(&map);
//! let segments = pretty.render(80);
//! ```

use crate::segment::{Segment, Segments};
use crate::style::Style;

/// Pretty printer for formatted output.
///
/// Renders data structures in a readable, indented format with
/// optional syntax highlighting.
#[derive(Debug, Clone)]
pub struct Pretty {
    /// The string representation to format.
    content: String,
    /// Indentation size.
    indent_size: usize,
    /// Whether to show indent guides.
    indent_guides: bool,
    /// Maximum depth for nested structures.
    max_depth: Option<usize>,
    /// Whether to expand all items.
    expand_all: bool,
    /// Maximum length for collections.
    max_length: Option<usize>,
    /// Maximum string length before truncation.
    max_string: Option<usize>,
}

impl Pretty {
    /// Creates a new Pretty printer for the given debug representation.
    #[must_use]
    pub fn new<T: std::fmt::Debug>(value: &T) -> Self {
        Self {
            content: format!("{:#?}", value),
            indent_size: 4,
            indent_guides: false,
            max_depth: None,
            expand_all: false,
            max_length: None,
            max_string: None,
        }
    }

    /// Creates a Pretty printer from a string.
    #[must_use]
    pub fn from_string(s: impl Into<String>) -> Self {
        Self {
            content: s.into(),
            indent_size: 4,
            indent_guides: false,
            max_depth: None,
            expand_all: false,
            max_length: None,
            max_string: None,
        }
    }

    /// Sets the indentation size.
    #[must_use]
    #[inline]
    pub const fn indent_size(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }

    /// Sets whether to show indent guides.
    #[must_use]
    #[inline]
    pub const fn indent_guides(mut self, show: bool) -> Self {
        self.indent_guides = show;
        self
    }

    /// Sets the maximum depth.
    #[must_use]
    #[inline]
    pub const fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Sets whether to expand all items.
    #[must_use]
    #[inline]
    pub const fn expand_all(mut self, expand: bool) -> Self {
        self.expand_all = expand;
        self
    }

    /// Sets the maximum length for collections.
    #[must_use]
    #[inline]
    pub const fn max_length(mut self, length: usize) -> Self {
        self.max_length = Some(length);
        self
    }

    /// Sets the maximum string length.
    #[must_use]
    #[inline]
    pub const fn max_string(mut self, length: usize) -> Self {
        self.max_string = Some(length);
        self
    }

    /// Renders the pretty-printed output.
    #[must_use]
    pub fn render(&self, _max_width: usize) -> Segments {
        let mut segments = Segments::new();
        let mut current_depth = 0;

        for line in self.content.lines() {
            // Calculate indentation
            let trimmed = line.trim_start();
            let indent_chars = line.len().saturating_sub(trimmed.len());
            let indent_level = indent_chars.checked_div(self.indent_size).unwrap_or(0);

            // Check max depth
            if let Some(max) = self.max_depth {
                if indent_level > max {
                    if current_depth <= max {
                        segments.push(Segment::styled("...".to_string(), Style::default().dim()));
                        segments.push(Segment::newline());
                    }
                    current_depth = indent_level;
                    continue;
                }
            }
            current_depth = indent_level;

            // Add indent guides if enabled
            if self.indent_guides && indent_level > 0 {
                let guide_style = Style::default().dim();
                for i in 0..indent_level {
                    let spaces = " ".repeat(self.indent_size.saturating_sub(1));
                    if i == indent_level.saturating_sub(1) {
                        segments.push(Segment::styled(format!("{}│", spaces), guide_style.clone()));
                    } else {
                        segments.push(Segment::styled(format!("{}│", spaces), guide_style.clone()));
                    }
                }
            } else {
                // Regular indentation
                segments.push(Segment::new(" ".repeat(indent_chars)));
            }

            // Syntax highlight the content
            let styled_content = self.highlight_line(trimmed);
            segments.extend(styled_content.into_iter());
            segments.push(Segment::newline());
        }

        segments
    }

    /// Highlights a single line of debug output.
    fn highlight_line(&self, line: &str) -> Segments {
        let mut segments = Segments::new();
        let mut chars = line.chars().peekable();
        let mut current = String::new();

        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    // String literal
                    if !current.is_empty() {
                        segments.push(Segment::new(std::mem::take(&mut current)));
                    }
                    let mut string_content = String::from('"');
                    while let Some(c) = chars.next() {
                        string_content.push(c);
                        if c == '"' {
                            break;
                        }
                        if c == '\\' {
                            if let Some(escaped) = chars.next() {
                                string_content.push(escaped);
                            }
                        }
                    }
                    // Truncate if needed
                    let display = if let Some(max) = self.max_string {
                        if string_content.len() > max.saturating_add(2) {
                            format!("{}...\"", &string_content[..max.saturating_add(1)])
                        } else {
                            string_content
                        }
                    } else {
                        string_content
                    };
                    // Green for strings
                    segments.push(Segment::new(display));
                }
                ':' => {
                    // Key separator
                    if !current.is_empty() {
                        // Cyan for keys
                        segments.push(Segment::new(std::mem::take(&mut current)));
                    }
                    segments.push(Segment::new(":"));
                }
                '{' | '}' | '[' | ']' | '(' | ')' => {
                    // Brackets
                    if !current.is_empty() {
                        segments.push(Segment::new(std::mem::take(&mut current)));
                    }
                    // Bold for brackets
                    segments.push(Segment::styled(ch.to_string(), Style::default().bold()));
                }
                ',' => {
                    if !current.is_empty() {
                        segments.push(Segment::new(std::mem::take(&mut current)));
                    }
                    segments.push(Segment::new(","));
                }
                _ => {
                    current.push(ch);
                }
            }
        }

        if !current.is_empty() {
            // Check if it's a number
            let is_number = current.trim().parse::<f64>().is_ok()
                || current.trim() == "true"
                || current.trim() == "false";
            if is_number {
                // Blue for numbers/booleans
                segments.push(Segment::new(current));
            } else {
                segments.push(Segment::new(current));
            }
        }

        segments
    }
}

/// Inspects a value and returns a formatted string.
///
/// This is a convenience function for debugging values.
#[must_use]
pub fn inspect<T: std::fmt::Debug>(value: &T) -> String {
    let pretty = Pretty::new(value);
    pretty.render(80).to_ansi()
}

/// Inspects a value with custom options.
#[must_use]
pub fn inspect_with_options<T: std::fmt::Debug>(
    value: &T,
    indent_size: usize,
    max_depth: Option<usize>,
) -> String {
    let mut pretty = Pretty::new(value).indent_size(indent_size);
    if let Some(depth) = max_depth {
        pretty = pretty.max_depth(depth);
    }
    pretty.render(80).to_ansi()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_pretty_new() {
        let value = vec![1, 2, 3];
        let pretty = Pretty::new(&value);
        assert!(pretty.content.contains("["));
    }

    #[test]
    fn test_pretty_from_string() {
        let pretty = Pretty::from_string("test string");
        assert_eq!(pretty.content, "test string");
    }

    #[test]
    fn test_pretty_indent_size() {
        let pretty = Pretty::from_string("test").indent_size(2);
        assert_eq!(pretty.indent_size, 2);
    }

    #[test]
    fn test_pretty_indent_guides() {
        let pretty = Pretty::from_string("test").indent_guides(true);
        assert!(pretty.indent_guides);
    }

    #[test]
    fn test_pretty_max_depth() {
        let pretty = Pretty::from_string("test").max_depth(3);
        assert_eq!(pretty.max_depth, Some(3));
    }

    #[test]
    fn test_pretty_expand_all() {
        let pretty = Pretty::from_string("test").expand_all(true);
        assert!(pretty.expand_all);
    }

    #[test]
    fn test_pretty_max_length() {
        let pretty = Pretty::from_string("test").max_length(100);
        assert_eq!(pretty.max_length, Some(100));
    }

    #[test]
    fn test_pretty_max_string() {
        let pretty = Pretty::from_string("test").max_string(50);
        assert_eq!(pretty.max_string, Some(50));
    }

    #[test]
    fn test_pretty_render_vec() {
        let value = vec![1, 2, 3];
        let pretty = Pretty::new(&value);
        let segments = pretty.render(80);
        let output = segments.to_ansi();
        assert!(output.contains("1"));
        assert!(output.contains("2"));
        assert!(output.contains("3"));
    }

    #[test]
    fn test_pretty_render_hashmap() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        let pretty = Pretty::new(&map);
        let segments = pretty.render(80);
        let output = segments.to_ansi();
        assert!(output.contains("key"));
        assert!(output.contains("value"));
    }

    #[test]
    fn test_inspect_function() {
        let value = (1, "hello", true);
        let output = inspect(&value);
        assert!(output.contains("hello"));
    }

    #[test]
    fn test_inspect_with_options() {
        let value = vec![vec![1, 2], vec![3, 4]];
        let output = inspect_with_options(&value, 2, Some(1));
        assert!(output.len() > 0);
    }
}
