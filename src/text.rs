//! Rich text with inline styling support.
//!
//! The Text type represents styled text that can be rendered to the console.
//! It supports inline styling through spans, similar to how Rich handles text.

use crate::errors::Result;
use crate::measure::{Measurable, MeasureOptions, Measurement};
use crate::segment::{Segment, Segments};
use crate::style::Style;
use serde::{Deserialize, Serialize};
use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

/// A span of styled text within a Text object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// Start position (character index).
    pub start: usize,
    /// End position (character index, exclusive).
    pub end: usize,
    /// Style to apply to this span.
    pub style: Style,
}

impl Span {
    /// Creates a new span.
    #[inline]
    #[must_use]
    pub const fn new(start: usize, end: usize, style: Style) -> Self {
        Self { start, end, style }
    }

    /// Returns the length of this span.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Returns true if this span has zero length.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.end <= self.start
    }

    /// Returns true if this span overlaps with the given range.
    #[inline]
    #[must_use]
    pub const fn overlaps(&self, start: usize, end: usize) -> bool {
        self.start < end && self.end > start
    }

    /// Adjusts the span by an offset.
    #[inline]
    #[must_use]
    pub fn offset(self, offset: usize) -> Self {
        Self {
            start: self.start.saturating_add(offset),
            end: self.end.saturating_add(offset),
            style: self.style,
        }
    }
}

/// Text justification options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Justify {
    /// Default alignment (typically left).
    #[default]
    Default,
    /// Left-aligned text.
    Left,
    /// Right-aligned text.
    Right,
    /// Center-aligned text.
    Center,
    /// Full justification (stretch to fill width).
    Full,
}

/// Text overflow behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Overflow {
    /// Fold text at word boundaries.
    #[default]
    Fold,
    /// Crop text at the edge.
    Crop,
    /// Show ellipsis for cropped text.
    Ellipsis,
    /// Ignore width constraints.
    Ignore,
}

/// Rich text with styling support.
///
/// Text objects hold plain text content along with style spans that define
/// how different parts of the text should be rendered.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Text {
    /// The plain text content.
    plain: String,
    /// Style spans applied to the text.
    spans: Vec<Span>,
    /// Text justification.
    pub justify: Justify,
    /// Overflow behavior.
    pub overflow: Overflow,
    /// Disable word wrapping.
    pub no_wrap: bool,
    /// Tab size in spaces.
    pub tab_size: usize,
}

