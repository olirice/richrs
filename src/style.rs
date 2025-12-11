//! Style definitions for terminal text formatting.
//!
//! This module provides a comprehensive style system supporting:
//! - Foreground and background colors
//! - Text attributes (bold, italic, underline, etc.)
//! - Hyperlinks
//! - Style parsing from strings
//! - Style combination and inheritance

use crate::color::Color;
use crate::errors::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Text attributes that can be applied to styled text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Attributes {
    /// Bold text.
    pub bold: Option<bool>,
    /// Dim/faint text.
    pub dim: Option<bool>,
    /// Italic text.
    pub italic: Option<bool>,
    /// Underlined text.
    pub underline: Option<bool>,
    /// Double underlined text.
    pub underline2: Option<bool>,
    /// Blinking text.
    pub blink: Option<bool>,
    /// Rapid blinking text.
    pub blink2: Option<bool>,
    /// Reversed foreground/background.
    pub reverse: Option<bool>,
    /// Concealed/hidden text.
    pub conceal: Option<bool>,
    /// Strikethrough text.
    pub strike: Option<bool>,
    /// Framed text.
    pub frame: Option<bool>,
    /// Encircled text.
    pub encircle: Option<bool>,
    /// Overlined text.
    pub overline: Option<bool>,
}

impl Attributes {
    /// Creates new attributes with all values unset.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bold: None,
            dim: None,
            italic: None,
            underline: None,
            underline2: None,
            blink: None,
            blink2: None,
            reverse: None,
            conceal: None,
            strike: None,
            frame: None,
            encircle: None,
            overline: None,
        }
    }

    /// Returns true if all attributes are unset.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bold.is_none()
            && self.dim.is_none()
            && self.italic.is_none()
            && self.underline.is_none()
            && self.underline2.is_none()
            && self.blink.is_none()
            && self.blink2.is_none()
            && self.reverse.is_none()
            && self.conceal.is_none()
            && self.strike.is_none()
            && self.frame.is_none()
            && self.encircle.is_none()
            && self.overline.is_none()
    }

    /// Combines this attributes with another, with `other` taking precedence.
    #[inline]
    #[must_use]
    pub const fn combine(self, other: Self) -> Self {
        Self {
            bold: if other.bold.is_some() {
                other.bold
            } else {
                self.bold
            },
            dim: if other.dim.is_some() {
                other.dim
            } else {
                self.dim
            },
            italic: if other.italic.is_some() {
                other.italic
            } else {
                self.italic
            },
            underline: if other.underline.is_some() {
                other.underline
            } else {
                self.underline
            },
            underline2: if other.underline2.is_some() {
                other.underline2
            } else {
                self.underline2
            },
            blink: if other.blink.is_some() {
                other.blink
            } else {
                self.blink
            },
            blink2: if other.blink2.is_some() {
                other.blink2
            } else {
                self.blink2
            },
            reverse: if other.reverse.is_some() {
                other.reverse
            } else {
                self.reverse
            },
            conceal: if other.conceal.is_some() {
                other.conceal
            } else {
                self.conceal
            },
            strike: if other.strike.is_some() {
                other.strike
            } else {
                self.strike
            },
            frame: if other.frame.is_some() {
                other.frame
            } else {
                self.frame
            },
            encircle: if other.encircle.is_some() {
                other.encircle
            } else {
                self.encircle
            },
            overline: if other.overline.is_some() {
                other.overline
            } else {
                self.overline
            },
        }
    }
}

/// A style that can be applied to text.
///
/// Styles consist of colors and attributes that define how text should be rendered.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Style {
    /// Foreground (text) color.
    pub color: Option<Color>,
    /// Background color.
    pub bgcolor: Option<Color>,
    /// Text attributes.
    pub attributes: Attributes,
    /// Hyperlink URL.
    pub link: Option<String>,
}

