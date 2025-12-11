// Default theme lookup uses expect() on a known-valid theme name
#![allow(clippy::expect_used)]

//! Syntax highlighting for code.
//!
//! This module provides syntax highlighting using the `syntect` library.
//! It requires the `syntax` feature to be enabled.
//!
//! # Example
//!
//! ```ignore
//! use richrs::syntax::Syntax;
//!
//! let code = r#"
//! fn main() {
//!     println!("Hello, World!");
//! }
//! "#;
//!
//! let syntax = Syntax::new(code, "rust")
//!     .line_numbers(true)
//!     .theme("base16-ocean.dark");
//!
//! let segments = syntax.render(80);
//! ```

#[cfg(feature = "syntax")]
use syntect::easy::HighlightLines;
#[cfg(feature = "syntax")]
use syntect::highlighting::{Style as SyntectStyle, ThemeSet};
#[cfg(feature = "syntax")]
use syntect::parsing::SyntaxSet;
#[cfg(feature = "syntax")]
use syntect::util::LinesWithEndings;

#[cfg(feature = "syntax")]
use crate::color::Color;
use crate::segment::{Segment, Segments};
use crate::style::Style;

#[cfg(feature = "syntax")]
use std::sync::LazyLock;

#[cfg(feature = "syntax")]
static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);

#[cfg(feature = "syntax")]
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

/// Syntax-highlighted code display.
///
/// Renders code with syntax highlighting based on the specified language.
#[derive(Debug, Clone)]
pub struct Syntax {
    /// The code to highlight.
    code: String,
    /// The language/lexer name.
    lexer: String,
    /// The theme name.
    theme: String,
    /// Whether to show line numbers.
    line_numbers: bool,
    /// Starting line number.
    start_line: usize,
    /// Range of lines to display (start, end).
    line_range: Option<(usize, usize)>,
    /// Lines to highlight specially.
    highlight_lines: Option<Vec<usize>>,
    /// Tab size for indentation.
    tab_size: usize,
    /// Whether to word wrap long lines.
    word_wrap: bool,
    /// Whether to show indent guides.
    indent_guides: bool,
    /// Padding around the code.
    padding: usize,
}

