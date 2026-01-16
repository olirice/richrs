//! Markdown rendering for terminal output.
//!
//! This module provides Markdown parsing and rendering using the `pulldown-cmark` library.
//! It requires the `markdown` feature to be enabled.
//!
//! # Example
//!
//! ```ignore
//! use richrs::markdown::Markdown;
//!
//! let md = "# Hello\n\nThis is **bold** and *italic* text.";
//! let markdown = Markdown::new(md);
//! let segments = markdown.render(80);
//! ```

#[cfg(feature = "markdown")]
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

use crate::segment::{Segment, Segments};
use crate::style::Style;

/// Wraps text to fit within the specified width.
/// Returns a vector of wrapped lines.
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut result = Vec::new();

    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            result.push(String::new());
            continue;
        }

        let mut current_line = String::new();
        let mut current_width = 0;

        for word in paragraph.split_whitespace() {
            let word_len = word.chars().count();

            if current_width + word_len + if current_width > 0 { 1 } else { 0 } <= max_width {
                if current_width > 0 {
                    current_line.push(' ');
                    current_width += 1;
                }
                current_line.push_str(word);
                current_width += word_len;
            } else {
                if !current_line.is_empty() {
                    result.push(current_line);
                }
                current_line = word.to_string();
                current_width = word_len;
            }
        }

        if !current_line.is_empty() {
            result.push(current_line);
        }
    }

    result
}

/// Markdown renderer for terminal output.
///
/// Parses Markdown and renders it as styled terminal output.
#[derive(Debug, Clone)]
pub struct Markdown {
    /// The Markdown source text.
    source: String,
    /// Whether to show inline code with background.
    code_theme: bool,
    /// Whether to show hyperlinks.
    hyperlinks: bool,
    /// Inline code style.
    inline_code_style: Option<Style>,
}