impl Text {
    /// Creates a new empty Text object.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            plain: String::new(),
            spans: Vec::new(),
            justify: Justify::Default,
            overflow: Overflow::Fold,
            no_wrap: false,
            tab_size: 8,
        }
    }

    /// Creates a Text object from a plain string.
    #[inline]
    #[must_use]
    pub fn from_str(s: impl Into<String>) -> Self {
        Self {
            plain: s.into(),
            ..Self::new()
        }
    }

    /// Creates a Text object from a string with a style applied to all of it.
    #[must_use]
    pub fn styled(s: impl Into<String>, style: Style) -> Self {
        let plain: String = s.into();
        let len = plain.graphemes(true).count();
        let mut text = Self {
            plain,
            ..Self::new()
        };
        if !style.is_empty() {
            text.spans.push(Span::new(0, len, style));
        }
        text
    }

    /// Creates a Text object by assembling multiple string/style pairs.
    ///
    /// # Arguments
    ///
    /// * `parts` - An iterator of (text, optional style) tuples
    #[must_use]
    pub fn assemble<I, S>(parts: I) -> Self
    where
        I: IntoIterator<Item = (S, Option<Style>)>,
        S: AsRef<str>,
    {
        let mut text = Self::new();
        for (s, style) in parts {
            text.append(s.as_ref(), style);
        }
        text
    }

    /// Returns the plain text content.
    #[inline]
    #[must_use]
    pub fn plain(&self) -> &str {
        &self.plain
    }

    /// Returns the character count (grapheme clusters).
    #[must_use]
    pub fn char_count(&self) -> usize {
        self.plain.graphemes(true).count()
    }

    /// Returns true if the text is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.plain.is_empty()
    }

    /// Appends text with an optional style.
    pub fn append(&mut self, s: &str, style: Option<Style>) {
        let start = self.char_count();
        self.plain.push_str(s);

        if let Some(st) = style {
            if !st.is_empty() {
                let end = self.char_count();
                self.spans.push(Span::new(start, end, st));
            }
        }
    }

    /// Appends styled text.
    #[inline]
    pub fn append_styled(&mut self, s: &str, style: Style) {
        self.append(s, Some(style));
    }

    /// Appends plain text without styling.
    #[inline]
    pub fn append_plain(&mut self, s: &str) {
        self.append(s, None);
    }

    /// Appends another Text object.
    pub fn append_text(&mut self, other: &Text) {
        let offset = self.char_count();
        self.plain.push_str(&other.plain);

        for span in &other.spans {
            self.spans.push(span.clone().offset(offset));
        }
    }

    /// Applies a style to a range of the text.
    ///
    /// # Arguments
    ///
    /// * `start` - Start character index
    /// * `end` - End character index (exclusive)
    /// * `style` - Style to apply
    pub fn stylize(&mut self, start: usize, end: usize, style: Style) {
        if !style.is_empty() && start < end {
            self.spans.push(Span::new(start, end, style));
        }
    }

    /// Applies a style to the entire text.
    pub fn stylize_all(&mut self, style: Style) {
        if !style.is_empty() {
            let len = self.char_count();
            self.spans.push(Span::new(0, len, style));
        }
    }

    /// Highlights occurrences of words with a style.
    ///
    /// # Arguments
    ///
    /// * `words` - Words to highlight
    /// * `style` - Style to apply to matching words
    /// * `case_sensitive` - Whether matching is case-sensitive
    pub fn highlight_words(&mut self, words: &[&str], style: Style, case_sensitive: bool) {
        let plain_text = self.plain.clone();
        let plain_lower = plain_text.to_lowercase();
        let search_text = if case_sensitive {
            &plain_text
        } else {
            &plain_lower
        };

        let mut spans_to_add = Vec::new();

        for word in words {
            let search_word = if case_sensitive {
                (*word).to_owned()
            } else {
                word.to_lowercase()
            };

            let mut start = 0;
            while let Some(pos) = search_text.get(start..).and_then(|s| s.find(&search_word)) {
                let abs_pos = start.saturating_add(pos);
                // Convert byte position to character position
                let char_start = search_text
                    .get(..abs_pos)
                    .map(|s| s.graphemes(true).count())
                    .unwrap_or(0);
                let char_end = char_start.saturating_add(word.graphemes(true).count());

                spans_to_add.push((char_start, char_end, style.clone()));
                start = abs_pos.saturating_add(search_word.len());
            }
        }

        for (char_start, char_end, st) in spans_to_add {
            self.stylize(char_start, char_end, st);
        }
    }

    /// Highlights matches of a regex pattern with a style.
    ///
    /// # Arguments
    ///
    /// * `pattern` - Regex pattern to match
    /// * `style` - Style to apply to matches
    ///
    /// # Errors
    ///
    /// Returns an error if the regex pattern is invalid.
    pub fn highlight_regex(&mut self, pattern: &str, style: Style) -> Result<()> {
        let re = regex::Regex::new(pattern)?;
        let plain_text = self.plain.clone();

        let mut spans_to_add = Vec::new();
        for mat in re.find_iter(&plain_text) {
            // Convert byte positions to character positions
            let char_start = plain_text
                .get(..mat.start())
                .map(|s| s.graphemes(true).count())
                .unwrap_or(0);
            let char_end = plain_text
                .get(..mat.end())
                .map(|s| s.graphemes(true).count())
                .unwrap_or(0);

            spans_to_add.push((char_start, char_end, style.clone()));
        }

        for (char_start, char_end, st) in spans_to_add {
            self.stylize(char_start, char_end, st);
        }

        Ok(())
    }

    /// Returns the style at a given character position.
    ///
    /// If multiple styles overlap, they are combined.
    #[must_use]
    pub fn style_at(&self, position: usize) -> Style {
        let mut result = Style::new();

        for span in &self.spans {
            if span.overlaps(position, position.saturating_add(1)) {
                result = result.combine(&span.style);
            }
        }

        result
    }

    /// Converts the text to segments for rendering.
    #[must_use]
    pub fn to_segments(&self) -> Segments {
        let mut segments = Segments::new();

        if self.plain.is_empty() {
            return segments;
        }

        // Build a list of style change points
        let mut events: Vec<(usize, bool, usize)> = Vec::new(); // (position, is_start, span_index)

        for (idx, span) in self.spans.iter().enumerate() {
            events.push((span.start, true, idx));
            events.push((span.end, false, idx));
        }

        // Sort events by position
        events.sort_by_key(|e| e.0);

        // Track active spans
        let mut active_spans: Vec<usize> = Vec::new();
        let mut last_pos = 0;

        let graphemes: Vec<&str> = self.plain.graphemes(true).collect();

        for (pos, is_start, span_idx) in events {
            // Emit segment for text before this event
            if pos > last_pos {
                let text: String = graphemes
                    .get(last_pos..pos)
                    .map(|g| g.iter().copied().collect())
                    .unwrap_or_default();

                if !text.is_empty() {
                    let style = self.combine_active_styles(&active_spans);
                    if style.is_empty() {
                        segments.push(Segment::new(text));
                    } else {
                        segments.push(Segment::styled(text, style));
                    }
                }
            }

            // Update active spans
            if is_start {
                active_spans.push(span_idx);
            } else {
                active_spans.retain(|&idx| idx != span_idx);
            }

            last_pos = pos;
        }

        // Emit remaining text
        if last_pos < graphemes.len() {
            let text: String = graphemes
                .get(last_pos..)
                .map(|g| g.iter().copied().collect())
                .unwrap_or_default();

            if !text.is_empty() {
                let style = self.combine_active_styles(&active_spans);
                if style.is_empty() {
                    segments.push(Segment::new(text));
                } else {
                    segments.push(Segment::styled(text, style));
                }
            }
        }

        segments
    }

    /// Combines styles from active spans.
    fn combine_active_styles(&self, active_spans: &[usize]) -> Style {
        let mut style = Style::new();
        for &idx in active_spans {
            if let Some(span) = self.spans.get(idx) {
                style = style.combine(&span.style);
            }
        }
        style
    }

    /// Splits the text into lines.
    #[must_use]
    pub fn split_lines(&self) -> Vec<Self> {
        let lines: Vec<&str> = self.plain.lines().collect();
        let mut result = Vec::with_capacity(lines.len());

        let mut char_offset: usize = 0;
        for (idx, line) in lines.iter().enumerate() {
            let line_len = line.graphemes(true).count();
            let mut text = Self::from_str(*line);
            text.justify = self.justify;
            text.overflow = self.overflow;
            text.no_wrap = self.no_wrap;
            text.tab_size = self.tab_size;

            // Copy relevant spans
            let line_end = char_offset.saturating_add(line_len);
            for span in &self.spans {
                if span.overlaps(char_offset, line_end) {
                    let new_start = if span.start > char_offset {
                        span.start.saturating_sub(char_offset)
                    } else {
                        0
                    };
                    let new_end = if span.end < line_end {
                        span.end.saturating_sub(char_offset)
                    } else {
                        line_len
                    };
                    text.spans.push(Span::new(new_start, new_end, span.style.clone()));
                }
            }

            result.push(text);

            // Account for the newline character
            char_offset = line_end.saturating_add(1);

            // Handle case where there's a trailing newline
            if idx == lines.len().saturating_sub(1) && self.plain.ends_with('\n') {
                result.push(Self::new());
            }
        }

        if result.is_empty() {
            result.push(Self::new());
        }

        result
    }

    /// Truncates the text to the given width.
    #[must_use]
    pub fn truncate(&self, width: usize, suffix: Option<&str>) -> Self {
        let cell_width = unicode_width::UnicodeWidthStr::width(self.plain.as_str());
        if cell_width <= width {
            return self.clone();
        }

        let suffix_width = suffix
            .map(unicode_width::UnicodeWidthStr::width)
            .unwrap_or(0);
        let target_width = width.saturating_sub(suffix_width);

        let mut result_text = String::new();
        let mut current_width: usize = 0;
        let mut char_count: usize = 0;

        for grapheme in self.plain.graphemes(true) {
            let grapheme_width = unicode_width::UnicodeWidthStr::width(grapheme);
            if current_width.saturating_add(grapheme_width) > target_width {
                break;
            }
            result_text.push_str(grapheme);
            current_width = current_width.saturating_add(grapheme_width);
            char_count = char_count.saturating_add(1);
        }

        if let Some(suf) = suffix {
            result_text.push_str(suf);
        }

        let mut result = Self::from_str(result_text);
        result.justify = self.justify;
        result.overflow = self.overflow;
        result.no_wrap = self.no_wrap;
        result.tab_size = self.tab_size;

        // Copy relevant spans
        for span in &self.spans {
            if span.start < char_count {
                let new_end = if span.end > char_count {
                    char_count
                } else {
                    span.end
                };
                result.spans.push(Span::new(span.start, new_end, span.style.clone()));
            }
        }

        result
    }

    /// Pads the text to the given width.
    #[must_use]
    pub fn pad(&self, width: usize, justify: Justify) -> Self {
        let cell_width = unicode_width::UnicodeWidthStr::width(self.plain.as_str());
        if cell_width >= width {
            return self.clone();
        }

        let padding = width.saturating_sub(cell_width);

        let (left_pad, right_pad) = match justify {
            Justify::Left | Justify::Default => (0, padding),
            Justify::Right => (padding, 0),
            Justify::Center => {
                let half = padding.checked_div(2).unwrap_or(0);
                (half, padding.saturating_sub(half))
            }
            Justify::Full => (0, padding), // Full justification needs more complex logic
        };

        let mut result = Self::new();
        if left_pad > 0 {
            result.plain.push_str(&" ".repeat(left_pad));
        }
        result.append_text(self);
        if right_pad > 0 {
            result.plain.push_str(&" ".repeat(right_pad));
        }

        // Adjust spans for left padding
        if left_pad > 0 {
            for span in &mut result.spans {
                span.start = span.start.saturating_add(left_pad);
                span.end = span.end.saturating_add(left_pad);
            }
        }

        result
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.plain)
    }
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.plain == other.plain
    }
}

