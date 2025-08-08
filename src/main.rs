// src/main.rs

mod tui;
mod scan;
mod utils;

use tui::start_tui;
use scan::run_scan;

fn main() {
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
