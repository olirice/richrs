//! Live display demo
use richrs::live::Live;
use std::time::Duration;
use std::thread;

fn main() {
    eprintln!();
    eprintln!("  \x1b[1mLive Display Demo\x1b[0m");
    eprintln!();

    let mut live = Live::new()
        .refresh_per_second(8.0)
        .auto_refresh(true);

    live.start();

    // Simulate a countdown
    for i in (1..=10).rev() {
        live.update(format!("  Countdown: {} ", i));
        thread::sleep(Duration::from_millis(400));
    }

    live.stop();
    eprintln!("  \x1b[32m✓\x1b[0m Countdown complete!");
    eprintln!();

    // Second demo: updating status text
    let mut live = Live::new()
        .refresh_per_second(8.0)
        .auto_refresh(true);

    live.start();

    let stages = [
        "Initializing...",
        "Loading modules...",
        "Connecting...",
        "Authenticating...",
        "Syncing data...",
        "Finalizing...",
    ];

    for stage in &stages {
        live.update(format!("  {} ", stage));
        thread::sleep(Duration::from_millis(600));
    }

    live.stop();
    eprintln!("  \x1b[32m✓\x1b[0m All stages complete!");
    eprintln!();
}
