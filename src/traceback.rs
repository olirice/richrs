//! Traceback formatting for enhanced error display.
//!
//! This module provides beautiful formatting for backtraces and error chains,
//! similar to Python Rich's traceback feature.
//!
//! # Example
//!
//! ```ignore
//! use richrs::traceback::Traceback;
//! use std::backtrace::Backtrace;
//!
//! let bt = Backtrace::capture();
//! let traceback = Traceback::from_backtrace(&bt);
//! let segments = traceback.render(80);
//! ```

use crate::box_chars::BoxChars;
use crate::segment::{Segment, Segments};
use crate::style::Style;

/// A formatted traceback display.
///
/// Provides rich formatting for backtraces with syntax highlighting,
/// context lines, and clean visual presentation.
#[derive(Debug, Clone)]
pub struct Traceback {
    /// The formatted frames of the traceback.
    frames: Vec<Frame>,
    /// Title to display at the top.
    title: String,
    /// Whether to show local variables (if available).
    show_locals: bool,
    /// Number of context lines to show around the error.
    extra_lines: usize,
    /// Whether to suppress internal frames (e.g., from std library).
    suppress_internals: bool,
    /// Maximum number of frames to show.
    max_frames: Option<usize>,
    /// Width for rendering.
    width: usize,
    /// Box style for borders.
    box_chars: BoxChars,
}

/// A single frame in a traceback.
#[derive(Debug, Clone)]
pub struct Frame {
    /// The function or method name.
    pub name: String,
    /// The file path.
    pub file: Option<String>,
    /// The line number.
    pub line: Option<u32>,
    /// The column number.
    pub column: Option<u32>,
    /// Whether this is an internal frame (std library, etc.).
    pub is_internal: bool,
}

impl Frame {
    /// Creates a new frame with the given name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            file: None,
            line: None,
            column: None,
            is_internal: false,
        }
    }

    /// Sets the file path for this frame.
    #[must_use]
    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    /// Sets the line number for this frame.
    #[must_use]
    pub const fn line(mut self, line: u32) -> Self {
        self.line = Some(line);
        self
    }

    /// Sets the column number for this frame.
    #[must_use]
    pub const fn column(mut self, column: u32) -> Self {
        self.column = Some(column);
        self
    }

    /// Marks this frame as internal.
    #[must_use]
    pub const fn internal(mut self, is_internal: bool) -> Self {
        self.is_internal = is_internal;
        self
    }

    /// Checks if this frame appears to be from the standard library or internal code.
    fn detect_internal(&self) -> bool {
        if let Some(ref file) = self.file {
            // Common patterns for internal frames
            file.contains("/rustc/")
                || file.contains("/.cargo/")
                || file.contains("/library/")
                || file.starts_with("<")
        } else {
            // Frames without file info are often internal
            self.name.starts_with("std::")
                || self.name.starts_with("core::")
                || self.name.starts_with("alloc::")
                || self.name.contains("__rust_")
                || self.name.contains("lang_start")
        }
    }
}

impl Default for Traceback {
    fn default() -> Self {
        Self::new()
    }
}

