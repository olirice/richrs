//! Color definitions and parsing for terminal output.
//!
//! This module provides a comprehensive color system supporting:
//! - Standard 16 ANSI colors
//! - 256-color palette
//! - 24-bit true color (RGB)
//! - Named colors with web color names
//! - Hex color codes

use crate::errors::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a color that can be used for terminal output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Color {
    /// Default terminal color (no color applied).
    Default,
    /// Standard ANSI color (0-15).
    Standard(StandardColor),
    /// 256-color palette index (0-255).
    Palette(u8),
    /// 24-bit true color RGB.
    Rgb {
        /// Red component (0-255).
        r: u8,
        /// Green component (0-255).
        g: u8,
        /// Blue component (0-255).
        b: u8,
    },
}

/// Standard ANSI colors (0-15).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StandardColor {
    /// Black (color 0).
    Black,
    /// Red (color 1).
    Red,
    /// Green (color 2).
    Green,
    /// Yellow (color 3).
    Yellow,
    /// Blue (color 4).
    Blue,
    /// Magenta (color 5).
    Magenta,
    /// Cyan (color 6).
    Cyan,
    /// White (color 7).
    White,
    /// Bright black / gray (color 8).
    BrightBlack,
    /// Bright red (color 9).
    BrightRed,
    /// Bright green (color 10).
    BrightGreen,
    /// Bright yellow (color 11).
    BrightYellow,
    /// Bright blue (color 12).
    BrightBlue,
    /// Bright magenta (color 13).
    BrightMagenta,
    /// Bright cyan (color 14).
    BrightCyan,
    /// Bright white (color 15).
    BrightWhite,
}

impl StandardColor {
    /// Returns the ANSI color code for this standard color.
    #[inline]
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Black => 0,
            Self::Red => 1,
            Self::Green => 2,
            Self::Yellow => 3,
            Self::Blue => 4,
            Self::Magenta => 5,
            Self::Cyan => 6,
            Self::White => 7,
            Self::BrightBlack => 8,
            Self::BrightRed => 9,
            Self::BrightGreen => 10,
            Self::BrightYellow => 11,
            Self::BrightBlue => 12,
            Self::BrightMagenta => 13,
            Self::BrightCyan => 14,
            Self::BrightWhite => 15,
        }
    }

    /// Creates a standard color from its ANSI code.
    ///
    /// # Errors
    ///
    /// Returns an error if the code is not in the range 0-15.
    #[inline]
    pub fn from_code(code: u8) -> Result<Self> {
        match code {
            0 => Ok(Self::Black),
            1 => Ok(Self::Red),
            2 => Ok(Self::Green),
            3 => Ok(Self::Yellow),
            4 => Ok(Self::Blue),
            5 => Ok(Self::Magenta),
            6 => Ok(Self::Cyan),
            7 => Ok(Self::White),
            8 => Ok(Self::BrightBlack),
            9 => Ok(Self::BrightRed),
            10 => Ok(Self::BrightGreen),
            11 => Ok(Self::BrightYellow),
            12 => Ok(Self::BrightBlue),
            13 => Ok(Self::BrightMagenta),
            14 => Ok(Self::BrightCyan),
            15 => Ok(Self::BrightWhite),
            _ => Err(Error::OutOfRange {
                message: format!("standard color code must be 0-15, got {code}"),
            }),
        }
    }
}