impl Markdown {
    /// Creates a new Markdown renderer with the given source.
    #[must_use]
    #[inline]
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            code_theme: true,
            hyperlinks: true,
            inline_code_style: None,
        }
    }

    /// Sets whether to show inline code with background styling.
    #[must_use]
    #[inline]
    pub const fn code_theme(mut self, enabled: bool) -> Self {
        self.code_theme = enabled;
        self
    }

    /// Sets whether to render hyperlinks.
    #[must_use]
    #[inline]
    pub const fn hyperlinks(mut self, enabled: bool) -> Self {
        self.hyperlinks = enabled;
        self
    }

    /// Sets the inline code style.
    #[must_use]
    #[inline]
    pub fn inline_code_style(mut self, style: Style) -> Self {
        self.inline_code_style = Some(style);
        self
    }

    /// Returns the source Markdown text.
    #[must_use]
    #[inline]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Renders the Markdown to segments.
    #[cfg(feature = "markdown")]
    #[must_use]
    pub fn render(&self, _max_width: usize) -> Segments {
        let mut segments = Segments::new();
        let parser = Parser::new(&self.source);

        let mut in_heading = false;
        let mut heading_level = 0;
        let mut in_emphasis = false;
        let mut in_strong = false;
        let mut in_code = false;
        let mut _in_link = false;
        let mut link_url = String::new();
        let mut _in_list = false;
        let mut list_item_number: Option<u64> = None;
        let mut in_blockquote = false;

        for event in parser {
            match event {
                Event::Start(tag) => match tag {
                    Tag::Heading { level, .. } => {
                        in_heading = true;
                        heading_level = match level {
                            HeadingLevel::H1 => 1,
                            HeadingLevel::H2 => 2,
                            HeadingLevel::H3 => 3,
                            HeadingLevel::H4 => 4,
                            HeadingLevel::H5 => 5,
                            HeadingLevel::H6 => 6,
                        };
                    }
                    Tag::Emphasis => in_emphasis = true,
                    Tag::Strong => in_strong = true,
                    Tag::CodeBlock(_) => {
                        in_code = true;
                        segments.push(Segment::newline());
                    }
                    Tag::Link { dest_url, .. } => {
                        _in_link = true;
                        link_url = dest_url.to_string();
                    }
                    Tag::List(start) => {
                        _in_list = true;
                        list_item_number = start;
                    }
                    Tag::Item => {
                        if let Some(ref mut n) = list_item_number {
                            segments.push(Segment::new(format!("  {}. ", n)));
                            *n = n.saturating_add(1);
                        } else {
                            segments.push(Segment::new("  • "));
                        }
                    }
                    Tag::BlockQuote => {
                        in_blockquote = true;
                    }
                    Tag::Paragraph => {}
                    _ => {}
                },
                Event::End(tag_end) => match tag_end {
                    TagEnd::Heading(_) => {
                        in_heading = false;
                        segments.push(Segment::newline());
                        // Add underline for h1/h2
                        if heading_level <= 2 {
                            let char = if heading_level == 1 { '═' } else { '─' };
                            let width = if _max_width > 0 {_max_width} else {40};
                            segments.push(Segment::new(char.to_string().repeat(width)));
                            segments.push(Segment::newline());
                        }
                    }
                    TagEnd::Emphasis => in_emphasis = false,
                    TagEnd::Strong => in_strong = false,
                    TagEnd::CodeBlock => {
                        in_code = false;
                        segments.push(Segment::newline());
                    }
                    TagEnd::Link => {
                        if self.hyperlinks && !link_url.is_empty() {
                            let style = Style::default().underline();
                            segments.push(Segment::styled(format!(" ({})", link_url), style.dim()));
                        }
                        _in_link = false;
                        link_url.clear();
                    }
                    TagEnd::List(_) => {
                        _in_list = false;
                        list_item_number = None;
                    }
                    TagEnd::Item => {
                        segments.push(Segment::newline());
                    }
                    TagEnd::BlockQuote => {
                        in_blockquote = false;
                    }
                    TagEnd::Paragraph => {
                        segments.push(Segment::newline());
                        segments.push(Segment::newline());
                    }
                    _ => {}
                },
                Event::Text(text) => {
                    let content = text.to_string();

                    // Apply styles based on context
                    let mut style = Style::default();

                    if in_heading {
                        style = style.bold();
                        if heading_level == 1 {
                            // H1 gets extra emphasis
                        }
                    }

                    if in_emphasis {
                        style = style.italic();
                    }

                    if in_strong {
                        style = style.bold();
                    }

                    if in_blockquote {
                        style = style.dim().italic();
                        segments.push(Segment::styled("│ ", style.clone()));
                    }

                    let wrapped_lines = if _max_width > 0 {
                        wrap_text(&content, _max_width)
                    } else {
                        vec![content.clone()]
                    };

                    for line in wrapped_lines {
                        if in_code {
                            if self.code_theme {
                                style = style.dim();
                            }
                            // Indent code blocks
                            let indented = line
                                .lines()
                                .map(|l| format!("    {}", l))
                                .collect::<Vec<_>>()
                                .join("\n");
                            segments.push(Segment::styled(indented, style.clone()));
                            segments.push(Segment::newline())
                        } else if style.is_empty() {
                            segments.push(Segment::new(line));
                            segments.push(Segment::newline())
                        } else {
                            segments.push(Segment::styled(line, style.clone()));
                            segments.push(Segment::newline())
                        }
                    }
                }
                Event::Code(code) => {
                    let style = self
                        .inline_code_style
                        .clone()
                        .unwrap_or_else(|| Style::default().reverse());
                    segments.push(Segment::styled(format!(" {} ", code), style));
                }
                Event::SoftBreak | Event::HardBreak => {
                    segments.push(Segment::newline());
                }
                Event::Rule => {
                    segments.push(Segment::newline());
                    let width = if _max_width > 0 {_max_width} else {40};
                    segments.push(Segment::new("─".repeat(width)));
                    segments.push(Segment::newline());
                }
                _ => {}
            }
        }

        segments
    }

    /// Renders the Markdown as plain text (fallback when feature is disabled).
    #[cfg(not(feature = "markdown"))]
    #[must_use]
    pub fn render(&self, _max_width: usize) -> Segments {
        let mut segments = Segments::new();

        // Simple fallback: just output the raw markdown
        for line in self.source.lines() {
            // Basic transformations
            let line = if line.starts_with("# ") {
                // H1
                let text = line.trim_start_matches("# ");
                segments.push(Segment::styled(text.to_string(), Style::default().bold()));
                segments.push(Segment::newline());
                segments.push(Segment::new("═".repeat(40)));
                segments.push(Segment::newline());
                continue;
            } else if line.starts_with("## ") {
                // H2
                let text = line.trim_start_matches("## ");
                segments.push(Segment::styled(text.to_string(), Style::default().bold()));
                segments.push(Segment::newline());
                segments.push(Segment::new("─".repeat(40)));
                segments.push(Segment::newline());
                continue;
            } else if line.starts_with("- ") || line.starts_with("* ") {
                // List items
                let text = line.trim_start_matches("- ").trim_start_matches("* ");
                format!("  • {}", text)
            } else if line.starts_with("> ") {
                // Blockquote
                let text = line.trim_start_matches("> ");
                let style = Style::default().dim().italic();
                segments.push(Segment::styled("│ ", style.clone()));
                segments.push(Segment::styled(text.to_string(), style));
                segments.push(Segment::newline());
                continue;
            } else if line == "---" || line == "***" {
                // Horizontal rule
                segments.push(Segment::new("─".repeat(40)));
                segments.push(Segment::newline());
                continue;
            } else {
                line.to_string()
            };

            // Handle inline formatting (basic)
            let processed = process_inline_markdown(&line);
            segments.extend(processed.into_iter());
            segments.push(Segment::newline());
        }

        segments
    }
}

