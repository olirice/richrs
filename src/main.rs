//! richrs - Beautiful Terminal Output for Rust
//!
//! This binary provides an interactive demonstration showcasing all richrs capabilities.
//!
//! Run with: `cargo run --release`

use richrs::box_chars::BoxChars;
use richrs::prelude::*;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let mut console = Console::new();
    let width = console.width().min(100);

    // Clear screen and show header
    clear_screen();

    // ═══════════════════════════════════════════════════════════════════════════
    // HEADER
    // ═══════════════════════════════════════════════════════════════════════════
    console.print("")?;
    let header = Panel::new(Text::styled(
        "richrs - Beautiful Terminal Output for Rust",
        Style::parse("bold white")?,
    ))
    .box_chars(BoxChars::DOUBLE)
    .border_style(Style::parse("bright_blue")?);
    console.write_segments(&header.render(width))?;
    console.print("")?;

    console.print("[bold bright_green]Welcome to richrs![/] A Rust port of Python's [bold cyan]Rich[/] library.")?;
    console.print("This demo showcases beautiful terminal output capabilities.")?;
    console.print("")?;
    console.flush()?;

    // ═══════════════════════════════════════════════════════════════════════════
    // SECTION: Text Styling
    // ═══════════════════════════════════════════════════════════════════════════
    section_header(&mut console, "Text Styling")?;

    console.print("[bold]Bold[/], [italic]Italic[/], [underline]Underline[/], [strike]Strikethrough[/], [dim]Dim[/]")?;
    console.print("")?;

    console.print("Standard: [red]red[/] [green]green[/] [yellow]yellow[/] [blue]blue[/] [magenta]magenta[/] [cyan]cyan[/] [white]white[/]")?;
    console.print("Bright:   [bright_red]bright[/] [bright_green]bright[/] [bright_yellow]bright[/] [bright_blue]bright[/] [bright_magenta]bright[/] [bright_cyan]bright[/] [bright_white]bright[/]")?;
    console.print("")?;

    console.print("[bold red]Bold Red[/] | [italic green]Italic Green[/] | [underline blue]Underline Blue[/] | [bold italic yellow]Bold Italic Yellow[/]")?;
    console.print("")?;
    console.flush()?;

    pause(1);

    // ═══════════════════════════════════════════════════════════════════════════
    // SECTION: Panels
    // ═══════════════════════════════════════════════════════════════════════════
    section_header(&mut console, "Panels")?;

    let panel1 = Panel::new("Panels draw borders around content with customizable styles.")
        .title("Simple Panel")
        .box_chars(BoxChars::ROUNDED);
    console.write_segments(&panel1.render(65))?;
    console.print("")?;

    let panel2 = Panel::new(
        "Panels support titles, subtitles, and various box styles.\n\
         They wrap content automatically and can be styled.",
    )
    .title("Featured Panel")
    .subtitle("with subtitle")
    .box_chars(BoxChars::DOUBLE);
    console.write_segments(&panel2.render(65))?;
    console.print("")?;

    console.print("[dim]Box styles: ROUNDED, SQUARE, MINIMAL, HEAVY, DOUBLE, ASCII[/]")?;
    console.print("")?;
    console.flush()?;

    pause(1);

    // ═══════════════════════════════════════════════════════════════════════════
    // SECTION: Tables
    // ═══════════════════════════════════════════════════════════════════════════
    section_header(&mut console, "Tables")?;

    let mut table = Table::new();
    table.add_column(Column::new("Feature"));
    table.add_column(Column::new("Description"));
    table.add_column(Column::new("Status"));

    table.add_row_cells(["Console", "Terminal output interface", "Complete"]);
    table.add_row_cells(["Markup", "BBCode-like styling syntax", "Complete"]);
    table.add_row_cells(["Panel", "Boxed content with borders", "Complete"]);
    table.add_row_cells(["Table", "Tabular data display", "Complete"]);
    table.add_row_cells(["Tree", "Hierarchical visualization", "Complete"]);
    table.add_row_cells(["Progress", "Progress bar tracking", "Complete"]);
    table.add_row_cells(["Spinner", "Animated indicators", "Complete"]);
    table.add_row_cells(["Emoji", "670+ emoji mappings", "Complete"]);

    console.write_segments(&table.render(width.min(70)))?;
    console.print("")?;
    console.flush()?;

    pause(1);

    // ═══════════════════════════════════════════════════════════════════════════
    // SECTION: Trees
    // ═══════════════════════════════════════════════════════════════════════════
    section_header(&mut console, "Trees")?;

    let mut tree = Tree::new("richrs");
    tree.add(
        TreeNode::new("core")
            .with_child(TreeNode::new("console"))
            .with_child(TreeNode::new("style"))
            .with_child(TreeNode::new("color"))
            .with_child(TreeNode::new("text")),
    );
    tree.add(
        TreeNode::new("components")
            .with_child(TreeNode::new("panel"))
            .with_child(TreeNode::new("table"))
            .with_child(TreeNode::new("tree"))
            .with_child(TreeNode::new("progress")),
    );
    tree.add(
        TreeNode::new("extras")
            .with_child(TreeNode::new("emoji"))
            .with_child(TreeNode::new("spinner"))
            .with_child(TreeNode::new("highlighter")),
    );

    console.write_segments(&tree.render())?;
    console.print("")?;
    console.flush()?;

    pause(1);

    // ═══════════════════════════════════════════════════════════════════════════
    // SECTION: Emoji
    // ═══════════════════════════════════════════════════════════════════════════
    section_header(&mut console, "Emoji")?;

    let emoji_text = Emoji::replace(
        ":rocket: Launch | :fire: Hot | :star: Star | :heart: Love | :thumbs_up: Yes | :warning: Alert",
    );
    console.print(&emoji_text)?;

    let emoji_text2 = Emoji::replace(
        ":coffee: Coffee | :bug: Debug | :bulb: Idea | :lock: Secure | :gear: Settings | :sparkles: Magic",
    );
    console.print(&emoji_text2)?;
    console.print("")?;

    console.print(&format!(
        "[dim]richrs includes {} emoji definitions[/]",
        Emoji::count()
    ))?;
    console.print("")?;
    console.flush()?;

    pause(1);

    // ═══════════════════════════════════════════════════════════════════════════
    // SECTION: Spinners (Full Animation)
    // ═══════════════════════════════════════════════════════════════════════════
    section_header(&mut console, "Spinners (Live Animation)")?;

    console.print("[bold]Watch the spinners animate:[/]")?;
    console.print("")?;
    console.flush()?;

    // Animate spinners
    animate_spinners();

    console.print("")?;
    console.print(&format!(
        "[dim]richrs includes {} spinner animations[/]",
        Spinner::count()
    ))?;
    console.print("")?;
    console.flush()?;

    pause(1);

    // ═══════════════════════════════════════════════════════════════════════════
    // SECTION: Progress Bars (Full Animation)
    // ═══════════════════════════════════════════════════════════════════════════
    section_header(&mut console, "Progress Bars (Live Animation)")?;

    console.print("[bold]Watch the progress bars fill:[/]")?;
    console.print("")?;
    console.flush()?;

    animate_progress_bars();

    console.print("")?;
    console.flush()?;

    pause(1);

    // ═══════════════════════════════════════════════════════════════════════════
    // SECTION: Rules
    // ═══════════════════════════════════════════════════════════════════════════
    section_header(&mut console, "Rules / Dividers")?;

    console.print("Rules create horizontal dividers with optional titles:")?;
    console.print("")?;

    console.rule(None)?;
    console.rule(Some("Centered Title"))?;
    console.print("")?;
    console.flush()?;

    pause(1);

    // ═══════════════════════════════════════════════════════════════════════════
    // SECTION: Markup Syntax
    // ═══════════════════════════════════════════════════════════════════════════
    section_header(&mut console, "Markup Syntax")?;

    console.print("richrs supports BBCode-like markup for inline styling:")?;
    console.print("")?;

    let markup_content = Markup::parse(
        "[bold]\\[bold]text\\[/][/]             - Bold text\n\
         [italic]\\[italic]text\\[/][/]           - Italic text\n\
         [red]\\[red]text\\[/][/]               - Colored text\n\
         [bold red]\\[bold red]text\\[/][/]         - Combined styles\n\
         [red on white]\\[red on white]text\\[/][/]     - Background colors\n\
         [#ff6b6b]\\[#ff6b6b]text\\[/][/]           - Hex colors",
    )?;
    let code_panel = Panel::new(markup_content.to_text())
        .title("Markup Examples")
        .box_chars(BoxChars::ROUNDED);
    console.write_segments(&code_panel.render(55))?;
    console.print("")?;
    console.flush()?;

    pause(1);

    // ═══════════════════════════════════════════════════════════════════════════
    // SECTION: Status (Animated)
    // ═══════════════════════════════════════════════════════════════════════════
    section_header(&mut console, "Status Spinner")?;

    console.print("[bold]Running a task with status indicator:[/]")?;
    console.print("")?;
    console.flush()?;

    animate_status()?;

    console.print("")?;
    console.flush()?;

    pause(1);

    // ═══════════════════════════════════════════════════════════════════════════
    // SECTION: Live Display (Animated)
    // ═══════════════════════════════════════════════════════════════════════════
    section_header(&mut console, "Live Display")?;

    console.print("[bold]Real-time content updates:[/]")?;
    console.print("")?;
    console.flush()?;

    animate_live_display();

    console.print("")?;
    console.flush()?;

    pause(1);

    // ═══════════════════════════════════════════════════════════════════════════
    // FOOTER
    // ═══════════════════════════════════════════════════════════════════════════
    console.rule(None)?;
    console.print("")?;

    let rocket = Emoji::replace(":rocket:");
    let star = Emoji::replace(":star:");
    console.print(&format!(
        "{} [bold bright_green]Thank you for trying richrs![/] {} \
         Run [cyan]cargo doc --open[/] for full API docs.",
        rocket, star
    ))?;
    console.print("")?;

    console.flush()?;
    Ok(())
}