impl Style {
    /// Creates a new empty style.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            color: None,
            bgcolor: None,
            attributes: Attributes::new(),
            link: None,
        }
    }

    /// Creates a style with the given foreground color.
    #[inline]
    #[must_use]
    pub const fn color(color: Color) -> Self {
        Self {
            color: Some(color),
            bgcolor: None,
            attributes: Attributes::new(),
            link: None,
        }
    }

    /// Creates a style with the given background color.
    #[inline]
    #[must_use]
    pub const fn bgcolor(color: Color) -> Self {
        Self {
            color: None,
            bgcolor: Some(color),
            attributes: Attributes::new(),
            link: None,
        }
    }

    /// Returns true if this style has no colors, attributes, or links.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.color.is_none()
            && self.bgcolor.is_none()
            && self.attributes.is_empty()
            && self.link.is_none()
    }

    /// Sets the foreground color.
    #[inline]
    #[must_use]
    pub const fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the background color.
    #[inline]
    #[must_use]
    pub const fn with_bgcolor(mut self, color: Color) -> Self {
        self.bgcolor = Some(color);
        self
    }

    /// Sets the bold attribute.
    #[inline]
    #[must_use]
    pub const fn bold(mut self) -> Self {
        self.attributes.bold = Some(true);
        self
    }

    /// Sets the dim attribute.
    #[inline]
    #[must_use]
    pub const fn dim(mut self) -> Self {
        self.attributes.dim = Some(true);
        self
    }

    /// Sets the italic attribute.
    #[inline]
    #[must_use]
    pub const fn italic(mut self) -> Self {
        self.attributes.italic = Some(true);
        self
    }

    /// Sets the underline attribute.
    #[inline]
    #[must_use]
    pub const fn underline(mut self) -> Self {
        self.attributes.underline = Some(true);
        self
    }

    /// Sets the double underline attribute.
    #[inline]
    #[must_use]
    pub const fn underline2(mut self) -> Self {
        self.attributes.underline2 = Some(true);
        self
    }

    /// Sets the blink attribute.
    #[inline]
    #[must_use]
    pub const fn blink(mut self) -> Self {
        self.attributes.blink = Some(true);
        self
    }

    /// Sets the rapid blink attribute.
    #[inline]
    #[must_use]
    pub const fn blink2(mut self) -> Self {
        self.attributes.blink2 = Some(true);
        self
    }

    /// Sets the reverse attribute.
    #[inline]
    #[must_use]
    pub const fn reverse(mut self) -> Self {
        self.attributes.reverse = Some(true);
        self
    }

    /// Sets the conceal attribute.
    #[inline]
    #[must_use]
    pub const fn conceal(mut self) -> Self {
        self.attributes.conceal = Some(true);
        self
    }

    /// Sets the strikethrough attribute.
    #[inline]
    #[must_use]
    pub const fn strike(mut self) -> Self {
        self.attributes.strike = Some(true);
        self
    }

    /// Sets the frame attribute.
    #[inline]
    #[must_use]
    pub const fn frame(mut self) -> Self {
        self.attributes.frame = Some(true);
        self
    }

    /// Sets the encircle attribute.
    #[inline]
    #[must_use]
    pub const fn encircle(mut self) -> Self {
        self.attributes.encircle = Some(true);
        self
    }

    /// Sets the overline attribute.
    #[inline]
    #[must_use]
    pub const fn overline(mut self) -> Self {
        self.attributes.overline = Some(true);
        self
    }

    /// Sets the hyperlink URL.
    #[inline]
    #[must_use]
    pub fn link(mut self, url: String) -> Self {
        self.link = Some(url);
        self
    }

    /// Parses a style from a string.
    ///
    /// # Format
    ///
    /// The style string can contain:
    /// - Color names: "red", "green", "blue", etc.
    /// - Hex colors: "#ff0000"
    /// - RGB colors: "rgb(255, 0, 0)"
    /// - Background colors: "on red", "on #ff0000"
    /// - Attributes: "bold", "italic", "underline", etc.
    /// - Attribute negation: "not bold", "not italic"
    /// - Hyperlinks: `link https://example.com`
    ///
    /// Multiple values can be combined with spaces:
    /// "bold red on white"
    ///
    /// # Errors
    ///
    /// Returns an error if the string contains invalid style components.
    #[allow(clippy::too_many_lines)]
    pub fn parse(s: &str) -> Result<Self> {
        let mut style = Self::new();
        let s = s.trim();

        if s.is_empty() {
            return Ok(style);
        }

        let mut tokens = StyleTokenizer::new(s);
        let mut expecting_bg = false;
        let mut expecting_not = false;
        let mut expecting_link = false;

        while let Some(token) = tokens.next_token()? {
            if expecting_link {
                style.link = Some(token.to_owned());
                expecting_link = false;
                continue;
            }

            let token_lower = token.to_lowercase();

            match token_lower.as_str() {
                "on" => {
                    expecting_bg = true;
                    continue;
                }
                "not" => {
                    expecting_not = true;
                    continue;
                }
                "link" => {
                    expecting_link = true;
                    continue;
                }
                // Attributes
                "bold" | "b" => {
                    style.attributes.bold = Some(!expecting_not);
                    expecting_not = false;
                    continue;
                }
                "dim" | "d" => {
                    style.attributes.dim = Some(!expecting_not);
                    expecting_not = false;
                    continue;
                }
                "italic" | "i" => {
                    style.attributes.italic = Some(!expecting_not);
                    expecting_not = false;
                    continue;
                }
                "underline" | "u" => {
                    style.attributes.underline = Some(!expecting_not);
                    expecting_not = false;
                    continue;
                }
                "underline2" | "uu" => {
                    style.attributes.underline2 = Some(!expecting_not);
                    expecting_not = false;
                    continue;
                }
                "blink" => {
                    style.attributes.blink = Some(!expecting_not);
                    expecting_not = false;
                    continue;
                }
                "blink2" => {
                    style.attributes.blink2 = Some(!expecting_not);
                    expecting_not = false;
                    continue;
                }
                "reverse" | "r" => {
                    style.attributes.reverse = Some(!expecting_not);
                    expecting_not = false;
                    continue;
                }
                "conceal" => {
                    style.attributes.conceal = Some(!expecting_not);
                    expecting_not = false;
                    continue;
                }
                "strike" | "s" => {
                    style.attributes.strike = Some(!expecting_not);
                    expecting_not = false;
                    continue;
                }
                "frame" => {
                    style.attributes.frame = Some(!expecting_not);
                    expecting_not = false;
                    continue;
                }
                "encircle" => {
                    style.attributes.encircle = Some(!expecting_not);
                    expecting_not = false;
                    continue;
                }
                "overline" | "o" => {
                    style.attributes.overline = Some(!expecting_not);
                    expecting_not = false;
                    continue;
                }
                _ => {}
            }

            // If we got here, it should be a color
            if expecting_not {
                return Err(Error::StyleParse {
                    message: format!("cannot negate color: '{token}'"),
                });
            }

            let color = Color::parse(&token_lower)?;
            if expecting_bg {
                style.bgcolor = Some(color);
                expecting_bg = false;
            } else {
                style.color = Some(color);
            }
        }

        if expecting_bg {
            return Err(Error::StyleParse {
                message: "'on' must be followed by a color".to_owned(),
            });
        }

        if expecting_link {
            return Err(Error::StyleParse {
                message: "'link' must be followed by a URL".to_owned(),
            });
        }

        Ok(style)
    }

    /// Combines this style with another, with `other` taking precedence.
    #[inline]
    #[must_use]
    pub fn combine(&self, other: &Self) -> Self {
        Self {
            color: other.color.or(self.color),
            bgcolor: other.bgcolor.or(self.bgcolor),
            attributes: self.attributes.combine(other.attributes),
            link: other.link.clone().or_else(|| self.link.clone()),
        }
    }

    /// Generates the ANSI escape sequence to apply this style.
    #[must_use]
    pub fn to_ansi(&self) -> String {
        let mut codes = Vec::new();

        // Foreground color
        if let Some(ref color) = self.color {
            codes.push(color.to_ansi_fg());
        }

        // Background color
        if let Some(ref bgcolor) = self.bgcolor {
            codes.push(bgcolor.to_ansi_bg());
        }

        // Attributes
        if self.attributes.bold == Some(true) {
            codes.push("\x1b[1m".to_owned());
        }
        if self.attributes.dim == Some(true) {
            codes.push("\x1b[2m".to_owned());
        }
        if self.attributes.italic == Some(true) {
            codes.push("\x1b[3m".to_owned());
        }
        if self.attributes.underline == Some(true) {
            codes.push("\x1b[4m".to_owned());
        }
        if self.attributes.blink == Some(true) {
            codes.push("\x1b[5m".to_owned());
        }
        if self.attributes.blink2 == Some(true) {
            codes.push("\x1b[6m".to_owned());
        }
        if self.attributes.reverse == Some(true) {
            codes.push("\x1b[7m".to_owned());
        }
        if self.attributes.conceal == Some(true) {
            codes.push("\x1b[8m".to_owned());
        }
        if self.attributes.strike == Some(true) {
            codes.push("\x1b[9m".to_owned());
        }
        if self.attributes.underline2 == Some(true) {
            codes.push("\x1b[21m".to_owned());
        }
        if self.attributes.frame == Some(true) {
            codes.push("\x1b[51m".to_owned());
        }
        if self.attributes.encircle == Some(true) {
            codes.push("\x1b[52m".to_owned());
        }
        if self.attributes.overline == Some(true) {
            codes.push("\x1b[53m".to_owned());
        }

        codes.join("")
    }

    /// Generates the ANSI escape sequence to reset this style.
    ///
    /// Uses a full reset (`\x1b[0m`) for maximum terminal compatibility,
    /// especially with RGB/TrueColor sequences which some terminals
    /// don't handle well with selective resets.
    #[must_use]
    pub fn to_ansi_reset(&self) -> String {
        // Use full reset for better terminal compatibility
        "\x1b[0m".to_owned()
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if self.attributes.bold == Some(true) {
            parts.push("bold".to_owned());
        }
        if self.attributes.dim == Some(true) {
            parts.push("dim".to_owned());
        }
        if self.attributes.italic == Some(true) {
            parts.push("italic".to_owned());
        }
        if self.attributes.underline == Some(true) {
            parts.push("underline".to_owned());
        }
        if self.attributes.underline2 == Some(true) {
            parts.push("underline2".to_owned());
        }
        if self.attributes.blink == Some(true) {
            parts.push("blink".to_owned());
        }
        if self.attributes.blink2 == Some(true) {
            parts.push("blink2".to_owned());
        }
        if self.attributes.reverse == Some(true) {
            parts.push("reverse".to_owned());
        }
        if self.attributes.conceal == Some(true) {
            parts.push("conceal".to_owned());
        }
        if self.attributes.strike == Some(true) {
            parts.push("strike".to_owned());
        }
        if self.attributes.frame == Some(true) {
            parts.push("frame".to_owned());
        }
        if self.attributes.encircle == Some(true) {
            parts.push("encircle".to_owned());
        }
        if self.attributes.overline == Some(true) {
            parts.push("overline".to_owned());
        }

        if let Some(ref color) = self.color {
            parts.push(color.to_string());
        }

        if let Some(ref bgcolor) = self.bgcolor {
            parts.push(format!("on {bgcolor}"));
        }

        if let Some(ref link) = self.link {
            parts.push(format!("link {link}"));
        }

        write!(f, "{}", parts.join(" "))
    }
}

