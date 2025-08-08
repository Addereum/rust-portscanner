// src/utils.rs

pub fn parse_ports(input: &str) -> Vec<u16> {
    let mut ports = Vec::new();
    for part in input.split(',') {
        if let Some((start, end)) = part.trim().split_once('-') {
            let start = start.parse().unwrap_or(0);
            let end = end.parse().unwrap_or(0);
            ports.extend(start..=end);
        } else if let Ok(p) = part.trim().parse() {
            ports.push(p);
        }
    }
    ports
}
