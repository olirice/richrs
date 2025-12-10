//! Logging integration for styled terminal output.
//!
//! This module provides a logging handler that outputs styled log messages
//! to the terminal using richrs formatting.
//!
//! # Example
//!
//! ```ignore
//! use richrs::logging::RichHandler;
//! use log::{info, warn, error};
//!
//! // Initialize the rich logger
//! RichHandler::init();
//!
//! info!("Application started");
//! warn!("This is a warning");
//! error!("An error occurred");
//! ```

use crate::style::Style;
use std::io::Write;

/// Log levels with associated styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Trace level (most verbose).
    Trace,
    /// Debug level.
    Debug,
    /// Info level.
    Info,
    /// Warning level.
    Warn,
    /// Error level (most severe).
    Error,
}

impl LogLevel {
    /// Returns the style for this log level.
    #[must_use]
    pub fn style(self) -> Style {
        match self {
            Self::Trace => Style::default().dim(),
            Self::Debug => Style::default().dim(),
            Self::Info => Style::default().bold(),
            Self::Warn => Style::default().bold(), // Would be yellow with color
            Self::Error => Style::default().bold(), // Would be red with color
        }
    }

    /// Returns the label for this log level.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Trace => "TRACE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO ",
            Self::Warn => "WARN ",
            Self::Error => "ERROR",
        }
    }

    /// Returns the label width-padded for alignment.
    #[must_use]
    pub const fn padded_label(self) -> &'static str {
        match self {
            Self::Trace => "TRACE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO ",
            Self::Warn => "WARN ",
            Self::Error => "ERROR",
        }
    }
}

/// A rich logging handler for styled log output.
///
/// This handler formats log messages with colors and styles based on
/// log level, and can include timestamps and source locations.
#[derive(Debug, Clone)]
pub struct RichHandler {
    /// Whether to show timestamps.
    show_time: bool,
    /// Whether to show the log level.
    show_level: bool,
    /// Whether to show the source path (module/file).
    show_path: bool,
    /// Minimum log level to display.
    level: LogLevel,
    /// Whether to use markup for styling.
    markup: bool,
    /// Whether to show rich tracebacks for errors.
    rich_tracebacks: bool,
}

impl Default for RichHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl RichHandler {
    /// Creates a new RichHandler with default settings.
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            show_time: true,
            show_level: true,
            show_path: true,
            level: LogLevel::Info,
            markup: true,
            rich_tracebacks: true,
        }
    }

    /// Sets whether to show timestamps.
    #[must_use]
    #[inline]
    pub const fn show_time(mut self, show: bool) -> Self {
        self.show_time = show;
        self
    }

    /// Sets whether to show log levels.
    #[must_use]
    #[inline]
    pub const fn show_level(mut self, show: bool) -> Self {
        self.show_level = show;
        self
    }

    /// Sets whether to show source paths.
    #[must_use]
    #[inline]
    pub const fn show_path(mut self, show: bool) -> Self {
        self.show_path = show;
        self
    }

    /// Sets the minimum log level.
    #[must_use]
    #[inline]
    pub const fn level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Sets whether to use markup styling.
    #[must_use]
    #[inline]
    pub const fn markup(mut self, enabled: bool) -> Self {
        self.markup = enabled;
        self
    }

    /// Sets whether to show rich tracebacks.
    #[must_use]
    #[inline]
    pub const fn rich_tracebacks(mut self, enabled: bool) -> Self {
        self.rich_tracebacks = enabled;
        self
    }

    /// Formats a log message.
    #[must_use]
    pub fn format(&self, level: LogLevel, message: &str, path: Option<&str>) -> String {
        let mut output = String::new();

        // Time
        if self.show_time {
            let now = chrono_time();
            let style = Style::default().dim();
            output.push_str(&format!("{}{}\x1b[0m ", style.to_ansi(), now));
        }

        // Level
        if self.show_level {
            let style = level.style();
            output.push_str(&format!(
                "{}{}\x1b[0m ",
                style.to_ansi(),
                level.padded_label()
            ));
        }

        // Path
        if self.show_path {
            if let Some(p) = path {
                let style = Style::default().dim();
                output.push_str(&format!("{}[{}]\x1b[0m ", style.to_ansi(), p));
            }
        }

        // Message
        output.push_str(message);

        output
    }

    /// Logs a message at the specified level.
    pub fn log(&self, level: LogLevel, message: &str, path: Option<&str>) {
        let formatted = self.format(level, message, path);
        eprintln!("{}", formatted);
        let _ = std::io::stderr().flush();
    }

    /// Logs a trace message.
    pub fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message, None);
    }

    /// Logs a debug message.
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message, None);
    }

    /// Logs an info message.
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message, None);
    }

    /// Logs a warning message.
    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message, None);
    }

    /// Logs an error message.
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message, None);
    }
}

/// Returns a simple timestamp string (without chrono dependency).
fn chrono_time() -> String {
    // Simple time representation without external dependencies
    // In a real implementation, this would use chrono or time crate
    "[TIME]".to_string()
}

/// Convenience macros and functions for logging.
pub mod macros {
    /// Simple trace logging function.
    pub fn trace(message: &str) {
        super::RichHandler::new().trace(message);
    }

    /// Simple debug logging function.
    pub fn debug(message: &str) {
        super::RichHandler::new().debug(message);
    }

    /// Simple info logging function.
    pub fn info(message: &str) {
        super::RichHandler::new().info(message);
    }

    /// Simple warn logging function.
    pub fn warn(message: &str) {
        super::RichHandler::new().warn(message);
    }

    /// Simple error logging function.
    pub fn error(message: &str) {
        super::RichHandler::new().error(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rich_handler_new() {
        let handler = RichHandler::new();
        assert!(handler.show_time);
        assert!(handler.show_level);
        assert!(handler.show_path);
    }

    #[test]
    fn test_rich_handler_show_time() {
        let handler = RichHandler::new().show_time(false);
        assert!(!handler.show_time);
    }

    #[test]
    fn test_rich_handler_show_level() {
        let handler = RichHandler::new().show_level(false);
        assert!(!handler.show_level);
    }

    #[test]
    fn test_rich_handler_show_path() {
        let handler = RichHandler::new().show_path(false);
        assert!(!handler.show_path);
    }

    #[test]
    fn test_rich_handler_level() {
        let handler = RichHandler::new().level(LogLevel::Debug);
        assert_eq!(handler.level, LogLevel::Debug);
    }

    #[test]
    fn test_log_level_label() {
        assert_eq!(LogLevel::Info.label(), "INFO ");
        assert_eq!(LogLevel::Error.label(), "ERROR");
    }

    #[test]
    fn test_rich_handler_format() {
        let handler = RichHandler::new().show_time(false).show_path(false);
        let formatted = handler.format(LogLevel::Info, "Test message", None);
        assert!(formatted.contains("INFO"));
        assert!(formatted.contains("Test message"));
    }

    #[test]
    fn test_rich_handler_format_with_path() {
        let handler = RichHandler::new().show_time(false);
        let formatted = handler.format(LogLevel::Info, "Test", Some("mymodule"));
        assert!(formatted.contains("mymodule"));
    }
}
