//! Live display for real-time terminal updates.
//!
//! The [`Live`] struct provides a way to display content that updates
//! in real-time without scrolling the terminal.
//!
//! # Example
//!
//! ```ignore
//! use richrs::prelude::*;
//! use richrs::live::Live;
//!
//! let mut live = Live::new();
//! live.start();
//!
//! for i in 0..10 {
//!     live.update(format!("Count: {}", i));
//!     std::thread::sleep(std::time::Duration::from_millis(500));
//! }
//!
//! live.stop();
//! ```

use crate::text::Text;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// A live display that can update content in place.
///
/// Live provides a way to render content that changes over time
/// without scrolling the terminal. The display updates in place.
#[derive(Debug)]
pub struct Live {
    /// The current content to display.
    content: Arc<Mutex<Option<Text>>>,
    /// Number of lines in the current display.
    line_count: Arc<Mutex<usize>>,
    /// Refresh rate in updates per second.
    refresh_per_second: f64,
    /// Whether the content should be cleared when stopped.
    transient: bool,
    /// Whether auto-refresh is enabled.
    auto_refresh: bool,
    /// Whether the live display is currently running.
    running: Arc<AtomicBool>,
    /// Handle to the refresh thread.
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl Default for Live {
    fn default() -> Self {
        Self::new()
    }
}

impl Live {
    /// Creates a new Live display.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self {
            content: Arc::new(Mutex::new(None)),
            line_count: Arc::new(Mutex::new(0)),
            refresh_per_second: 4.0,
            transient: false,
            auto_refresh: true,
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        }
    }

    /// Creates a new Live display with initial content.
    #[must_use]
    #[inline]
    pub fn with_content<T: Into<Text>>(content: T) -> Self {
        let live = Self::new();
        if let Ok(mut guard) = live.content.lock() {
            *guard = Some(content.into());
        }
        live
    }

    /// Sets the refresh rate in updates per second.
    #[must_use]
    #[inline]
    pub fn refresh_per_second(mut self, rate: f64) -> Self {
        self.refresh_per_second = rate;
        self
    }

    /// Sets whether the content should be cleared when stopped.
    #[must_use]
    #[inline]
    pub const fn transient(mut self, transient: bool) -> Self {
        self.transient = transient;
        self
    }

    /// Sets whether auto-refresh is enabled.
    #[must_use]
    #[inline]
    pub const fn auto_refresh(mut self, auto_refresh: bool) -> Self {
        self.auto_refresh = auto_refresh;
        self
    }

    /// Updates the displayed content.
    pub fn update<T: Into<Text>>(&self, content: T) {
        if let Ok(mut guard) = self.content.lock() {
            *guard = Some(content.into());
        }
    }

    /// Manually triggers a refresh.
    pub fn refresh(&self) {
        self.render_content();
    }

    /// Starts the live display.
    ///
    /// If auto_refresh is enabled, spawns a background thread that
    /// periodically refreshes the display.
    pub fn start(&mut self) {
        if self.running.load(Ordering::SeqCst) {
            return;
        }

        self.running.store(true, Ordering::SeqCst);

        // Hide cursor
        eprint!("\x1b[?25l");
        let _ = std::io::stderr().flush();

        if self.auto_refresh {
            let running = Arc::clone(&self.running);
            let content = Arc::clone(&self.content);
            let line_count = Arc::clone(&self.line_count);
            let interval = Duration::from_secs_f64(1.0 / self.refresh_per_second);

            let handle = thread::spawn(move || {
                while running.load(Ordering::SeqCst) {
                    Self::render_content_inner(&content, &line_count);
                    thread::sleep(interval);
                }
            });

            self.thread_handle = Some(handle);
        } else {
            self.render_content();
        }
    }

    /// Stops the live display.
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);

        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }

        if self.transient {
            // Clear the display
            if let Ok(count) = self.line_count.lock() {
                if *count > 0 {
                    Self::clear_lines(*count);
                }
            }
        }

        // Show cursor
        eprint!("\x1b[?25h");
        let _ = std::io::stderr().flush();
    }

    /// Returns whether the live display is currently running.
    #[must_use]
    #[inline]
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Renders the current content.
    fn render_content(&self) {
        Self::render_content_inner(&self.content, &self.line_count);
    }

    /// Inner render function for use in threads.
    fn render_content_inner(content: &Arc<Mutex<Option<Text>>>, line_count: &Arc<Mutex<usize>>) {
        let Ok(content_guard) = content.lock() else {
            return;
        };

        let Some(ref text) = *content_guard else {
            return;
        };

        // Get previous line count
        let prev_lines = match line_count.lock() {
            Ok(guard) => *guard,
            Err(_) => return,
        };

        // Clear previous lines
        if prev_lines > 0 {
            Self::clear_lines(prev_lines);
        }

        // Render new content
        let segments = text.to_segments();
        let output = segments.to_ansi();

        // Count lines in new output
        let new_lines = output
            .chars()
            .filter(|&c| c == '\n')
            .count()
            .saturating_add(1);

        eprint!("{}", output);
        let _ = std::io::stderr().flush();

        // Update line count
        if let Ok(mut guard) = line_count.lock() {
            *guard = new_lines;
        }
    }

    /// Clears the specified number of lines.
    fn clear_lines(count: usize) {
        for _ in 0..count {
            // Move up and clear line
            eprint!("\x1b[1A\x1b[2K");
        }
        // Move to beginning of line
        eprint!("\r");
        let _ = std::io::stderr().flush();
    }

    /// Runs a closure while the live display is active.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Live::with_content("Working...").run(|live| {
    ///     for i in 0..10 {
    ///         live.update(format!("Progress: {}/10", i + 1));
    ///         std::thread::sleep(std::time::Duration::from_millis(200));
    ///     }
    /// });
    /// ```
    pub fn run<F, R>(mut self, f: F) -> R
    where
        F: FnOnce(&Self) -> R,
    {
        self.start();
        let result = f(&self);
        self.stop();
        result
    }
}

impl Drop for Live {
    fn drop(&mut self) {
        if self.is_running() {
            self.stop();
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_live_new() {
        let live = Live::new();
        assert!(!live.is_running());
    }

    #[test]
    fn test_live_with_content() {
        let live = Live::with_content("Hello");
        let content = live.content.lock().unwrap();
        assert!(content.is_some());
    }

    #[test]
    fn test_live_refresh_rate() {
        let live = Live::new().refresh_per_second(10.0);
        assert!((live.refresh_per_second - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_live_transient() {
        let live = Live::new().transient(true);
        assert!(live.transient);
    }

    #[test]
    fn test_live_auto_refresh() {
        let live = Live::new().auto_refresh(false);
        assert!(!live.auto_refresh);
    }

    #[test]
    fn test_live_update() {
        let live = Live::new();
        live.update("Test");
        let content = live.content.lock().unwrap();
        assert!(content.is_some());
    }

    #[test]
    fn test_live_not_running_initially() {
        let live = Live::new();
        assert!(!live.is_running());
    }
}
