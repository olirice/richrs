//! Test runner binary for bilingual Python/Rust testing.
//!
//! This binary accepts JSON input describing tests to run and outputs
//! JSON results that can be compared with Python Rich output.

use richrs::color::Color;
use richrs::console::{Console, StringWriter};
use richrs::emoji::Emoji;
use richrs::panel::Panel;
use richrs::rule::Rule;
use richrs::style::Style;
use richrs::table::{Column, Table};
use richrs::text::Text;
use richrs::tree::{Tree, TreeNode};
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};

/// Test request from Python test harness.
#[derive(Debug, Deserialize)]
struct TestRequest {
    /// Name of the test to run.
    test: String,
    /// Test-specific data.
    data: serde_json::Value,
}

/// Test response to Python test harness.
#[derive(Debug, Serialize)]
struct TestResponse {
    /// Whether the test succeeded.
    success: bool,
    /// Test output (ANSI string or other data).
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<String>,
    /// Error message if test failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    /// Additional structured data.
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

/// Style data for comparison.
#[derive(Debug, Serialize)]
struct StyleData {
    /// Input style string.
    input: String,
    /// Bold attribute.
    bold: Option<bool>,
    /// Italic attribute.
    italic: Option<bool>,
    /// Underline attribute.
    underline: Option<bool>,
    /// Strike attribute.
    strike: Option<bool>,
    /// Dim attribute.
    dim: Option<bool>,
    /// Reverse attribute.
    reverse: Option<bool>,
    /// Blink attribute.
    blink: Option<bool>,
    /// Conceal attribute.
    conceal: Option<bool>,
    /// Foreground color as string.
    color: Option<String>,
    /// Background color as string.
    bgcolor: Option<String>,
}

impl StyleData {
    /// Creates style data from a parsed style.
    fn from_style(input: &str, style: &Style) -> Self {
        Self {
            input: input.to_owned(),
            bold: style.attributes.bold,
            italic: style.attributes.italic,
            underline: style.attributes.underline,
            strike: style.attributes.strike,
            dim: style.attributes.dim,
            reverse: style.attributes.reverse,
            blink: style.attributes.blink,
            conceal: style.attributes.conceal,
            color: style.color.as_ref().map(color_to_string),
            bgcolor: style.bgcolor.as_ref().map(color_to_string),
        }
    }
}

/// Converts a color to a string representation matching Python Rich.
fn color_to_string(color: &Color) -> String {
    match *color {
        Color::Default => "default".to_owned(),
        Color::Standard(std) => format!("{std:?}").to_lowercase(),
        Color::Palette(idx) => format!("color({idx})"),
        Color::Rgb { r, g, b } => format!("#{r:02x}{g:02x}{b:02x}"),
        // Handle any future color variants
        _ => "unknown".to_owned(),
    }
}

/// Runs a style parsing test.
fn run_style_parse_test(data: &serde_json::Value) -> TestResponse {
    let style_str = match data.get("style").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => {
            return TestResponse {
                success: false,
                output: None,
                error: Some("missing 'style' field in test data".to_owned()),
                data: None,
            };
        }
    };

    match Style::parse(style_str) {
        Ok(style) => {
            let style_data = StyleData::from_style(style_str, &style);
            TestResponse {
                success: true,
                output: None,
                error: None,
                data: serde_json::to_value(style_data).ok(),
            }
        }
        Err(e) => TestResponse {
            success: false,
            output: None,
            error: Some(format!("{e}")),
            data: None,
        },
    }
}

/// Runs a style rendering test (outputs ANSI).
fn run_style_render_test(data: &serde_json::Value) -> TestResponse {
    let style_str = match data.get("style").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => {
            return TestResponse {
                success: false,
                output: None,
                error: Some("missing 'style' field in test data".to_owned()),
                data: None,
            };
        }
    };

    let text_str = data
        .get("text")
        .and_then(|v| v.as_str())
        .unwrap_or("test");

    match Style::parse(style_str) {
        Ok(style) => {
            let ansi = style.to_ansi();
            let reset = style.to_ansi_reset();
            let output = format!("{ansi}{text_str}{reset}");
            TestResponse {
                success: true,
                output: Some(output),
                error: None,
                data: None,
            }
        }
        Err(e) => TestResponse {
            success: false,
            output: None,
            error: Some(format!("{e}")),
            data: None,
        },
    }
}