impl Traceback {
    /// Creates a new empty Traceback.
    #[must_use]
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            title: "Traceback (most recent call last)".to_string(),
            show_locals: false,
            extra_lines: 3,
            suppress_internals: true,
            max_frames: Some(100),
            width: 88,
            box_chars: BoxChars::ROUNDED,
        }
    }

    /// Creates a Traceback from a backtrace string.
    ///
    /// Parses the standard Rust backtrace format.
    #[must_use]
    pub fn from_backtrace_string(bt: &str) -> Self {
        let mut traceback = Self::new();
        traceback.frames = parse_backtrace(bt);
        traceback
    }

    /// Adds a frame to the traceback.
    pub fn add_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    /// Sets the title for the traceback.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets whether to show local variables.
    #[must_use]
    #[inline]
    pub const fn show_locals(mut self, show: bool) -> Self {
        self.show_locals = show;
        self
    }

    /// Sets the number of extra context lines to show.
    #[must_use]
    #[inline]
    pub const fn extra_lines(mut self, lines: usize) -> Self {
        self.extra_lines = lines;
        self
    }

    /// Sets whether to suppress internal frames.
    #[must_use]
    #[inline]
    pub const fn suppress_internals(mut self, suppress: bool) -> Self {
        self.suppress_internals = suppress;
        self
    }

    /// Sets the maximum number of frames to display.
    #[must_use]
    #[inline]
    pub const fn max_frames(mut self, max: usize) -> Self {
        self.max_frames = Some(max);
        self
    }

    /// Sets the width for rendering.
    #[must_use]
    #[inline]
    pub const fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Sets the box style for borders.
    #[must_use]
    #[inline]
    pub const fn box_chars(mut self, box_chars: BoxChars) -> Self {
        self.box_chars = box_chars;
        self
    }

    /// Returns the frames in this traceback.
    #[must_use]
    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    /// Renders the traceback to segments.
    #[must_use]
    pub fn render(&self, max_width: usize) -> Segments {
        let mut segments = Segments::new();
        let width = max_width.min(self.width);

        // Styles
        let title_style = Style::default().bold();
        let file_style = Style::default().dim();
        let function_style = Style::default().bold();
        let line_num_style = Style::default().dim();
        let border_style = Style::default().dim();

        // Top border with title
        let title_len = self.title.len();
        let padding = width.saturating_sub(title_len).saturating_sub(4);
        let left_pad = padding / 2;
        let right_pad = padding.saturating_sub(left_pad);

        segments.push(Segment::styled(
            self.box_chars.top_left.to_string(),
            border_style.clone(),
        ));
        segments.push(Segment::styled(
            self.box_chars.horizontal.to_string().repeat(left_pad),
            border_style.clone(),
        ));
        segments.push(Segment::new(" "));
        segments.push(Segment::styled(self.title.clone(), title_style));
        segments.push(Segment::new(" "));
        segments.push(Segment::styled(
            self.box_chars.horizontal.to_string().repeat(right_pad),
            border_style.clone(),
        ));
        segments.push(Segment::styled(
            self.box_chars.top_right.to_string(),
            border_style.clone(),
        ));
        segments.push(Segment::newline());

        // Filter frames
        let frames: Vec<&Frame> = self
            .frames
            .iter()
            .filter(|f| {
                if self.suppress_internals {
                    !f.is_internal && !f.detect_internal()
                } else {
                    true
                }
            })
            .take(self.max_frames.unwrap_or(usize::MAX))
            .collect();

        // Render each frame
        for (i, frame) in frames.iter().enumerate() {
            // Left border
            segments.push(Segment::styled(
                self.box_chars.vertical.to_string(),
                border_style.clone(),
            ));
            segments.push(Segment::new(" "));

            // Frame number
            segments.push(Segment::styled(
                format!("{:>3}. ", i),
                line_num_style.clone(),
            ));

            // Function name
            segments.push(Segment::styled(
                frame.name.clone(),
                function_style.clone(),
            ));
            segments.push(Segment::newline());

            // File and line info
            if let Some(ref file) = frame.file {
                segments.push(Segment::styled(
                    self.box_chars.vertical.to_string(),
                    border_style.clone(),
                ));
                segments.push(Segment::new("      "));

                let location = match (frame.line, frame.column) {
                    (Some(line), Some(col)) => format!("{}:{}:{}", file, line, col),
                    (Some(line), None) => format!("{}:{}", file, line),
                    _ => file.clone(),
                };

                segments.push(Segment::styled(format!("at {}", location), file_style.clone()));
                segments.push(Segment::newline());
            }

            // Separator between frames (except for last)
            if i < frames.len().saturating_sub(1) {
                segments.push(Segment::styled(
                    self.box_chars.vertical.to_string(),
                    border_style.clone(),
                ));
                segments.push(Segment::newline());
            }
        }

        // Show count of suppressed frames if any were filtered
        let total_frames = self.frames.len();
        let shown_frames = frames.len();
        if self.suppress_internals && shown_frames < total_frames {
            let suppressed = total_frames.saturating_sub(shown_frames);
            segments.push(Segment::styled(
                self.box_chars.vertical.to_string(),
                border_style.clone(),
            ));
            segments.push(Segment::new(" "));
            segments.push(Segment::styled(
                format!("... {} internal frames hidden", suppressed),
                Style::default().dim().italic(),
            ));
            segments.push(Segment::newline());
        }

        // Bottom border
        segments.push(Segment::styled(
            self.box_chars.bottom_left.to_string(),
            border_style.clone(),
        ));
        segments.push(Segment::styled(
            self.box_chars.horizontal.to_string().repeat(width.saturating_sub(2)),
            border_style.clone(),
        ));
        segments.push(Segment::styled(
            self.box_chars.bottom_right.to_string(),
            border_style,
        ));
        segments.push(Segment::newline());

        segments
    }
}