impl Syntax {
    /// Creates a new Syntax highlighter.
    ///
    /// # Arguments
    ///
    /// * `code` - The source code to highlight
    /// * `lexer` - The language name (e.g., "rust", "python", "javascript")
    #[must_use]
    #[inline]
    pub fn new(code: impl Into<String>, lexer: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            lexer: lexer.into(),
            theme: "base16-ocean.dark".to_owned(),
            line_numbers: false,
            start_line: 1,
            line_range: None,
            highlight_lines: None,
            tab_size: 4,
            word_wrap: false,
            indent_guides: false,
            padding: 0,
        }
    }

    /// Sets the color theme.
    ///
    /// Available themes: "base16-ocean.dark", "base16-eighties.dark",
    /// "base16-mocha.dark", "base16-ocean.light", "InspiredGitHub", "Solarized (dark)", "Solarized (light)"
    #[must_use]
    #[inline]
    pub fn theme(mut self, theme: impl Into<String>) -> Self {
        self.theme = theme.into();
        self
    }

    /// Enables or disables line numbers.
    #[must_use]
    #[inline]
    pub const fn line_numbers(mut self, show: bool) -> Self {
        self.line_numbers = show;
        self
    }

    /// Sets the starting line number.
    #[must_use]
    #[inline]
    pub const fn start_line(mut self, line: usize) -> Self {
        self.start_line = line;
        self
    }

    /// Sets the range of lines to display.
    #[must_use]
    #[inline]
    pub const fn line_range(mut self, start: usize, end: usize) -> Self {
        self.line_range = Some((start, end));
        self
    }

    /// Sets lines to highlight specially.
    #[must_use]
    #[inline]
    pub fn highlight_lines(mut self, lines: Vec<usize>) -> Self {
        self.highlight_lines = Some(lines);
        self
    }

    /// Sets the tab size.
    #[must_use]
    #[inline]
    pub const fn tab_size(mut self, size: usize) -> Self {
        self.tab_size = size;
        self
    }

    /// Enables or disables word wrapping.
    #[must_use]
    #[inline]
    pub const fn word_wrap(mut self, wrap: bool) -> Self {
        self.word_wrap = wrap;
        self
    }

    /// Enables or disables indent guides.
    #[must_use]
    #[inline]
    pub const fn indent_guides(mut self, show: bool) -> Self {
        self.indent_guides = show;
        self
    }

    /// Sets padding around the code.
    #[must_use]
    #[inline]
    pub const fn padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    /// Renders the syntax-highlighted code to segments.
    #[cfg(feature = "syntax")]
    #[must_use]
    pub fn render(&self, _max_width: usize) -> Segments {
        let mut segments = Segments::new();

        // Find the syntax definition
        let syntax = SYNTAX_SET
            .find_syntax_by_token(&self.lexer)
            .or_else(|| SYNTAX_SET.find_syntax_by_extension(&self.lexer))
            .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());

        // Get the theme
        let theme = THEME_SET
            .themes
            .get(&self.theme)
            .or_else(|| THEME_SET.themes.get("base16-ocean.dark"))
            .expect("default theme should exist");

        let mut highlighter = HighlightLines::new(syntax, theme);

        // Process lines
        let lines: Vec<&str> = LinesWithEndings::from(&self.code).collect();
        let total_lines = lines.len();

        // Determine line range
        let (start_idx, end_idx) = match self.line_range {
            Some((start, end)) => (
                start.saturating_sub(1).min(total_lines),
                end.min(total_lines),
            ),
            None => (0, total_lines),
        };

        // Calculate line number width
        let max_line_num = self
            .start_line
            .saturating_add(end_idx)
            .saturating_sub(start_idx);
        let line_num_width = max_line_num.to_string().len();

        // Add top padding
        for _ in 0..self.padding {
            segments.push(Segment::newline());
        }

        for (idx, line) in lines
            .iter()
            .enumerate()
            .skip(start_idx)
            .take(end_idx.saturating_sub(start_idx))
        {
            let line_num = self
                .start_line
                .saturating_add(idx)
                .saturating_sub(start_idx);

            // Check if this line should be highlighted
            let is_highlighted = self
                .highlight_lines
                .as_ref()
                .is_some_and(|hl| hl.contains(&line_num));

            // Add left padding
            if self.padding > 0 {
                segments.push(Segment::new(" ".repeat(self.padding)));
            }

            // Add line number if enabled
            if self.line_numbers {
                let line_num_str = format!("{:>width$} ", line_num, width = line_num_width);
                let style = Style::default().dim();
                segments.push(Segment::styled(line_num_str, style));
            }

            // Highlight the line
            let highlighted = highlighter
                .highlight_line(line, &SYNTAX_SET)
                .unwrap_or_default();

            for (syntect_style, text) in highlighted {
                let style = syntect_style_to_style(syntect_style, is_highlighted);
                let text = text.replace('\t', &" ".repeat(self.tab_size));
                segments.push(Segment::styled(text, style));
            }

            // Ensure line ends with newline
            if !line.ends_with('\n') {
                segments.push(Segment::newline());
            }
        }

        // Add bottom padding
        for _ in 0..self.padding {
            segments.push(Segment::newline());
        }

        segments
    }

    /// Renders the code without syntax highlighting (fallback when feature is disabled).
    #[cfg(not(feature = "syntax"))]
    #[must_use]
    pub fn render(&self, _max_width: usize) -> Segments {
        let mut segments = Segments::new();

        let lines: Vec<&str> = self.code.lines().collect();
        let total_lines = lines.len();

        // Determine line range
        let (start_idx, end_idx) = match self.line_range {
            Some((start, end)) => (
                start.saturating_sub(1).min(total_lines),
                end.min(total_lines),
            ),
            None => (0, total_lines),
        };

        // Calculate line number width
        let max_line_num = self
            .start_line
            .saturating_add(end_idx)
            .saturating_sub(start_idx);
        let line_num_width = max_line_num.to_string().len();

        // Add top padding
        for _ in 0..self.padding {
            segments.push(Segment::newline());
        }

        for (idx, line) in lines
            .iter()
            .enumerate()
            .skip(start_idx)
            .take(end_idx.saturating_sub(start_idx))
        {
            let line_num = self
                .start_line
                .saturating_add(idx)
                .saturating_sub(start_idx);

            // Add left padding
            if self.padding > 0 {
                segments.push(Segment::new(" ".repeat(self.padding)));
            }

            // Add line number if enabled
            if self.line_numbers {
                let line_num_str = format!("{:>width$} ", line_num, width = line_num_width);
                let style = Style::default().dim();
                segments.push(Segment::styled(line_num_str, style));
            }

            // Add the line (no highlighting)
            let text = line.replace('\t', &" ".repeat(self.tab_size));
            segments.push(Segment::new(text));
            segments.push(Segment::newline());
        }

        // Add bottom padding
        for _ in 0..self.padding {
            segments.push(Segment::newline());
        }

        segments
    }

    /// Returns the code as plain text.
    #[must_use]
    #[inline]
    pub fn plain(&self) -> &str {
        &self.code
    }

    /// Returns the lexer/language name.
    #[must_use]
    #[inline]
    pub fn lexer(&self) -> &str {
        &self.lexer
    }

    /// Returns a list of available themes.
    #[cfg(feature = "syntax")]
    #[must_use]
    pub fn available_themes() -> Vec<String> {
        THEME_SET.themes.keys().cloned().collect()
    }

    /// Returns a list of available themes (empty when feature is disabled).
    #[cfg(not(feature = "syntax"))]
    #[must_use]
    pub fn available_themes() -> Vec<String> {
        Vec::new()
    }

    /// Returns a list of available languages.
    #[cfg(feature = "syntax")]
    #[must_use]
    pub fn available_languages() -> Vec<String> {
        SYNTAX_SET
            .syntaxes()
            .iter()
            .map(|s| s.name.clone())
            .collect()
    }

    /// Returns a list of available languages (empty when feature is disabled).
    #[cfg(not(feature = "syntax"))]
    #[must_use]
    pub fn available_languages() -> Vec<String> {
        Vec::new()
    }
}

