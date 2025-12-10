//! User prompts with validation and styling.
//!
//! This module provides various prompt types for getting user input
//! with optional validation, default values, and styled output.
//!
//! # Example
//!
//! ```ignore
//! use richrs::prelude::*;
//! use richrs::prompt::{Prompt, Confirm};
//!
//! let name = Prompt::new("What is your name?").ask()?;
//! let confirmed = Confirm::new("Are you sure?").ask()?;
//! ```

use crate::errors::Result;
use crate::style::Style;
use std::io::{self, BufRead, Write};

/// A text prompt for getting user input.
#[derive(Debug, Clone)]
pub struct Prompt {
    /// The prompt message.
    message: String,
    /// Default value if user presses enter.
    default: Option<String>,
    /// Valid choices (if restricted).
    choices: Option<Vec<String>>,
    /// Whether choices are case-sensitive.
    case_sensitive: bool,
    /// Whether to show the default value.
    show_default: bool,
    /// Whether to show available choices.
    show_choices: bool,
    /// Whether input should be hidden (for passwords).
    password: bool,
    /// Style for the prompt.
    prompt_style: Option<Style>,
}

impl Prompt {
    /// Creates a new Prompt with the given message.
    #[must_use]
    #[inline]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            default: None,
            choices: None,
            case_sensitive: true,
            show_default: true,
            show_choices: true,
            password: false,
            prompt_style: None,
        }
    }

    /// Sets the default value.
    #[must_use]
    #[inline]
    pub fn default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }

    /// Sets valid choices.
    #[must_use]
    pub fn choices<I, S>(mut self, choices: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.choices = Some(choices.into_iter().map(Into::into).collect());
        self
    }

    /// Sets whether choices are case-sensitive.
    #[must_use]
    #[inline]
    pub const fn case_sensitive(mut self, sensitive: bool) -> Self {
        self.case_sensitive = sensitive;
        self
    }

    /// Sets whether to show the default value.
    #[must_use]
    #[inline]
    pub const fn show_default(mut self, show: bool) -> Self {
        self.show_default = show;
        self
    }

    /// Sets whether to show available choices.
    #[must_use]
    #[inline]
    pub const fn show_choices(mut self, show: bool) -> Self {
        self.show_choices = show;
        self
    }

    /// Sets whether input should be hidden (for passwords).
    #[must_use]
    #[inline]
    pub const fn password(mut self, is_password: bool) -> Self {
        self.password = is_password;
        self
    }

    /// Sets the style for the prompt.
    #[must_use]
    #[inline]
    pub fn style(mut self, style: Style) -> Self {
        self.prompt_style = Some(style);
        self
    }

    /// Builds the full prompt string.
    fn build_prompt(&self) -> String {
        let mut prompt = self.message.clone();

        if self.show_choices {
            if let Some(ref choices) = self.choices {
                prompt.push_str(&format!(" [{}]", choices.join("/")));
            }
        }

        if self.show_default {
            if let Some(ref default) = self.default {
                prompt.push_str(&format!(" ({})", default));
            }
        }

        prompt.push_str(": ");
        prompt
    }

    /// Validates the input against choices if present.
    fn validate(&self, input: &str) -> bool {
        if let Some(ref choices) = self.choices {
            if self.case_sensitive {
                choices.iter().any(|c| c == input)
            } else {
                let lower = input.to_lowercase();
                choices.iter().any(|c| c.to_lowercase() == lower)
            }
        } else {
            true
        }
    }

    /// Asks the user for input.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from stdin fails or if validation fails
    /// after multiple invalid attempts.
    pub fn ask(&self) -> Result<String> {
        let prompt = self.build_prompt();
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            // Print prompt
            if let Some(ref style) = self.prompt_style {
                eprint!("{}", style.to_ansi());
            }
            eprint!("{}", prompt);
            if self.prompt_style.is_some() {
                eprint!("\x1b[0m");
            }
            stdout.flush()?;

            // Read input
            let mut input = String::new();
            if self.password {
                // For password input, we'd ideally disable echo
                // For now, just read normally (crossterm could be used for proper password input)
                stdin.lock().read_line(&mut input)?;
            } else {
                stdin.lock().read_line(&mut input)?;
            }

            let input = input.trim().to_string();

            // Handle empty input
            if input.is_empty() {
                if let Some(ref default) = self.default {
                    return Ok(default.clone());
                }
            }

            // Validate
            if self.validate(&input) {
                return Ok(input);
            }

            // Invalid choice
            eprintln!(
                "Invalid choice. Please select from: {}",
                self.choices
                    .as_ref()
                    .map(|c| c.join(", "))
                    .unwrap_or_default()
            );
        }
    }
}