impl std::str::FromStr for Style {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self> {
        Self::parse(s)
    }
}

impl std::ops::Add for Style {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        self.combine(&other)
    }
}

impl std::ops::Add<&Style> for Style {
    type Output = Self;

    #[inline]
    fn add(self, other: &Self) -> Self {
        self.combine(other)
    }
}

/// A simple tokenizer for style strings.
struct StyleTokenizer<'a> {
    /// The remaining input.
    input: &'a str,
}

impl<'a> StyleTokenizer<'a> {
    /// Creates a new tokenizer.
    #[inline]
    const fn new(input: &'a str) -> Self {
        Self { input }
    }

    /// Returns the next token.
    fn next_token(&mut self) -> Result<Option<&'a str>> {
        // Skip whitespace
        self.input = self.input.trim_start();

        if self.input.is_empty() {
            return Ok(None);
        }

        // Handle special cases for parenthesized expressions
        if self.input.starts_with("rgb(") || self.input.starts_with("color(") {
            let end = self.input.find(')').ok_or_else(|| Error::StyleParse {
                message: "unclosed parenthesis".to_owned(),
            })?;
            let token_end = end.saturating_add(1);
            let (token, rest) = self.input.split_at(token_end);
            self.input = rest;
            return Ok(Some(token));
        }

