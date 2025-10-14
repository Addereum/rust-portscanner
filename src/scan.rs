// src/scan.rs
use std::fs::File;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};
use zip::write::FileOptions;
use zip::ZipWriter;

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

/// Max parallel worker threads. Tune this for your machine/network.
/// Consider making this configurable (CLI or config file).
const CONCURRENCY: usize = 64;

pub fn run_scan(target: &str, ports_str: &str, format: &str, tx: Sender<String>) -> std::io::Result<()> {
    let ports = match crate::utils::parse_ports(ports_str) {
        Ok(p) => p,
        Err(e) => {
            tx.send(format!("❌ Port-Eingabe ungültig: {e}")).ok();
            return Ok(());
        }
    };

    run_scan_from_vec(target, &ports, format, tx)
}

fn run_scan_from_vec(target: &str, ports: &[u16], format: &str, tx: Sender<String>) -> std::io::Result<()> {
    let start = Instant::now();
    let total = ports.len();

    tx.send(format!("🔍 Scanne {} Ports auf {}", total, target)).ok();

    // Build a local Rayon pool with a concurrency cap so we don't saturate the system.
    let pool = ThreadPoolBuilder::new()
        .num_threads(CONCURRENCY.min(num_cpus::get().max(1)))
        .build()
        .expect("failed to build rayon thread pool");

    // Run the parallel scan inside the pool.
    let results: Vec<(u16, bool)> = pool.install(|| {
        ports
            .par_iter()
            .map(|port| {
                let addr = format!("{}:{}", target, port);
                let mut open = false;

                if let Ok(mut addrs) = addr.to_socket_addrs()
                    && let Some(sock) = addrs.next()
                    && TcpStream::connect_timeout(&sock, Duration::from_millis(300)).is_ok()
                {
                    open = true;
                }


                // Best-effort send progress message as soon as this port is done.
                // We don't include a strict index because tasks finish out-of-order.
                tx.send(format!("Port {} {}", port, if open { "offen" } else { "geschlossen" }))
                    .ok();

                (*port, open)
            })
            .collect()
    });

    tx.send(format!("⏱️  Scan abgeschlossen in {:.2?}", start.elapsed())).ok();

    match format {
        "txt" => export_txt(target, &results)?,
        "html" => export_html(target, &results)?,
        "zip" => export_zip(target, &results)?,
        _ => {
            let _ = tx.send(format!("Unbekanntes Format: {}", format));
        }
    }

    tx.send(format!("✅ Ergebnisse exportiert als {}", format)).ok();
    Ok(())
}

fn export_txt(target: &str, results: &[(u16, bool)]) -> std::io::Result<()> {
    let mut file = File::create("scan.txt")?;
    writeln!(file, "Scan für: {}\n", target)?;
    for (port, open) in results {
        writeln!(file, "Port {}: {}", port, if *open { "offen" } else { "geschlossen" })?;
    }
    Ok(())
}

fn export_html(target: &str, results: &[(u16, bool)]) -> std::io::Result<()> {
    let mut file = File::create("scan.html")?;
    writeln!(file, "<html><body><h1>Scan für {}</h1><ul>", target)?;
    for (port, open) in results {
        writeln!(
            file,
            "<li>Port {}: <strong>{}</strong></li>",
            port,
            if *open { "offen" } else { "geschlossen" }
        )?;
    }
    writeln!(file, "</ul></body></html>")?;
    Ok(())
}

fn export_zip(target: &str, results: &[(u16, bool)]) -> std::io::Result<()> {
    let mut txt_buf = Vec::new();
    writeln!(txt_buf, "Scan für: {}\n", target)?;
    for (port, open) in results {
        writeln!(txt_buf, "Port {}: {}", port, if *open { "offen" } else { "geschlossen" })?;
    }

    let mut html_buf = Vec::new();
    writeln!(html_buf, "<html><body><h1>Scan für {}</h1><ul>", target)?;
    for (port, open) in results {
        writeln!(
            html_buf,
            "<li>Port {}: <strong>{}</strong></li>",
            port,
            if *open { "offen" } else { "geschlossen" }
        )?;
    }
    writeln!(html_buf, "</ul></body></html>")?;

    let file = File::create("scan_export.zip")?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    zip.start_file("scan.txt", options)?;
    zip.write_all(&txt_buf)?;
    zip.start_file("scan.html", options)?;
    zip.write_all(&html_buf)?;
    zip.finish()?;

    Ok(())
}