/// Clears the terminal screen.
fn clear_screen() {
    eprint!("\x1b[2J\x1b[H");
    let _ = std::io::stderr().flush();
}

/// Prints a section header with a rule.
fn section_header(console: &mut Console, title: &str) -> Result<()> {
    console.rule(Some(title))?;
    console.print("")?;
    console.flush()?;
    Ok(())
}

/// Pauses for a number of seconds.
fn pause(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}

/// Animates multiple spinners for demonstration.
fn animate_spinners() {
    let spinner_demos = [
        ("dots", "Loading data..."),
        ("line", "Processing files..."),
        ("arc", "Analyzing results..."),
        ("moon", "Computing values..."),
        ("arrow", "Fetching resources..."),
        ("toggle", "Syncing state..."),
        ("bouncingBall", "Optimizing..."),
        ("clock", "Waiting..."),
    ];

    let mut spinners: Vec<(Spinner, &str)> = spinner_demos
        .iter()
        .filter_map(|(name, msg)| Spinner::new(name).ok().map(|s| (s, *msg)))
        .collect();

    // Run animation for ~4 seconds
    let iterations = 50;
    let frame_delay = Duration::from_millis(80);

    // Print initial lines
    for (spinner, message) in &mut spinners {
        let name = spinner.name().to_owned();
        let frame = spinner.next_frame();
        eprintln!("  \x1b[36m{:14}\x1b[0m {} {}", name, frame, message);
    }
    let _ = std::io::stderr().flush();

    for _ in 1..iterations {
        // Move cursor up to overwrite previous output
        eprint!("\x1b[{}A", spinners.len());

        for (spinner, message) in &mut spinners {
            let name = spinner.name().to_owned();
            let frame = spinner.next_frame();
            eprintln!("  \x1b[36m{:14}\x1b[0m {} {}", name, frame, message);
        }

        let _ = std::io::stderr().flush();
        thread::sleep(frame_delay);
    }

    // Clear spinner lines and print completion
    eprint!("\x1b[{}A", spinners.len());
    for _ in &spinners {
        eprintln!("\x1b[2K  \x1b[32m\x1b[0m Done!                              ");
    }
    let _ = std::io::stderr().flush();
}

