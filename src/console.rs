//! Console is the main interface for terminal output.
//!
//! The Console provides methods for printing styled text, tables,
//! progress bars, and other rich content to the terminal.

use crate::errors::{Error, Result};
use crate::markup::Markup;
use crate::measure::{Measurable, MeasureOptions, Measurement};
use crate::segment::{Control, ControlType, Segment, Segments};
use crate::style::Style;
use crate::text::Text;
use crossterm::terminal;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{self, Write};

/// Color system capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ColorSystem {
    /// No color support.
    None,
    /// Standard 16 colors.
    Standard,
    /// 256 color palette.
    EightBit,
    /// 24-bit true color.
    #[default]
    TrueColor,
    /// Windows legacy console.
    Windows,
}

impl ColorSystem {
    /// Returns true if this color system supports colors.
    #[inline]
    #[must_use]
    pub const fn has_colors(&self) -> bool {
        !matches!(self, Self::None)
    }

    /// Returns true if this color system supports true color.
    #[inline]
    #[must_use]
    pub const fn is_true_color(&self) -> bool {
        matches!(self, Self::TrueColor)
    }
}

/// Console output recording mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RecordMode {
    /// Not recording.
    #[default]
    Off,
    /// Recording output.
    On,
}

/// Options for console output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConsoleOptions {
    /// Maximum width for output.
    pub max_width: usize,
    /// Minimum width for output.
    pub min_width: usize,
    /// Text justification.
    pub justify: Justify,
    /// Overflow behavior.
    pub overflow: Overflow,
    /// Whether to disable word wrapping.
    pub no_wrap: bool,
    /// Whether to highlight output.
    pub highlight: bool,
    /// Whether to use markup.
    pub markup: bool,
    /// Whether to enable emoji.
    pub emoji: bool,
}

impl Default for ConsoleOptions {
    fn default() -> Self {
        Self {
            max_width: 80,
            min_width: 1,
            justify: Justify::Default,
            overflow: Overflow::Fold,
            no_wrap: false,
            highlight: false,
            markup: true,
            emoji: true,
        }
    }
}

impl ConsoleOptions {
    /// Creates new console options with the given maximum width.
    #[inline]
    #[must_use]
    pub const fn new(max_width: usize) -> Self {
        Self {
            max_width,
            min_width: 1,
            justify: Justify::Default,
            overflow: Overflow::Fold,
            no_wrap: false,
            highlight: false,
            markup: true,
            emoji: true,
        }
    }

    /// Updates the maximum width.
    #[inline]
    #[must_use]
    pub const fn with_max_width(mut self, width: usize) -> Self {
        self.max_width = width;
        self
    }

    /// Updates the justification.
    #[inline]
    #[must_use]
    pub const fn with_justify(mut self, justify: Justify) -> Self {
        self.justify = justify;
        self
    }

    /// Updates the overflow behavior.
    #[inline]
    #[must_use]
    pub const fn with_overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }
}

/// Terminal size (width and height).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TerminalSize {
    /// Width in columns.
    pub width: usize,
    /// Height in rows.
    pub height: usize,
}

impl TerminalSize {
    /// Creates a new terminal size.
    #[inline]
    #[must_use]
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

/// A captured output buffer.
#[derive(Debug, Clone, Default)]
pub struct CapturedOutput {
    /// The captured text.
    pub text: String,
}

impl CapturedOutput {
    /// Creates an empty captured output.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            text: String::new(),
        }
    }
}

/// The main console interface for rich terminal output.
///
/// Console provides methods for printing styled text, handling
/// terminal capabilities, and managing output.
#[derive(Debug)]
pub struct Console {
    /// The output writer.
    writer: Box<dyn ConsoleWriter>,
    /// Console options.
    options: ConsoleOptions,
    /// Color system to use.
    color_system: ColorSystem,
    /// Whether this is a terminal.
    is_terminal: bool,
    /// Whether to force terminal mode.
    force_terminal: bool,
    /// Default style for output.
    style: Option<Style>,
    /// Recording buffer.
    record_buffer: Vec<Segment>,
    /// Recording mode.
    record_mode: RecordMode,
    /// Captured output (when capturing).
    captured: Option<String>,
    /// Whether currently capturing.
    is_capturing: bool,
    /// Soft wrap mode.
    #[allow(dead_code)]
    soft_wrap: bool,
    /// Terminal width (cached).
    width: Option<usize>,
    /// Terminal height (cached).
    height: Option<usize>,
}