impl Color {
    /// Creates a new RGB color.
    #[inline]
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb { r, g, b }
    }

    /// Parses a color from a string.
    ///
    /// Supported formats:
    /// - Named colors: "red", "green", "blue", etc.
    /// - Hex colors: "#ff0000", "#f00"
    /// - RGB colors: "rgb(255, 0, 0)"
    /// - Palette colors: "color(5)" for 256-color palette
    /// - "default" for default color
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed as a valid color.
    #[inline]
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim().to_lowercase();

        if s == "default" {
            return Ok(Self::Default);
        }

        // Try hex color
        if let Some(hex) = s.strip_prefix('#') {
            return Self::parse_hex(hex);
        }

        // Try rgb() format
        if let Some(rgb_content) = s.strip_prefix("rgb(").and_then(|s| s.strip_suffix(')')) {
            return Self::parse_rgb_tuple(rgb_content);
        }

        // Try color() format for palette
        if let Some(color_content) = s.strip_prefix("color(").and_then(|s| s.strip_suffix(')')) {
            return Self::parse_palette(color_content);
        }

        // Try named color
        Self::parse_named(&s)
    }

    /// Parses a hex color string (without the # prefix).
    fn parse_hex(hex: &str) -> Result<Self> {
        let hex = hex.trim();

        let (r, g, b) = if hex.len() == 3 {
            // Short form: #rgb -> #rrggbb
            let chars: Vec<char> = hex.chars().collect();
            let r_char = chars.first().ok_or_else(|| Error::ColorParse {
                message: "empty hex color".to_owned(),
            })?;
            let g_char = chars.get(1).ok_or_else(|| Error::ColorParse {
                message: "incomplete hex color".to_owned(),
            })?;
            let b_char = chars.get(2).ok_or_else(|| Error::ColorParse {
                message: "incomplete hex color".to_owned(),
            })?;

            let r = Self::parse_hex_char(*r_char)?;
            let g = Self::parse_hex_char(*g_char)?;
            let b = Self::parse_hex_char(*b_char)?;

            // Duplicate the nibble: f -> ff
            let r = Self::duplicate_nibble(r);
            let g = Self::duplicate_nibble(g);
            let b = Self::duplicate_nibble(b);

            (r, g, b)
        } else if hex.len() == 6 {
            let r = Self::parse_hex_byte(&hex[0..2])?;
            let g = Self::parse_hex_byte(&hex[2..4])?;
            let b = Self::parse_hex_byte(&hex[4..6])?;
            (r, g, b)
        } else {
            return Err(Error::ColorParse {
                message: format!(
                    "invalid hex color length: expected 3 or 6, got {}",
                    hex.len()
                ),
            });
        };

        Ok(Self::Rgb { r, g, b })
    }

    /// Parses a single hex character (0-9, a-f, A-F).
    fn parse_hex_char(c: char) -> Result<u8> {
        match c {
            '0'..='9' => {
                let digit = c as u32;
                let zero = '0' as u32;
                u8::try_from(digit.saturating_sub(zero)).map_err(|_| Error::ColorParse {
                    message: "hex digit conversion failed".to_owned(),
                })
            }
            'a'..='f' => {
                let digit = c as u32;
                let a = 'a' as u32;
                u8::try_from(digit.saturating_sub(a).saturating_add(10)).map_err(|_| {
                    Error::ColorParse {
                        message: "hex digit conversion failed".to_owned(),
                    }
                })
            }
            'A'..='F' => {
                let digit = c as u32;
                let a = 'A' as u32;
                u8::try_from(digit.saturating_sub(a).saturating_add(10)).map_err(|_| {
                    Error::ColorParse {
                        message: "hex digit conversion failed".to_owned(),
                    }
                })
            }
            _ => Err(Error::ColorParse {
                message: format!("invalid hex character: '{c}'"),
            }),
        }
    }

    /// Duplicates a nibble: 0xf -> 0xff.
    #[inline]
    const fn duplicate_nibble(nibble: u8) -> u8 {
        // nibble << 4 | nibble, but avoiding arithmetic
        nibble.saturating_mul(16).saturating_add(nibble)
    }

    /// Parses a two-character hex byte.
    fn parse_hex_byte(s: &str) -> Result<u8> {
        let chars: Vec<char> = s.chars().collect();
        let high = chars.first().ok_or_else(|| Error::ColorParse {
            message: "empty hex byte".to_owned(),
        })?;
        let low = chars.get(1).ok_or_else(|| Error::ColorParse {
            message: "incomplete hex byte".to_owned(),
        })?;

        let high = Self::parse_hex_char(*high)?;
        let low = Self::parse_hex_char(*low)?;

        Ok(high.saturating_mul(16).saturating_add(low))
    }

    /// Parses an RGB tuple from "r, g, b" format.
    fn parse_rgb_tuple(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(',').map(str::trim).collect();
        if parts.len() != 3 {
            return Err(Error::ColorParse {
                message: format!("rgb() requires 3 components, got {}", parts.len()),
            });
        }

        let r_str = parts.first().ok_or_else(|| Error::ColorParse {
            message: "missing red component".to_owned(),
        })?;
        let g_str = parts.get(1).ok_or_else(|| Error::ColorParse {
            message: "missing green component".to_owned(),
        })?;
        let b_str = parts.get(2).ok_or_else(|| Error::ColorParse {
            message: "missing blue component".to_owned(),
        })?;

        let r: u8 = r_str.parse().map_err(|_| Error::ColorParse {
            message: format!("invalid red component: '{r_str}'"),
        })?;
        let g: u8 = g_str.parse().map_err(|_| Error::ColorParse {
            message: format!("invalid green component: '{g_str}'"),
        })?;
        let b: u8 = b_str.parse().map_err(|_| Error::ColorParse {
            message: format!("invalid blue component: '{b_str}'"),
        })?;

        Ok(Self::Rgb { r, g, b })
    }

    /// Parses a palette color index.
    fn parse_palette(s: &str) -> Result<Self> {
        let index: u8 = s.trim().parse().map_err(|_| Error::ColorParse {
            message: format!("invalid palette index: '{s}'"),
        })?;
        Ok(Self::Palette(index))
    }

    /// Parses a named color.
    #[allow(clippy::too_many_lines)]
    fn parse_named(name: &str) -> Result<Self> {
        match name {
            // Standard colors
            "black" => Ok(Self::Standard(StandardColor::Black)),
            "red" => Ok(Self::Standard(StandardColor::Red)),
            "green" => Ok(Self::Standard(StandardColor::Green)),
            "yellow" => Ok(Self::Standard(StandardColor::Yellow)),
            "blue" => Ok(Self::Standard(StandardColor::Blue)),
            "magenta" => Ok(Self::Standard(StandardColor::Magenta)),
            "cyan" => Ok(Self::Standard(StandardColor::Cyan)),
            "white" => Ok(Self::Standard(StandardColor::White)),

            // Bright variants
            "bright_black" | "brightblack" | "grey" | "gray" => {
                Ok(Self::Standard(StandardColor::BrightBlack))
            }
            "bright_red" | "brightred" => Ok(Self::Standard(StandardColor::BrightRed)),
            "bright_green" | "brightgreen" => Ok(Self::Standard(StandardColor::BrightGreen)),
            "bright_yellow" | "brightyellow" => Ok(Self::Standard(StandardColor::BrightYellow)),
            "bright_blue" | "brightblue" => Ok(Self::Standard(StandardColor::BrightBlue)),
            "bright_magenta" | "brightmagenta" => Ok(Self::Standard(StandardColor::BrightMagenta)),
            "bright_cyan" | "brightcyan" => Ok(Self::Standard(StandardColor::BrightCyan)),
            "bright_white" | "brightwhite" => Ok(Self::Standard(StandardColor::BrightWhite)),

            // Web colors (a selection of common ones)
            "maroon" => Ok(Self::Rgb { r: 128, g: 0, b: 0 }),
            "dark_red" | "darkred" => Ok(Self::Rgb { r: 139, g: 0, b: 0 }),
            "brown" => Ok(Self::Rgb {
                r: 165,
                g: 42,
                b: 42,
            }),
            "firebrick" => Ok(Self::Rgb {
                r: 178,
                g: 34,
                b: 34,
            }),
            "crimson" => Ok(Self::Rgb {
                r: 220,
                g: 20,
                b: 60,
            }),
            "tomato" => Ok(Self::Rgb {
                r: 255,
                g: 99,
                b: 71,
            }),
            "coral" => Ok(Self::Rgb {
                r: 255,
                g: 127,
                b: 80,
            }),
            "indian_red" | "indianred" => Ok(Self::Rgb {
                r: 205,
                g: 92,
                b: 92,
            }),
            "light_coral" | "lightcoral" => Ok(Self::Rgb {
                r: 240,
                g: 128,
                b: 128,
            }),
            "dark_salmon" | "darksalmon" => Ok(Self::Rgb {
                r: 233,
                g: 150,
                b: 122,
            }),
            "salmon" => Ok(Self::Rgb {
                r: 250,
                g: 128,
                b: 114,
            }),
            "light_salmon" | "lightsalmon" => Ok(Self::Rgb {
                r: 255,
                g: 160,
                b: 122,
            }),
            "orange_red" | "orangered" => Ok(Self::Rgb {
                r: 255,
                g: 69,
                b: 0,
            }),
            "dark_orange" | "darkorange" => Ok(Self::Rgb {
                r: 255,
                g: 140,
                b: 0,
            }),
            "orange" => Ok(Self::Rgb {
                r: 255,
                g: 165,
                b: 0,
            }),
            "gold" => Ok(Self::Rgb {
                r: 255,
                g: 215,
                b: 0,
            }),
            "dark_golden_rod" | "darkgoldenrod" => Ok(Self::Rgb {
                r: 184,
                g: 134,
                b: 11,
            }),
            "golden_rod" | "goldenrod" => Ok(Self::Rgb {
                r: 218,
                g: 165,
                b: 32,
            }),
            "pale_golden_rod" | "palegoldenrod" => Ok(Self::Rgb {
                r: 238,
                g: 232,
                b: 170,
            }),
            "dark_khaki" | "darkkhaki" => Ok(Self::Rgb {
                r: 189,
                g: 183,
                b: 107,
            }),
            "khaki" => Ok(Self::Rgb {
                r: 240,
                g: 230,
                b: 140,
            }),
            "olive" => Ok(Self::Rgb {
                r: 128,
                g: 128,
                b: 0,
            }),
            "yellow_green" | "yellowgreen" => Ok(Self::Rgb {
                r: 154,
                g: 205,
                b: 50,
            }),
            "dark_olive_green" | "darkolivegreen" => Ok(Self::Rgb {
                r: 85,
                g: 107,
                b: 47,
            }),
            "olive_drab" | "olivedrab" => Ok(Self::Rgb {
                r: 107,
                g: 142,
                b: 35,
            }),
            "lawn_green" | "lawngreen" => Ok(Self::Rgb {
                r: 124,
                g: 252,
                b: 0,
            }),
            "chartreuse" => Ok(Self::Rgb {
                r: 127,
                g: 255,
                b: 0,
            }),
            "green_yellow" | "greenyellow" => Ok(Self::Rgb {
                r: 173,
                g: 255,
                b: 47,
            }),
            "dark_green" | "darkgreen" => Ok(Self::Rgb { r: 0, g: 100, b: 0 }),
            "forest_green" | "forestgreen" => Ok(Self::Rgb {
                r: 34,
                g: 139,
                b: 34,
            }),
            "lime" => Ok(Self::Rgb { r: 0, g: 255, b: 0 }),
            "lime_green" | "limegreen" => Ok(Self::Rgb {
                r: 50,
                g: 205,
                b: 50,
            }),
            "light_green" | "lightgreen" => Ok(Self::Rgb {
                r: 144,
                g: 238,
                b: 144,
            }),
            "pale_green" | "palegreen" => Ok(Self::Rgb {
                r: 152,
                g: 251,
                b: 152,
            }),
            "dark_sea_green" | "darkseagreen" => Ok(Self::Rgb {
                r: 143,
                g: 188,
                b: 143,
            }),
            "medium_spring_green" | "mediumspringgreen" => Ok(Self::Rgb {
                r: 0,
                g: 250,
                b: 154,
            }),
            "spring_green" | "springgreen" => Ok(Self::Rgb {
                r: 0,
                g: 255,
                b: 127,
            }),
            "sea_green" | "seagreen" => Ok(Self::Rgb {
                r: 46,
                g: 139,
                b: 87,
            }),
            "medium_aqua_marine" | "mediumaquamarine" => Ok(Self::Rgb {
                r: 102,
                g: 205,
                b: 170,
            }),
            "medium_sea_green" | "mediumseagreen" => Ok(Self::Rgb {
                r: 60,
                g: 179,
                b: 113,
            }),
            "light_sea_green" | "lightseagreen" => Ok(Self::Rgb {
                r: 32,
                g: 178,
                b: 170,
            }),
            "dark_slate_gray" | "darkslategray" => Ok(Self::Rgb {
                r: 47,
                g: 79,
                b: 79,
            }),
            "teal" => Ok(Self::Rgb {
                r: 0,
                g: 128,
                b: 128,
            }),
            "dark_cyan" | "darkcyan" => Ok(Self::Rgb {
                r: 0,
                g: 139,
                b: 139,
            }),
            "aqua" => Ok(Self::Rgb {
                r: 0,
                g: 255,
                b: 255,
            }),
            "light_cyan" | "lightcyan" => Ok(Self::Rgb {
                r: 224,
                g: 255,
                b: 255,
            }),
            "dark_turquoise" | "darkturquoise" => Ok(Self::Rgb {
                r: 0,
                g: 206,
                b: 209,
            }),
            "turquoise" => Ok(Self::Rgb {
                r: 64,
                g: 224,
                b: 208,
            }),
            "medium_turquoise" | "mediumturquoise" => Ok(Self::Rgb {
                r: 72,
                g: 209,
                b: 204,
            }),
            "pale_turquoise" | "paleturquoise" => Ok(Self::Rgb {
                r: 175,
                g: 238,
                b: 238,
            }),
            "aqua_marine" | "aquamarine" => Ok(Self::Rgb {
                r: 127,
                g: 255,
                b: 212,
            }),
            "powder_blue" | "powderblue" => Ok(Self::Rgb {
                r: 176,
                g: 224,
                b: 230,
            }),
            "cadet_blue" | "cadetblue" => Ok(Self::Rgb {
                r: 95,
                g: 158,
                b: 160,
            }),
            "steel_blue" | "steelblue" => Ok(Self::Rgb {
                r: 70,
                g: 130,
                b: 180,
            }),
            "corn_flower_blue" | "cornflowerblue" => Ok(Self::Rgb {
                r: 100,
                g: 149,
                b: 237,
            }),
            "deep_sky_blue" | "deepskyblue" => Ok(Self::Rgb {
                r: 0,
                g: 191,
                b: 255,
            }),
            "dodger_blue" | "dodgerblue" => Ok(Self::Rgb {
                r: 30,
                g: 144,
                b: 255,
            }),
            "light_blue" | "lightblue" => Ok(Self::Rgb {
                r: 173,
                g: 216,
                b: 230,
            }),
            "sky_blue" | "skyblue" => Ok(Self::Rgb {
                r: 135,
                g: 206,
                b: 235,
            }),
            "light_sky_blue" | "lightskyblue" => Ok(Self::Rgb {
                r: 135,
                g: 206,
                b: 250,
            }),
            "midnight_blue" | "midnightblue" => Ok(Self::Rgb {
                r: 25,
                g: 25,
                b: 112,
            }),
            "navy" => Ok(Self::Rgb { r: 0, g: 0, b: 128 }),
            "dark_blue" | "darkblue" => Ok(Self::Rgb { r: 0, g: 0, b: 139 }),
            "medium_blue" | "mediumblue" => Ok(Self::Rgb { r: 0, g: 0, b: 205 }),
            "royal_blue" | "royalblue" => Ok(Self::Rgb {
                r: 65,
                g: 105,
                b: 225,
            }),
            "blue_violet" | "blueviolet" => Ok(Self::Rgb {
                r: 138,
                g: 43,
                b: 226,
            }),
            "indigo" => Ok(Self::Rgb {
                r: 75,
                g: 0,
                b: 130,
            }),
            "dark_slate_blue" | "darkslateblue" => Ok(Self::Rgb {
                r: 72,
                g: 61,
                b: 139,
            }),
            "slate_blue" | "slateblue" => Ok(Self::Rgb {
                r: 106,
                g: 90,
                b: 205,
            }),
            "medium_slate_blue" | "mediumslateblue" => Ok(Self::Rgb {
                r: 123,
                g: 104,
                b: 238,
            }),
            "medium_purple" | "mediumpurple" => Ok(Self::Rgb {
                r: 147,
                g: 112,
                b: 219,
            }),
            "dark_magenta" | "darkmagenta" => Ok(Self::Rgb {
                r: 139,
                g: 0,
                b: 139,
            }),
            "dark_violet" | "darkviolet" => Ok(Self::Rgb {
                r: 148,
                g: 0,
                b: 211,
            }),
            "dark_orchid" | "darkorchid" => Ok(Self::Rgb {
                r: 153,
                g: 50,
                b: 204,
            }),
            "medium_orchid" | "mediumorchid" => Ok(Self::Rgb {
                r: 186,
                g: 85,
                b: 211,
            }),
            "purple" => Ok(Self::Rgb {
                r: 128,
                g: 0,
                b: 128,
            }),
            "thistle" => Ok(Self::Rgb {
                r: 216,
                g: 191,
                b: 216,
            }),
            "plum" => Ok(Self::Rgb {
                r: 221,
                g: 160,
                b: 221,
            }),
            "violet" => Ok(Self::Rgb {
                r: 238,
                g: 130,
                b: 238,
            }),
            "fuchsia" => Ok(Self::Rgb {
                r: 255,
                g: 0,
                b: 255,
            }),
            "orchid" => Ok(Self::Rgb {
                r: 218,
                g: 112,
                b: 214,
            }),
            "medium_violet_red" | "mediumvioletred" => Ok(Self::Rgb {
                r: 199,
                g: 21,
                b: 133,
            }),
            "pale_violet_red" | "palevioletred" => Ok(Self::Rgb {
                r: 219,
                g: 112,
                b: 147,
            }),
            "deep_pink" | "deeppink" => Ok(Self::Rgb {
                r: 255,
                g: 20,
                b: 147,
            }),
            "hot_pink" | "hotpink" => Ok(Self::Rgb {
                r: 255,
                g: 105,
                b: 180,
            }),
            "light_pink" | "lightpink" => Ok(Self::Rgb {
                r: 255,
                g: 182,
                b: 193,
            }),
            "pink" => Ok(Self::Rgb {
                r: 255,
                g: 192,
                b: 203,
            }),
            "antique_white" | "antiquewhite" => Ok(Self::Rgb {
                r: 250,
                g: 235,
                b: 215,
            }),
            "beige" => Ok(Self::Rgb {
                r: 245,
                g: 245,
                b: 220,
            }),
            "bisque" => Ok(Self::Rgb {
                r: 255,
                g: 228,
                b: 196,
            }),
            "blanched_almond" | "blanchedalmond" => Ok(Self::Rgb {
                r: 255,
                g: 235,
                b: 205,
            }),
            "wheat" => Ok(Self::Rgb {
                r: 245,
                g: 222,
                b: 179,
            }),
            "corn_silk" | "cornsilk" => Ok(Self::Rgb {
                r: 255,
                g: 248,
                b: 220,
            }),
            "lemon_chiffon" | "lemonchiffon" => Ok(Self::Rgb {
                r: 255,
                g: 250,
                b: 205,
            }),
            "light_golden_rod_yellow" | "lightgoldenrodyellow" => Ok(Self::Rgb {
                r: 250,
                g: 250,
                b: 210,
            }),
            "light_yellow" | "lightyellow" => Ok(Self::Rgb {
                r: 255,
                g: 255,
                b: 224,
            }),
            "saddle_brown" | "saddlebrown" => Ok(Self::Rgb {
                r: 139,
                g: 69,
                b: 19,
            }),
            "sienna" => Ok(Self::Rgb {
                r: 160,
                g: 82,
                b: 45,
            }),
            "chocolate" => Ok(Self::Rgb {
                r: 210,
                g: 105,
                b: 30,
            }),
            "peru" => Ok(Self::Rgb {
                r: 205,
                g: 133,
                b: 63,
            }),
            "sandy_brown" | "sandybrown" => Ok(Self::Rgb {
                r: 244,
                g: 164,
                b: 96,
            }),
            "burly_wood" | "burlywood" => Ok(Self::Rgb {
                r: 222,
                g: 184,
                b: 135,
            }),
            "tan" => Ok(Self::Rgb {
                r: 210,
                g: 180,
                b: 140,
            }),
            "rosy_brown" | "rosybrown" => Ok(Self::Rgb {
                r: 188,
                g: 143,
                b: 143,
            }),
            "moccasin" => Ok(Self::Rgb {
                r: 255,
                g: 228,
                b: 181,
            }),
            "navajo_white" | "navajowhite" => Ok(Self::Rgb {
                r: 255,
                g: 222,
                b: 173,
            }),
            "peach_puff" | "peachpuff" => Ok(Self::Rgb {
                r: 255,
                g: 218,
                b: 185,
            }),
            "misty_rose" | "mistyrose" => Ok(Self::Rgb {
                r: 255,
                g: 228,
                b: 225,
            }),
            "lavender_blush" | "lavenderblush" => Ok(Self::Rgb {
                r: 255,
                g: 240,
                b: 245,
            }),
            "linen" => Ok(Self::Rgb {
                r: 250,
                g: 240,
                b: 230,
            }),
            "old_lace" | "oldlace" => Ok(Self::Rgb {
                r: 253,
                g: 245,
                b: 230,
            }),
            "papaya_whip" | "papayawhip" => Ok(Self::Rgb {
                r: 255,
                g: 239,
                b: 213,
            }),
            "sea_shell" | "seashell" => Ok(Self::Rgb {
                r: 255,
                g: 245,
                b: 238,
            }),
            "mint_cream" | "mintcream" => Ok(Self::Rgb {
                r: 245,
                g: 255,
                b: 250,
            }),
            "slate_gray" | "slategray" => Ok(Self::Rgb {
                r: 112,
                g: 128,
                b: 144,
            }),
            "light_slate_gray" | "lightslategray" => Ok(Self::Rgb {
                r: 119,
                g: 136,
                b: 153,
            }),
            "light_steel_blue" | "lightsteelblue" => Ok(Self::Rgb {
                r: 176,
                g: 196,
                b: 222,
            }),
            "lavender" => Ok(Self::Rgb {
                r: 230,
                g: 230,
                b: 250,
            }),
            "floral_white" | "floralwhite" => Ok(Self::Rgb {
                r: 255,
                g: 250,
                b: 240,
            }),
            "alice_blue" | "aliceblue" => Ok(Self::Rgb {
                r: 240,
                g: 248,
                b: 255,
            }),
            "ghost_white" | "ghostwhite" => Ok(Self::Rgb {
                r: 248,
                g: 248,
                b: 255,
            }),
            "honeydew" => Ok(Self::Rgb {
                r: 240,
                g: 255,
                b: 240,
            }),
            "ivory" => Ok(Self::Rgb {
                r: 255,
                g: 255,
                b: 240,
            }),
            "azure" => Ok(Self::Rgb {
                r: 240,
                g: 255,
                b: 255,
            }),
            "snow" => Ok(Self::Rgb {
                r: 255,
                g: 250,
                b: 250,
            }),
            "dim_gray" | "dimgray" | "dim_grey" | "dimgrey" => Ok(Self::Rgb {
                r: 105,
                g: 105,
                b: 105,
            }),
            "dark_gray" | "darkgray" | "dark_grey" | "darkgrey" => Ok(Self::Rgb {
                r: 169,
                g: 169,
                b: 169,
            }),
            "silver" => Ok(Self::Rgb {
                r: 192,
                g: 192,
                b: 192,
            }),
            "light_gray" | "lightgray" | "light_grey" | "lightgrey" => Ok(Self::Rgb {
                r: 211,
                g: 211,
                b: 211,
            }),
            "gainsboro" => Ok(Self::Rgb {
                r: 220,
                g: 220,
                b: 220,
            }),
            "white_smoke" | "whitesmoke" => Ok(Self::Rgb {
                r: 245,
                g: 245,
                b: 245,
            }),

            _ => Err(Error::ColorParse {
                message: format!("unknown color name: '{name}'"),
            }),
        }
    }

    /// Converts this color to its ANSI escape sequence for foreground.
    #[inline]
    #[must_use]
    pub fn to_ansi_fg(&self) -> String {
        match self {
            Self::Default => "\x1b[39m".to_owned(),
            Self::Standard(std) => {
                let code = std.code();
                if code < 8 {
                    format!("\x1b[{}m", u16::from(code).saturating_add(30))
                } else {
                    format!("\x1b[{}m", u16::from(code).saturating_add(82))
                }
            }
            Self::Palette(idx) => format!("\x1b[38;5;{idx}m"),
            Self::Rgb { r, g, b } => format!("\x1b[38;2;{r};{g};{b}m"),
        }
    }

    /// Converts this color to its ANSI escape sequence for background.
    #[inline]
    #[must_use]
    pub fn to_ansi_bg(&self) -> String {
        match self {
            Self::Default => "\x1b[49m".to_owned(),
            Self::Standard(std) => {
                let code = std.code();
                if code < 8 {
                    format!("\x1b[{}m", u16::from(code).saturating_add(40))
                } else {
                    format!("\x1b[{}m", u16::from(code).saturating_add(92))
                }
            }
            Self::Palette(idx) => format!("\x1b[48;5;{idx}m"),
            Self::Rgb { r, g, b } => format!("\x1b[48;2;{r};{g};{b}m"),
        }
    }
}

