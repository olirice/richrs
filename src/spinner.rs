//! Spinner animations for console output.
//!
//! This module provides animated spinner indicators compatible with Python Rich.
//!
//! # Example
//!
//! ```rust,ignore
//! use richrs::spinner::Spinner;
//!
//! let spinner = Spinner::new("dots");
//! for frame in spinner.frames().take(10) {
//!     println!("{}", frame);
//! }
//! ```

use crate::errors::{Error, Result};
use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;

/// Spinner configuration data.
#[derive(Debug, Clone)]
struct SpinnerData {
    /// The animation interval in milliseconds.
    interval: u64,
    /// The frames of the animation.
    frames: Vec<&'static str>,
}

/// A map of spinner names to their configurations.
static SPINNERS: LazyLock<HashMap<&'static str, SpinnerData>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    // dots - Classic Braille dots spinner
    m.insert(
        "dots",
        SpinnerData {
            interval: 80,
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
        },
    );

    // dots2 - Variant dots spinner
    m.insert(
        "dots2",
        SpinnerData {
            interval: 80,
            frames: vec!["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"],
        },
    );

    // dots3 - Another variant
    m.insert(
        "dots3",
        SpinnerData {
            interval: 80,
            frames: vec!["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"],
        },
    );

    // line - Classic ASCII line spinner
    m.insert(
        "line",
        SpinnerData {
            interval: 130,
            frames: vec!["-", "\\", "|", "/"],
        },
    );

    // line2 - Double line spinner
    m.insert(
        "line2",
        SpinnerData {
            interval: 100,
            frames: vec!["⠂", "-", "–", "—", "–", "-"],
        },
    );

    // pipe - Pipe spinner
    m.insert(
        "pipe",
        SpinnerData {
            interval: 100,
            frames: vec!["┤", "┘", "┴", "└", "├", "┌", "┬", "┐"],
        },
    );

    // simpleDots
    m.insert(
        "simpleDots",
        SpinnerData {
            interval: 400,
            frames: vec![".  ", ".. ", "...", "   "],
        },
    );

    // star
    m.insert(
        "star",
        SpinnerData {
            interval: 70,
            frames: vec!["✶", "✸", "✹", "✺", "✹", "✷"],
        },
    );

    // star2
    m.insert(
        "star2",
        SpinnerData {
            interval: 80,
            frames: vec!["+", "x", "*"],
        },
    );

    // arc
    m.insert(
        "arc",
        SpinnerData {
            interval: 100,
            frames: vec!["◜", "◠", "◝", "◞", "◡", "◟"],
        },
    );

    // circle
    m.insert(
        "circle",
        SpinnerData {
            interval: 120,
            frames: vec!["◡", "⊙", "◠"],
        },
    );

    // squareCorners
    m.insert(
        "squareCorners",
        SpinnerData {
            interval: 180,
            frames: vec!["◰", "◳", "◲", "◱"],
        },
    );

    // circleQuarters
    m.insert(
        "circleQuarters",
        SpinnerData {
            interval: 120,
            frames: vec!["◴", "◷", "◶", "◵"],
        },
    );

    // circleHalves
    m.insert(
        "circleHalves",
        SpinnerData {
            interval: 50,
            frames: vec!["◐", "◓", "◑", "◒"],
        },
    );

    // moon
    m.insert(
        "moon",
        SpinnerData {
            interval: 80,
            frames: vec!["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
        },
    );

    // runner
    m.insert(
        "runner",
        SpinnerData {
            interval: 140,
            frames: vec!["🚶", "🏃"],
        },
    );

    // bounce - Bouncing ball
    m.insert(
        "bounce",
        SpinnerData {
            interval: 120,
            frames: vec!["⠁", "⠂", "⠄", "⠂"],
        },
    );

    // bouncingBar
    m.insert(
        "bouncingBar",
        SpinnerData {
            interval: 80,
            frames: vec![
                "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]",
                "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
            ],
        },
    );

    // bouncingBall
    m.insert(
        "bouncingBall",
        SpinnerData {
            interval: 80,
            frames: vec![
                "( ●    )",
                "(  ●   )",
                "(   ●  )",
                "(    ● )",
                "(     ●)",
                "(    ● )",
                "(   ●  )",
                "(  ●   )",
                "( ●    )",
                "(●     )",
            ],
        },
    );

    // clock
    m.insert(
        "clock",
        SpinnerData {
            interval: 100,
            frames: vec![
                "🕛", "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚",
            ],
        },
    );

    // earth
    m.insert(
        "earth",
        SpinnerData {
            interval: 180,
            frames: vec!["🌍", "🌎", "🌏"],
        },
    );

    // hearts
    m.insert(
        "hearts",
        SpinnerData {
            interval: 100,
            frames: vec!["💛", "💙", "💜", "💚", "❤️"],
        },
    );

    // arrow
    m.insert(
        "arrow",
        SpinnerData {
            interval: 100,
            frames: vec!["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"],
        },
    );

    // arrow2
    m.insert(
        "arrow2",
        SpinnerData {
            interval: 80,
            frames: vec!["⬆️", "↗️", "➡️", "↘️", "⬇️", "↙️", "⬅️", "↖️"],
        },
    );

    // arrow3
    m.insert(
        "arrow3",
        SpinnerData {
            interval: 120,
            frames: vec!["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"],
        },
    );

    // toggle
    m.insert(
        "toggle",
        SpinnerData {
            interval: 250,
            frames: vec!["⊶", "⊷"],
        },
    );

    // toggle2
    m.insert(
        "toggle2",
        SpinnerData {
            interval: 80,
            frames: vec!["▫", "▪"],
        },
    );

    // toggle3
    m.insert(
        "toggle3",
        SpinnerData {
            interval: 120,
            frames: vec!["□", "■"],
        },
    );

    // toggle4
    m.insert(
        "toggle4",
        SpinnerData {
            interval: 100,
            frames: vec!["■", "□", "▪", "▫"],
        },
    );

    // toggle5
    m.insert(
        "toggle5",
        SpinnerData {
            interval: 100,
            frames: vec!["▮", "▯"],
        },
    );

    // toggle6
    m.insert(
        "toggle6",
        SpinnerData {
            interval: 300,
            frames: vec!["ဝ", "၀"],
        },
    );

    // toggle7
    m.insert(
        "toggle7",
        SpinnerData {
            interval: 80,
            frames: vec!["⦾", "⦿"],
        },
    );

    // toggle8
    m.insert(
        "toggle8",
        SpinnerData {
            interval: 100,
            frames: vec!["◍", "◌"],
        },
    );

    // toggle9
    m.insert(
        "toggle9",
        SpinnerData {
            interval: 100,
            frames: vec!["◉", "◎"],
        },
    );

    // toggle10
    m.insert(
        "toggle10",
        SpinnerData {
            interval: 100,
            frames: vec!["㊂", "㊀", "㊁"],
        },
    );

    // toggle11
    m.insert(
        "toggle11",
        SpinnerData {
            interval: 50,
            frames: vec!["⧇", "⧆"],
        },
    );

    // toggle12
    m.insert(
        "toggle12",
        SpinnerData {
            interval: 120,
            frames: vec!["☗", "☖"],
        },
    );

    // toggle13
    m.insert(
        "toggle13",
        SpinnerData {
            interval: 80,
            frames: vec!["=", "*", "-"],
        },
    );

    // boxBounce
    m.insert(
        "boxBounce",
        SpinnerData {
            interval: 120,
            frames: vec!["▖", "▘", "▝", "▗"],
        },
    );

    // boxBounce2
    m.insert(
        "boxBounce2",
        SpinnerData {
            interval: 100,
            frames: vec!["▌", "▀", "▐", "▄"],
        },
    );

    // triangle
    m.insert(
        "triangle",
        SpinnerData {
            interval: 50,
            frames: vec!["◢", "◣", "◤", "◥"],
        },
    );

    // point
    m.insert(
        "point",
        SpinnerData {
            interval: 125,
            frames: vec!["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"],
        },
    );

    // layer
    m.insert(
        "layer",
        SpinnerData {
            interval: 150,
            frames: vec!["-", "=", "≡"],
        },
    );

    // aesthetic
    m.insert(
        "aesthetic",
        SpinnerData {
            interval: 80,
            frames: vec![
                "▰▱▱▱▱▱▱",
                "▰▰▱▱▱▱▱",
                "▰▰▰▱▱▱▱",
                "▰▰▰▰▱▱▱",
                "▰▰▰▰▰▱▱",
                "▰▰▰▰▰▰▱",
                "▰▰▰▰▰▰▰",
                "▰▱▱▱▱▱▱",
            ],
        },
    );

    // simple - Most basic spinner
    m.insert(
        "simple",
        SpinnerData {
            interval: 100,
            frames: vec!["/", "-", "\\", "|"],
        },
    );

    m
});