/// Runs a text rendering test.
fn run_text_render_test(data: &serde_json::Value) -> TestResponse {
    let text_str = match data.get("text").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => {
            return TestResponse {
                success: false,
                output: None,
                error: Some("missing 'text' field in test data".to_owned()),
                data: None,
            };
        }
    };

    let style_str = data.get("style").and_then(|v| v.as_str());

    let text = if let Some(style_str) = style_str {
        match Style::parse(style_str) {
            Ok(style) => Text::styled(text_str, style),
            Err(e) => {
                return TestResponse {
                    success: false,
                    output: None,
                    error: Some(format!("{e}")),
                    data: None,
                };
            }
        }
    } else {
        Text::from_str(text_str)
    };

    // width is reserved for future wrapping support
    let _width = data.get("width").and_then(|v| v.as_u64()).unwrap_or(80);

    let mut output = String::new();
    for segment in text.to_segments() {
        output.push_str(&segment.to_ansi());
    }

    TestResponse {
        success: true,
        output: Some(output),
        error: None,
        data: None,
    }
}

/// Runs a panel rendering test.
fn run_panel_render_test(data: &serde_json::Value) -> TestResponse {
    let content = match data.get("content").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => {
            return TestResponse {
                success: false,
                output: None,
                error: Some("missing 'content' field in test data".to_owned()),
                data: None,
            };
        }
    };

    let title = data.get("title").and_then(|v| v.as_str());
    let subtitle = data.get("subtitle").and_then(|v| v.as_str());
    let width = data.get("width").and_then(|v| v.as_u64()).unwrap_or(80);
    let width = usize::try_from(width).unwrap_or(80);

    let text = Text::from_str(content);
    let mut panel = Panel::new(text);

    if let Some(title) = title {
        panel = panel.title(title);
    }
    if let Some(subtitle) = subtitle {
        panel = panel.subtitle(subtitle);
    }

    let segments = panel.render(width);
    let mut output = String::new();
    for segment in segments {
        output.push_str(&segment.to_ansi());
    }

    TestResponse {
        success: true,
        output: Some(output),
        error: None,
        data: None,
    }
}

/// Runs a console print test with markup.
fn run_console_render_test(data: &serde_json::Value) -> TestResponse {
    let markup = match data.get("markup").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => {
            return TestResponse {
                success: false,
                output: None,
                error: Some("missing 'markup' field in test data".to_owned()),
                data: None,
            };
        }
    };

    let width = data.get("width").and_then(|v| v.as_u64()).unwrap_or(80);
    let width = usize::try_from(width).unwrap_or(80);

    let writer = StringWriter::new();
    let mut console = Console::with_writer(writer);
    console.set_width(width);

    match console.print(markup) {
        Ok(()) => {
            // We can't easily get the output from StringWriter through Console
            // so we use a different approach - render markup directly
            let output = render_markup_to_string(markup, width);
            TestResponse {
                success: true,
                output: Some(output),
                error: None,
                data: None,
            }
        }
        Err(e) => TestResponse {
            success: false,
            output: None,
            error: Some(format!("{e}")),
            data: None,
        },
    }
}

/// Renders markup to a string (simplified version).
fn render_markup_to_string(markup: &str, _width: usize) -> String {
    // Parse the markup and render to ANSI
    // This is a simplified version - for full compatibility we'd need to
    // extract the StringWriter's contents from Console
    use richrs::markup::Markup;

    match Markup::parse(markup) {
        Ok(parsed) => {
            let text = parsed.to_text();
            let mut output = String::new();
            for segment in text.to_segments() {
                output.push_str(&segment.to_ansi());
            }
            output
        }
        Err(_) => markup.to_owned(),
    }
}

/// Runs a color parsing test.
fn run_color_parse_test(data: &serde_json::Value) -> TestResponse {
    let color_str = match data.get("color").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => {
            return TestResponse {
                success: false,
                output: None,
                error: Some("missing 'color' field in test data".to_owned()),
                data: None,
            };
        }
    };

    match Color::parse(color_str) {
        Ok(color) => {
            let color_data = serde_json::json!({
                "input": color_str,
                "display": color.to_string(),
                "ansi_fg": color.to_ansi_fg(),
                "ansi_bg": color.to_ansi_bg(),
            });
            TestResponse {
                success: true,
                output: None,
                error: None,
                data: Some(color_data),
            }
        }
        Err(e) => TestResponse {
            success: false,
            output: None,
            error: Some(format!("{e}")),
            data: None,
        },
    }
}

/// Runs an emoji lookup test.
fn run_emoji_lookup_test(data: &serde_json::Value) -> TestResponse {
    let name = match data.get("name").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => {
            return TestResponse {
                success: false,
                output: None,
                error: Some("missing 'name' field in test data".to_owned()),
                data: None,
            };
        }
    };

    match Emoji::new(name) {
        Ok(emoji) => TestResponse {
            success: true,
            output: Some(emoji.to_string()),
            error: None,
            data: None,
        },
        Err(e) => TestResponse {
            success: false,
            output: None,
            error: Some(format!("{e}")),
            data: None,
        },
    }
}