/// Parses a Rust backtrace string into frames.
fn parse_backtrace(bt: &str) -> Vec<Frame> {
    let mut frames = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_file: Option<String> = None;
    let mut current_line: Option<u32> = None;

    for line in bt.lines() {
        let trimmed = line.trim();

        // Match frame number and function name: "   0: function_name"
        if let Some(rest) = trimmed.strip_prefix(|c: char| c.is_ascii_digit()) {
            // Find the colon after the number
            if let Some(colon_pos) = rest.find(':') {
                // Save previous frame
                if let Some(name) = current_name.take() {
                    let mut frame = Frame::new(name);
                    if let Some(file) = current_file.take() {
                        frame = frame.file(file);
                    }
                    if let Some(line) = current_line.take() {
                        frame = frame.line(line);
                    }
                    frames.push(frame);
                }

                // Extract function name
                let func_name = rest[colon_pos + 1..].trim();
                current_name = Some(func_name.to_string());
            }
        }
        // Match file location: "             at /path/to/file.rs:123:45"
        else if let Some(at_part) = trimmed.strip_prefix("at ") {
            // Parse file:line:column
            let parts: Vec<&str> = at_part.rsplitn(3, ':').collect();
            match parts.len() {
                3 => {
                    // file:line:column
                    current_file = Some(parts[2].to_string());
                    current_line = parts[1].parse().ok();
                }
                2 => {
                    // file:line
                    current_file = Some(parts[1].to_string());
                    current_line = parts[0].parse().ok();
                }
                1 => {
                    current_file = Some(at_part.to_string());
                }
                _ => {}
            }
        }
    }

    // Don't forget the last frame
    if let Some(name) = current_name {
        let mut frame = Frame::new(name);
        if let Some(file) = current_file {
            frame = frame.file(file);
        }
        if let Some(line) = current_line {
            frame = frame.line(line);
        }
        frames.push(frame);
    }

    frames
}

/// Formats an error with its source chain.
///
/// Creates a traceback-like display for error chains.
#[must_use]
pub fn format_error_chain<E: std::error::Error>(error: &E) -> Segments {
    let mut segments = Segments::new();
    let error_style = Style::default().bold();
    let cause_style = Style::default().dim();

    // Main error
    segments.push(Segment::styled("Error: ", error_style.clone()));
    segments.push(Segment::new(error.to_string()));
    segments.push(Segment::newline());

    // Cause chain
    let mut source = error.source();
    let mut depth = 0;
    while let Some(cause) = source {
        depth += 1;
        segments.push(Segment::styled(
            format!("  Caused by ({}): ", depth),
            cause_style.clone(),
        ));
        segments.push(Segment::new(cause.to_string()));
        segments.push(Segment::newline());
        source = cause.source();
    }

    segments
}

