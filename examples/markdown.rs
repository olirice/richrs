//! Markdown rendering demo
use richrs::prelude::*;

fn main() -> Result<()> {
    let mut console = Console::new();

    eprintln!();

    let md = r#"# Welcome to richrs

A **Rust** port of the Python *Rich* library.

## Features

- Beautiful terminal output
- Markdown rendering
- Syntax highlighting
- Progress bars and spinners

## Code Example

Use `Console::print()` for styled output.

> Rich output makes CLI tools more user-friendly
> and easier to understand at a glance.

---

Visit the [documentation](https://docs.rs/richrs) for more details.
"#;

    let markdown = Markdown::new(md);
    console.write_segments(&markdown.render(70))?;
    eprintln!();

    Ok(())
}
