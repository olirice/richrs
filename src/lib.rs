//! # richrs - Beautiful Terminal Output for Rust
//!
//! `richrs` is a Rust port of the [Rich](https://github.com/Textualize/rich) Python library,
//! providing beautiful terminal output with colors, styles, tables, progress bars, and more.
//!
//! ## Features
//!
//! - **Console**: The main interface for terminal output with markup support
//! - **Style**: Define text styles with colors and attributes
//! - **Text**: Rich text objects with inline styling
//! - **Table**: Beautiful data tables with borders and alignment
//! - **Panel**: Boxed content with titles and subtitles
//! - **Progress**: Progress bars with multiple concurrent tasks
//! - **Tree**: Hierarchical data visualization
//! - **Markup**: Console markup syntax for inline styling
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use richrs::prelude::*;
//!
//! let console = Console::new();
//! console.print("[bold red]Hello[/] [green]World![/]")?;
//! ```
//!
//! ## Compatibility
//!
//! This library aims for pixel-perfect compatibility with Python Rich output.
//! All options available in Python Rich should be available in `richrs`.

#![doc(html_root_url = "https://docs.rs/richrs/0.1.0")]

// Re-export core types
pub mod align;
pub mod box_chars;
pub mod color;
pub mod columns;
pub mod console;
pub mod emoji;
pub mod errors;
pub mod highlighter;
pub mod live;
pub mod logging;
pub mod markdown;
pub mod markup;
pub mod measure;
pub mod padding;
pub mod panel;
pub mod pretty;
pub mod progress;
pub mod prompt;
pub mod protocol;
pub mod rule;
pub mod segment;
pub mod spinner;
pub mod status;
pub mod style;
pub mod syntax;
pub mod table;
pub mod text;
pub mod theme;
pub mod traceback;
pub mod tree;

/// Prelude module for convenient imports.
///
/// # Example
///
/// ```rust,ignore
/// use richrs::prelude::*;
/// ```
pub mod prelude {

    pub use crate::align::Align;
    pub use crate::color::Color;
    pub use crate::columns::{ColumnAlign, Columns};
    pub use crate::console::{Console, ConsoleOptions, Justify, Overflow};
    pub use crate::emoji::Emoji;
    pub use crate::errors::{Error, Result};
    pub use crate::highlighter::{
        Highlighter, ISOHighlighter, JSONHighlighter, RegexHighlighter, ReprHighlighter,
    };
    pub use crate::live::Live;
    pub use crate::logging::{LogLevel, RichHandler};
    pub use crate::markdown::Markdown;
    pub use crate::markup::Markup;
    pub use crate::panel::Panel;
    pub use crate::pretty::{inspect, inspect_with_options, Pretty};
    pub use crate::progress::{Progress, ProgressBar, Task};
    pub use crate::prompt::{Confirm, FloatPrompt, IntPrompt, Prompt};
    pub use crate::rule::Rule;
    pub use crate::segment::Segment;
    pub use crate::spinner::Spinner;
    pub use crate::status::Status;
    pub use crate::style::Style;
    pub use crate::syntax::Syntax;
    pub use crate::table::{Column, Table};
    pub use crate::text::Text;
    pub use crate::theme::Theme;
    pub use crate::traceback::{format_error_chain, Frame, Traceback};
    pub use crate::tree::{Tree, TreeNode};
}
