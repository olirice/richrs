//! Highlighter for automatic text highlighting.
//!
//! This module provides automatic highlighting of patterns in text,
//! such as URLs, numbers, strings, and keywords.
//!
//! # Example
//!
//! ```ignore
//! use richrs::highlighter::{Highlighter, ReprHighlighter};
//!
//! let highlighter = ReprHighlighter::new();
//! let segments = highlighter.highlight("value = 42, url = https://example.com");
//! ```

use crate::segment::{Segment, Segments};
use crate::style::Style;
use regex::Regex;
use std::sync::LazyLock;

/// A trait for highlighting text patterns.
pub trait Highlighter {
    /// Highlights the given text and returns styled segments.
    fn highlight(&self, text: &str) -> Segments;
}

/// A highlighter for repr-style output (numbers, strings, booleans, None).
///
/// This highlighter is similar to Python Rich's `ReprHighlighter` and
/// automatically highlights common programming patterns.
#[derive(Debug, Clone)]
pub struct ReprHighlighter {
    /// Style for numbers.
    number_style: Style,
    /// Style for strings.
    string_style: Style,
    /// Style for boolean values.
    bool_style: Style,
    /// Style for None/null values.
    none_style: Style,
    /// Style for attribute names.
    attr_style: Style,
    /// Style for URLs.
    url_style: Style,
    /// Style for UUIDs.
    uuid_style: Style,
}

impl Default for ReprHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl ReprHighlighter {
    /// Creates a new ReprHighlighter with default styles.
    #[must_use]
    pub fn new() -> Self {
        Self {
            number_style: Style::default().bold(),
            string_style: Style::default().italic(),
            bool_style: Style::default().italic().bold(),
            none_style: Style::default().italic().bold(),
            attr_style: Style::default(),
            url_style: Style::default().underline(),
            uuid_style: Style::default().bold(),
        }
    }

    /// Sets the style for numbers.
    #[must_use]
    pub fn number_style(mut self, style: Style) -> Self {
        self.number_style = style;
        self
    }

    /// Sets the style for strings.
    #[must_use]
    pub fn string_style(mut self, style: Style) -> Self {
        self.string_style = style;
        self
    }

    /// Sets the style for boolean values.
    #[must_use]
    pub fn bool_style(mut self, style: Style) -> Self {
        self.bool_style = style;
        self
    }

    /// Sets the style for None/null values.
    #[must_use]
    pub fn none_style(mut self, style: Style) -> Self {
        self.none_style = style;
        self
    }

    /// Sets the style for attribute names.
    #[must_use]
    pub fn attr_style(mut self, style: Style) -> Self {
        self.attr_style = style;
        self
    }

    /// Sets the style for URLs.
    #[must_use]
    pub fn url_style(mut self, style: Style) -> Self {
        self.url_style = style;
        self
    }

    /// Sets the style for UUIDs.
    #[must_use]
    pub fn uuid_style(mut self, style: Style) -> Self {
        self.uuid_style = style;
        self
    }
}

// Regex patterns for highlighting
static NUMBER_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?x)
        (?P<number>
            # Hex numbers
            0x[0-9a-fA-F]+
            |
            # Binary numbers
            0b[01]+
            |
            # Octal numbers
            0o[0-7]+
            |
            # Float with exponent
            -?[0-9]+\.?[0-9]*(?:e[+-]?[0-9]+)?
            |
            # Regular integers
            -?[0-9]+
        )
    ").unwrap()
});

static STRING_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?P<string>"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*')"#).unwrap()
});

static BOOL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?P<bool>true|false|True|False)\b").unwrap()
});

static NONE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?P<none>None|null|nil|NULL)\b").unwrap()
});

static URL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?P<url>https?://[^\s<>"')]+)"#).unwrap()
});

static UUID_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?P<uuid>[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})").unwrap()
});

// Note: Attribute pattern with lookahead not supported in regex crate.
// We'll handle attribute highlighting differently.

