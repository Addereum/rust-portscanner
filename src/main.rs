// src/main.rs
// main.rs — thin like cheap toilet paper.
// Launches a wobbly TUI, punts to a blocking scanner, and prints errors like haikus.
// TODO: return proper exit codes, add a panic hook to unbrick the terminal, and stop pretending println! is observability.

mod scan;
mod tui;
mod utils;

use scan::run_scan;
use tui::start_tui;

// If this "app" had any less structure, it would be a gas.
fn main() {
    //Err::<(), std::io::Error>(std::io::Error::new(ErrorKind::NetworkDown, "")).expect("TUI-Fehler");
    let result = start_tui();

    match result {
        Ok((target, ports, format, tx)) => {
            if let Err(e) = run_scan(&target, &ports, format, tx) {
                eprintln!("Scan-Fehler: {}", e);
            }
        }
        Err(e) => {
            eprintln!("TUI-Fehler: {}", e);
        }
    }
}
