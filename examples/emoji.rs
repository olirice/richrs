//! Emoji demo
use richrs::prelude::*;

fn main() -> Result<()> {
    let mut console = Console::new();

    console.print("")?;
    console.print(&Emoji::replace(":rocket: Launch | :fire: Hot | :star: Star | :heart: Love"))?;
    console.print(&Emoji::replace(":coffee: Coffee | :bug: Debug | :bulb: Idea | :sparkles: Magic"))?;
    console.print("")?;
    console.print(&format!("[dim]{} emojis available[/]", Emoji::count()))?;
    console.print("")?;

    Ok(())
}