/// An animated spinner for console output.
///
/// # Example
///
/// ```rust,ignore
/// use richrs::spinner::Spinner;
///
/// let spinner = Spinner::new("dots")?;
/// println!("Interval: {}ms", spinner.interval());
/// for frame in spinner.frames().take(5) {
///     println!("{}", frame);
/// }
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Spinner {
    /// The name of the spinner.
    name: String,
    /// The animation interval in milliseconds.
    interval: u64,
    /// The frames of the animation.
    frames: Vec<String>,
    /// Current frame index.
    current: usize,
}

impl Spinner {
    /// Creates a new spinner with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the spinner (e.g., "dots", "line", "arc")
    ///
    /// # Returns
    ///
    /// Returns `Ok(Spinner)` if the name is found, or an error if not.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use richrs::spinner::Spinner;
    ///
    /// let spinner = Spinner::new("dots")?;
    /// ```
    #[inline]
    pub fn new(name: &str) -> Result<Self> {
        SPINNERS.get(name).map_or_else(
            || Err(Error::NoSpinner(name.to_owned())),
            |data| {
                Ok(Self {
                    name: name.to_owned(),
                    interval: data.interval,
                    frames: data.frames.iter().map(|&s| s.to_owned()).collect(),
                    current: 0,
                })
            },
        )
    }

