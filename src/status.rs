// Known-valid spinner names use expect
#![allow(clippy::expect_used)]

//! Status display with animated spinner.
//!
//! The [`Status`] struct provides a way to show a spinner with a status message
//! that can be updated while a long-running operation is in progress.
//!
//! # Example
//!
//! ```ignore
//! use richrs::prelude::*;
//! use richrs::status::Status;
//!
//! let mut status = Status::new("Loading data...");
//! status.start();
//! // ... do work ...
//! status.update("Processing...");
//! // ... do more work ...
//! status.stop();
//! ```

use crate::errors::Result;
use crate::spinner::Spinner;
use crate::style::Style;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

/// A status indicator with animated spinner.
///
/// Status displays a spinner animation alongside a message that can be
/// updated during long-running operations.
#[derive(Debug)]
pub struct Status {
    /// The current status message.
    message: String,
    /// The spinner to use.
    spinner: Spinner,
    /// Style for the spinner.
    spinner_style: Option<Style>,
    /// Animation speed multiplier.
    speed: f64,
    /// Refresh rate in updates per second.
    refresh_per_second: f64,
    /// Whether the status is currently running.
    running: Arc<AtomicBool>,
    /// Handle to the animation thread.
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl Status {
    /// Creates a new Status with the given message.
    #[must_use]
    #[inline]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            spinner: Spinner::new("dots").unwrap_or_else(|_| {
                Spinner::new("line").expect("line spinner should always exist")
            }),
            spinner_style: None,
            speed: 1.0,
            refresh_per_second: 12.5,
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        }
    }

    /// Sets the spinner type.
    ///
    /// # Errors
    ///
    /// Returns an error if the spinner name is not recognized.
    pub fn spinner(mut self, name: &str) -> Result<Self> {
        self.spinner = Spinner::new(name)?;
        Ok(self)
    }

    /// Sets the spinner style.
    #[must_use]
    #[inline]
    pub fn spinner_style(mut self, style: Style) -> Self {
        self.spinner_style = Some(style);
        self
    }

    /// Sets the animation speed multiplier.
    ///
    /// A value of 1.0 is normal speed, 2.0 is twice as fast, etc.
    #[must_use]
    #[inline]
    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = speed;
        self
    }

    /// Sets the refresh rate in updates per second.
    #[must_use]
    #[inline]
    pub fn refresh_per_second(mut self, rate: f64) -> Self {
        self.refresh_per_second = rate;
        self
    }

    /// Updates the status message.
    #[inline]
    pub fn update(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    /// Starts the spinner animation.
    ///
    /// This spawns a background thread that renders the spinner.
    pub fn start(&mut self) {
        if self.running.load(Ordering::SeqCst) {
            return;
        }

        self.running.store(true, Ordering::SeqCst);

        let running = Arc::clone(&self.running);
        let mut spinner = self.spinner.clone();
        let message = self.message.clone();
        let style = self.spinner_style.clone();
        let interval = (1000.0 / self.refresh_per_second / self.speed) as u64;

        let handle = thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                let frame = spinner.next_frame();

                // Clear line and render
                eprint!("\r\x1b[K");

                if let Some(ref s) = style {
                    eprint!("{}", s.to_ansi());
                }
                eprint!("{}", frame);
                if style.is_some() {
                    eprint!("\x1b[0m");
                }
                eprint!(" {}", message);

                let _ = std::io::stderr().flush();
                thread::sleep(Duration::from_millis(interval));
            }

            // Clear the line when stopping
            eprint!("\r\x1b[K");
            let _ = std::io::stderr().flush();
        });

        self.thread_handle = Some(handle);
    }

    /// Stops the spinner animation.
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);

        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }

    /// Returns whether the status is currently running.
    #[must_use]
    #[inline]
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Runs a closure while showing the status.
    ///
    /// The spinner starts before the closure runs and stops after it completes.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Status::new("Working...").run(|| {
    ///     // Do some work
    /// });
    /// ```
    pub fn run<F, R>(mut self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        self.start();
        let result = f();
        self.stop();
        result
    }

    /// Runs an async-style closure with the ability to update the message.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Status::new("Starting...").run_with_updates(|status| {
    ///     // Do work
    ///     status.update("Still working...");
    ///     // More work
    /// });
    /// ```
    pub fn run_with_updates<F, R>(mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.start();
        let result = f(&mut self);
        self.stop();
        result
    }
}

impl Drop for Status {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_status_new() {
        let status = Status::new("Loading...");
        assert_eq!(status.message, "Loading...");
        assert!(!status.is_running());
    }

    #[test]
    fn test_status_update() {
        let mut status = Status::new("First");
        status.update("Second");
        assert_eq!(status.message, "Second");
    }

    #[test]
    fn test_status_update_string() {
        let mut status = Status::new("First");
        status.update(String::from("Second"));
        assert_eq!(status.message, "Second");
    }

    #[test]
    fn test_status_speed() {
        let status = Status::new("Test").speed(2.0);
        assert!((status.speed - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_status_speed_slow() {
        let status = Status::new("Test").speed(0.5);
        assert!((status.speed - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_status_refresh_rate() {
        let status = Status::new("Test").refresh_per_second(20.0);
        assert!((status.refresh_per_second - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_status_spinner() {
        let status = Status::new("Test").spinner("line").unwrap();
        assert_eq!(status.spinner.name(), "line");
    }

    #[test]
    fn test_status_spinner_style() {
        let style = Style::default().bold();
        let status = Status::new("Test").spinner_style(style.clone());
        assert!(status.spinner_style.is_some());
    }

    #[test]
    fn test_status_run() {
        let mut called = false;
        Status::new("Running...").run(|| {
            called = true;
        });
        assert!(called);
    }

    #[test]
    fn test_status_run_returns_value() {
        let result = Status::new("Computing...").run(|| 42);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_status_not_running_initially() {
        let status = Status::new("Test");
        assert!(!status.is_running());
    }

    #[test]
    fn test_status_start_stop() {
        let mut status = Status::new("Test").speed(100.0); // Fast for testing
        status.start();
        assert!(status.is_running());
        thread::sleep(Duration::from_millis(20));
        status.stop();
        assert!(!status.is_running());
    }

    #[test]
    fn test_status_double_start() {
        let mut status = Status::new("Test").speed(100.0);
        status.start();
        status.start(); // Should be no-op
        assert!(status.is_running());
        status.stop();
    }

    #[test]
    fn test_status_run_with_updates() {
        let result = Status::new("Starting...").run_with_updates(|status| {
            status.update("Working...");
            status.update("Finishing...");
            42
        });
        assert_eq!(result, 42);
    }

    #[test]
    fn test_status_builder_chain() {
        let status = Status::new("Test")
            .speed(1.5)
            .refresh_per_second(15.0)
            .spinner("dots")
            .unwrap()
            .spinner_style(Style::new().bold());

        assert!((status.speed - 1.5).abs() < f64::EPSILON);
        assert!((status.refresh_per_second - 15.0).abs() < f64::EPSILON);
        assert!(status.spinner_style.is_some());
    }

    #[test]
    fn test_status_drop_stops() {
        let mut status = Status::new("Test").speed(100.0);
        status.start();
        assert!(status.is_running());
        drop(status);
        // No assertion needed - just verify it doesn't panic
    }
}