        // Find the end of the current token
        let end = self
            .input
            .find(char::is_whitespace)
            .unwrap_or(self.input.len());
        let (token, rest) = self.input.split_at(end);
        self.input = rest;

        Ok(Some(token))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    use crate::color::StandardColor;

    #[test]
    fn test_parse_empty() {
        let style = Style::parse("").unwrap();
        assert!(style.is_empty());
    }

    #[test]
    fn test_parse_whitespace() {
        let style = Style::parse("   ").unwrap();
        assert!(style.is_empty());
    }

    #[test]
    fn test_parse_color() {
        let style = Style::parse("red").unwrap();
        assert_eq!(style.color, Some(Color::Standard(StandardColor::Red)));
    }

    #[test]
    fn test_parse_bgcolor() {
        let style = Style::parse("on blue").unwrap();
        assert_eq!(style.bgcolor, Some(Color::Standard(StandardColor::Blue)));
    }

    #[test]
    fn test_parse_color_and_bgcolor() {
        let style = Style::parse("red on blue").unwrap();
        assert_eq!(style.color, Some(Color::Standard(StandardColor::Red)));
        assert_eq!(style.bgcolor, Some(Color::Standard(StandardColor::Blue)));
    }

    #[test]
    fn test_parse_bold() {
        let style = Style::parse("bold").unwrap();
        assert_eq!(style.attributes.bold, Some(true));
    }