    /// Creates a custom spinner with the given frames and interval.
    ///
    /// # Arguments
    ///
    /// * `frames` - The animation frames
    /// * `interval` - The animation interval in milliseconds
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use richrs::spinner::Spinner;
    ///
    /// let spinner = Spinner::custom(vec![".", "..", "..."], 200);
    /// ```
    #[inline]
    #[must_use]
    pub fn custom(frames: Vec<String>, interval: u64) -> Self {
        Self {
            name: "custom".to_owned(),
            interval,
            frames,
            current: 0,
        }
    }

    /// Returns the name of this spinner.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the animation interval in milliseconds.
    #[inline]
    #[must_use]
    pub fn interval(&self) -> u64 {
        self.interval
    }

    /// Returns the number of frames in the animation.
    #[inline]
    #[must_use]
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Returns the current frame.
    #[inline]
    #[must_use]
    pub fn current_frame(&self) -> &str {
        self.frames
            .get(self.current)
            .map_or("", |frame| frame.as_str())
    }

    /// Advances to the next frame and returns it.
    #[inline]
    pub fn next_frame(&mut self) -> &str {
        if self.frames.is_empty() {
            return "";
        }
        let frame = self.frames.get(self.current).map_or("", |f| f.as_str());
        self.current = self
            .current
            .checked_add(1)
            .map_or(0, |n| n % self.frames.len());
        frame
    }

    /// Resets the spinner to the first frame.
    #[inline]
    pub fn reset(&mut self) {
        self.current = 0;
    }

    /// Returns an iterator over the spinner frames.
    ///
    /// The iterator cycles infinitely through the frames.
    #[inline]
    pub fn frames(&self) -> SpinnerFrames<'_> {
        SpinnerFrames {
            frames: &self.frames,
            current: 0,
        }
    }

    /// Checks if a spinner name exists.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use richrs::spinner::Spinner;
    ///
    /// assert!(Spinner::exists("dots"));
    /// assert!(!Spinner::exists("nonexistent"));
    /// ```
    #[inline]
    #[must_use]
    pub fn exists(name: &str) -> bool {
        SPINNERS.contains_key(name)
    }

    /// Returns the number of built-in spinners.
    #[inline]
    #[must_use]
    pub fn count() -> usize {
        SPINNERS.len()
    }

    /// Returns an iterator over all spinner names.
    #[inline]
    pub fn names() -> impl Iterator<Item = &'static str> {
        SPINNERS.keys().copied()
    }
}

