//! Syntax highlighting demo
use richrs::prelude::*;

fn main() -> Result<()> {
    let mut console = Console::new();

    eprintln!();
    eprintln!("  \x1b[1mRust Code\x1b[0m");

    let rust_code = r#"fn main() {
    let message = "Hello, World!";
    println!("{}", message);

    for i in 0..5 {
        println!("Count: {}", i);
    }
}
"#;

    let syntax = Syntax::new(rust_code, "rust")
        .line_numbers(true)
        .theme("base16-ocean.dark");

    console.write_segments(&syntax.render(80))?;

    eprintln!("  \x1b[1mPython Code\x1b[0m");

    let python_code = r#"def greet(name: str) -> str:
    """Return a greeting message."""
    return f"Hello, {name}!"

if __name__ == "__main__":
    print(greet("World"))
"#;

    let syntax = Syntax::new(python_code, "python")
        .line_numbers(true)
        .theme("base16-ocean.dark");

    console.write_segments(&syntax.render(80))?;
    eprintln!();

    Ok(())
}
