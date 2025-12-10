//! Markup parsing for console output.
//!
//! This module provides Rich-compatible markup syntax parsing.
//! Markup allows inline styling using tags like `[bold red]text[/]`.

use crate::errors::{Error, Result};
use crate::style::Style;
use crate::text::Text;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;

/// A regex pattern for matching markup tags.
static TAG_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[(/?)([^\]]*)\]").unwrap_or_else(|_| {
        // This should never fail as the pattern is valid
        Regex::new(r"$^").unwrap_or_else(|_| panic!("failed to compile regex"))
    })
});

/// Represents a parsed markup tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tag {
    /// Opening style tag with style definition.
    Open(Style),
    /// Closing tag (resets to previous style).
    Close,
    /// Named closing tag (closes specific style).
    CloseNamed(String),
}

impl Tag {
    /// Parses a tag from its content (without brackets).
    ///
    /// # Errors
    ///
    /// Returns an error if the tag content cannot be parsed.
    pub fn parse(content: &str, is_closing: bool) -> Result<Self> {
        let content = content.trim();

        if is_closing {
            if content.is_empty() {
                return Ok(Self::Close);
            }
            return Ok(Self::CloseNamed(content.to_owned()));
        }

        // Parse as style
        let style = Style::parse(content)?;
        Ok(Self::Open(style))
    }
}

/// A segment of parsed markup (either text or a tag).
#[derive(Debug, Clone)]
pub enum MarkupSegment {
    /// Plain text content.
    Text(String),
    /// A markup tag.
    Tag(Tag),
}

/// Parsed markup that can be converted to styled text.
#[derive(Debug, Clone, Default)]
pub struct Markup {
    /// The segments that make up this markup.
    segments: Vec<MarkupSegment>,
}

impl Markup {
    /// Creates empty markup.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    /// Parses markup from a string.
    ///
    /// # Syntax
    ///
    /// - `[style]text[/]` - Apply style to text
    /// - `[bold]`, `[italic]`, etc. - Style attributes
    /// - `[red]`, `[#ff0000]`, etc. - Colors
    /// - `[bold red on white]` - Combined styles
    /// - `[/]` - Close the most recent tag
    /// - `[/bold]` - Close a specific style (for nested styles)
    /// - `\\[` - Escaped bracket (literal `[`)
    ///
    /// # Errors
    ///
    /// Returns an error if the markup syntax is invalid.
    pub fn parse(s: &str) -> Result<Self> {
        let mut markup = Self::new();
        let mut last_end = 0;

        for cap in TAG_PATTERN.captures_iter(s) {
            let full_match = cap.get(0).ok_or_else(|| Error::MarkupParse {
                message: "regex capture failed".to_owned(),
            })?;

            // Check if this tag is escaped
            let match_start = full_match.start();
            if match_start > 0 {
                let prev_char_start = match_start.saturating_sub(1);
                if s.get(prev_char_start..match_start) == Some("\\") {
                    continue;
                }
            }

            // Add text before this tag
            if match_start > last_end {
                let text = s
                    .get(last_end..match_start)
                    .ok_or_else(|| Error::MarkupParse {
                        message: "invalid string slice".to_owned(),
                    })?;
                let unescaped = unescape_brackets(text);
                if !unescaped.is_empty() {
                    markup.segments.push(MarkupSegment::Text(unescaped));
                }
            }

            // Parse the tag
            let is_closing = cap.get(1).map(|m| m.as_str() == "/").unwrap_or(false);
            let tag_content = cap.get(2).map(|m| m.as_str()).unwrap_or("");

            // Skip if the tag is just whitespace inside brackets
            if tag_content.trim().is_empty() && !is_closing {
                // Treat as literal text
                let tag_text = full_match.as_str();
                markup.segments.push(MarkupSegment::Text(tag_text.to_owned()));
            } else {
                let tag = Tag::parse(tag_content, is_closing)?;
                markup.segments.push(MarkupSegment::Tag(tag));
            }

            last_end = full_match.end();
        }

        // Add remaining text
        if last_end < s.len() {
            let text = s.get(last_end..).ok_or_else(|| Error::MarkupParse {
                message: "invalid string slice".to_owned(),
            })?;
            let unescaped = unescape_brackets(text);
            if !unescaped.is_empty() {
                markup.segments.push(MarkupSegment::Text(unescaped));
            }
        }

        Ok(markup)
    }

