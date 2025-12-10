//! Segment is the fundamental unit of console output.
//!
//! A segment represents a piece of text with optional styling and control codes.
//! Segments are the building blocks that renderables produce and the console consumes.

use crate::style::Style;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The type of control code a segment represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ControlType {
    /// Move cursor to home position.
    Home,
    /// Carriage return (move to start of line).
    CarriageReturn,
    /// Clear screen.
    Clear,
    /// Show cursor.
    ShowCursor,
    /// Hide cursor.
    HideCursor,
    /// Enable alternate screen buffer.
    EnableAlternateScreen,
    /// Disable alternate screen buffer.
    DisableAlternateScreen,
    /// Ring the terminal bell.
    Bell,
    /// Set window title.
    SetWindowTitle,
    /// Move cursor up N lines.
    CursorUp,
    /// Move cursor down N lines.
    CursorDown,
    /// Move cursor forward N columns.
    CursorForward,
    /// Move cursor backward N columns.
    CursorBackward,
    /// Move cursor to specific position.
    CursorMoveTo,
    /// Erase from cursor to end of line.
    EraseEndOfLine,
    /// Erase entire line.
    EraseLine,
}

/// A control segment for cursor/screen manipulation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Control {
    /// The type of control code.
    pub control_type: ControlType,
    /// Optional parameters for the control code.
    pub parameters: Vec<i32>,
}

impl Control {
    /// Creates a new control code.
    #[inline]
    #[must_use]
    pub const fn new(control_type: ControlType) -> Self {
        Self {
            control_type,
            parameters: Vec::new(),
        }
    }

    /// Creates a control code with parameters.
    #[inline]
    #[must_use]
    pub fn with_params(control_type: ControlType, parameters: Vec<i32>) -> Self {
        Self {
            control_type,
            parameters,
        }
    }

    /// Returns the ANSI escape sequence for this control code.
    #[must_use]
    pub fn to_ansi(&self) -> String {
        match self.control_type {
            ControlType::Home => "\x1b[H".to_owned(),
            ControlType::CarriageReturn => "\r".to_owned(),
            ControlType::Clear => "\x1b[2J".to_owned(),
            ControlType::ShowCursor => "\x1b[?25h".to_owned(),
            ControlType::HideCursor => "\x1b[?25l".to_owned(),
            ControlType::EnableAlternateScreen => "\x1b[?1049h".to_owned(),
            ControlType::DisableAlternateScreen => "\x1b[?1049l".to_owned(),
            ControlType::Bell => "\x07".to_owned(),
            ControlType::SetWindowTitle => {
                // Parameters would contain title as string, not supported here
                String::new()
            }
            ControlType::CursorUp => {
                let n = self.parameters.first().copied().unwrap_or(1);
                format!("\x1b[{n}A")
            }
            ControlType::CursorDown => {
                let n = self.parameters.first().copied().unwrap_or(1);
                format!("\x1b[{n}B")
            }
            ControlType::CursorForward => {
                let n = self.parameters.first().copied().unwrap_or(1);
                format!("\x1b[{n}C")
            }
            ControlType::CursorBackward => {
                let n = self.parameters.first().copied().unwrap_or(1);
                format!("\x1b[{n}D")
            }
            ControlType::CursorMoveTo => {
                let row = self.parameters.first().copied().unwrap_or(1);
                let col = self.parameters.get(1).copied().unwrap_or(1);
                format!("\x1b[{row};{col}H")
            }
            ControlType::EraseEndOfLine => "\x1b[K".to_owned(),
            ControlType::EraseLine => "\x1b[2K".to_owned(),
        }
    }
}

/// A segment of renderable output.
///
/// Segments are the fundamental unit that renderables produce.
/// They consist of text with optional styling, or control codes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Segment {
    /// The text content of the segment.
    pub text: String,
    /// Optional style for the text.
    pub style: Option<Style>,
    /// Optional control code (if this is a control segment).
    pub control: Option<Control>,
}

