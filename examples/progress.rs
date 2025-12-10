//! Progress bars demo
use richrs::prelude::*;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    eprintln!();

    let mut progress = Progress::new();
    let task1 = progress.add_task("Downloading", Some(100), true);
    let task2 = progress.add_task("Installing", Some(80), true);
    let task3 = progress.add_task("Compiling", Some(120), true);

    let tasks = [(task1, 100_u64, 3_u64), (task2, 80_u64, 4_u64), (task3, 120_u64, 2_u64)];

    // Print initial
    let output = progress.render(80);
    eprint!("{}", output.to_ansi());
    let _ = std::io::stderr().flush();

    for _ in 0..50 {
        eprint!("\x1b[{}A", 3);

        for (task_id, total, increment) in &tasks {
            if let Some(t) = progress.get_task(*task_id) {
                if t.completed < *total {
                    let _ = progress.advance(*task_id, *increment);
                }
            }
        }

        let output = progress.render(80);
        eprint!("{}", output.to_ansi());
        let _ = std::io::stderr().flush();
        thread::sleep(Duration::from_millis(80));

        if progress.finished() {
            break;
        }
    }

    eprintln!();
    Ok(())
}