    /// Converts the parsed markup to a Text object.
    #[must_use]
    pub fn to_text(&self) -> Text {
        let mut text = Text::new();
        let mut style_stack: Vec<Style> = Vec::new();

        for segment in &self.segments {
            match segment {
                MarkupSegment::Text(s) => {
                    let current_style = style_stack.last().cloned().unwrap_or_default();
                    if current_style.is_empty() {
                        text.append_plain(s);
                    } else {
                        text.append_styled(s, current_style);
                    }
                }
                MarkupSegment::Tag(tag) => match tag {
                    Tag::Open(style) => {
                        // Combine with current style and push
                        let current = style_stack.last().cloned().unwrap_or_default();
                        let combined = current.combine(style);
                        style_stack.push(combined);
                    }
                    Tag::Close => {
                        style_stack.pop();
                    }
                    Tag::CloseNamed(_name) => {
                        // For simplicity, just pop the last style
                        // A more sophisticated implementation would track named styles
                        style_stack.pop();
                    }
                },
            }
        }

        text
    }

    /// Returns the plain text content without markup.
    #[must_use]
    pub fn plain_text(&self) -> String {
        let mut result = String::new();
        for segment in &self.segments {
            if let MarkupSegment::Text(s) = segment {
                result.push_str(s);
            }
        }
        result
    }

    /// Returns true if the markup is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
            || self
                .segments
                .iter()
                .all(|s| matches!(s, MarkupSegment::Text(t) if t.is_empty()))
    }
}

impl fmt::Display for Markup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.plain_text())
    }
}

impl From<&str> for Markup {
    fn from(s: &str) -> Self {
        Self::parse(s).unwrap_or_default()
    }
}

impl From<String> for Markup {
    fn from(s: String) -> Self {
        Self::parse(&s).unwrap_or_default()
    }
}

/// Unescapes bracket characters in text.
fn unescape_brackets(s: &str) -> String {
    s.replace("\\[", "[").replace("\\]", "]")
}

/// Escapes markup characters in plain text.
#[must_use]
pub fn escape(s: &str) -> String {
    s.replace('[', "\\[").replace(']', "\\]")
}

/// Renders markup to ANSI-escaped text.
///
/// # Errors
///
/// Returns an error if the markup is invalid.
pub fn render(s: &str) -> Result<String> {
    let markup = Markup::parse(s)?;
    let text = markup.to_text();
    let segments = text.to_segments();
    Ok(segments.to_ansi())
}

