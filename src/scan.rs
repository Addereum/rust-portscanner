// src/scan.rs
use anyhow::Result;
use futures::stream::{self, StreamExt};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::time::Instant;
use tokio::time::{Duration, timeout};

/// Async scanner: iterate targets, scan ports concurrently.
/// Sends textual progress messages via `tx`.
pub async fn run_scan_async(
    targets: &[String],
    ports: &[u16],
    timeout_ms: u64,
    concurrency: usize,
    tx: Option<Sender<String>>,
) -> Result<HashMap<String, Vec<(u16, bool)>>> {
    let start_all = Instant::now();
    let mut map = HashMap::new();
    let concurrency = concurrency.max(1);

    for target in targets {
        if let Some(tx) = &tx {
            let _ = tx.send(format!("🔍 Scanning {} ports on {}", ports.len(), target));
        }

        // Stream each port and run concurrently
        let stream = stream::iter(ports.to_owned())
            .map(|port| {
                let target = target.clone();
                async move {
                    let addr = (target.as_str(), port);
                    let result = timeout(
                        Duration::from_millis(timeout_ms),
                        tokio::net::TcpStream::connect(addr),
                    )
                    .await;
                    let open = matches!(result, Ok(Ok(_)));
                    (port, open)
                }
            })
            .buffer_unordered(concurrency);

        // Collect all scan results for this target
        let mut results: Vec<(u16, bool)> = stream
            .inspect(|res| {
                if let Some(tx) = &tx {
                    let _ = tx.send(format!(
                        "Port {} {}",
                        res.0,
                        if res.1 { "open" } else { "closed" }
                    ));
                }
            })
            .collect()
            .await;

        results.sort_by_key(|(p, _)| *p);
        map.insert(target.clone(), results);
    }

    if let Some(tx) = &tx {
        let _ = tx.send(format!(
            "⏱️  All scans finished in {:.2?}",
            start_all.elapsed()
        ));
    }

    Ok(map)
}