/// Trait for console output writers.
pub trait ConsoleWriter: fmt::Debug + Send {
    /// Writes a string to the output.
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    fn write_str(&mut self, s: &str) -> Result<()>;

    /// Flushes the output.
    ///
    /// # Errors
    ///
    /// Returns an error if flushing fails.
    fn flush(&mut self) -> Result<()>;

    /// Returns true if this is a terminal.
    fn is_terminal(&self) -> bool;
}

/// Standard output writer.
#[derive(Debug)]
pub struct StdoutWriter;

impl ConsoleWriter for StdoutWriter {
    fn write_str(&mut self, s: &str) -> Result<()> {
        let mut stdout = io::stdout();
        stdout.write_all(s.as_bytes())?;
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        io::stdout().flush()?;
        Ok(())
    }

    fn is_terminal(&self) -> bool {
        atty_check()
    }
}

/// Standard error writer.
#[derive(Debug)]
pub struct StderrWriter;

impl ConsoleWriter for StderrWriter {
    fn write_str(&mut self, s: &str) -> Result<()> {
        let mut stderr = io::stderr();
        stderr.write_all(s.as_bytes())?;
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        io::stderr().flush()?;
        Ok(())
    }

    fn is_terminal(&self) -> bool {
        atty_check_stderr()
    }
}

/// String buffer writer for testing.
#[derive(Debug, Default)]
pub struct StringWriter {
    /// The buffer.
    buffer: String,
}

impl StringWriter {
    /// Creates a new string writer.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    /// Returns the buffer contents.
    #[inline]
    #[must_use]
    pub fn contents(&self) -> &str {
        &self.buffer
    }

    /// Takes the buffer contents.
    #[inline]
    pub fn take(&mut self) -> String {
        std::mem::take(&mut self.buffer)
    }
}

impl ConsoleWriter for StringWriter {
    fn write_str(&mut self, s: &str) -> Result<()> {
        self.buffer.push_str(s);
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }

    fn is_terminal(&self) -> bool {
        true // Pretend to be a terminal for testing
    }
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Console {
    /// Creates a new console with default settings.
    #[must_use]
    pub fn new() -> Self {
        let is_terminal = atty_check();
        Self {
            writer: Box::new(StdoutWriter),
            options: ConsoleOptions::default(),
            color_system: detect_color_system(),
            is_terminal,
            force_terminal: false,
            style: None,
            record_buffer: Vec::new(),
            record_mode: RecordMode::Off,
            captured: None,
            is_capturing: false,
            soft_wrap: false,
            width: None,
            height: None,
        }
    }

    /// Creates a console that writes to stderr.
    #[must_use]
    pub fn stderr() -> Self {
        let is_terminal = atty_check_stderr();
        Self {
            writer: Box::new(StderrWriter),
            options: ConsoleOptions::default(),
            color_system: detect_color_system(),
            is_terminal,
            force_terminal: false,
            style: None,
            record_buffer: Vec::new(),
            record_mode: RecordMode::Off,
            captured: None,
            is_capturing: false,
            soft_wrap: false,
            width: None,
            height: None,
        }
    }

    /// Creates a console with a custom writer.
    #[must_use]
    pub fn with_writer<W: ConsoleWriter + 'static>(writer: W) -> Self {
        let is_terminal = writer.is_terminal();
        Self {
            writer: Box::new(writer),
            options: ConsoleOptions::default(),
            color_system: ColorSystem::TrueColor,
            is_terminal,
            force_terminal: false,
            style: None,
            record_buffer: Vec::new(),
            record_mode: RecordMode::Off,
            captured: None,
            is_capturing: false,
            soft_wrap: false,
            width: None,
            height: None,
        }
    }