impl Default for Color {
    #[inline]
    fn default() -> Self {
        Self::Default
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Standard(std) => write!(f, "{std:?}"),
            Self::Palette(idx) => write!(f, "color({idx})"),
            Self::Rgb { r, g, b } => write!(f, "rgb({r}, {g}, {b})"),
        }
    }
}

impl std::str::FromStr for Color {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_named_colors() {
        assert_eq!(
            Color::parse("red").ok(),
            Some(Color::Standard(StandardColor::Red))
        );
        assert_eq!(
            Color::parse("green").ok(),
            Some(Color::Standard(StandardColor::Green))
        );
        assert_eq!(
            Color::parse("blue").ok(),
            Some(Color::Standard(StandardColor::Blue))
        );
        assert_eq!(
            Color::parse("BLACK").ok(),
            Some(Color::Standard(StandardColor::Black))
        );
    }

    #[test]
    fn test_parse_bright_colors() {
        assert_eq!(
            Color::parse("bright_red").ok(),
            Some(Color::Standard(StandardColor::BrightRed))
        );
        assert_eq!(
            Color::parse("brightblue").ok(),
            Some(Color::Standard(StandardColor::BrightBlue))
        );
        assert_eq!(
            Color::parse("grey").ok(),
            Some(Color::Standard(StandardColor::BrightBlack))
        );
    }

