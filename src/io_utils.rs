// src/io_utils.rs
use anyhow::Result;
use std::collections::HashMap;
use std::io::Write;
use tokio::fs;

/// Formats scan results into plain text.
pub fn make_txt(results: &HashMap<String, Vec<(u16, bool)>>) -> String {
    let mut out = String::new();
    for (target, vec) in results {
        out.push_str(&format!("Scan for: {}\n\n", target));
        for (port, open) in vec {
            out.push_str(&format!(
                "Port {}: {}\n",
                port,
                if *open { "open" } else { "closed" }
            ));
        }
        out.push('\n');
    }
    out
}

/// Formats scan results into HTML.
pub fn make_html(results: &HashMap<String, Vec<(u16, bool)>>) -> String {
    use chrono::Local;
    let mut html = String::new();
    html.push_str(r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Portscan</title><style>body{font-family:Segoe UI,Arial,sans-serif;margin:20px}table{border-collapse:collapse;width:100%}th,td{padding:8px;border-bottom:1px solid #ddd}th{background:#0078d4;color:#fff}</style></head><body>"#);
    for (target, vec) in results {
        html.push_str(&format!("<h2>Scan results for {}</h2>", target));
        html.push_str("<table><thead><tr><th>Port</th><th>Status</th></tr></thead><tbody>");
        for (port, open) in vec {
            let status = if *open {
                "<span style=\"color:#008000;font-weight:bold\">Open</span>"
            } else {
                "<span style=\"color:#c00;font-weight:bold\">Closed</span>"
            };
            html.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", port, status));
        }
        html.push_str("</tbody></table>");
    }
    html.push_str(&format!(
        "<footer>Generated at {}</footer></body></html>",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    ));
    html
}

/// Export results according to chosen format.
/// Uses async fs for txt/html and spawn_blocking for zip creation.
pub async fn export_results(
    results: &HashMap<String, Vec<(u16, bool)>>,
    format: super::cli::Format,
) -> Result<()> {
    match format {
        super::cli::Format::Txt => {
            let txt = make_txt(results);
            fs::write("scan.txt", txt).await?;
        }
        super::cli::Format::Html => {
            let html = make_html(results);
            fs::write("scan.html", html).await?;
        }
        super::cli::Format::Zip => {
            let txt = make_txt(results);
            let html = make_html(results);
            // blocking zip creation
            let txt_owned = txt.clone();
            let html_owned = html.clone();
            tokio::task::spawn_blocking(move || -> Result<(), anyhow::Error> {
                let file = std::fs::File::create("scan_export.zip")?;
                let mut zip = zip::ZipWriter::new(file);
                let options = zip::write::FileOptions::default()
                    .compression_method(zip::CompressionMethod::Deflated);
                zip.start_file("scan.txt", options)?;
                zip.write_all(txt_owned.as_bytes())?;
                zip.start_file("scan.html", options)?;
                zip.write_all(html_owned.as_bytes())?;
                zip.finish()?;
                Ok(())
            })
            .await??;
        }
    }
    Ok(())
}
