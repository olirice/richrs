//! Status display demo
use richrs::status::Status;
use std::thread;
use std::time::Duration;

fn main() {
    eprintln!();

    // Demo 1: Simple status with dots spinner
    let mut status = Status::new("Loading configuration...");
    status.start();
    thread::sleep(Duration::from_secs(2));
    status.stop();
    eprintln!("  \x1b[32m✓\x1b[0m Configuration loaded");

    // Demo 2: Status with different spinner
    let mut status = Status::new("Connecting to server...")
        .spinner("arc")
        .unwrap();
    status.start();
    thread::sleep(Duration::from_secs(2));
    status.stop();
    eprintln!("  \x1b[32m✓\x1b[0m Connected");

    // Demo 3: Status with moon spinner
    let mut status = Status::new("Processing data...")
        .spinner("moon")
        .unwrap();
    status.start();
    thread::sleep(Duration::from_secs(2));
    status.stop();
    eprintln!("  \x1b[32m✓\x1b[0m Processing complete");

    eprintln!();
}