impl Eq for Text {}

impl From<&str> for Text {
    #[inline]
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<String> for Text {
    #[inline]
    fn from(s: String) -> Self {
        Self::from_str(s)
    }
}

impl Measurable for Text {
    fn measure(&self, options: &MeasureOptions) -> Result<Measurement> {
        let min = crate::measure::min_width(&self.plain);
        let max = crate::measure::max_line_width(&self.plain);
        Ok(Measurement::new(min, max).clamp_max(options.max_width))
    }
}

impl std::ops::Add for Text {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.append_text(&other);
        self
    }
}

impl std::ops::Add<&str> for Text {
    type Output = Self;

    fn add(mut self, s: &str) -> Self {
        self.append_plain(s);
        self
    }
}

impl std::ops::AddAssign<&str> for Text {
    fn add_assign(&mut self, s: &str) {
        self.append_plain(s);
    }
}

impl std::ops::AddAssign<Text> for Text {
    fn add_assign(&mut self, other: Self) {
        self.append_text(&other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Color, StandardColor};

    #[test]
    fn test_text_new() {
        let text = Text::new();
        assert!(text.is_empty());
        assert_eq!(text.char_count(), 0);
    }

    #[test]
    fn test_text_from_str() {
        let text = Text::from_str("hello");
        assert_eq!(text.plain(), "hello");
        assert_eq!(text.char_count(), 5);
    }

    #[test]
    fn test_text_styled() {
        let style = Style::new().bold();
        let text = Text::styled("hello", style);
        assert_eq!(text.plain(), "hello");
        assert_eq!(text.spans.len(), 1);
    }

    #[test]
    fn test_text_append() {
        let mut text = Text::new();
        text.append_plain("hello ");
        text.append_styled("world", Style::new().bold());
        assert_eq!(text.plain(), "hello world");
        assert_eq!(text.spans.len(), 1);
        assert_eq!(text.spans.first().map(|s| s.start), Some(6));
        assert_eq!(text.spans.first().map(|s| s.end), Some(11));
    }

    #[test]
    fn test_text_stylize() {
        let mut text = Text::from_str("hello world");
        text.stylize(0, 5, Style::new().bold());
        assert_eq!(text.spans.len(), 1);
        assert_eq!(text.spans.first().map(|s| s.start), Some(0));
        assert_eq!(text.spans.first().map(|s| s.end), Some(5));
    }

    #[test]
    fn test_text_style_at() {
        let mut text = Text::from_str("hello world");
        text.stylize(0, 5, Style::new().bold());
        text.stylize(6, 11, Style::new().italic());

        let style = text.style_at(2);
        assert_eq!(style.attributes.bold, Some(true));

        let style = text.style_at(8);
        assert_eq!(style.attributes.italic, Some(true));

        let style = text.style_at(5);
        assert!(style.is_empty());
    }

    #[test]
    fn test_text_to_segments() {
        let mut text = Text::from_str("hello world");
        text.stylize(0, 5, Style::new().bold());

        let segments = text.to_segments();
        assert!(segments.len() >= 2);
    }

    #[test]
    fn test_text_assemble() {
        let text = Text::assemble([
            ("hello ", None),
            ("world", Some(Style::new().bold())),
        ]);
        assert_eq!(text.plain(), "hello world");
        assert_eq!(text.spans.len(), 1);
    }

    #[test]
    fn test_text_split_lines() {
        let text = Text::from_str("hello\nworld");
        let lines = text.split_lines();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines.first().map(|t| t.plain()), Some("hello"));
        assert_eq!(lines.get(1).map(|t| t.plain()), Some("world"));
    }

