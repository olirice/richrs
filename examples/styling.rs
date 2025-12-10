//! Text styling demo
use richrs::prelude::*;

fn main() -> Result<()> {
    let mut console = Console::new();

    console.print("")?;
    console.print("[bold]Bold[/], [italic]Italic[/], [underline]Underline[/], [strike]Strikethrough[/], [dim]Dim[/]")?;
    console.print("")?;
    console.print("Standard: [red]red[/] [green]green[/] [yellow]yellow[/] [blue]blue[/] [magenta]magenta[/] [cyan]cyan[/]")?;
    console.print("Bright:   [bright_red]bright[/] [bright_green]bright[/] [bright_yellow]bright[/] [bright_blue]bright[/] [bright_magenta]bright[/] [bright_cyan]bright[/]")?;
    console.print("")?;
    console.print("[bold red]Bold Red[/] | [italic green]Italic Green[/] | [underline blue]Underline Blue[/]")?;
    console.print("")?;

    Ok(())
}