impl Highlighter for ReprHighlighter {
    fn highlight(&self, text: &str) -> Segments {
        let mut segments = Segments::new();
        let mut last_end = 0;

        // Collect all matches with their positions
        let mut matches: Vec<(usize, usize, &str, Style)> = Vec::new();

        // Find all pattern matches
        for cap in URL_PATTERN.captures_iter(text) {
            if let Some(m) = cap.name("url") {
                matches.push((m.start(), m.end(), m.as_str(), self.url_style.clone()));
            }
        }

        for cap in UUID_PATTERN.captures_iter(text) {
            if let Some(m) = cap.name("uuid") {
                matches.push((m.start(), m.end(), m.as_str(), self.uuid_style.clone()));
            }
        }

        for cap in STRING_PATTERN.captures_iter(text) {
            if let Some(m) = cap.name("string") {
                matches.push((m.start(), m.end(), m.as_str(), self.string_style.clone()));
            }
        }

        for cap in BOOL_PATTERN.captures_iter(text) {
            if let Some(m) = cap.name("bool") {
                matches.push((m.start(), m.end(), m.as_str(), self.bool_style.clone()));
            }
        }

        for cap in NONE_PATTERN.captures_iter(text) {
            if let Some(m) = cap.name("none") {
                matches.push((m.start(), m.end(), m.as_str(), self.none_style.clone()));
            }
        }

        for cap in NUMBER_PATTERN.captures_iter(text) {
            if let Some(m) = cap.name("number") {
                matches.push((m.start(), m.end(), m.as_str(), self.number_style.clone()));
            }
        }

        // Handle attribute names manually (word followed by = or :)
        // This is a simple approach since regex crate doesn't support lookahead
        let attr_re = Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)\s*[=:]").unwrap();
        for cap in attr_re.captures_iter(text) {
            if let Some(m) = cap.get(1) {
                // Only add if attr_style is not empty
                if !self.attr_style.is_empty() {
                    matches.push((m.start(), m.end(), m.as_str(), self.attr_style.clone()));
                }
            }
        }

        // Sort matches by start position
        matches.sort_by_key(|m| m.0);

        // Remove overlapping matches (keep earlier/longer ones)
        let mut filtered_matches: Vec<(usize, usize, &str, Style)> = Vec::new();
        for m in matches {
            if filtered_matches
                .last()
                .map_or(true, |last| m.0 >= last.1)
            {
                filtered_matches.push(m);
            }
        }

        // Build segments
        for (start, end, matched_text, style) in filtered_matches {
            // Add any text before this match
            if start > last_end {
                segments.push(Segment::new(&text[last_end..start]));
            }

            // Add the highlighted match
            if style.is_empty() {
                segments.push(Segment::new(matched_text));
            } else {
                segments.push(Segment::styled(matched_text, style));
            }

            last_end = end;
        }

        // Add any remaining text
        if last_end < text.len() {
            segments.push(Segment::new(&text[last_end..]));
        }

        // Handle empty text
        if segments.is_empty() {
            segments.push(Segment::new(text));
        }

        segments
    }
}

/// A highlighter for ISO 8601 timestamps.
#[derive(Debug, Clone)]
pub struct ISOHighlighter {
    /// Style for the date part.
    date_style: Style,
    /// Style for the time part.
    time_style: Style,
    /// Style for the timezone.
    timezone_style: Style,
}

impl Default for ISOHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl ISOHighlighter {
    /// Creates a new ISOHighlighter with default styles.
    #[must_use]
    pub fn new() -> Self {
        Self {
            date_style: Style::default().bold(),
            time_style: Style::default(),
            timezone_style: Style::default().dim(),
        }
    }

    /// Sets the style for the date part.
    #[must_use]
    pub fn date_style(mut self, style: Style) -> Self {
        self.date_style = style;
        self
    }

    /// Sets the style for the time part.
    #[must_use]
    pub fn time_style(mut self, style: Style) -> Self {
        self.time_style = style;
        self
    }

    /// Sets the style for the timezone.
    #[must_use]
    pub fn timezone_style(mut self, style: Style) -> Self {
        self.timezone_style = style;
        self
    }
}

static ISO_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?P<iso>(?P<date>\d{4}-\d{2}-\d{2})(?:T(?P<time>\d{2}:\d{2}:\d{2}(?:\.\d+)?)(?P<tz>Z|[+-]\d{2}:?\d{2})?)?)").unwrap()
});