impl Segment {
    /// Creates a new text segment.
    #[inline]
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: None,
            control: None,
        }
    }

    /// Creates a new styled text segment.
    #[inline]
    #[must_use]
    pub fn styled(text: impl Into<String>, style: Style) -> Self {
        Self {
            text: text.into(),
            style: Some(style),
            control: None,
        }
    }

    /// Creates a control segment.
    #[inline]
    #[must_use]
    pub fn control(control: Control) -> Self {
        Self {
            text: String::new(),
            style: None,
            control: Some(control),
        }
    }

    /// Creates a newline segment.
    #[inline]
    #[must_use]
    pub fn newline() -> Self {
        Self::new("\n")
    }

    /// Creates a line of text (text followed by newline).
    #[inline]
    #[must_use]
    pub fn line(text: impl Into<String>) -> Self {
        let mut s: String = text.into();
        s.push('\n');
        Self::new(s)
    }

    /// Returns true if this is a control segment.
    #[inline]
    #[must_use]
    pub const fn is_control(&self) -> bool {
        self.control.is_some()
    }

    /// Returns the cell length of this segment (visible character width).
    #[inline]
    #[must_use]
    pub fn cell_length(&self) -> usize {
        if self.is_control() {
            0
        } else {
            unicode_width::UnicodeWidthStr::width(self.text.as_str())
        }
    }

    /// Returns the ANSI-encoded string for this segment.
    #[must_use]
    pub fn to_ansi(&self) -> String {
        if let Some(ref ctrl) = self.control {
            return ctrl.to_ansi();
        }

        match &self.style {
            Some(style) if !style.is_empty() => {
                let mut result = style.to_ansi();
                result.push_str(&self.text);
                result.push_str(&style.to_ansi_reset());
                result
            }
            _ => self.text.clone(),
        }
    }

    /// Splits this segment at the given position.
    ///
    /// Returns a tuple of (left, right) segments.
    #[must_use]
    pub fn split_at(&self, position: usize) -> (Self, Self) {
        if self.is_control() {
            return (self.clone(), Self::new(""));
        }

        // Find the byte position corresponding to the cell position
        let mut cell_pos = 0;
        let mut byte_pos = 0;

        for (idx, ch) in self.text.char_indices() {
            if cell_pos >= position {
                byte_pos = idx;
                break;
            }
            cell_pos =
                cell_pos.saturating_add(unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0));
            byte_pos = idx.saturating_add(ch.len_utf8());
        }

        let (left, right) = self.text.split_at(byte_pos);
        (
            Self {
                text: left.to_owned(),
                style: self.style.clone(),
                control: None,
            },
            Self {
                text: right.to_owned(),
                style: self.style.clone(),
                control: None,
            },
        )
    }

    /// Truncates this segment to the given cell width.
    #[must_use]
    pub fn truncate(&self, width: usize) -> Self {
        let (left, _) = self.split_at(width);
        left
    }
}

