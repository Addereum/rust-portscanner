// src/utils.rs
use anyhow::{Result, bail};

/// Parses a comma-separated list of ports or ranges, e.g. "22,80-85,443".
/// Returns sorted unique Vec<u16> or an error.
pub fn parse_ports(input: &str) -> Result<Vec<u16>> {
    let mut ports = Vec::new();

    for part in input.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        if let Some((start_s, end_s)) = part.split_once('-') {
            let start = parse_port(start_s)?;
            let end = parse_port(end_s)?;
            if start > end {
                bail!("invalid range: {}", part);
            }
            for p in start..=end {
                ports.push(p);
            }
        } else {
            let p = parse_port(part)?;
            ports.push(p);
        }
    }

    if ports.is_empty() {
        bail!("no valid ports found");
    }

    ports.sort_unstable();
    ports.dedup();
    Ok(ports)
}

fn parse_port(s: &str) -> Result<u16> {
    let p: i64 = s
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid port value: {}", s))?;
    if !(0..=65535).contains(&p) {
        bail!("port out of range (0-65535): {}", s);
    }
    Ok(p as u16)
}
