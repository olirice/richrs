//! Tables demo
use richrs::prelude::*;

fn main() -> Result<()> {
    let mut console = Console::new();

    console.print("")?;

    let mut table = Table::new();
    table.add_column(Column::new("Planet"));
    table.add_column(Column::new("Moons"));
    table.add_column(Column::new("Rings"));

    table.add_row_cells(["Earth", "1", "No"]);
    table.add_row_cells(["Mars", "2", "No"]);
    table.add_row_cells(["Saturn", "146", "Yes"]);
    table.add_row_cells(["Neptune", "16", "Yes"]);

    console.write_segments(&table.render(50))?;
    console.print("")?;

    Ok(())
}