impl Highlighter for ISOHighlighter {
    fn highlight(&self, text: &str) -> Segments {
        let mut segments = Segments::new();
        let mut last_end = 0;

        for cap in ISO_PATTERN.captures_iter(text) {
            if let Some(full_match) = cap.name("iso") {
                // Add text before the match
                if full_match.start() > last_end {
                    segments.push(Segment::new(&text[last_end..full_match.start()]));
                }

                // Add date part
                if let Some(date) = cap.name("date") {
                    segments.push(Segment::styled(date.as_str(), self.date_style.clone()));
                }

                // Add time part
                if let Some(time) = cap.name("time") {
                    segments.push(Segment::new("T"));
                    segments.push(Segment::styled(time.as_str(), self.time_style.clone()));
                }

                // Add timezone
                if let Some(tz) = cap.name("tz") {
                    segments.push(Segment::styled(tz.as_str(), self.timezone_style.clone()));
                }

                last_end = full_match.end();
            }
        }

        // Add remaining text
        if last_end < text.len() {
            segments.push(Segment::new(&text[last_end..]));
        }

        if segments.is_empty() {
            segments.push(Segment::new(text));
        }

        segments
    }
}

/// A highlighter for regular expressions with custom styles.
#[derive(Debug, Clone)]
pub struct RegexHighlighter {
    /// Patterns and their associated styles.
    patterns: Vec<(Regex, Style)>,
}

impl Default for RegexHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl RegexHighlighter {
    /// Creates a new empty RegexHighlighter.
    #[must_use]
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    /// Adds a pattern with an associated style.
    ///
    /// # Panics
    ///
    /// Panics if the pattern is not a valid regex.
    #[must_use]
    pub fn pattern(mut self, pattern: &str, style: Style) -> Self {
        let regex = Regex::new(pattern).expect("Invalid regex pattern");
        self.patterns.push((regex, style));
        self
    }

    /// Adds a pattern with an associated style, returning an error on invalid regex.
    pub fn try_pattern(
        mut self,
        pattern: &str,
        style: Style,
    ) -> Result<Self, regex::Error> {
        let regex = Regex::new(pattern)?;
        self.patterns.push((regex, style));
        Ok(self)
    }
}

impl Highlighter for RegexHighlighter {
    fn highlight(&self, text: &str) -> Segments {
        let mut segments = Segments::new();
        let mut last_end = 0;

        // Collect all matches
        let mut matches: Vec<(usize, usize, Style)> = Vec::new();

        for (regex, style) in &self.patterns {
            for m in regex.find_iter(text) {
                matches.push((m.start(), m.end(), style.clone()));
            }
        }

        // Sort by position
        matches.sort_by_key(|m| m.0);

        // Remove overlaps
        let mut filtered: Vec<(usize, usize, Style)> = Vec::new();
        for m in matches {
            if filtered.last().map_or(true, |last| m.0 >= last.1) {
                filtered.push(m);
            }
        }

        // Build segments
        for (start, end, style) in filtered {
            if start > last_end {
                segments.push(Segment::new(&text[last_end..start]));
            }
            segments.push(Segment::styled(&text[start..end], style));
            last_end = end;
        }

        if last_end < text.len() {
            segments.push(Segment::new(&text[last_end..]));
        }

        if segments.is_empty() {
            segments.push(Segment::new(text));
        }

        segments
    }
}

/// A highlighter for JSON-like data.
#[derive(Debug, Clone)]
pub struct JSONHighlighter {
    /// Style for keys.
    key_style: Style,
    /// Style for string values.
    string_style: Style,
    /// Style for number values.
    number_style: Style,
    /// Style for boolean values.
    bool_style: Style,
    /// Style for null.
    null_style: Style,
    /// Style for brackets.
    bracket_style: Style,
}

impl Default for JSONHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl JSONHighlighter {
    /// Creates a new JSONHighlighter with default styles.
    #[must_use]
    pub fn new() -> Self {
        Self {
            key_style: Style::default().bold(),
            string_style: Style::default().italic(),
            number_style: Style::default().bold(),
            bool_style: Style::default().bold().italic(),
            null_style: Style::default().dim(),
            bracket_style: Style::default().bold(),
        }
    }

    /// Sets the style for JSON keys.
    #[must_use]
    pub fn key_style(mut self, style: Style) -> Self {
        self.key_style = style;
        self
    }

    /// Sets the style for string values.
    #[must_use]
    pub fn string_style(mut self, style: Style) -> Self {
        self.string_style = style;
        self
    }

