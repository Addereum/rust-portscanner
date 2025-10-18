// src/main.rs
mod cli;
mod io_utils;
mod scan;
mod tui;
mod utils;

use anyhow::Result;
use clap::Parser;
use std::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = cli::Opts::parse();

    // If user requested TUI, run TUI and collect options from it
    if opts.tui {
        match tui::start_tui() {
            Ok((target, ports, format_str, tx)) => {
                // convert format_str to enum
                let format = match format_str {
                    "html" => cli::Format::Html,
                    "zip" => cli::Format::Zip,
                    _ => cli::Format::Txt,
                };

                // parse ports
                let ports_vec = match utils::parse_ports(&ports) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Port parse error: {}", e);
                        return Ok(());
                    }
                };

                // expand single target (no CIDR in TUI)
                let targets = vec![target];

                let sender = Some(tx);
                let results = scan::run_scan_async(&targets, &ports_vec, 300, 200, sender).await?;
                io_utils::export_results(&results, format).await?;
                return Ok(());
            }
            Err(e) => {
                eprintln!("TUI error: {}", e);
                return Ok(());
            }
        }
    }

    // Non-TUI CLI path
    // Expand targets (max 1024 host entries by default)
    let targets = cli::expand_targets(&opts.targets, 1024)?;
    let ports = utils::parse_ports(&opts.ports)?;

    // Create a simple logging channel for progress (prints to stdout)
    let (tx_log, rx_log) = mpsc::channel::<String>();
    std::thread::spawn(move || {
        while let Ok(line) = rx_log.recv() {
            println!("{}", line);
        }
    });

    // run async scan
    let results = scan::run_scan_async(
        &targets,
        &ports,
        opts.timeout,
        opts.concurrency,
        Some(tx_log),
    )
    .await?;

    // export results
    io_utils::export_results(&results, opts.format).await?;

    println!("Done.");
    Ok(())
}