    #[test]
    fn test_parse_not_bold() {
        let style = Style::parse("not bold").unwrap();
        assert_eq!(style.attributes.bold, Some(false));
    }

    #[test]
    fn test_parse_not_dim() {
        let style = Style::parse("not dim").unwrap();
        assert_eq!(style.attributes.dim, Some(false));
    }

    #[test]
    fn test_parse_not_italic() {
        let style = Style::parse("not italic").unwrap();
        assert_eq!(style.attributes.italic, Some(false));
    }

    #[test]
    fn test_parse_not_underline() {
        let style = Style::parse("not underline").unwrap();
        assert_eq!(style.attributes.underline, Some(false));
    }

    #[test]
    fn test_parse_not_blink() {
        let style = Style::parse("not blink").unwrap();
        assert_eq!(style.attributes.blink, Some(false));
    }

    #[test]
    fn test_parse_not_reverse() {
        let style = Style::parse("not reverse").unwrap();
        assert_eq!(style.attributes.reverse, Some(false));
    }

    #[test]
    fn test_parse_not_conceal() {
        let style = Style::parse("not conceal").unwrap();
        assert_eq!(style.attributes.conceal, Some(false));
    }

    #[test]
    fn test_parse_not_strike() {
        let style = Style::parse("not strike").unwrap();
        assert_eq!(style.attributes.strike, Some(false));
    }

    #[test]
    fn test_parse_not_frame() {
        let style = Style::parse("not frame").unwrap();
        assert_eq!(style.attributes.frame, Some(false));
    }

    #[test]
    fn test_parse_not_encircle() {
        let style = Style::parse("not encircle").unwrap();
        assert_eq!(style.attributes.encircle, Some(false));
    }

    #[test]
    fn test_parse_not_overline() {
        let style = Style::parse("not overline").unwrap();
        assert_eq!(style.attributes.overline, Some(false));
    }

    #[test]
    fn test_parse_not_underline2() {
        let style = Style::parse("not underline2").unwrap();
        assert_eq!(style.attributes.underline2, Some(false));
    }

    #[test]
    fn test_parse_not_blink2() {
        let style = Style::parse("not blink2").unwrap();
        assert_eq!(style.attributes.blink2, Some(false));
    }

    #[test]
    fn test_parse_multiple_attributes() {
        let style = Style::parse("bold italic underline").unwrap();
        assert_eq!(style.attributes.bold, Some(true));
        assert_eq!(style.attributes.italic, Some(true));
        assert_eq!(style.attributes.underline, Some(true));
    }