/// Runs a table rendering test.
fn run_table_render_test(data: &serde_json::Value) -> TestResponse {
    let columns = match data.get("columns").and_then(|v| v.as_array()) {
        Some(cols) => cols
            .iter()
            .filter_map(|c| c.as_str())
            .collect::<Vec<_>>(),
        None => {
            return TestResponse {
                success: false,
                output: None,
                error: Some("missing 'columns' field in test data".to_owned()),
                data: None,
            };
        }
    };

    let rows = match data.get("rows").and_then(|v| v.as_array()) {
        Some(r) => r,
        None => {
            return TestResponse {
                success: false,
                output: None,
                error: Some("missing 'rows' field in test data".to_owned()),
                data: None,
            };
        }
    };

    let width = data.get("width").and_then(|v| v.as_u64()).unwrap_or(80);
    let width = usize::try_from(width).unwrap_or(80);

    let mut table = Table::new();

    for col_name in &columns {
        table.add_column(Column::new(*col_name));
    }

    for row in rows {
        if let Some(cells) = row.as_array() {
            let cell_strs: Vec<&str> = cells.iter().filter_map(|c| c.as_str()).collect();
            table.add_row_cells(cell_strs);
        }
    }

    let segments = table.render(width);
    let mut output = String::new();
    for segment in segments {
        output.push_str(&segment.to_ansi());
    }

    TestResponse {
        success: true,
        output: Some(output),
        error: None,
        data: None,
    }
}

/// Runs a rule rendering test.
fn run_rule_render_test(data: &serde_json::Value) -> TestResponse {
    let title = data.get("title").and_then(|v| v.as_str());
    let width = data.get("width").and_then(|v| v.as_u64()).unwrap_or(80);
    let width = usize::try_from(width).unwrap_or(80);

    let rule = if let Some(title) = title {
        Rule::with_title(title)
    } else {
        Rule::new()
    };

    let segments = rule.render(width);
    let mut output = String::new();
    for segment in segments {
        output.push_str(&segment.to_ansi());
    }

    TestResponse {
        success: true,
        output: Some(output),
        error: None,
        data: None,
    }
}

/// Runs a tree rendering test.
fn run_tree_render_test(data: &serde_json::Value) -> TestResponse {
    let label = data.get("label").and_then(|v| v.as_str()).unwrap_or("root");
    let children = data.get("children").and_then(|v| v.as_array());

    let mut tree = Tree::new(label);

    // Add children if provided
    if let Some(children) = children {
        for child in children {
            if let Some(child_label) = child.as_str() {
                tree.add(TreeNode::new(child_label));
            }
        }
    }

    let segments = tree.render();
    let mut output = String::new();
    for segment in segments {
        output.push_str(&segment.to_ansi());
    }

    TestResponse {
        success: true,
        output: Some(output),
        error: None,
        data: None,
    }
}

/// Dispatches to the appropriate test handler.
fn run_test(request: &TestRequest) -> TestResponse {
    match request.test.as_str() {
        "style_parse" => run_style_parse_test(&request.data),
        "style_render" => run_style_render_test(&request.data),
        "text_render" => run_text_render_test(&request.data),
        "panel_render" => run_panel_render_test(&request.data),
        "console_render" => run_console_render_test(&request.data),
        "color_parse" => run_color_parse_test(&request.data),
        "emoji_lookup" => run_emoji_lookup_test(&request.data),
        "table_render" => run_table_render_test(&request.data),
        "rule_render" => run_rule_render_test(&request.data),
        "tree_render" => run_tree_render_test(&request.data),
        _ => TestResponse {
            success: false,
            output: None,
            error: Some(format!("unknown test: '{}'", request.test)),
            data: None,
        },
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                let response = TestResponse {
                    success: false,
                    output: None,
                    error: Some(format!("failed to read input: {e}")),
                    data: None,
                };
                if let Ok(json) = serde_json::to_string(&response) {
                    let _ = writeln!(stdout, "{json}");
                }
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        let request: TestRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let response = TestResponse {
                    success: false,
                    output: None,
                    error: Some(format!("failed to parse request: {e}")),
                    data: None,
                };
                if let Ok(json) = serde_json::to_string(&response) {
                    let _ = writeln!(stdout, "{json}");
                }
                continue;
            }
        };

        let response = run_test(&request);
        if let Ok(json) = serde_json::to_string(&response) {
            let _ = writeln!(stdout, "{json}");
        }
        let _ = stdout.flush();
    }
}