    #[test]
    fn test_text_truncate() {
        let text = Text::from_str("hello world");
        let truncated = text.truncate(8, Some("..."));
        assert_eq!(truncated.plain(), "hello...");
    }

    #[test]
    fn test_text_truncate_no_suffix() {
        let text = Text::from_str("hello world");
        let truncated = text.truncate(5, None);
        assert_eq!(truncated.plain(), "hello");
    }

    #[test]
    fn test_text_pad_left() {
        let text = Text::from_str("hi");
        let padded = text.pad(5, Justify::Left);
        assert_eq!(padded.plain(), "hi   ");
    }

    #[test]
    fn test_text_pad_right() {
        let text = Text::from_str("hi");
        let padded = text.pad(5, Justify::Right);
        assert_eq!(padded.plain(), "   hi");
    }

    #[test]
    fn test_text_pad_center() {
        let text = Text::from_str("hi");
        let padded = text.pad(6, Justify::Center);
        assert_eq!(padded.plain(), "  hi  ");
    }

    #[test]
    fn test_text_highlight_words() {
        let mut text = Text::from_str("hello world hello");
        let style = Style::new().with_color(Color::Standard(StandardColor::Red));
        text.highlight_words(&["hello"], style, false);

        // Should have 2 spans for "hello"
        assert_eq!(text.spans.len(), 2);
    }