    #[test]
    fn test_parse_all_attributes() {
        let style = Style::parse("bold dim italic underline underline2 blink blink2 reverse conceal strike frame encircle overline").unwrap();
        assert_eq!(style.attributes.bold, Some(true));
        assert_eq!(style.attributes.dim, Some(true));
        assert_eq!(style.attributes.italic, Some(true));
        assert_eq!(style.attributes.underline, Some(true));
        assert_eq!(style.attributes.underline2, Some(true));
        assert_eq!(style.attributes.blink, Some(true));
        assert_eq!(style.attributes.blink2, Some(true));
        assert_eq!(style.attributes.reverse, Some(true));
        assert_eq!(style.attributes.conceal, Some(true));
        assert_eq!(style.attributes.strike, Some(true));
        assert_eq!(style.attributes.frame, Some(true));
        assert_eq!(style.attributes.encircle, Some(true));
        assert_eq!(style.attributes.overline, Some(true));
    }

    #[test]
    fn test_parse_complex() {
        let style = Style::parse("bold red on white").unwrap();
        assert_eq!(style.attributes.bold, Some(true));
        assert_eq!(style.color, Some(Color::Standard(StandardColor::Red)));
        assert_eq!(style.bgcolor, Some(Color::Standard(StandardColor::White)));
    }

    #[test]
    fn test_parse_hex_color() {
        let style = Style::parse("#ff0000 on #00ff00").unwrap();
        assert_eq!(style.color, Some(Color::Rgb { r: 255, g: 0, b: 0 }));
        assert_eq!(style.bgcolor, Some(Color::Rgb { r: 0, g: 255, b: 0 }));
    }

    #[test]
    fn test_parse_rgb_color() {
        let style = Style::parse("rgb(255, 128, 0)").unwrap();
        assert_eq!(
            style.color,
            Some(Color::Rgb {
                r: 255,
                g: 128,
                b: 0
            })
        );
    }

    #[test]
    fn test_parse_rgb_bgcolor() {
        let style = Style::parse("on rgb(100, 150, 200)").unwrap();
        assert_eq!(
            style.bgcolor,
            Some(Color::Rgb {
                r: 100,
                g: 150,
                b: 200
            })
        );
    }

    #[test]
    fn test_parse_link() {
        let style = Style::parse("link https://example.com").unwrap();
        assert_eq!(style.link, Some("https://example.com".to_owned()));
    }

    #[test]
    fn test_parse_link_with_style() {
        let style = Style::parse("bold blue link https://example.com").unwrap();
        assert_eq!(style.attributes.bold, Some(true));
        assert_eq!(style.color, Some(Color::Standard(StandardColor::Blue)));
        assert_eq!(style.link, Some("https://example.com".to_owned()));
    }

    #[test]
    fn test_parse_shortcuts() {
        let style = Style::parse("b i u s").unwrap();
        assert_eq!(style.attributes.bold, Some(true));
        assert_eq!(style.attributes.italic, Some(true));
        assert_eq!(style.attributes.underline, Some(true));
        assert_eq!(style.attributes.strike, Some(true));
    }

    #[test]
    fn test_parse_more_shortcuts() {
        let style = Style::parse("d uu r o").unwrap();
        assert_eq!(style.attributes.dim, Some(true));
        assert_eq!(style.attributes.underline2, Some(true));
        assert_eq!(style.attributes.reverse, Some(true));
        assert_eq!(style.attributes.overline, Some(true));
    }

    #[test]
    fn test_combine_styles() {
        let base = Style::parse("bold red").unwrap();
        let overlay = Style::parse("italic on white").unwrap();
        let combined = base.combine(&overlay);

        assert_eq!(combined.attributes.bold, Some(true));
        assert_eq!(combined.attributes.italic, Some(true));
        assert_eq!(combined.color, Some(Color::Standard(StandardColor::Red)));
        assert_eq!(
            combined.bgcolor,
            Some(Color::Standard(StandardColor::White))
        );
    }

    #[test]
    fn test_combine_styles_with_link() {
        let base = Style::new().link("https://base.com".to_owned());
        let overlay = Style::new().link("https://overlay.com".to_owned());
        let combined = base.combine(&overlay);
        assert_eq!(combined.link, Some("https://overlay.com".to_owned()));
    }

    #[test]
    fn test_combine_styles_base_link_wins() {
        let base = Style::new().link("https://base.com".to_owned());
        let overlay = Style::new().bold();
        let combined = base.combine(&overlay);
        assert_eq!(combined.link, Some("https://base.com".to_owned()));
        assert_eq!(combined.attributes.bold, Some(true));
    }

