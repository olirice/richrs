//! Spinners demo
use richrs::prelude::*;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    eprintln!();

    let spinner_demos = [
        ("dots", "Loading..."),
        ("line", "Processing..."),
        ("arc", "Working..."),
        ("moon", "Please wait..."),
        ("arrow", "Fetching..."),
        ("bouncingBall", "Optimizing..."),
    ];

    let mut spinners: Vec<(Spinner, &str)> = spinner_demos
        .iter()
        .filter_map(|(name, msg)| Spinner::new(name).ok().map(|s| (s, *msg)))
        .collect();

    // Print initial
    for (spinner, message) in &mut spinners {
        let name = spinner.name().to_owned();
        let frame = spinner.next_frame();
        eprintln!("  {:14} {} {}", name, frame, message);
    }
    let _ = std::io::stderr().flush();

    for _ in 0..40 {
        eprint!("\x1b[{}A", spinners.len());

        for (spinner, message) in &mut spinners {
            let name = spinner.name().to_owned();
            let frame = spinner.next_frame();
            eprintln!("  \x1b[36m{:14}\x1b[0m {} {}", name, frame, message);
        }

        let _ = std::io::stderr().flush();
        thread::sleep(Duration::from_millis(80));
    }

    eprintln!();
    Ok(())
}