    /// Sets the style for number values.
    #[must_use]
    pub fn number_style(mut self, style: Style) -> Self {
        self.number_style = style;
        self
    }

    /// Sets the style for boolean values.
    #[must_use]
    pub fn bool_style(mut self, style: Style) -> Self {
        self.bool_style = style;
        self
    }

    /// Sets the style for null values.
    #[must_use]
    pub fn null_style(mut self, style: Style) -> Self {
        self.null_style = style;
        self
    }

    /// Sets the style for brackets.
    #[must_use]
    pub fn bracket_style(mut self, style: Style) -> Self {
        self.bracket_style = style;
        self
    }
}

static JSON_KEY_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#""([^"\\]|\\.)*"\s*:"#).unwrap()
});

impl Highlighter for JSONHighlighter {
    fn highlight(&self, text: &str) -> Segments {
        let mut segments = Segments::new();
        let mut last_end = 0;

        // Collect matches
        let mut matches: Vec<(usize, usize, &str, Style)> = Vec::new();

        // Find JSON keys (strings followed by :)
        for m in JSON_KEY_PATTERN.find_iter(text) {
            let key_text = &text[m.start()..m.end() - 1]; // Exclude the :
            matches.push((m.start(), m.end() - 1, key_text, self.key_style.clone()));
        }

        // Find brackets
        for (i, c) in text.char_indices() {
            if matches!(c, '{' | '}' | '[' | ']') {
                // Check not inside a key region
                if !matches.iter().any(|(s, e, _, _)| i >= *s && i < *e) {
                    matches.push((i, i + 1, &text[i..i + 1], self.bracket_style.clone()));
                }
            }
        }

        // Find string values (not keys)
        for cap in STRING_PATTERN.captures_iter(text) {
            if let Some(m) = cap.name("string") {
                // Check it's not a key
                let is_key = text[m.end()..].trim_start().starts_with(':');
                if !is_key {
                    matches.push((m.start(), m.end(), m.as_str(), self.string_style.clone()));
                }
            }
        }

        // Find numbers
        for cap in NUMBER_PATTERN.captures_iter(text) {
            if let Some(m) = cap.name("number") {
                matches.push((m.start(), m.end(), m.as_str(), self.number_style.clone()));
            }
        }

        // Find booleans and null
        for cap in BOOL_PATTERN.captures_iter(text) {
            if let Some(m) = cap.name("bool") {
                matches.push((m.start(), m.end(), m.as_str(), self.bool_style.clone()));
            }
        }

        for cap in NONE_PATTERN.captures_iter(text) {
            if let Some(m) = cap.name("none") {
                matches.push((m.start(), m.end(), m.as_str(), self.null_style.clone()));
            }
        }

        // Sort and filter overlaps
        matches.sort_by_key(|m| m.0);
        let mut filtered: Vec<(usize, usize, &str, Style)> = Vec::new();
        for m in matches {
            if filtered.last().map_or(true, |last| m.0 >= last.1) {
                filtered.push(m);
            }
        }

        // Build segments
        for (start, end, matched_text, style) in filtered {
            if start > last_end {
                segments.push(Segment::new(&text[last_end..start]));
            }
            segments.push(Segment::styled(matched_text, style));
            last_end = end;
        }

        if last_end < text.len() {
            segments.push(Segment::new(&text[last_end..]));
        }

        if segments.is_empty() {
            segments.push(Segment::new(text));
        }

        segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repr_highlighter_new() {
        let highlighter = ReprHighlighter::new();
        assert!(!highlighter.number_style.is_empty());
    }

    #[test]
    fn test_repr_highlighter_number() {
        let highlighter = ReprHighlighter::new();
        let segments = highlighter.highlight("value = 42");
        let output = segments.to_ansi();
        assert!(output.contains("42"));
    }

    #[test]
    fn test_repr_highlighter_string() {
        let highlighter = ReprHighlighter::new();
        let segments = highlighter.highlight("name = \"hello\"");
        let output = segments.to_ansi();
        assert!(output.contains("hello"));
    }

    #[test]
    fn test_repr_highlighter_bool() {
        let highlighter = ReprHighlighter::new();
        let segments = highlighter.highlight("flag = true");
        let output = segments.to_ansi();
        assert!(output.contains("true"));
    }

    #[test]
    fn test_repr_highlighter_none() {
        let highlighter = ReprHighlighter::new();
        let segments = highlighter.highlight("value = None");
        let output = segments.to_ansi();
        assert!(output.contains("None"));
    }

    #[test]
    fn test_repr_highlighter_url() {
        let highlighter = ReprHighlighter::new();
        let segments = highlighter.highlight("Visit https://example.com");
        let output = segments.to_ansi();
        assert!(output.contains("https://example.com"));
    }

    #[test]
    fn test_repr_highlighter_uuid() {
        let highlighter = ReprHighlighter::new();
        let segments = highlighter.highlight("id = 123e4567-e89b-12d3-a456-426614174000");
        let output = segments.to_ansi();
        assert!(output.contains("123e4567"));
    }

    #[test]
    fn test_repr_highlighter_hex() {
        let highlighter = ReprHighlighter::new();
        let segments = highlighter.highlight("color = 0xff00ff");
        let output = segments.to_ansi();
        assert!(output.contains("0xff00ff"));
    }

    #[test]
    fn test_iso_highlighter_new() {
        let highlighter = ISOHighlighter::new();
        assert!(!highlighter.date_style.is_empty());
    }

    #[test]
    fn test_iso_highlighter_date() {
        let highlighter = ISOHighlighter::new();
        let segments = highlighter.highlight("Created 2024-01-15");
        let output = segments.to_ansi();
        assert!(output.contains("2024-01-15"));
    }

    #[test]
    fn test_iso_highlighter_datetime() {
        let highlighter = ISOHighlighter::new();
        let segments = highlighter.highlight("Time: 2024-01-15T10:30:00Z");
        let output = segments.to_ansi();
        assert!(output.contains("2024-01-15"));
        assert!(output.contains("10:30:00"));
    }

    #[test]
    fn test_regex_highlighter_new() {
        let highlighter = RegexHighlighter::new();
        assert!(highlighter.patterns.is_empty());
    }

    #[test]
    fn test_regex_highlighter_pattern() {
        let highlighter = RegexHighlighter::new()
            .pattern(r"\bERROR\b", Style::default().bold());
        let segments = highlighter.highlight("ERROR: something failed");
        let output = segments.to_ansi();
        assert!(output.contains("ERROR"));
    }

    #[test]
    fn test_regex_highlighter_try_pattern() {
        let result = RegexHighlighter::new()
            .try_pattern(r"\bWARN\b", Style::default());
        assert!(result.is_ok());
    }

    #[test]
    fn test_regex_highlighter_invalid_pattern() {
        let result = RegexHighlighter::new()
            .try_pattern(r"[invalid", Style::default());
        assert!(result.is_err());
    }

    #[test]
    fn test_json_highlighter_new() {
        let highlighter = JSONHighlighter::new();
        assert!(!highlighter.key_style.is_empty());
    }

    #[test]
    fn test_json_highlighter_key() {
        let highlighter = JSONHighlighter::new();
        let segments = highlighter.highlight(r#"{"name": "value"}"#);
        let output = segments.to_ansi();
        assert!(output.contains("name"));
    }

    #[test]
    fn test_json_highlighter_number() {
        let highlighter = JSONHighlighter::new();
        let segments = highlighter.highlight(r#"{"count": 42}"#);
        let output = segments.to_ansi();
        assert!(output.contains("42"));
    }

    #[test]
    fn test_json_highlighter_bool() {
        let highlighter = JSONHighlighter::new();
        let segments = highlighter.highlight(r#"{"active": true}"#);
        let output = segments.to_ansi();
        assert!(output.contains("true"));
    }

    #[test]
    fn test_json_highlighter_null() {
        let highlighter = JSONHighlighter::new();
        let segments = highlighter.highlight(r#"{"value": null}"#);
        let output = segments.to_ansi();
        assert!(output.contains("null"));
    }

    #[test]
    fn test_empty_text() {
        let highlighter = ReprHighlighter::new();
        let segments = highlighter.highlight("");
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_no_matches() {
        let highlighter = ReprHighlighter::new();
        let segments = highlighter.highlight("just plain text");
        let output = segments.to_ansi();
        assert_eq!(output, "just plain text");
    }
}