/// Convenience function to print a traceback from a backtrace string.
pub fn print_traceback(bt: &str) {
    let traceback = Traceback::from_backtrace_string(bt);
    let segments = traceback.render(80);
    eprint!("{}", segments.to_ansi());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traceback_new() {
        let tb = Traceback::new();
        assert!(tb.frames.is_empty());
        assert!(tb.title.contains("Traceback"));
    }

    #[test]
    fn test_traceback_title() {
        let tb = Traceback::new().title("Custom Error");
        assert_eq!(tb.title, "Custom Error");
    }

    #[test]
    fn test_traceback_show_locals() {
        let tb = Traceback::new().show_locals(true);
        assert!(tb.show_locals);
    }

    #[test]
    fn test_traceback_extra_lines() {
        let tb = Traceback::new().extra_lines(5);
        assert_eq!(tb.extra_lines, 5);
    }

    #[test]
    fn test_traceback_suppress_internals() {
        let tb = Traceback::new().suppress_internals(false);
        assert!(!tb.suppress_internals);
    }

    #[test]
    fn test_traceback_max_frames() {
        let tb = Traceback::new().max_frames(10);
        assert_eq!(tb.max_frames, Some(10));
    }

    #[test]
    fn test_traceback_width() {
        let tb = Traceback::new().width(100);
        assert_eq!(tb.width, 100);
    }

    #[test]
    fn test_frame_new() {
        let frame = Frame::new("my_function");
        assert_eq!(frame.name, "my_function");
        assert!(frame.file.is_none());
        assert!(frame.line.is_none());
    }

    #[test]
    fn test_frame_file() {
        let frame = Frame::new("test").file("/path/to/file.rs");
        assert_eq!(frame.file, Some("/path/to/file.rs".to_string()));
    }

    #[test]
    fn test_frame_line() {
        let frame = Frame::new("test").line(42);
        assert_eq!(frame.line, Some(42));
    }

    #[test]
    fn test_frame_column() {
        let frame = Frame::new("test").column(10);
        assert_eq!(frame.column, Some(10));
    }

    #[test]
    fn test_frame_internal() {
        let frame = Frame::new("test").internal(true);
        assert!(frame.is_internal);
    }

    #[test]
    fn test_frame_detect_internal() {
        let internal_frame = Frame::new("std::panicking::begin_panic");
        assert!(internal_frame.detect_internal());

        let user_frame = Frame::new("my_app::main");
        assert!(!user_frame.detect_internal());
    }

    #[test]
    fn test_traceback_add_frame() {
        let mut tb = Traceback::new();
        tb.add_frame(Frame::new("function_a").file("src/main.rs").line(10));
        tb.add_frame(Frame::new("function_b").file("src/lib.rs").line(20));
        assert_eq!(tb.frames.len(), 2);
    }

    #[test]
    fn test_traceback_render() {
        let mut tb = Traceback::new();
        tb.add_frame(Frame::new("my_function").file("src/main.rs").line(42));
        let segments = tb.render(80);
        let output = segments.to_ansi();
        assert!(output.contains("my_function"));
        assert!(output.contains("src/main.rs"));
        assert!(output.contains("42"));
    }

    #[test]
    fn test_traceback_render_suppressed() {
        let mut tb = Traceback::new().suppress_internals(true);
        tb.add_frame(Frame::new("user::main"));
        tb.add_frame(Frame::new("std::rt::lang_start"));
        let segments = tb.render(80);
        let output = segments.to_ansi();
        assert!(output.contains("user::main"));
        assert!(output.contains("internal frames hidden"));
    }

    #[test]
    fn test_parse_backtrace() {
        let bt = r"
   0: my_crate::my_function
             at ./src/lib.rs:42:5
   1: my_crate::main
             at ./src/main.rs:10:1
";
        let frames = parse_backtrace(bt);
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].name, "my_crate::my_function");
        assert_eq!(frames[1].name, "my_crate::main");
    }

    #[test]
    fn test_format_error_chain() {
        use std::io;
        let error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let segments = format_error_chain(&error);
        let output = segments.to_ansi();
        assert!(output.contains("Error:"));
        assert!(output.contains("file not found"));
    }
}