    #[test]
    fn test_parse_hex_colors() {
        assert_eq!(
            Color::parse("#ff0000").ok(),
            Some(Color::Rgb { r: 255, g: 0, b: 0 })
        );
        assert_eq!(
            Color::parse("#00ff00").ok(),
            Some(Color::Rgb { r: 0, g: 255, b: 0 })
        );
        assert_eq!(
            Color::parse("#0000ff").ok(),
            Some(Color::Rgb { r: 0, g: 0, b: 255 })
        );
        // Short form
        assert_eq!(
            Color::parse("#f00").ok(),
            Some(Color::Rgb { r: 255, g: 0, b: 0 })
        );
        assert_eq!(
            Color::parse("#abc").ok(),
            Some(Color::Rgb {
                r: 170,
                g: 187,
                b: 204
            })
        );
    }

    #[test]
    fn test_parse_rgb_colors() {
        assert_eq!(
            Color::parse("rgb(255, 0, 0)").ok(),
            Some(Color::Rgb { r: 255, g: 0, b: 0 })
        );
        assert_eq!(
            Color::parse("rgb(128,128,128)").ok(),
            Some(Color::Rgb {
                r: 128,
                g: 128,
                b: 128
            })
        );
    }

    #[test]
    fn test_parse_palette_colors() {
        assert_eq!(Color::parse("color(5)").ok(), Some(Color::Palette(5)));
        assert_eq!(Color::parse("color(255)").ok(), Some(Color::Palette(255)));
    }