/// Animates progress bars for demonstration using the Progress API.
fn animate_progress_bars() {
    // Use the actual Progress API from richrs
    let mut progress = Progress::new();

    // Add tasks with different totals
    let task1 = progress.add_task("Downloading", Some(100), true);
    let task2 = progress.add_task("Installing", Some(75), true);
    let task3 = progress.add_task("Compiling", Some(120), true);

    let tasks = [
        (task1, 100_u64, 2_u64),
        (task2, 75_u64, 3_u64),
        (task3, 120_u64, 1_u64),
    ];
    let total_steps = 70;
    let step_delay = Duration::from_millis(50);

    // Print initial state
    let initial_output = progress.render(80);
    eprint!("{}", initial_output.to_ansi());
    let _ = std::io::stderr().flush();

    // Animate
    for _ in 0..total_steps {
        // Move cursor up
        eprint!("\x1b[{}A", tasks.len());

        // Update each task
        for (task_id, total, increment) in &tasks {
            let task = progress.get_task(*task_id);
            if let Some(t) = task {
                if t.completed < *total {
                    let _ = progress.advance(*task_id, *increment);
                }
            }
        }

        // Render and display
        let output = progress.render(80);
        eprint!("{}", output.to_ansi());
        let _ = std::io::stderr().flush();

        thread::sleep(step_delay);

        // Check if all done
        if progress.finished() {
            break;
        }
    }
}