    /// Sets the color system to use.
    #[inline]
    pub fn set_color_system(&mut self, color_system: ColorSystem) {
        self.color_system = color_system;
    }

    /// Returns the current color system.
    #[inline]
    #[must_use]
    pub const fn color_system(&self) -> ColorSystem {
        self.color_system
    }

    /// Sets whether to force terminal mode.
    #[inline]
    pub fn set_force_terminal(&mut self, force: bool) {
        self.force_terminal = force;
    }

    /// Returns true if this is a terminal (or forced to act like one).
    #[inline]
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.is_terminal || self.force_terminal
    }

    /// Sets the default style.
    #[inline]
    pub fn set_style(&mut self, style: Option<Style>) {
        self.style = style;
    }

    /// Returns the default style.
    #[inline]
    #[must_use]
    pub const fn style(&self) -> Option<&Style> {
        self.style.as_ref()
    }

    /// Sets the console width.
    #[inline]
    pub fn set_width(&mut self, width: usize) {
        self.width = Some(width);
    }

    /// Sets the console height.
    #[inline]
    pub fn set_height(&mut self, height: usize) {
        self.height = Some(height);
    }

    /// Returns the terminal size.
    #[must_use]
    pub fn size(&self) -> TerminalSize {
        if let (Some(w), Some(h)) = (self.width, self.height) {
            return TerminalSize::new(w, h);
        }

        terminal::size()
            .map(|(w, h)| TerminalSize::new(usize::from(w), usize::from(h)))
            .unwrap_or(TerminalSize::new(80, 24))
    }

    /// Returns the terminal width.
    #[inline]
    #[must_use]
    pub fn width(&self) -> usize {
        self.width.unwrap_or_else(|| self.size().width)
    }

    /// Returns the terminal height.
    #[inline]
    #[must_use]
    pub fn height(&self) -> usize {
        self.height.unwrap_or_else(|| self.size().height)
    }

    /// Returns the console options.
    #[inline]
    #[must_use]
    pub const fn options(&self) -> &ConsoleOptions {
        &self.options
    }

    /// Returns mutable console options.
    #[inline]
    pub fn options_mut(&mut self) -> &mut ConsoleOptions {
        &mut self.options
    }

    /// Enables recording mode.
    #[inline]
    pub fn begin_recording(&mut self) {
        self.record_mode = RecordMode::On;
        self.record_buffer.clear();
    }

    /// Disables recording mode and returns the recorded segments.
    #[inline]
    pub fn end_recording(&mut self) -> Vec<Segment> {
        self.record_mode = RecordMode::Off;
        std::mem::take(&mut self.record_buffer)
    }

    /// Begins capturing output.
    #[inline]
    pub fn begin_capture(&mut self) {
        self.is_capturing = true;
        self.captured = Some(String::new());
    }

    /// Ends capturing and returns the captured output.
    #[inline]
    pub fn end_capture(&mut self) -> String {
        self.is_capturing = false;
        self.captured.take().unwrap_or_default()
    }

    /// Writes raw text to the console.
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    pub fn write(&mut self, s: &str) -> Result<()> {
        if self.is_capturing {
            if let Some(ref mut captured) = self.captured {
                captured.push_str(s);
            }
            return Ok(());
        }

        self.writer.write_str(s)?;
        Ok(())
    }

    /// Writes a segment to the console.
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    pub fn write_segment(&mut self, segment: &Segment) -> Result<()> {
        if self.record_mode == RecordMode::On {
            self.record_buffer.push(segment.clone());
        }

        let output = if self.color_system.has_colors() {
            segment.to_ansi()
        } else {
            segment.text.clone()
        };

        self.write(&output)
    }

    /// Writes multiple segments to the console.
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    pub fn write_segments(&mut self, segments: &Segments) -> Result<()> {
        for segment in segments.iter() {
            self.write_segment(segment)?;
        }
        Ok(())
    }

    /// Flushes the output.
    ///
    /// # Errors
    ///
    /// Returns an error if flushing fails.
    #[inline]
    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }

    /// Prints text with optional markup support.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to print (may contain markup if enabled)
    ///
    /// # Errors
    ///
    /// Returns an error if printing fails.
    pub fn print(&mut self, text: &str) -> Result<()> {
        self.print_with_options(text, &self.options.clone())
    }

    /// Prints text with specific options.
    ///
    /// # Errors
    ///
    /// Returns an error if printing fails.
    pub fn print_with_options(&mut self, text: &str, options: &ConsoleOptions) -> Result<()> {
        let rendered = if options.markup {
            let markup = Markup::parse(text)?;
            markup.to_text()
        } else {
            Text::from_str(text)
        };

        let segments = rendered.to_segments();
        self.write_segments(&segments)?;
        self.write("\n")?;
        self.flush()
    }

    /// Prints styled text.
    ///
    /// # Errors
    ///
    /// Returns an error if printing fails.
    pub fn print_styled(&mut self, text: &str, style: &Style) -> Result<()> {
        let styled_text = Text::styled(text, style.clone());
        let segments = styled_text.to_segments();
        self.write_segments(&segments)?;
        self.write("\n")?;
        self.flush()
    }

    /// Prints a Text object.
    ///
    /// # Errors
    ///
    /// Returns an error if printing fails.
    pub fn print_text(&mut self, text: &Text) -> Result<()> {
        let segments = text.to_segments();
        self.write_segments(&segments)?;
        self.write("\n")?;
        self.flush()
    }

    /// Logs text with a timestamp.
    ///
    /// # Errors
    ///
    /// Returns an error if logging fails.
    pub fn log(&mut self, text: &str) -> Result<()> {
        // For now, just print with a simple timestamp-like prefix
        // A full implementation would include actual timestamps and formatting
        let prefix = "[LOG] ";
        self.write(prefix)?;
        self.print(text)
    }

    /// Prints without markup processing.
    ///
    /// # Errors
    ///
    /// Returns an error if printing fails.
    pub fn out(&mut self, text: &str) -> Result<()> {
        self.write(text)?;
        self.write("\n")?;
        self.flush()
    }

    /// Draws a horizontal rule.
    ///
    /// # Errors
    ///
    /// Returns an error if drawing fails.
    pub fn rule(&mut self, title: Option<&str>) -> Result<()> {
        let width = self.width();
        let rule_char = '─';

        match title {
            Some(t) if !t.is_empty() => {
                let title_len = unicode_width::UnicodeWidthStr::width(t);
                let padding: usize = 2; // Space around title
                let rule_len = width
                    .saturating_sub(title_len)
                    .saturating_sub(padding.saturating_mul(2));
                let half = rule_len.checked_div(2).unwrap_or(0);
                let left = half;
                let right = rule_len.saturating_sub(left);

                let line = format!(
                    "{} {} {}",
                    rule_char.to_string().repeat(left),
                    t,
                    rule_char.to_string().repeat(right)
                );
                self.write(&line)?;
            }
            _ => {
                self.write(&rule_char.to_string().repeat(width))?;
            }
        }

        self.write("\n")?;
        self.flush()
    }

    /// Clears the screen.
    ///
    /// # Errors
    ///
    /// Returns an error if clearing fails.
    pub fn clear(&mut self) -> Result<()> {
        let ctrl = Control::new(ControlType::Clear);
        let seg = Segment::control(ctrl);
        self.write_segment(&seg)?;

        let home = Control::new(ControlType::Home);
        let seg = Segment::control(home);
        self.write_segment(&seg)?;

        self.flush()
    }

    /// Shows the cursor.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub fn show_cursor(&mut self) -> Result<()> {
        let ctrl = Control::new(ControlType::ShowCursor);
        let seg = Segment::control(ctrl);
        self.write_segment(&seg)?;
        self.flush()
    }

    /// Hides the cursor.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub fn hide_cursor(&mut self) -> Result<()> {
        let ctrl = Control::new(ControlType::HideCursor);
        let seg = Segment::control(ctrl);
        self.write_segment(&seg)?;
        self.flush()
    }

    /// Exports the recorded output as plain text.
    ///
    /// # Errors
    ///
    /// Returns an error if export fails.
    pub fn export_text(&self) -> Result<String> {
        Ok(self
            .record_buffer
            .iter()
            .filter(|s| !s.is_control())
            .map(|s| s.text.as_str())
            .collect())
    }

    /// Exports the recorded output as HTML.
    ///
    /// # Errors
    ///
    /// Returns an error if export fails.
    pub fn export_html(&self) -> Result<String> {
        let mut html = String::from("<pre><code>");

        for segment in &self.record_buffer {
            if segment.is_control() {
                continue;
            }

            let escaped = html_escape(&segment.text);

            match &segment.style {
                Some(style) if !style.is_empty() => {
                    let css = style_to_css(style);
                    html.push_str(&format!("<span style=\"{css}\">{escaped}</span>"));
                }
                _ => {
                    html.push_str(&escaped);
                }
            }
        }

        html.push_str("</code></pre>");
        Ok(html)
    }

    /// Measures a renderable and returns its dimensions.
    ///
    /// # Errors
    ///
    /// Returns an error if measurement fails.
    pub fn measure<M: Measurable>(&self, renderable: &M) -> Result<Measurement> {
        let options = MeasureOptions::new(self.width());
        renderable.measure(&options)
    }

    /// Prompts for input with optional markup.
    ///
    /// # Errors
    ///
    /// Returns an error if prompting fails.
    pub fn input(&mut self, prompt: &str) -> Result<String> {
        self.print(prompt)?;
        self.flush()?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| Error::ConsoleIo { source: e })?;

        // Remove trailing newline
        if input.ends_with('\n') {
            input.pop();
            if input.ends_with('\r') {
                input.pop();
            }
        }

        Ok(input)
    }
}

