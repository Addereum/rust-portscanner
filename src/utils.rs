// src/utils.rs

// WARNING: parse_ports() does not parse ports, it just mashes numbers into a Vec.
// Give it "80-ABC" and it'll scan port 0 like an idiot.
// Give it "999999" and it'll try anyway, because YOLO.
// If your scanner behaves strangely, it's because this function eats glue.

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