/// Converts a syntect style to our Style.
#[cfg(feature = "syntax")]
fn syntect_style_to_style(syntect_style: SyntectStyle, highlighted: bool) -> Style {
    let fg = syntect_style.foreground;
    let fg_color = Color::Rgb {
        r: fg.r,
        g: fg.g,
        b: fg.b,
    };
    let mut style = Style::default().with_color(fg_color);

    if highlighted {
        // Add a subtle background for highlighted lines
        let bg_color = Color::Rgb {
            r: 60,
            g: 60,
            b: 80,
        };
        style = style.with_bgcolor(bg_color);
    }

    style
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_new() {
        let syntax = Syntax::new("let x = 1;", "rust");
        assert_eq!(syntax.code, "let x = 1;");
        assert_eq!(syntax.lexer, "rust");
    }

    #[test]
    fn test_syntax_new_strings() {
        let syntax = Syntax::new(String::from("code"), String::from("python"));
        assert_eq!(syntax.code, "code");
        assert_eq!(syntax.lexer, "python");
    }

    #[test]
    fn test_syntax_theme() {
        let syntax = Syntax::new("code", "rust").theme("Solarized (dark)");
        assert_eq!(syntax.theme, "Solarized (dark)");
    }

    #[test]
    fn test_syntax_theme_string() {
        let syntax = Syntax::new("code", "rust").theme(String::from("base16-ocean.dark"));
        assert_eq!(syntax.theme, "base16-ocean.dark");
    }

    #[test]
    fn test_syntax_line_numbers() {
        let syntax = Syntax::new("code", "rust").line_numbers(true);
        assert!(syntax.line_numbers);
    }

    #[test]
    fn test_syntax_line_numbers_false() {
        let syntax = Syntax::new("code", "rust").line_numbers(false);
        assert!(!syntax.line_numbers);
    }

    #[test]
    fn test_syntax_start_line() {
        let syntax = Syntax::new("code", "rust").start_line(10);
        assert_eq!(syntax.start_line, 10);
    }

    #[test]
    fn test_syntax_line_range() {
        let syntax = Syntax::new("code", "rust").line_range(5, 15);
        assert_eq!(syntax.line_range, Some((5, 15)));
    }

    #[test]
    fn test_syntax_highlight_lines() {
        let syntax = Syntax::new("code", "rust").highlight_lines(vec![1, 3, 5]);
        assert_eq!(syntax.highlight_lines, Some(vec![1, 3, 5]));
    }

    #[test]
    fn test_syntax_tab_size() {
        let syntax = Syntax::new("code", "rust").tab_size(2);
        assert_eq!(syntax.tab_size, 2);
    }

    #[test]
    fn test_syntax_word_wrap() {
        let syntax = Syntax::new("code", "rust").word_wrap(true);
        assert!(syntax.word_wrap);
    }

    #[test]
    fn test_syntax_indent_guides() {
        let syntax = Syntax::new("code", "rust").indent_guides(true);
        assert!(syntax.indent_guides);
    }

    #[test]
    fn test_syntax_padding() {
        let syntax = Syntax::new("code", "rust").padding(2);
        assert_eq!(syntax.padding, 2);
    }

    #[test]
    fn test_syntax_plain() {
        let syntax = Syntax::new("let x = 1;", "rust");
        assert_eq!(syntax.plain(), "let x = 1;");
    }

    #[test]
    fn test_syntax_lexer() {
        let syntax = Syntax::new("code", "python");
        assert_eq!(syntax.lexer(), "python");
    }

    #[test]
    fn test_syntax_render() {
        let syntax = Syntax::new("fn main() {}\n", "rust").line_numbers(true);
        let segments = syntax.render(80);
        let output = segments.to_ansi();
        // With or without syntax feature, the code should appear in output
        assert!(output.contains("main"));
    }

    #[test]
    fn test_syntax_render_with_padding() {
        let syntax = Syntax::new("let x = 1;\n", "rust").padding(2);
        let segments = syntax.render(80);
        // Should render without errors
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_syntax_render_with_line_range() {
        let code = "line1\nline2\nline3\nline4\nline5\n";
        let syntax = Syntax::new(code, "text").line_range(2, 4);
        let segments = syntax.render(80);
        let output = segments.plain_text();
        assert!(output.contains("line2"));
        assert!(output.contains("line3"));
        assert!(output.contains("line4"));
        assert!(!output.contains("line1"));
        assert!(!output.contains("line5"));
    }

    #[test]
    fn test_syntax_render_with_highlight_lines() {
        let code = "line1\nline2\nline3\n";
        let syntax = Syntax::new(code, "text")
            .line_numbers(true)
            .highlight_lines(vec![2]);
        let segments = syntax.render(80);
        // Should render without errors
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_syntax_render_with_tabs() {
        let code = "fn main() {\n\tprintln!(\"hello\");\n}\n";
        let syntax = Syntax::new(code, "rust").tab_size(4);
        let segments = syntax.render(80);
        let output = segments.plain_text();
        // Tabs should be replaced with spaces
        assert!(!output.contains('\t'));
    }

    #[test]
    fn test_syntax_render_without_trailing_newline() {
        let code = "let x = 1"; // No trailing newline
        let syntax = Syntax::new(code, "rust");
        let segments = syntax.render(80);
        // Should still render properly
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_syntax_render_empty_code() {
        let syntax = Syntax::new("", "rust");
        let segments = syntax.render(80);
        // Empty code should produce empty or minimal output
        let text = segments.plain_text();
        assert!(text.is_empty() || text.trim().is_empty());
    }

    #[test]
    fn test_syntax_available_themes() {
        let themes = Syntax::available_themes();
        // With syntax feature, should have some themes; without, empty
        #[cfg(feature = "syntax")]
        assert!(!themes.is_empty());
        #[cfg(not(feature = "syntax"))]
        assert!(themes.is_empty());
    }

    #[test]
    fn test_syntax_available_languages() {
        let languages = Syntax::available_languages();
        // With syntax feature, should have some languages; without, empty
        #[cfg(feature = "syntax")]
        assert!(!languages.is_empty());
        #[cfg(not(feature = "syntax"))]
        assert!(languages.is_empty());
    }

    #[test]
    fn test_syntax_builder_chain() {
        let syntax = Syntax::new("fn main() {}", "rust")
            .theme("base16-ocean.dark")
            .line_numbers(true)
            .start_line(10)
            .line_range(1, 100)
            .highlight_lines(vec![1, 2, 3])
            .tab_size(2)
            .word_wrap(true)
            .indent_guides(true)
            .padding(1);

        assert_eq!(syntax.theme, "base16-ocean.dark");
        assert!(syntax.line_numbers);
        assert_eq!(syntax.start_line, 10);
        assert_eq!(syntax.line_range, Some((1, 100)));
        assert_eq!(syntax.highlight_lines, Some(vec![1, 2, 3]));
        assert_eq!(syntax.tab_size, 2);
        assert!(syntax.word_wrap);
        assert!(syntax.indent_guides);
        assert_eq!(syntax.padding, 1);
    }

    #[test]
    fn test_syntax_render_multiline() {
        let code = r#"fn main() {
    let x = 1;
    let y = 2;
    println!("{}", x + y);
}
"#;
        let syntax = Syntax::new(code, "rust").line_numbers(true);
        let segments = syntax.render(80);
        let output = segments.plain_text();
        assert!(output.contains("main"));
        assert!(output.contains("println"));
    }

    #[test]
    fn test_syntax_render_python() {
        let code = "def main():\n    print('hello')\n";
        let syntax = Syntax::new(code, "python");
        let segments = syntax.render(80);
        let output = segments.plain_text();
        assert!(output.contains("main"));
        assert!(output.contains("print"));
    }

    #[test]
    fn test_syntax_render_javascript() {
        let code = "function hello() { console.log('hi'); }\n";
        let syntax = Syntax::new(code, "javascript");
        let segments = syntax.render(80);
        let output = segments.plain_text();
        assert!(output.contains("hello"));
    }

    #[test]
    fn test_syntax_render_unknown_language() {
        let code = "some random code\n";
        let syntax = Syntax::new(code, "nonexistent_language");
        let segments = syntax.render(80);
        // Should fall back to plain text
        let output = segments.plain_text();
        assert!(output.contains("some random code"));
    }

    #[test]
    fn test_syntax_default_values() {
        let syntax = Syntax::new("code", "rust");
        assert_eq!(syntax.theme, "base16-ocean.dark");
        assert!(!syntax.line_numbers);
        assert_eq!(syntax.start_line, 1);
        assert!(syntax.line_range.is_none());
        assert!(syntax.highlight_lines.is_none());
        assert_eq!(syntax.tab_size, 4);
        assert!(!syntax.word_wrap);
        assert!(!syntax.indent_guides);
        assert_eq!(syntax.padding, 0);
    }
}