impl fmt::Display for Spinner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.current_frame())
    }
}

/// An iterator that cycles through spinner frames.
#[derive(Debug)]
pub struct SpinnerFrames<'a> {
    frames: &'a [String],
    current: usize,
}

impl<'a> Iterator for SpinnerFrames<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.frames.is_empty() {
            return None;
        }
        let frame = self.frames.get(self.current)?;
        self.current = self
            .current
            .checked_add(1)
            .map_or(0, |n| n % self.frames.len());
        Some(frame.as_str())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_new_valid() {
        let spinner = Spinner::new("dots").unwrap();
        assert_eq!(spinner.name(), "dots");
        assert_eq!(spinner.interval(), 80);
        assert_eq!(spinner.frame_count(), 10);
    }

    #[test]
    fn test_spinner_new_invalid() {
        let result = Spinner::new("nonexistent_spinner");
        assert!(result.is_err());
    }

    #[test]
    fn test_spinner_exists() {
        assert!(Spinner::exists("dots"));
        assert!(Spinner::exists("line"));
        assert!(Spinner::exists("arc"));
        assert!(!Spinner::exists("nonexistent"));
    }

    #[test]
    fn test_spinner_count() {
        // We should have a substantial number of spinners
        assert!(Spinner::count() > 30);
    }

    #[test]
    fn test_spinner_names() {
        let names: Vec<_> = Spinner::names().collect();
        assert!(names.contains(&"dots"));
        assert!(names.contains(&"line"));
        assert!(names.contains(&"arc"));
    }

    #[test]
    fn test_spinner_current_frame() {
        let spinner = Spinner::new("dots").unwrap();
        let frame = spinner.current_frame();
        assert!(!frame.is_empty());
    }

    #[test]
    fn test_spinner_next_frame() {
        let mut spinner = Spinner::new("line").unwrap();
        assert_eq!(spinner.next_frame(), "-");
        assert_eq!(spinner.next_frame(), "\\");
        assert_eq!(spinner.next_frame(), "|");
        assert_eq!(spinner.next_frame(), "/");
        // Should cycle back
        assert_eq!(spinner.next_frame(), "-");
    }

    #[test]
    fn test_spinner_reset() {
        let mut spinner = Spinner::new("line").unwrap();
        spinner.next_frame();
        spinner.next_frame();
        spinner.reset();
        assert_eq!(spinner.next_frame(), "-");
    }

    #[test]
    fn test_spinner_frames_iterator() {
        let spinner = Spinner::new("line").unwrap();
        let frames: Vec<_> = spinner.frames().take(8).collect();
        assert_eq!(frames.len(), 8);
        assert_eq!(frames[0], "-");
        assert_eq!(frames[1], "\\");
        assert_eq!(frames[4], "-"); // Cycles back
    }

    #[test]
    fn test_spinner_custom() {
        let spinner = Spinner::custom(vec![".".to_owned(), "..".to_owned(), "...".to_owned()], 200);
        assert_eq!(spinner.name(), "custom");
        assert_eq!(spinner.interval(), 200);
        assert_eq!(spinner.frame_count(), 3);
    }

    #[test]
    fn test_spinner_display() {
        let spinner = Spinner::new("dots").unwrap();
        let display = format!("{spinner}");
        assert!(!display.is_empty());
    }

    #[test]
    fn test_common_spinners() {
        // Test that common spinners exist and have valid data
        let test_spinners = [
            "dots",
            "line",
            "arc",
            "moon",
            "clock",
            "earth",
            "bouncingBar",
            "arrow",
            "toggle",
        ];

        for name in test_spinners {
            let spinner = Spinner::new(name).unwrap();
            assert!(spinner.frame_count() > 0, "{name} should have frames");
            assert!(spinner.interval() > 0, "{name} should have interval");
        }
    }

    #[test]
    fn test_spinner_clone() {
        let spinner = Spinner::new("dots").unwrap();
        let cloned = spinner.clone();
        assert_eq!(spinner.name(), cloned.name());
        assert_eq!(spinner.interval(), cloned.interval());
    }
}