/// Process inline markdown (bold, italic, code) without pulldown-cmark.
#[cfg(not(feature = "markdown"))]
fn process_inline_markdown(text: &str) -> Segments {
    let mut segments = Segments::new();
    let mut current = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '*' && chars.peek() == Some(&'*') {
            // Bold
            if !current.is_empty() {
                segments.push(Segment::new(std::mem::take(&mut current)));
            }
            chars.next(); // consume second *
            let mut bold_text = String::new();
            while let Some(c) = chars.next() {
                if c == '*' && chars.peek() == Some(&'*') {
                    chars.next();
                    break;
                }
                bold_text.push(c);
            }
            segments.push(Segment::styled(bold_text, Style::default().bold()));
        } else if ch == '*' || ch == '_' {
            // Italic
            if !current.is_empty() {
                segments.push(Segment::new(std::mem::take(&mut current)));
            }
            let mut italic_text = String::new();
            for c in chars.by_ref() {
                if c == ch {
                    break;
                }
                italic_text.push(c);
            }
            segments.push(Segment::styled(italic_text, Style::default().italic()));
        } else if ch == '`' {
            // Inline code
            if !current.is_empty() {
                segments.push(Segment::new(std::mem::take(&mut current)));
            }
            let mut code_text = String::new();
            for c in chars.by_ref() {
                if c == '`' {
                    break;
                }
                code_text.push(c);
            }
            segments.push(Segment::styled(
                format!(" {} ", code_text),
                Style::default().reverse(),
            ));
        } else {
            current.push(ch);
        }
    }

    if !current.is_empty() {
        segments.push(Segment::new(current));
    }

    segments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_new() {
        let md = Markdown::new("# Hello");
        assert_eq!(md.source(), "# Hello");
    }

    #[test]
    fn test_markdown_code_theme() {
        let md = Markdown::new("text").code_theme(false);
        assert!(!md.code_theme);
    }

    #[test]
    fn test_markdown_hyperlinks() {
        let md = Markdown::new("text").hyperlinks(false);
        assert!(!md.hyperlinks);
    }

    #[test]
    fn test_markdown_inline_code_style() {
        let style = Style::default().bold();
        let md = Markdown::new("text").inline_code_style(style);
        assert!(md.inline_code_style.is_some());
    }

    #[test]
    fn test_markdown_render_heading() {
        let md = Markdown::new("# Hello\n\nWorld");
        let segments = md.render(80);
        let output = segments.to_ansi();
        assert!(output.contains("Hello"));
        assert!(output.contains("World"));
    }

    #[test]
    fn test_markdown_render_list() {
        let md = Markdown::new("- Item 1\n- Item 2");
        let segments = md.render(80);
        let output = segments.to_ansi();
        assert!(output.contains("Item 1"));
        assert!(output.contains("Item 2"));
    }

    #[test]
    fn test_markdown_render_code() {
        let md = Markdown::new("Use `code` here");
        let segments = md.render(80);
        let output = segments.to_ansi();
        assert!(output.contains("code"));
    }
}