/// Renders markup and strips to plain text.
///
/// # Errors
///
/// Returns an error if the markup is invalid.
pub fn render_plain(s: &str) -> Result<String> {
    let markup = Markup::parse(s)?;
    Ok(markup.plain_text())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_plain_text() {
        let markup = Markup::parse("hello world").ok().unwrap_or_default();
        assert_eq!(markup.plain_text(), "hello world");
    }

    #[test]
    fn test_parse_simple_tag() {
        let markup = Markup::parse("[bold]hello[/]").ok().unwrap_or_default();
        assert_eq!(markup.plain_text(), "hello");
        assert_eq!(markup.segments.len(), 3);
    }

    #[test]
    fn test_parse_nested_tags() {
        let markup = Markup::parse("[bold][red]hello[/][/]").ok().unwrap_or_default();
        assert_eq!(markup.plain_text(), "hello");
    }

    #[test]
    fn test_parse_color() {
        let markup = Markup::parse("[red]hello[/]").ok().unwrap_or_default();
        assert_eq!(markup.plain_text(), "hello");
    }

    #[test]
    fn test_parse_combined_style() {
        let markup = Markup::parse("[bold red on white]hello[/]")
            .ok()
            .unwrap_or_default();
        assert_eq!(markup.plain_text(), "hello");
    }

    #[test]
    fn test_parse_escaped_bracket() {
        let markup = Markup::parse("hello \\[world\\]").ok().unwrap_or_default();
        assert_eq!(markup.plain_text(), "hello [world]");
    }

    #[test]
    fn test_to_text_plain() {
        let markup = Markup::parse("hello").ok().unwrap_or_default();
        let text = markup.to_text();
        assert_eq!(text.plain(), "hello");
        assert!(text.to_segments().iter().all(|s| s.style.is_none()));
    }

    #[test]
    fn test_to_text_styled() {
        let markup = Markup::parse("[bold]hello[/]").ok().unwrap_or_default();
        let text = markup.to_text();
        assert_eq!(text.plain(), "hello");

        let segments = text.to_segments();
        let styled_segment = segments.iter().find(|s| s.text == "hello");
        assert!(styled_segment.is_some());
        if let Some(seg) = styled_segment {
            assert!(seg.style.is_some());
            assert_eq!(seg.style.as_ref().and_then(|s| s.attributes.bold), Some(true));
        }
    }

    #[test]
    fn test_to_text_nested() {
        let markup = Markup::parse("[bold][italic]hello[/][/]")
            .ok()
            .unwrap_or_default();
        let text = markup.to_text();
        assert_eq!(text.plain(), "hello");

        let segments = text.to_segments();
        let styled_segment = segments.iter().find(|s| s.text == "hello");
        if let Some(seg) = styled_segment {
            let style = seg.style.as_ref();
            assert_eq!(style.and_then(|s| s.attributes.bold), Some(true));
            assert_eq!(style.and_then(|s| s.attributes.italic), Some(true));
        }
    }

    #[test]
    fn test_render() {
        let result = render("[bold]hello[/]").ok().unwrap_or_default();
        assert!(result.contains("hello"));
        assert!(result.contains("\x1b[")); // Contains ANSI codes
    }

    #[test]
    fn test_render_plain() {
        let result = render_plain("[bold red]hello[/] [italic]world[/]")
            .ok()
            .unwrap_or_default();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_escape() {
        assert_eq!(escape("hello [world]"), "hello \\[world\\]");
        assert_eq!(escape("no brackets"), "no brackets");
    }

    #[test]
    fn test_tag_parse_open() {
        let tag = Tag::parse("bold", false).ok();
        assert!(matches!(tag, Some(Tag::Open(_))));
    }

    #[test]
    fn test_tag_parse_close() {
        let tag = Tag::parse("", true).ok();
        assert!(matches!(tag, Some(Tag::Close)));
    }

    #[test]
    fn test_tag_parse_close_named() {
        let tag = Tag::parse("bold", true).ok();
        assert!(matches!(tag, Some(Tag::CloseNamed(name)) if name == "bold"));
    }

    #[test]
    fn test_markup_from_str() {
        let markup: Markup = "[bold]hello[/]".into();
        assert_eq!(markup.plain_text(), "hello");
    }

    #[test]
    fn test_markup_from_string() {
        let markup: Markup = "[italic]world[/]".to_owned().into();
        assert_eq!(markup.plain_text(), "world");
    }

    #[test]
    fn test_markup_display() {
        let markup = Markup::parse("[bold]hello[/]").ok().unwrap_or_default();
        assert_eq!(format!("{markup}"), "hello");
    }

    #[test]
    fn test_markup_is_empty() {
        let markup = Markup::new();
        assert!(markup.is_empty());

        let markup = Markup::parse("").ok().unwrap_or_default();
        assert!(markup.is_empty());

        let markup = Markup::parse("hello").ok().unwrap_or_default();
        assert!(!markup.is_empty());
    }

    #[test]
    fn test_complex_markup() {
        let input = "[bold]Hello[/], [red]world[/]! This is [italic blue on white]styled[/] text.";
        let markup = Markup::parse(input).ok().unwrap_or_default();
        assert_eq!(
            markup.plain_text(),
            "Hello, world! This is styled text."
        );
    }

    #[test]
    fn test_hex_color_in_markup() {
        let markup = Markup::parse("[#ff0000]red text[/]").ok().unwrap_or_default();
        assert_eq!(markup.plain_text(), "red text");
    }

    #[test]
    fn test_rgb_color_in_markup() {
        let markup = Markup::parse("[rgb(255,0,0)]red text[/]")
            .ok()
            .unwrap_or_default();
        assert_eq!(markup.plain_text(), "red text");
    }
}