    #[test]
    fn test_style_add() {
        let base = Style::new().bold();
        let overlay = Style::new().italic();
        let combined = base + overlay;

        assert_eq!(combined.attributes.bold, Some(true));
        assert_eq!(combined.attributes.italic, Some(true));
    }

    #[test]
    fn test_style_add_ref() {
        let base = Style::new().bold();
        let overlay = Style::new().italic();
        let combined = base + &overlay;

        assert_eq!(combined.attributes.bold, Some(true));
        assert_eq!(combined.attributes.italic, Some(true));
    }

    #[test]
    fn test_to_ansi() {
        let style = Style::parse("bold red").unwrap();
        let ansi = style.to_ansi();
        assert!(ansi.contains("\x1b[31m")); // red foreground
        assert!(ansi.contains("\x1b[1m")); // bold
    }

    #[test]
    fn test_to_ansi_bgcolor() {
        let style = Style::parse("on blue").unwrap();
        let ansi = style.to_ansi();
        assert!(ansi.contains("\x1b[44m")); // blue background
    }

    #[test]
    fn test_to_ansi_all_attributes() {
        let style = Style::new()
            .bold()
            .dim()
            .italic()
            .underline()
            .blink()
            .blink2()
            .reverse()
            .conceal()
            .strike()
            .underline2()
            .frame()
            .encircle()
            .overline();
        let ansi = style.to_ansi();
        assert!(ansi.contains("\x1b[1m")); // bold
        assert!(ansi.contains("\x1b[2m")); // dim
        assert!(ansi.contains("\x1b[3m")); // italic
        assert!(ansi.contains("\x1b[4m")); // underline
        assert!(ansi.contains("\x1b[5m")); // blink
        assert!(ansi.contains("\x1b[6m")); // blink2
        assert!(ansi.contains("\x1b[7m")); // reverse
        assert!(ansi.contains("\x1b[8m")); // conceal
        assert!(ansi.contains("\x1b[9m")); // strike
        assert!(ansi.contains("\x1b[21m")); // underline2
        assert!(ansi.contains("\x1b[51m")); // frame
        assert!(ansi.contains("\x1b[52m")); // encircle
        assert!(ansi.contains("\x1b[53m")); // overline
    }

    #[test]
    fn test_to_ansi_reset() {
        let style = Style::new().bold();
        let reset = style.to_ansi_reset();
        assert_eq!(reset, "\x1b[0m");
    }

    #[test]
    fn test_display() {
        let style = Style::parse("bold red on blue").unwrap();
        let display = style.to_string();
        assert!(display.contains("bold"));
        assert!(display.contains("Red"));
        assert!(display.contains("on"));
        assert!(display.contains("Blue"));
    }

    #[test]
    fn test_display_all_attributes() {
        let style = Style::new()
            .bold()
            .dim()
            .italic()
            .underline()
            .underline2()
            .blink()
            .blink2()
            .reverse()
            .conceal()
            .strike()
            .frame()
            .encircle()
            .overline();
        let display = style.to_string();
        assert!(display.contains("bold"));
        assert!(display.contains("dim"));
        assert!(display.contains("italic"));
        assert!(display.contains("underline"));
        assert!(display.contains("underline2"));
        assert!(display.contains("blink"));
        assert!(display.contains("blink2"));
        assert!(display.contains("reverse"));
        assert!(display.contains("conceal"));
        assert!(display.contains("strike"));
        assert!(display.contains("frame"));
        assert!(display.contains("encircle"));
        assert!(display.contains("overline"));
    }

    #[test]
    fn test_display_with_link() {
        let style = Style::new().link("https://example.com".to_owned());
        let display = style.to_string();
        assert!(display.contains("link https://example.com"));
    }

    #[test]
    fn test_from_str() {
        let style: Style = "bold green".parse().unwrap();
        assert_eq!(style.attributes.bold, Some(true));
        assert_eq!(style.color, Some(Color::Standard(StandardColor::Green)));
    }