    #[test]
    fn test_text_highlight_regex() {
        let mut text = Text::from_str("foo123bar456");
        let style = Style::new().bold();
        text.highlight_regex(r"\d+", style).ok();

        // Should have 2 spans for numbers
        assert_eq!(text.spans.len(), 2);
    }

    #[test]
    fn test_span_overlaps() {
        let span = Span::new(5, 10, Style::new());
        assert!(span.overlaps(0, 6));
        assert!(span.overlaps(9, 15));
        assert!(span.overlaps(6, 9));
        assert!(!span.overlaps(0, 5));
        assert!(!span.overlaps(10, 15));
    }

    #[test]
    fn test_text_add() {
        let t1 = Text::from_str("hello ");
        let t2 = Text::from_str("world");
        let combined = t1 + t2;
        assert_eq!(combined.plain(), "hello world");
    }

    #[test]
    fn test_text_add_str() {
        let t1 = Text::from_str("hello");
        let combined = t1 + " world";
        assert_eq!(combined.plain(), "hello world");
    }

    #[test]
    fn test_text_add_assign() {
        let mut text = Text::from_str("hello");
        text += " world";
        assert_eq!(text.plain(), "hello world");
    }

    #[test]
    fn test_text_from() {
        let text: Text = "hello".into();
        assert_eq!(text.plain(), "hello");

        let text: Text = "world".to_owned().into();
        assert_eq!(text.plain(), "world");
    }

    #[test]
    fn test_text_display() {
        let text = Text::from_str("hello");
        assert_eq!(format!("{text}"), "hello");
    }

    #[test]
    fn test_text_eq() {
        let t1 = Text::from_str("hello");
        let t2 = Text::from_str("hello");
        let t3 = Text::from_str("world");
        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
    }

    #[test]
    fn test_measurable() {
        let text = Text::from_str("hello world");
        let opts = MeasureOptions::new(80);
        let measurement = text.measure(&opts).ok().unwrap_or_default();
        assert_eq!(measurement.minimum, 5); // "hello" or "world"
        assert_eq!(measurement.maximum, 11);
    }
}