    #[test]
    fn test_parse_default() {
        assert_eq!(Color::parse("default").ok(), Some(Color::Default));
    }

    #[test]
    fn test_parse_invalid_colors() {
        assert!(Color::parse("not_a_color").is_err());
        assert!(Color::parse("#gggggg").is_err());
        assert!(Color::parse("rgb(256, 0, 0)").is_err());
        assert!(Color::parse("color(-1)").is_err());
    }

    #[test]
    fn test_to_ansi_fg() {
        assert_eq!(Color::Default.to_ansi_fg(), "\x1b[39m");
        assert_eq!(Color::Standard(StandardColor::Red).to_ansi_fg(), "\x1b[31m");
        assert_eq!(
            Color::Standard(StandardColor::BrightRed).to_ansi_fg(),
            "\x1b[91m"
        );
        assert_eq!(Color::Palette(5).to_ansi_fg(), "\x1b[38;5;5m");
        assert_eq!(
            Color::Rgb { r: 255, g: 0, b: 0 }.to_ansi_fg(),
            "\x1b[38;2;255;0;0m"
        );
    }

    #[test]
    fn test_to_ansi_bg() {
        assert_eq!(Color::Default.to_ansi_bg(), "\x1b[49m");
        assert_eq!(Color::Standard(StandardColor::Red).to_ansi_bg(), "\x1b[41m");
        assert_eq!(
            Color::Standard(StandardColor::BrightRed).to_ansi_bg(),
            "\x1b[101m"
        );
        assert_eq!(Color::Palette(5).to_ansi_bg(), "\x1b[48;5;5m");
        assert_eq!(
            Color::Rgb { r: 255, g: 0, b: 0 }.to_ansi_bg(),
            "\x1b[48;2;255;0;0m"
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(Color::Default.to_string(), "default");
        assert_eq!(Color::Palette(42).to_string(), "color(42)");
        assert_eq!(
            Color::Rgb {
                r: 255,
                g: 128,
                b: 0
            }
            .to_string(),
            "rgb(255, 128, 0)"
        );
    }

    #[test]
    fn test_from_str() {
        let color: Color = "red".parse().ok().unwrap_or_default();
        assert_eq!(color, Color::Standard(StandardColor::Red));
    }

    #[test]
    fn test_standard_color_code() {
        assert_eq!(StandardColor::Black.code(), 0);
        assert_eq!(StandardColor::Red.code(), 1);
        assert_eq!(StandardColor::BrightWhite.code(), 15);
    }

    #[test]
    fn test_standard_color_from_code() {
        assert_eq!(StandardColor::from_code(0).ok(), Some(StandardColor::Black));
        assert_eq!(StandardColor::from_code(1).ok(), Some(StandardColor::Red));
        assert_eq!(
            StandardColor::from_code(15).ok(),
            Some(StandardColor::BrightWhite)
        );
        assert!(StandardColor::from_code(16).is_err());
    }

    #[test]
    fn test_web_colors() {
        assert_eq!(
            Color::parse("maroon").ok(),
            Some(Color::Rgb { r: 128, g: 0, b: 0 })
        );
        assert_eq!(
            Color::parse("navy").ok(),
            Some(Color::Rgb { r: 0, g: 0, b: 128 })
        );
        assert_eq!(
            Color::parse("teal").ok(),
            Some(Color::Rgb {
                r: 0,
                g: 128,
                b: 128
            })
        );
    }

    #[test]
    fn test_more_web_colors() {
        // Reds/oranges
        assert!(Color::parse("dark_red").is_ok());
        assert!(Color::parse("darkred").is_ok());
        assert!(Color::parse("brown").is_ok());
        assert!(Color::parse("firebrick").is_ok());
        assert!(Color::parse("crimson").is_ok());
        assert!(Color::parse("tomato").is_ok());
        assert!(Color::parse("coral").is_ok());
        assert!(Color::parse("indian_red").is_ok());
        assert!(Color::parse("light_coral").is_ok());
        assert!(Color::parse("salmon").is_ok());
        assert!(Color::parse("orange_red").is_ok());
        assert!(Color::parse("dark_orange").is_ok());
        assert!(Color::parse("orange").is_ok());
        assert!(Color::parse("gold").is_ok());

        // Yellows
        assert!(Color::parse("dark_golden_rod").is_ok());
        assert!(Color::parse("golden_rod").is_ok());
        assert!(Color::parse("pale_golden_rod").is_ok());
        assert!(Color::parse("dark_khaki").is_ok());
        assert!(Color::parse("khaki").is_ok());
        assert!(Color::parse("olive").is_ok());
        assert!(Color::parse("yellow_green").is_ok());

        // Greens
        assert!(Color::parse("dark_olive_green").is_ok());
        assert!(Color::parse("olive_drab").is_ok());
        assert!(Color::parse("lawn_green").is_ok());
        assert!(Color::parse("chartreuse").is_ok());
        assert!(Color::parse("green_yellow").is_ok());
        assert!(Color::parse("dark_green").is_ok());
        assert!(Color::parse("forest_green").is_ok());
        assert!(Color::parse("lime").is_ok());
        assert!(Color::parse("lime_green").is_ok());
        assert!(Color::parse("light_green").is_ok());
        assert!(Color::parse("pale_green").is_ok());
        assert!(Color::parse("dark_sea_green").is_ok());
        assert!(Color::parse("medium_spring_green").is_ok());
        assert!(Color::parse("spring_green").is_ok());
        assert!(Color::parse("sea_green").is_ok());
        assert!(Color::parse("medium_aqua_marine").is_ok());
        assert!(Color::parse("medium_sea_green").is_ok());

        // Cyans
        assert!(Color::parse("light_sea_green").is_ok());
        assert!(Color::parse("dark_slate_gray").is_ok());
        assert!(Color::parse("dark_cyan").is_ok());
        assert!(Color::parse("aqua").is_ok());
        assert!(Color::parse("light_cyan").is_ok());
        assert!(Color::parse("dark_turquoise").is_ok());
        assert!(Color::parse("turquoise").is_ok());
        assert!(Color::parse("medium_turquoise").is_ok());
        assert!(Color::parse("pale_turquoise").is_ok());
        assert!(Color::parse("aqua_marine").is_ok());
        assert!(Color::parse("powder_blue").is_ok());
        assert!(Color::parse("cadet_blue").is_ok());

        // Blues
        assert!(Color::parse("steel_blue").is_ok());
        assert!(Color::parse("corn_flower_blue").is_ok());
        assert!(Color::parse("deep_sky_blue").is_ok());
        assert!(Color::parse("dodger_blue").is_ok());
        assert!(Color::parse("light_blue").is_ok());
        assert!(Color::parse("sky_blue").is_ok());
        assert!(Color::parse("light_sky_blue").is_ok());
        assert!(Color::parse("midnight_blue").is_ok());
        assert!(Color::parse("dark_blue").is_ok());
        assert!(Color::parse("medium_blue").is_ok());
        assert!(Color::parse("royal_blue").is_ok());

        // Purples
        assert!(Color::parse("blue_violet").is_ok());
        assert!(Color::parse("indigo").is_ok());
        assert!(Color::parse("dark_slate_blue").is_ok());
        assert!(Color::parse("slate_blue").is_ok());
        assert!(Color::parse("medium_slate_blue").is_ok());
        assert!(Color::parse("medium_purple").is_ok());
        assert!(Color::parse("dark_magenta").is_ok());
        assert!(Color::parse("dark_violet").is_ok());
        assert!(Color::parse("dark_orchid").is_ok());
        assert!(Color::parse("medium_orchid").is_ok());
        assert!(Color::parse("purple").is_ok());
        assert!(Color::parse("thistle").is_ok());
        assert!(Color::parse("plum").is_ok());
        assert!(Color::parse("violet").is_ok());
        assert!(Color::parse("fuchsia").is_ok());
        assert!(Color::parse("orchid").is_ok());

        // Pinks
        assert!(Color::parse("medium_violet_red").is_ok());
        assert!(Color::parse("pale_violet_red").is_ok());
        assert!(Color::parse("deep_pink").is_ok());
        assert!(Color::parse("hot_pink").is_ok());
        assert!(Color::parse("light_pink").is_ok());
        assert!(Color::parse("pink").is_ok());

        // Browns
        assert!(Color::parse("saddle_brown").is_ok());
        assert!(Color::parse("sienna").is_ok());
        assert!(Color::parse("chocolate").is_ok());
        assert!(Color::parse("peru").is_ok());
        assert!(Color::parse("sandy_brown").is_ok());
        assert!(Color::parse("burly_wood").is_ok());
        assert!(Color::parse("tan").is_ok());
        assert!(Color::parse("rosy_brown").is_ok());
        assert!(Color::parse("moccasin").is_ok());
        assert!(Color::parse("navajo_white").is_ok());
        assert!(Color::parse("peach_puff").is_ok());

        // Whites/creams
        assert!(Color::parse("antique_white").is_ok());
        assert!(Color::parse("beige").is_ok());
        assert!(Color::parse("bisque").is_ok());
        assert!(Color::parse("blanched_almond").is_ok());
        assert!(Color::parse("wheat").is_ok());
        assert!(Color::parse("corn_silk").is_ok());
        assert!(Color::parse("lemon_chiffon").is_ok());
        assert!(Color::parse("light_golden_rod_yellow").is_ok());
        assert!(Color::parse("light_yellow").is_ok());
        assert!(Color::parse("misty_rose").is_ok());
        assert!(Color::parse("lavender_blush").is_ok());
        assert!(Color::parse("linen").is_ok());
        assert!(Color::parse("old_lace").is_ok());
        assert!(Color::parse("papaya_whip").is_ok());
        assert!(Color::parse("sea_shell").is_ok());
        assert!(Color::parse("mint_cream").is_ok());
        assert!(Color::parse("floral_white").is_ok());
        assert!(Color::parse("alice_blue").is_ok());
        assert!(Color::parse("ghost_white").is_ok());
        assert!(Color::parse("honeydew").is_ok());
        assert!(Color::parse("ivory").is_ok());
        assert!(Color::parse("azure").is_ok());
        assert!(Color::parse("snow").is_ok());

        // Grays
        assert!(Color::parse("dim_gray").is_ok());
        assert!(Color::parse("dim_grey").is_ok());
        assert!(Color::parse("dark_gray").is_ok());
        assert!(Color::parse("dark_grey").is_ok());
        assert!(Color::parse("silver").is_ok());
        assert!(Color::parse("light_gray").is_ok());
        assert!(Color::parse("light_grey").is_ok());
        assert!(Color::parse("gainsboro").is_ok());
        assert!(Color::parse("white_smoke").is_ok());
        assert!(Color::parse("lavender").is_ok());
        assert!(Color::parse("slate_gray").is_ok());
        assert!(Color::parse("light_slate_gray").is_ok());
        assert!(Color::parse("light_steel_blue").is_ok());
    }

    #[test]
    fn test_all_standard_colors() {
        // Test all standard colors
        assert_eq!(StandardColor::Black.code(), 0);
        assert_eq!(StandardColor::Red.code(), 1);
        assert_eq!(StandardColor::Green.code(), 2);
        assert_eq!(StandardColor::Yellow.code(), 3);
        assert_eq!(StandardColor::Blue.code(), 4);
        assert_eq!(StandardColor::Magenta.code(), 5);
        assert_eq!(StandardColor::Cyan.code(), 6);
        assert_eq!(StandardColor::White.code(), 7);
        assert_eq!(StandardColor::BrightBlack.code(), 8);
        assert_eq!(StandardColor::BrightRed.code(), 9);
        assert_eq!(StandardColor::BrightGreen.code(), 10);
        assert_eq!(StandardColor::BrightYellow.code(), 11);
        assert_eq!(StandardColor::BrightBlue.code(), 12);
        assert_eq!(StandardColor::BrightMagenta.code(), 13);
        assert_eq!(StandardColor::BrightCyan.code(), 14);
        assert_eq!(StandardColor::BrightWhite.code(), 15);
    }

    #[test]
    fn test_all_standard_colors_from_code() {
        assert_eq!(StandardColor::from_code(0).ok(), Some(StandardColor::Black));
        assert_eq!(StandardColor::from_code(1).ok(), Some(StandardColor::Red));
        assert_eq!(StandardColor::from_code(2).ok(), Some(StandardColor::Green));
        assert_eq!(
            StandardColor::from_code(3).ok(),
            Some(StandardColor::Yellow)
        );
        assert_eq!(StandardColor::from_code(4).ok(), Some(StandardColor::Blue));
        assert_eq!(
            StandardColor::from_code(5).ok(),
            Some(StandardColor::Magenta)
        );
        assert_eq!(StandardColor::from_code(6).ok(), Some(StandardColor::Cyan));
        assert_eq!(StandardColor::from_code(7).ok(), Some(StandardColor::White));
        assert_eq!(
            StandardColor::from_code(8).ok(),
            Some(StandardColor::BrightBlack)
        );
        assert_eq!(
            StandardColor::from_code(9).ok(),
            Some(StandardColor::BrightRed)
        );
        assert_eq!(
            StandardColor::from_code(10).ok(),
            Some(StandardColor::BrightGreen)
        );
        assert_eq!(
            StandardColor::from_code(11).ok(),
            Some(StandardColor::BrightYellow)
        );
        assert_eq!(
            StandardColor::from_code(12).ok(),
            Some(StandardColor::BrightBlue)
        );
        assert_eq!(
            StandardColor::from_code(13).ok(),
            Some(StandardColor::BrightMagenta)
        );
        assert_eq!(
            StandardColor::from_code(14).ok(),
            Some(StandardColor::BrightCyan)
        );
        assert_eq!(
            StandardColor::from_code(15).ok(),
            Some(StandardColor::BrightWhite)
        );
    }

    #[test]
    fn test_all_standard_colors_ansi() {
        // Test ANSI codes for foreground
        assert_eq!(
            Color::Standard(StandardColor::Black).to_ansi_fg(),
            "\x1b[30m"
        );
        assert_eq!(
            Color::Standard(StandardColor::Green).to_ansi_fg(),
            "\x1b[32m"
        );
        assert_eq!(
            Color::Standard(StandardColor::Yellow).to_ansi_fg(),
            "\x1b[33m"
        );
        assert_eq!(
            Color::Standard(StandardColor::Blue).to_ansi_fg(),
            "\x1b[34m"
        );
        assert_eq!(
            Color::Standard(StandardColor::Magenta).to_ansi_fg(),
            "\x1b[35m"
        );
        assert_eq!(
            Color::Standard(StandardColor::Cyan).to_ansi_fg(),
            "\x1b[36m"
        );
        assert_eq!(
            Color::Standard(StandardColor::White).to_ansi_fg(),
            "\x1b[37m"
        );
        assert_eq!(
            Color::Standard(StandardColor::BrightBlack).to_ansi_fg(),
            "\x1b[90m"
        );
        assert_eq!(
            Color::Standard(StandardColor::BrightGreen).to_ansi_fg(),
            "\x1b[92m"
        );
        assert_eq!(
            Color::Standard(StandardColor::BrightYellow).to_ansi_fg(),
            "\x1b[93m"
        );
        assert_eq!(
            Color::Standard(StandardColor::BrightBlue).to_ansi_fg(),
            "\x1b[94m"
        );
        assert_eq!(
            Color::Standard(StandardColor::BrightMagenta).to_ansi_fg(),
            "\x1b[95m"
        );
        assert_eq!(
            Color::Standard(StandardColor::BrightCyan).to_ansi_fg(),
            "\x1b[96m"
        );
        assert_eq!(
            Color::Standard(StandardColor::BrightWhite).to_ansi_fg(),
            "\x1b[97m"
        );

        // Test ANSI codes for background
        assert_eq!(
            Color::Standard(StandardColor::Black).to_ansi_bg(),
            "\x1b[40m"
        );
        assert_eq!(
            Color::Standard(StandardColor::Green).to_ansi_bg(),
            "\x1b[42m"
        );
        assert_eq!(
            Color::Standard(StandardColor::Yellow).to_ansi_bg(),
            "\x1b[43m"
        );
        assert_eq!(
            Color::Standard(StandardColor::Blue).to_ansi_bg(),
            "\x1b[44m"
        );
        assert_eq!(
            Color::Standard(StandardColor::Magenta).to_ansi_bg(),
            "\x1b[45m"
        );
        assert_eq!(
            Color::Standard(StandardColor::Cyan).to_ansi_bg(),
            "\x1b[46m"
        );
        assert_eq!(
            Color::Standard(StandardColor::White).to_ansi_bg(),
            "\x1b[47m"
        );
        assert_eq!(
            Color::Standard(StandardColor::BrightBlack).to_ansi_bg(),
            "\x1b[100m"
        );
        assert_eq!(
            Color::Standard(StandardColor::BrightGreen).to_ansi_bg(),
            "\x1b[102m"
        );
        assert_eq!(
            Color::Standard(StandardColor::BrightYellow).to_ansi_bg(),
            "\x1b[103m"
        );
        assert_eq!(
            Color::Standard(StandardColor::BrightBlue).to_ansi_bg(),
            "\x1b[104m"
        );
        assert_eq!(
            Color::Standard(StandardColor::BrightMagenta).to_ansi_bg(),
            "\x1b[105m"
        );
        assert_eq!(
            Color::Standard(StandardColor::BrightCyan).to_ansi_bg(),
            "\x1b[106m"
        );
        assert_eq!(
            Color::Standard(StandardColor::BrightWhite).to_ansi_bg(),
            "\x1b[107m"
        );
    }

    #[test]
    fn test_hex_parsing_edge_cases() {
        // Invalid hex length
        assert!(Color::parse("#ff00").is_err());
        assert!(Color::parse("#ff00000").is_err());
        assert!(Color::parse("#").is_err());

        // Invalid hex characters
        assert!(Color::parse("#gg0000").is_err());
        assert!(Color::parse("#zzzzzz").is_err());
    }

    #[test]
    fn test_rgb_parsing_edge_cases() {
        // Missing components
        assert!(Color::parse("rgb(255)").is_err());
        assert!(Color::parse("rgb(255, 0)").is_err());

        // Invalid components
        assert!(Color::parse("rgb(abc, 0, 0)").is_err());
    }

    #[test]
    fn test_color_display_standard() {
        assert_eq!(Color::Standard(StandardColor::Red).to_string(), "Red");
        assert_eq!(
            Color::Standard(StandardColor::BrightBlue).to_string(),
            "BrightBlue"
        );
    }

    #[test]
    fn test_color_rgb_constructor() {
        let color = Color::rgb(100, 150, 200);
        assert_eq!(
            color,
            Color::Rgb {
                r: 100,
                g: 150,
                b: 200
            }
        );
    }
}