    #[test]
    fn test_builder_pattern() {
        let style = Style::new()
            .bold()
            .italic()
            .with_color(Color::Standard(StandardColor::Red))
            .with_bgcolor(Color::Standard(StandardColor::White));

        assert_eq!(style.attributes.bold, Some(true));
        assert_eq!(style.attributes.italic, Some(true));
        assert_eq!(style.color, Some(Color::Standard(StandardColor::Red)));
        assert_eq!(style.bgcolor, Some(Color::Standard(StandardColor::White)));
    }

    #[test]
    fn test_static_color_constructor() {
        let style = Style::color(Color::Standard(StandardColor::Red));
        assert_eq!(style.color, Some(Color::Standard(StandardColor::Red)));
        assert!(style.bgcolor.is_none());
    }

    #[test]
    fn test_static_bgcolor_constructor() {
        let style = Style::bgcolor(Color::Standard(StandardColor::Blue));
        assert!(style.color.is_none());
        assert_eq!(style.bgcolor, Some(Color::Standard(StandardColor::Blue)));
    }

    #[test]
    fn test_parse_invalid() {
        assert!(Style::parse("on").is_err());
        assert!(Style::parse("not notacolor").is_err());
        assert!(Style::parse("link").is_err());
    }

    #[test]
    fn test_parse_unclosed_parenthesis() {
        assert!(Style::parse("rgb(255, 0, 0").is_err());
    }

    #[test]
    fn test_attributes_is_empty() {
        let attrs = Attributes::new();
        assert!(attrs.is_empty());

        let mut attrs = Attributes::new();
        attrs.bold = Some(true);
        assert!(!attrs.is_empty());
    }

    #[test]
    fn test_attributes_combine() {
        let base = Attributes {
            bold: Some(true),
            italic: Some(true),
            ..Attributes::new()
        };
        let overlay = Attributes {
            bold: Some(false),
            underline: Some(true),
            ..Attributes::new()
        };
        let combined = base.combine(overlay);
        assert_eq!(combined.bold, Some(false)); // overlay wins
        assert_eq!(combined.italic, Some(true)); // base preserved
        assert_eq!(combined.underline, Some(true)); // overlay added
    }

    #[test]
    fn test_attributes_combine_all_fields() {
        let base = Attributes {
            bold: Some(true),
            dim: Some(true),
            italic: Some(true),
            underline: Some(true),
            underline2: Some(true),
            blink: Some(true),
            blink2: Some(true),
            reverse: Some(true),
            conceal: Some(true),
            strike: Some(true),
            frame: Some(true),
            encircle: Some(true),
            overline: Some(true),
        };
        let overlay = Attributes::new();
        let combined = base.combine(overlay);
        // All base values should be preserved
        assert_eq!(combined.bold, Some(true));
        assert_eq!(combined.dim, Some(true));
        assert_eq!(combined.italic, Some(true));
        assert_eq!(combined.underline, Some(true));
        assert_eq!(combined.underline2, Some(true));
        assert_eq!(combined.blink, Some(true));
        assert_eq!(combined.blink2, Some(true));
        assert_eq!(combined.reverse, Some(true));
        assert_eq!(combined.conceal, Some(true));
        assert_eq!(combined.strike, Some(true));
        assert_eq!(combined.frame, Some(true));
        assert_eq!(combined.encircle, Some(true));
        assert_eq!(combined.overline, Some(true));
    }

    #[test]
    fn test_style_is_empty() {
        let style = Style::new();
        assert!(style.is_empty());

        let style = Style::new().bold();
        assert!(!style.is_empty());
    }

    #[test]
    fn test_style_is_empty_with_link() {
        let style = Style::new().link("https://example.com".to_owned());
        assert!(!style.is_empty());
    }

    #[test]
    fn test_style_is_empty_with_color() {
        let style = Style::color(Color::Standard(StandardColor::Red));
        assert!(!style.is_empty());
    }

    #[test]
    fn test_style_is_empty_with_bgcolor() {
        let style = Style::bgcolor(Color::Standard(StandardColor::Blue));
        assert!(!style.is_empty());
    }

    #[test]
    fn test_attributes_default() {
        let attrs = Attributes::default();
        assert!(attrs.is_empty());
    }

    #[test]
    fn test_style_default() {
        let style = Style::default();
        assert!(style.is_empty());
    }
}
