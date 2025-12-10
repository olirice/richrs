//! Theme support for named styles.
//!
//! Themes provide a way to define named styles that can be referenced
//! in markup and throughout the application.

use crate::errors::{Error, Result};
use crate::style::Style;
use std::collections::HashMap;

/// A collection of named styles.
#[derive(Debug, Clone, Default)]
pub struct Theme {
    /// Named styles.
    styles: HashMap<String, Style>,
    /// Whether to inherit default styles.
    inherit_defaults: bool,
}

impl Theme {
    /// Creates a new empty theme.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            styles: HashMap::new(),
            inherit_defaults: true,
        }
    }

    /// Creates a theme from a map of styles.
    #[must_use]
    pub fn from_styles(styles: HashMap<String, Style>) -> Self {
        Self {
            styles,
            inherit_defaults: true,
        }
    }

    /// Sets whether to inherit default styles.
    #[inline]
    #[must_use]
    pub const fn inherit_defaults(mut self, inherit: bool) -> Self {
        self.inherit_defaults = inherit;
        self
    }

    /// Adds a style to the theme.
    ///
    /// # Errors
    ///
    /// Returns an error if the style name is invalid.
    pub fn add_style(&mut self, name: impl Into<String>, style: Style) -> Result<()> {
        let name = name.into();
        Self::validate_name(&name)?;
        self.styles.insert(name, style);
        Ok(())
    }

    /// Gets a style by name.
    #[must_use]
    pub fn get_style(&self, name: &str) -> Option<&Style> {
        self.styles.get(name).or_else(|| {
            if self.inherit_defaults {
                Self::default_style(name)
            } else {
                None
            }
        })
    }

    /// Returns an iterator over all styles.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Style)> {
        self.styles.iter()
    }

    /// Validates a style name.
    fn validate_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(Error::ThemeError {
                message: "style name cannot be empty".to_owned(),
            });
        }

        let first_char = name.chars().next().ok_or_else(|| Error::ThemeError {
            message: "style name cannot be empty".to_owned(),
        })?;

        if !first_char.is_ascii_lowercase() {
            return Err(Error::ThemeError {
                message: "style name must start with a lowercase letter".to_owned(),
            });
        }

        for ch in name.chars() {
            if !ch.is_ascii_lowercase()
                && !ch.is_ascii_digit()
                && ch != '.'
                && ch != '-'
                && ch != '_'
            {
                return Err(Error::ThemeError {
                    message: format!(
                        "style name can only contain lowercase letters, numbers, '.', '-', '_': got '{ch}'"
                    ),
                });
            }
        }

        Ok(())
    }

    /// Returns a default style by name.
    fn default_style(name: &str) -> Option<&'static Style> {
        // Built-in default styles (similar to Rich)
        static INFO_STYLE: Style = Style::new();
        static WARNING_STYLE: Style = Style::new();
        static ERROR_STYLE: Style = Style::new();
        static SUCCESS_STYLE: Style = Style::new();

        match name {
            "info" => Some(&INFO_STYLE),
            "warning" => Some(&WARNING_STYLE),
            "error" => Some(&ERROR_STYLE),
            "success" => Some(&SUCCESS_STYLE),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_new() {
        let theme = Theme::new();
        assert!(theme.styles.is_empty());
        assert!(theme.inherit_defaults);
    }

    #[test]
    fn test_theme_add_style() {
        let mut theme = Theme::new();
        let style = Style::new().bold();
        theme.add_style("my-style", style.clone()).ok();

        assert_eq!(theme.get_style("my-style"), Some(&style));
    }

    #[test]
    fn test_theme_invalid_name() {
        let mut theme = Theme::new();
        assert!(theme.add_style("", Style::new()).is_err());
        assert!(theme.add_style("Invalid", Style::new()).is_err());
        assert!(theme.add_style("has space", Style::new()).is_err());
    }

    #[test]
    fn test_theme_valid_names() {
        let mut theme = Theme::new();
        assert!(theme.add_style("valid", Style::new()).is_ok());
        assert!(theme.add_style("valid-name", Style::new()).is_ok());
        assert!(theme.add_style("valid_name", Style::new()).is_ok());
        assert!(theme.add_style("valid.name", Style::new()).is_ok());
        assert!(theme.add_style("valid123", Style::new()).is_ok());
    }

    #[test]
    fn test_theme_default_styles() {
        let theme = Theme::new();
        assert!(theme.get_style("info").is_some());
        assert!(theme.get_style("warning").is_some());
        assert!(theme.get_style("error").is_some());
    }

    #[test]
    fn test_theme_no_inherit() {
        let theme = Theme::new().inherit_defaults(false);
        assert!(theme.get_style("info").is_none());
    }

    #[test]
    fn test_theme_from_styles() {
        let mut styles = HashMap::new();
        styles.insert("custom".to_owned(), Style::new().italic());

        let theme = Theme::from_styles(styles);
        assert!(theme.get_style("custom").is_some());
    }

    #[test]
    fn test_theme_iter() {
        let mut theme = Theme::new();
        theme.add_style("a", Style::new()).ok();
        theme.add_style("b", Style::new()).ok();

        let names: Vec<_> = theme.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"a"));
        assert!(names.contains(&"b"));
    }
}
