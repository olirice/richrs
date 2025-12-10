//! Panels demo
use richrs::box_chars::BoxChars;
use richrs::prelude::*;

fn main() -> Result<()> {
    let mut console = Console::new();

    console.print("")?;

    let panel1 = Panel::new("Panels draw borders around content.")
        .title("Simple Panel")
        .box_chars(BoxChars::ROUNDED);
    console.write_segments(&panel1.render(50))?;
    console.print("")?;

    let panel2 = Panel::new("With titles, subtitles, and styles.")
        .title("Featured")
        .subtitle("subtitle")
        .box_chars(BoxChars::DOUBLE);
    console.write_segments(&panel2.render(50))?;
    console.print("")?;

    Ok(())
}
