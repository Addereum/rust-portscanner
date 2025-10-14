// src/scan.rs
use std::fs::File;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};
use zip::ZipWriter;
use zip::write::FileOptions;

use rayon::ThreadPoolBuilder;
use rayon::prelude::*;

/// Max parallel worker threads. Tune this for your machine/network.
/// Consider making this configurable (CLI or config file).
const CONCURRENCY: usize = 64;

pub fn run_scan(
    target: &str,
    ports_str: &str,
    format: &str,
    tx: Sender<String>,
) -> std::io::Result<()> {
    let ports = match crate::utils::parse_ports(ports_str) {
        Ok(p) => p,
        Err(e) => {
            tx.send(format!("❌ Port-Eingabe ungültig: {e}")).ok();
            return Ok(());
        }
    };

    run_scan_from_vec(target, &ports, format, tx)
}

fn run_scan_from_vec(
    target: &str,
    ports: &[u16],
    format: &str,
    tx: Sender<String>,
) -> std::io::Result<()> {
    let start = Instant::now();
    let total = ports.len();

    tx.send(format!("🔍 Scanne {} Ports auf {}", total, target))
        .ok();

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
                tx.send(format!(
                    "Port {} {}",
                    port,
                    if open { "offen" } else { "geschlossen" }
                ))
                .ok();

                (*port, open)
            })
            .collect()
    });

    tx.send(format!("⏱️  Scan abgeschlossen in {:.2?}", start.elapsed()))
        .ok();

    // Generate common contents once and dispatch to writers
    let txt = generate_txt(target, &results);
    let html = generate_html(target, &results);

    match format {
        "txt" => export_txt_content(&txt)?,
        "html" => export_html_content(&html)?,
        "zip" => export_zip_contents(&txt, &html)?,
        _ => {
            let _ = tx.send(format!("Unbekanntes Format: {}", format));
        }
    }

    tx.send(format!("✅ Ergebnisse exportiert als {}", format))
        .ok();
    Ok(())
}

/// Create plain-text content
fn generate_txt(target: &str, results: &[(u16, bool)]) -> String {
    let mut out = String::new();
    out.push_str(&format!("Scan für: {}\n\n", target));
    for (port, open) in results {
        out.push_str(&format!(
            "Port {}: {}\n",
            port,
            if *open { "offen" } else { "geschlossen" }
        ));
    }
    out
}

/// Create HTML content (the "pretty" corporate style)
fn generate_html(target: &str, results: &[(u16, bool)]) -> String {
    let mut html = String::new();

    html.push_str(&format!(
        r#"<!DOCTYPE html>
<html lang="de">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Portscan – {target}</title>
<style>
    body {{
        font-family: "Segoe UI", Arial, sans-serif;
        background-color: #f7f9fb;
        color: #333;
        margin: 40px;
        line-height: 1.5;
    }}
    h1 {{
        font-size: 1.8rem;
        border-bottom: 2px solid #0078d4;
        padding-bottom: 6px;
        margin-bottom: 20px;
    }}
    table {{
        width: 100%;
        border-collapse: collapse;
        margin-top: 10px;
        box-shadow: 0 2px 6px rgba(0,0,0,0.1);
    }}
    th, td {{
        padding: 10px 14px;
        border-bottom: 1px solid #ddd;
        text-align: left;
    }}
    th {{
        background-color: #0078d4;
        color: #fff;
        font-weight: 500;
    }}
    tr:hover {{
        background-color: #f1f1f1;
    }}
    .open {{
        color: #008000;
        font-weight: bold;
    }}
    .closed {{
        color: #c00;
        font-weight: bold;
    }}
    footer {{
        font-size: 0.85rem;
        color: #666;
        text-align: right;
        margin-top: 40px;
    }}
</style>
</head>
<body>
<h1>Portscan-Ergebnis für {target}</h1>
<table>
<thead>
<tr><th>Port</th><th>Status</th></tr>
</thead>
<tbody>"#
    ));

    for (port, open) in results {
        let status = if *open {
            r#"<span class="open">Offen</span>"#
        } else {
            r#"<span class="closed">Geschlossen</span>"#
        };
        html.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", port, status));
    }

    html.push_str(&format!(
        r#"</tbody>
</table>
<footer>
Erstellt am {} &nbsp;–&nbsp; Portscanner Export
</footer>
</body>
</html>"#,
        chrono::Local::now().format("%d.%m.%Y %H:%M:%S")
    ));

    html
}

/// write TXT file from generated content
fn export_txt_content(txt: &str) -> std::io::Result<()> {
    let mut file = File::create("scan.txt")?;
    file.write_all(txt.as_bytes())?;
    Ok(())
}

/// write HTML file from generated content
fn export_html_content(html: &str) -> std::io::Result<()> {
    let mut file = File::create("scan.html")?;
    file.write_all(html.as_bytes())?;
    Ok(())
}

/// write ZIP containing both the text and the pretty HTML
fn export_zip_contents(txt: &str, html: &str) -> std::io::Result<()> {
    let file = File::create("scan_export.zip")?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    zip.start_file("scan.txt", options)?;
    zip.write_all(txt.as_bytes())?;

    zip.start_file("scan.html", options)?;
    zip.write_all(html.as_bytes())?;

    zip.finish()?;
    Ok(())
}