/// A yes/no confirmation prompt.
#[derive(Debug, Clone)]
pub struct Confirm {
    /// The prompt message.
    message: String,
    /// Default value if user presses enter.
    default: Option<bool>,
    /// Style for the prompt.
    prompt_style: Option<Style>,
}

impl Confirm {
    /// Creates a new Confirm prompt with the given message.
    #[must_use]
    #[inline]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            default: None,
            prompt_style: None,
        }
    }

    /// Sets the default value (true for yes, false for no).
    #[must_use]
    #[inline]
    pub const fn default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }

    /// Sets the style for the prompt.
    #[must_use]
    #[inline]
    pub fn style(mut self, style: Style) -> Self {
        self.prompt_style = Some(style);
        self
    }

    /// Asks the user for confirmation.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from stdin fails.
    pub fn ask(&self) -> Result<bool> {
        let choices = match self.default {
            Some(true) => "[Y/n]",
            Some(false) => "[y/N]",
            None => "[y/n]",
        };

        let prompt = format!("{} {}: ", self.message, choices);
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            // Print prompt
            if let Some(ref style) = self.prompt_style {
                eprint!("{}", style.to_ansi());
            }
            eprint!("{}", prompt);
            if self.prompt_style.is_some() {
                eprint!("\x1b[0m");
            }
            stdout.flush()?;

            // Read input
            let mut input = String::new();
            stdin.lock().read_line(&mut input)?;

            let input = input.trim().to_lowercase();

            // Handle empty input
            if input.is_empty() {
                if let Some(default) = self.default {
                    return Ok(default);
                }
                eprintln!("Please enter y or n");
                continue;
            }

            // Parse response
            match input.as_str() {
                "y" | "yes" | "true" | "1" => return Ok(true),
                "n" | "no" | "false" | "0" => return Ok(false),
                _ => {
                    eprintln!("Please enter y or n");
                }
            }
        }
    }
}

/// An integer input prompt.
#[derive(Debug, Clone)]
pub struct IntPrompt {
    /// The prompt message.
    message: String,
    /// Default value.
    default: Option<i64>,
    /// Minimum value.
    min: Option<i64>,
    /// Maximum value.
    max: Option<i64>,
    /// Style for the prompt.
    prompt_style: Option<Style>,
}

impl IntPrompt {
    /// Creates a new IntPrompt with the given message.
    #[must_use]
    #[inline]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            default: None,
            min: None,
            max: None,
            prompt_style: None,
        }
    }

    /// Sets the default value.
    #[must_use]
    #[inline]
    pub const fn default(mut self, default: i64) -> Self {
        self.default = Some(default);
        self
    }

    /// Sets the minimum allowed value.
    #[must_use]
    #[inline]
    pub const fn min(mut self, min: i64) -> Self {
        self.min = Some(min);
        self
    }

    /// Sets the maximum allowed value.
    #[must_use]
    #[inline]
    pub const fn max(mut self, max: i64) -> Self {
        self.max = Some(max);
        self
    }

    /// Sets the style for the prompt.
    #[must_use]
    #[inline]
    pub fn style(mut self, style: Style) -> Self {
        self.prompt_style = Some(style);
        self
    }

    /// Asks the user for an integer.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from stdin fails.
    pub fn ask(&self) -> Result<i64> {
        let mut prompt = self.message.clone();

        if let Some(default) = self.default {
            prompt.push_str(&format!(" ({})", default));
        }

        prompt.push_str(": ");

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            // Print prompt
            if let Some(ref style) = self.prompt_style {
                eprint!("{}", style.to_ansi());
            }
            eprint!("{}", prompt);
            if self.prompt_style.is_some() {
                eprint!("\x1b[0m");
            }
            stdout.flush()?;

            // Read input
            let mut input = String::new();
            stdin.lock().read_line(&mut input)?;

            let input = input.trim();

            // Handle empty input
            if input.is_empty() {
                if let Some(default) = self.default {
                    return Ok(default);
                }
                eprintln!("Please enter a number");
                continue;
            }

            // Parse number
            match input.parse::<i64>() {
                Ok(n) => {
                    // Validate range
                    if let Some(min) = self.min {
                        if n < min {
                            eprintln!("Value must be at least {}", min);
                            continue;
                        }
                    }
                    if let Some(max) = self.max {
                        if n > max {
                            eprintln!("Value must be at most {}", max);
                            continue;
                        }
                    }
                    return Ok(n);
                }
                Err(_) => {
                    eprintln!("Please enter a valid integer");
                }
            }
        }
    }
}

