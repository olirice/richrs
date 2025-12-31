//! Status display demo
use richrs::spinner::Spinner;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    eprintln!();
    eprintln!("  \x1b[1mStatus Spinner Demo\x1b[0m");
    eprintln!();

    // Show different spinners with status messages
    let spinners = [
        ("dots", "Loading configuration..."),
        ("arc", "Connecting to server..."),
        ("moon", "Processing data..."),
    ];

    for (spinner_name, message) in spinners {
        let mut spinner = Spinner::new(spinner_name).unwrap();

        // Show spinner animation for a few frames
        for _ in 0..12 {
            eprint!("\r  {} {}", spinner.next_frame(), message);
            io::stderr().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        }

        // Show completion
        eprintln!("\r  \x1b[32m✓\x1b[0m {}                    ", message.replace("...", " complete"));
        thread::sleep(Duration::from_millis(300));
    }

    eprintln!();
}