/// Animates a status spinner.
fn animate_status() -> Result<()> {
    let mut spinner = Spinner::new("dots")?;
    let messages = [
        "Connecting to server...",
        "Authenticating...",
        "Loading configuration...",
        "Initializing components...",
        "Starting services...",
    ];

    for (i, message) in messages.iter().enumerate() {
        let iterations = 15;
        for _ in 0..iterations {
            let frame = spinner.next_frame();
            eprint!("\r  {} {}", frame, message);
            let _ = std::io::stderr().flush();
            thread::sleep(Duration::from_millis(80));
        }

        // Show completion
        if i < messages.len().saturating_sub(1) {
            eprintln!(
                "\r  \x1b[32m\x1b[0m {}                    ",
                message.replace("...", " - done")
            );
        } else {
            eprintln!("\r  \x1b[32m\x1b[0m All systems ready!              ");
        }
    }

    Ok(())
}

/// Animates a live display.
fn animate_live_display() {
    let items: [(&str, usize, usize, &str); 4] = [
        ("CPU Usage", 45, 78, "%"),
        ("Memory", 2048, 3584, " MB"),
        ("Requests", 1250, 2340, "/s"),
        ("Latency", 12, 8, " ms"),
    ];

    let iterations: usize = 30;
    let delay = Duration::from_millis(100);

    // Print initial state
    for (name, start, _, unit) in &items {
        eprintln!("  {:12}: {:>5}{}", name, start, unit);
    }
    let _ = std::io::stderr().flush();

    for step in 0..iterations {
        eprint!("\x1b[{}A", items.len());

        for (name, start, end, unit) in &items {
            // Interpolate value
            let progress = step.saturating_mul(100) / iterations;
            let diff = if end > start {
                end.saturating_sub(*start)
            } else {
                start.saturating_sub(*end)
            };
            let delta = diff.saturating_mul(progress) / 100;
            let value = if end > start {
                start.saturating_add(delta)
            } else {
                start.saturating_sub(delta)
            };

            // Color based on trend (yellow=increasing, green=decreasing)
            let color = if end > start { "\x1b[33m" } else { "\x1b[32m" };
            // Clear line and print with fixed width value field
            eprintln!("\x1b[2K  {:12}: {}{:>5}\x1b[0m{}", name, color, value, unit);
        }

        let _ = std::io::stderr().flush();
        thread::sleep(delay);
    }

    // Final state - clear and print clean
    eprint!("\x1b[{}A", items.len());
    for (name, _, end, unit) in &items {
        eprintln!("\x1b[2K  {:12}: {:>5}{}", name, end, unit);
    }
    let _ = std::io::stderr().flush();
}