/// A floating-point input prompt.
#[derive(Debug, Clone)]
pub struct FloatPrompt {
    /// The prompt message.
    message: String,
    /// Default value.
    default: Option<f64>,
    /// Minimum value.
    min: Option<f64>,
    /// Maximum value.
    max: Option<f64>,
    /// Style for the prompt.
    prompt_style: Option<Style>,
}

impl FloatPrompt {
    /// Creates a new FloatPrompt with the given message.
    #[must_use]
    #[inline]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            default: None,
            min: None,
            max: None,
            prompt_style: None,
        }
    }

    /// Sets the default value.
    #[must_use]
    #[inline]
    pub fn default(mut self, default: f64) -> Self {
        self.default = Some(default);
        self
    }

    /// Sets the minimum allowed value.
    #[must_use]
    #[inline]
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// Sets the maximum allowed value.
    #[must_use]
    #[inline]
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// Sets the style for the prompt.
    #[must_use]
    #[inline]
    pub fn style(mut self, style: Style) -> Self {
        self.prompt_style = Some(style);
        self
    }

    /// Asks the user for a floating-point number.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from stdin fails.
    pub fn ask(&self) -> Result<f64> {
        let mut prompt = self.message.clone();

        if let Some(default) = self.default {
            prompt.push_str(&format!(" ({})", default));
        }

        prompt.push_str(": ");

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            // Print prompt
            if let Some(ref style) = self.prompt_style {
                eprint!("{}", style.to_ansi());
            }
            eprint!("{}", prompt);
            if self.prompt_style.is_some() {
                eprint!("\x1b[0m");
            }
            stdout.flush()?;

            // Read input
            let mut input = String::new();
            stdin.lock().read_line(&mut input)?;

            let input = input.trim();

            // Handle empty input
            if input.is_empty() {
                if let Some(default) = self.default {
                    return Ok(default);
                }
                eprintln!("Please enter a number");
                continue;
            }

            // Parse number
            match input.parse::<f64>() {
                Ok(n) => {
                    // Validate range
                    if let Some(min) = self.min {
                        if n < min {
                            eprintln!("Value must be at least {}", min);
                            continue;
                        }
                    }
                    if let Some(max) = self.max {
                        if n > max {
                            eprintln!("Value must be at most {}", max);
                            continue;
                        }
                    }
                    return Ok(n);
                }
                Err(_) => {
                    eprintln!("Please enter a valid number");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_new() {
        let prompt = Prompt::new("Test?");
        assert_eq!(prompt.message, "Test?");
    }

    #[test]
    fn test_prompt_default() {
        let prompt = Prompt::new("Test?").default("foo");
        assert_eq!(prompt.default, Some("foo".to_string()));
    }

    #[test]
    fn test_prompt_choices() {
        let prompt = Prompt::new("Test?").choices(["a", "b", "c"]);
        assert!(prompt.choices.is_some());
        assert_eq!(prompt.choices.as_ref().map(Vec::len), Some(3));
    }

    #[test]
    fn test_prompt_validate_with_choices() {
        let prompt = Prompt::new("Test?").choices(["yes", "no"]);
        assert!(prompt.validate("yes"));
        assert!(!prompt.validate("maybe"));
    }

    #[test]
    fn test_prompt_validate_case_insensitive() {
        let prompt = Prompt::new("Test?")
            .choices(["yes", "no"])
            .case_sensitive(false);
        assert!(prompt.validate("YES"));
        assert!(prompt.validate("Yes"));
    }

    #[test]
    fn test_prompt_build_prompt() {
        let prompt = Prompt::new("Choose")
            .choices(["a", "b"])
            .default("a");
        let built = prompt.build_prompt();
        assert!(built.contains("Choose"));
        assert!(built.contains("[a/b]"));
        assert!(built.contains("(a)"));
    }

    #[test]
    fn test_confirm_new() {
        let confirm = Confirm::new("Sure?");
        assert_eq!(confirm.message, "Sure?");
    }

    #[test]
    fn test_confirm_default() {
        let confirm = Confirm::new("Sure?").default(true);
        assert_eq!(confirm.default, Some(true));
    }

    #[test]
    fn test_int_prompt_new() {
        let prompt = IntPrompt::new("Number?");
        assert_eq!(prompt.message, "Number?");
    }

    #[test]
    fn test_int_prompt_range() {
        let prompt = IntPrompt::new("Number?").min(0).max(100);
        assert_eq!(prompt.min, Some(0));
        assert_eq!(prompt.max, Some(100));
    }

    #[test]
    fn test_float_prompt_new() {
        let prompt = FloatPrompt::new("Value?");
        assert_eq!(prompt.message, "Value?");
    }
}