/// Re-exports for convenience.
pub use crate::text::Justify;
pub use crate::text::Overflow;

/// Detects the terminal color system.
fn detect_color_system() -> ColorSystem {
    // Check environment variables
    if let Ok(term) = std::env::var("COLORTERM") {
        if term == "truecolor" || term == "24bit" {
            return ColorSystem::TrueColor;
        }
    }

    if let Ok(term) = std::env::var("TERM") {
        if term.contains("256color") {
            return ColorSystem::EightBit;
        }
        if term.contains("color") || term.contains("xterm") || term.contains("screen") {
            return ColorSystem::Standard;
        }
    }

    // Default to true color on modern systems
    ColorSystem::TrueColor
}

/// Checks if stdout is a terminal.
fn atty_check() -> bool {
    // Use crossterm's detection
    crossterm::tty::IsTty::is_tty(&io::stdout())
}

/// Checks if stderr is a terminal.
fn atty_check_stderr() -> bool {
    crossterm::tty::IsTty::is_tty(&io::stderr())
}

/// Escapes HTML special characters.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Converts a style to CSS.
fn style_to_css(style: &Style) -> String {
    let mut css = Vec::new();

    if let Some(ref color) = style.color {
        css.push(format!("color: {}", color_to_css(color)));
    }

    if let Some(ref bgcolor) = style.bgcolor {
        css.push(format!("background-color: {}", color_to_css(bgcolor)));
    }

    if style.attributes.bold == Some(true) {
        css.push("font-weight: bold".to_owned());
    }

    if style.attributes.italic == Some(true) {
        css.push("font-style: italic".to_owned());
    }

    if style.attributes.underline == Some(true) {
        css.push("text-decoration: underline".to_owned());
    }

    if style.attributes.strike == Some(true) {
        css.push("text-decoration: line-through".to_owned());
    }

    css.join("; ")
}