impl Default for Segment {
    #[inline]
    fn default() -> Self {
        Self::new("")
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl From<&str> for Segment {
    #[inline]
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Segment {
    #[inline]
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

/// A collection of segments that make up a complete line or output.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Segments {
    /// The segments in this collection.
    segments: Vec<Segment>,
}

impl Segments {
    /// Creates an empty segments collection.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    /// Creates a segments collection from a vector of segments.
    #[inline]
    #[must_use]
    pub const fn from_vec(segments: Vec<Segment>) -> Self {
        Self { segments }
    }

    /// Returns true if there are no segments.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Returns the number of segments.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Adds a segment to the collection.
    #[inline]
    pub fn push(&mut self, segment: Segment) {
        self.segments.push(segment);
    }

    /// Returns an iterator over the segments.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Segment> {
        self.segments.iter()
    }

    /// Returns a mutable iterator over the segments.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Segment> {
        self.segments.iter_mut()
    }

    /// Returns the total cell width of all segments.
    #[must_use]
    pub fn cell_length(&self) -> usize {
        self.segments
            .iter()
            .map(Segment::cell_length)
            .fold(0, usize::saturating_add)
    }

    /// Returns the plain text content without styling.
    #[must_use]
    pub fn plain_text(&self) -> String {
        self.segments
            .iter()
            .filter(|s| !s.is_control())
            .map(|s| s.text.as_str())
            .collect()
    }

    /// Returns the ANSI-encoded string for all segments.
    #[must_use]
    pub fn to_ansi(&self) -> String {
        self.segments.iter().map(Segment::to_ansi).collect()
    }

    /// Splits the segments into lines.
    #[must_use]
    pub fn split_lines(&self) -> Vec<Self> {
        let mut lines = Vec::new();
        let mut current_line = Self::new();

        for segment in &self.segments {
            if segment.is_control() {
                current_line.push(segment.clone());
                continue;
            }

            let mut remaining = segment.text.as_str();
            while let Some(newline_pos) = remaining.find('\n') {
                let before = &remaining[..newline_pos];
                if !before.is_empty() {
                    current_line.push(Segment {
                        text: before.to_owned(),
                        style: segment.style.clone(),
                        control: None,
                    });
                }
                lines.push(current_line);
                current_line = Self::new();
                remaining = &remaining[newline_pos.saturating_add(1)..];
            }

            if !remaining.is_empty() {
                current_line.push(Segment {
                    text: remaining.to_owned(),
                    style: segment.style.clone(),
                    control: None,
                });
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }
}

impl IntoIterator for Segments {
    type Item = Segment;
    type IntoIter = std::vec::IntoIter<Segment>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.segments.into_iter()
    }
}

impl<'a> IntoIterator for &'a Segments {
    type Item = &'a Segment;
    type IntoIter = std::slice::Iter<'a, Segment>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.segments.iter()
    }
}

impl FromIterator<Segment> for Segments {
    fn from_iter<I: IntoIterator<Item = Segment>>(iter: I) -> Self {
        Self {
            segments: iter.into_iter().collect(),
        }
    }
}

impl Extend<Segment> for Segments {
    fn extend<I: IntoIterator<Item = Segment>>(&mut self, iter: I) {
        self.segments.extend(iter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Color, StandardColor};

    #[test]
    fn test_segment_new() {
        let seg = Segment::new("hello");
        assert_eq!(seg.text, "hello");
        assert!(seg.style.is_none());
        assert!(!seg.is_control());
    }

    #[test]
    fn test_segment_styled() {
        let style = Style::new().bold();
        let seg = Segment::styled("hello", style.clone());
        assert_eq!(seg.text, "hello");
        assert_eq!(seg.style, Some(style));
    }

    #[test]
    fn test_segment_control() {
        let ctrl = Control::new(ControlType::Clear);
        let seg = Segment::control(ctrl);
        assert!(seg.is_control());
        assert_eq!(seg.cell_length(), 0);
    }

    #[test]
    fn test_segment_cell_length() {
        let seg = Segment::new("hello");
        assert_eq!(seg.cell_length(), 5);

        // Wide characters
        let seg = Segment::new("日本語");
        assert_eq!(seg.cell_length(), 6);
    }

    #[test]
    fn test_segment_to_ansi() {
        let seg = Segment::new("hello");
        assert_eq!(seg.to_ansi(), "hello");

        let style = Style::new().with_color(Color::Standard(StandardColor::Red));
        let seg = Segment::styled("hello", style);
        let ansi = seg.to_ansi();
        assert!(ansi.contains("\x1b[31m")); // red
        assert!(ansi.contains("hello"));
        assert!(ansi.contains("\x1b[0m")); // full reset for terminal compatibility
    }

    #[test]
    fn test_segment_split_at() {
        let seg = Segment::new("hello world");
        let (left, right) = seg.split_at(5);
        assert_eq!(left.text, "hello");
        assert_eq!(right.text, " world");
    }

    #[test]
    fn test_segment_truncate() {
        let seg = Segment::new("hello world");
        let truncated = seg.truncate(5);
        assert_eq!(truncated.text, "hello");
    }

    #[test]
    fn test_segments_new() {
        let segs = Segments::new();
        assert!(segs.is_empty());
        assert_eq!(segs.len(), 0);
    }

    #[test]
    fn test_segments_push() {
        let mut segs = Segments::new();
        segs.push(Segment::new("hello"));
        segs.push(Segment::new(" world"));
        assert_eq!(segs.len(), 2);
        assert_eq!(segs.cell_length(), 11);
    }

    #[test]
    fn test_segments_plain_text() {
        let mut segs = Segments::new();
        segs.push(Segment::new("hello"));
        segs.push(Segment::new(" world"));
        assert_eq!(segs.plain_text(), "hello world");
    }

    #[test]
    fn test_segments_split_lines() {
        let mut segs = Segments::new();
        segs.push(Segment::new("hello\nworld"));
        let lines = segs.split_lines();
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines.first().map(Segments::plain_text),
            Some("hello".to_owned())
        );
        assert_eq!(
            lines.get(1).map(Segments::plain_text),
            Some("world".to_owned())
        );
    }

    #[test]
    fn test_control_to_ansi() {
        assert_eq!(Control::new(ControlType::Clear).to_ansi(), "\x1b[2J");
        assert_eq!(Control::new(ControlType::Home).to_ansi(), "\x1b[H");
        assert_eq!(Control::new(ControlType::CarriageReturn).to_ansi(), "\r");
        assert_eq!(
            Control::with_params(ControlType::CursorUp, vec![5]).to_ansi(),
            "\x1b[5A"
        );
        assert_eq!(
            Control::with_params(ControlType::CursorMoveTo, vec![10, 20]).to_ansi(),
            "\x1b[10;20H"
        );
    }

    #[test]
    fn test_segment_from_str() {
        let seg: Segment = "hello".into();
        assert_eq!(seg.text, "hello");
    }

    #[test]
    fn test_segment_from_string() {
        let seg: Segment = "hello".to_owned().into();
        assert_eq!(seg.text, "hello");
    }

    #[test]
    fn test_segments_iter() {
        let mut segs = Segments::new();
        segs.push(Segment::new("a"));
        segs.push(Segment::new("b"));

        let texts: Vec<_> = segs.iter().map(|s| s.text.as_str()).collect();
        assert_eq!(texts, vec!["a", "b"]);
    }

    #[test]
    fn test_segments_from_iterator() {
        let segs: Segments = vec![Segment::new("a"), Segment::new("b")]
            .into_iter()
            .collect();
        assert_eq!(segs.len(), 2);
    }

    #[test]
    fn test_segments_extend() {
        let mut segs = Segments::new();
        segs.push(Segment::new("a"));
        segs.extend(vec![Segment::new("b"), Segment::new("c")]);
        assert_eq!(segs.len(), 3);
    }
}
