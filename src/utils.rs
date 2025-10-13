// src/utils.rs

use clap::{error::ErrorKind, Error};

/// Parses a comma-separated list of ports or ranges, e.g. "22,80-85,443".
/// Returns an error for invalid input (non-numeric, negative, >65535).
pub fn parse_ports(input: &str) -> Result<Vec<u16>, Error> {
    let mut ports = Vec::new();

    for part in input.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        if let Some((start, end)) = part.split_once('-') {
            let start = parse_port(start)?;
            let end = parse_port(end)?;
            if start > end {
                return Err(Error::raw(ErrorKind::ValueValidation, format!("invalid range: {part}")));
            }
            ports.extend(start..=end);
        } else {
            ports.push(parse_port(part)?);
        }
    }

    if ports.is_empty() {
        return Err(Error::raw(ErrorKind::ValueValidation, "no valid ports found"));
    }

    Ok(ports)
}

fn parse_port(s: &str) -> Result<u16, Error> {
    let p: i64 = s.parse().map_err(|_| {
        Error::raw(ErrorKind::ValueValidation, format!("invalid port value: {s}"))
    })?;

    if !(0..=65535).contains(&p) {
        return Err(Error::raw(ErrorKind::ValueValidation, format!(
            "port out of range (0–65535): {s}"
        )));
    }

    Ok(p as u16)
}