/// Converts a color to CSS.
fn color_to_css(color: &crate::color::Color) -> String {
    use crate::color::{Color, StandardColor};

    match color {
        Color::Default => "inherit".to_owned(),
        Color::Standard(std) => match std {
            StandardColor::Black => "#000000",
            StandardColor::Red => "#cc0000",
            StandardColor::Green => "#00cc00",
            StandardColor::Yellow => "#cccc00",
            StandardColor::Blue => "#0000cc",
            StandardColor::Magenta => "#cc00cc",
            StandardColor::Cyan => "#00cccc",
            StandardColor::White => "#cccccc",
            StandardColor::BrightBlack => "#666666",
            StandardColor::BrightRed => "#ff0000",
            StandardColor::BrightGreen => "#00ff00",
            StandardColor::BrightYellow => "#ffff00",
            StandardColor::BrightBlue => "#0000ff",
            StandardColor::BrightMagenta => "#ff00ff",
            StandardColor::BrightCyan => "#00ffff",
            StandardColor::BrightWhite => "#ffffff",
        }
        .to_owned(),
        Color::Palette(idx) => format!("var(--palette-{idx})"),
        Color::Rgb { r, g, b } => format!("rgb({r}, {g}, {b})"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Color, StandardColor};

    #[test]
    fn test_console_new() {
        let console = Console::new();
        assert!(console.width() > 0);
        assert!(console.height() > 0);
    }

    #[test]
    fn test_console_with_writer() {
        let writer = StringWriter::new();
        let console = Console::with_writer(writer);
        assert!(console.is_terminal());
    }

    #[test]
    fn test_console_write() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);

        console.write("hello").ok();
        console.flush().ok();
    }

    #[test]
    fn test_console_capture() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);

        console.begin_capture();
        console.write("captured text").ok();
        let captured = console.end_capture();

        assert_eq!(captured, "captured text");
    }

    #[test]
    fn test_console_size() {
        let mut console = Console::new();
        console.set_width(120);
        console.set_height(40);

        assert_eq!(console.width(), 120);
        assert_eq!(console.height(), 40);
    }

    #[test]
    fn test_console_options() {
        let opts = ConsoleOptions::new(100);
        assert_eq!(opts.max_width, 100);

        let opts = opts.with_justify(Justify::Center);
        assert_eq!(opts.justify, Justify::Center);
    }

    #[test]
    fn test_console_options_default() {
        let opts = ConsoleOptions::default();
        assert_eq!(opts.max_width, 80);
        assert_eq!(opts.min_width, 1);
        assert_eq!(opts.justify, Justify::Default);
        assert_eq!(opts.overflow, Overflow::Fold);
        assert!(!opts.no_wrap);
        assert!(!opts.highlight);
        assert!(opts.markup);
        assert!(opts.emoji);
    }

    #[test]
    fn test_console_options_with_overflow() {
        let opts = ConsoleOptions::new(80).with_overflow(Overflow::Ellipsis);
        assert_eq!(opts.overflow, Overflow::Ellipsis);
    }

    #[test]
    fn test_console_options_no_wrap_field() {
        let mut opts = ConsoleOptions::new(80);
        opts.no_wrap = true;
        assert!(opts.no_wrap);
    }

    #[test]
    fn test_terminal_size() {
        let size = TerminalSize::new(80, 24);
        assert_eq!(size.width, 80);
        assert_eq!(size.height, 24);
    }

    #[test]
    fn test_terminal_size_default() {
        let size = TerminalSize::default();
        // Default-derived implementation initializes to 0
        assert_eq!(size.width, 0);
        assert_eq!(size.height, 0);
    }

    #[test]
    fn test_color_system() {
        assert!(!ColorSystem::None.has_colors());
        assert!(ColorSystem::Standard.has_colors());
        assert!(ColorSystem::EightBit.has_colors());
        assert!(ColorSystem::Windows.has_colors());
        assert!(ColorSystem::TrueColor.is_true_color());
        assert!(!ColorSystem::EightBit.is_true_color());
        assert!(!ColorSystem::Standard.is_true_color());
        assert!(!ColorSystem::None.is_true_color());
    }

    #[test]
    fn test_color_system_default() {
        let cs = ColorSystem::default();
        assert_eq!(cs, ColorSystem::TrueColor);
    }

    #[test]
    fn test_record_mode_default() {
        let mode = RecordMode::default();
        assert_eq!(mode, RecordMode::Off);
    }

    #[test]
    fn test_string_writer() {
        let mut writer = StringWriter::new();
        writer.write_str("hello").ok();
        assert_eq!(writer.contents(), "hello");

        let taken = writer.take();
        assert_eq!(taken, "hello");
        assert!(writer.contents().is_empty());
    }

    #[test]
    fn test_string_writer_flush() {
        let mut writer = StringWriter::new();
        writer.write_str("test").ok();
        assert!(writer.flush().is_ok());
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("a & b"), "a &amp; b");
        assert_eq!(html_escape("\"quoted\""), "&quot;quoted&quot;");
        assert_eq!(html_escape("'single'"), "&#39;single&#39;");
    }

    #[test]
    fn test_console_recording() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);

        console.begin_recording();
        console.write_segment(&Segment::new("test")).ok();
        let recorded = console.end_recording();

        assert_eq!(recorded.len(), 1);
        assert_eq!(recorded.first().map(|s| s.text.as_str()), Some("test"));
    }

    #[test]
    fn test_export_text() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);

        console.begin_recording();
        console.write_segment(&Segment::new("test")).ok();
        let text = console.export_text().ok().unwrap_or_default();
        assert_eq!(text, "test");
    }

    #[test]
    fn test_export_html() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);

        console.begin_recording();
        console.write_segment(&Segment::new("plain")).ok();
        let styled = Segment::styled("bold", Style::new().bold());
        console.write_segment(&styled).ok();
        let html = console.export_html().ok().unwrap_or_default();

        assert!(html.contains("<pre><code>"));
        assert!(html.contains("plain"));
        assert!(html.contains("</code></pre>"));
    }

    #[test]
    fn test_console_print() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);
        console.begin_capture();

        console.print("[bold]Hello[/]").ok();
        let captured = console.end_capture();

        assert!(captured.contains("Hello"));
    }

    #[test]
    fn test_console_print_styled() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);
        console.begin_capture();

        console.print_styled("Hello", &Style::new().bold()).ok();
        let captured = console.end_capture();

        assert!(captured.contains("Hello"));
    }

    #[test]
    fn test_console_print_text() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);
        console.begin_capture();

        let text = Text::from_str("Hello World");
        console.print_text(&text).ok();
        let captured = console.end_capture();

        assert!(captured.contains("Hello World"));
    }

    #[test]
    fn test_console_out() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);
        console.begin_capture();

        console.out("raw text").ok();
        let captured = console.end_capture();

        assert!(captured.contains("raw text"));
    }

    #[test]
    fn test_console_log() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);
        console.begin_capture();

        console.log("log message").ok();
        let captured = console.end_capture();

        assert!(captured.contains("[LOG]"));
        assert!(captured.contains("log message"));
    }

    #[test]
    fn test_console_rule() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);
        console.set_width(40);
        console.begin_capture();

        console.rule(None).ok();
        let captured = console.end_capture();

        assert!(captured.contains("─"));
    }

    #[test]
    fn test_console_rule_with_title() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);
        console.set_width(40);
        console.begin_capture();

        console.rule(Some("Title")).ok();
        let captured = console.end_capture();

        assert!(captured.contains("Title"));
        assert!(captured.contains("─"));
    }

    #[test]
    fn test_console_clear() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);
        console.begin_capture();

        console.clear().ok();
        let captured = console.end_capture();

        // Should contain ANSI clear and home sequences
        assert!(captured.contains("\x1b["));
    }

    #[test]
    fn test_console_show_hide_cursor() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);
        console.begin_capture();

        console.hide_cursor().ok();
        console.show_cursor().ok();
        let captured = console.end_capture();

        assert!(captured.contains("\x1b["));
    }

    #[test]
    fn test_console_write_segments() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);
        console.begin_capture();

        let mut segments = Segments::new();
        segments.push(Segment::new("Hello "));
        segments.push(Segment::new("World"));

        console.write_segments(&segments).ok();
        let captured = console.end_capture();

        assert!(captured.contains("Hello "));
        assert!(captured.contains("World"));
    }

    #[test]
    fn test_console_set_color_system() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);

        console.set_color_system(ColorSystem::None);
        assert_eq!(console.color_system(), ColorSystem::None);

        console.set_color_system(ColorSystem::TrueColor);
        assert_eq!(console.color_system(), ColorSystem::TrueColor);
    }

    #[test]
    fn test_console_force_terminal() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);

        console.set_force_terminal(true);
        assert!(console.is_terminal());

        console.set_force_terminal(false);
    }

    #[test]
    fn test_console_set_style() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);

        assert!(console.style().is_none());

        console.set_style(Some(Style::new().bold()));
        assert!(console.style().is_some());

        console.set_style(None);
        assert!(console.style().is_none());
    }

    #[test]
    fn test_console_options_mut() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);

        console.options_mut().max_width = 120;
        assert_eq!(console.options().max_width, 120);
    }

    #[test]
    fn test_style_to_css() {
        let style = Style::new()
            .bold()
            .italic()
            .underline()
            .strike()
            .with_color(Color::Standard(StandardColor::Red))
            .with_bgcolor(Color::Standard(StandardColor::White));

        let css = style_to_css(&style);
        assert!(css.contains("font-weight: bold"));
        assert!(css.contains("font-style: italic"));
        assert!(
            css.contains("text-decoration: underline")
                || css.contains("text-decoration: line-through")
        );
        assert!(css.contains("color:"));
        assert!(css.contains("background-color:"));
    }

    #[test]
    fn test_color_to_css() {
        assert_eq!(color_to_css(&Color::Default), "inherit");
        assert_eq!(
            color_to_css(&Color::Standard(StandardColor::Red)),
            "#cc0000"
        );
        assert_eq!(
            color_to_css(&Color::Standard(StandardColor::Green)),
            "#00cc00"
        );
        assert_eq!(
            color_to_css(&Color::Standard(StandardColor::Blue)),
            "#0000cc"
        );
        assert_eq!(
            color_to_css(&Color::Standard(StandardColor::Black)),
            "#000000"
        );
        assert_eq!(
            color_to_css(&Color::Standard(StandardColor::White)),
            "#cccccc"
        );
        assert_eq!(
            color_to_css(&Color::Standard(StandardColor::BrightRed)),
            "#ff0000"
        );
        assert_eq!(color_to_css(&Color::Palette(42)), "var(--palette-42)");
        assert_eq!(
            color_to_css(&Color::Rgb {
                r: 255,
                g: 128,
                b: 0
            }),
            "rgb(255, 128, 0)"
        );
    }

    #[test]
    fn test_console_write_segment_no_color() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);
        console.set_color_system(ColorSystem::None);
        console.begin_capture();

        let styled = Segment::styled("text", Style::new().bold());
        console.write_segment(&styled).ok();
        let captured = console.end_capture();

        // With no color system, should output plain text without ANSI codes
        assert_eq!(captured, "text");
    }

    #[test]
    fn test_console_print_with_options_no_markup() {
        let writer = StringWriter::new();
        let mut console = Console::with_writer(writer);
        console.begin_capture();

        let opts = ConsoleOptions::new(80);
        let mut opts_no_markup = opts;
        opts_no_markup.markup = false;

        console
            .print_with_options("[bold]Hello[/]", &opts_no_markup)
            .ok();
        let captured = console.end_capture();

        // Should contain literal brackets since markup is disabled
        assert!(captured.contains("[bold]"));
    }
}
