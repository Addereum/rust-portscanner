// src/cli.rs
use anyhow::{Result, bail};
use clap::Parser;
use ipnet::IpNet;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Portscanner CLI - supports CIDR, ports, async scanning"
)]
pub struct Opts {
    /// comma separated targets, hostnames, IPs or CIDR (e.g. 192.0.2.0/28 or example.com)
    #[arg(short, long, default_value = "127.0.0.1")]
    pub targets: String,

    /// ports (e.g. "22,80,8000-8100")
    #[arg(short, long, default_value = "80,443")]
    pub ports: String,

    /// timeout per connect in milliseconds
    #[arg(long, default_value_t = 300)]
    pub timeout: u64,

    /// concurrency (parallel connections)
    #[arg(long, default_value_t = 200)]
    pub concurrency: usize,

    /// export format: txt|html|zip
    #[arg(long, default_value = "txt")]
    pub format: Format,

    /// start old TUI (interactive)
    #[arg(long)]
    pub tui: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Txt,
    Html,
    Zip,
}

impl FromStr for Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "txt" => Ok(Format::Txt),
            "html" => Ok(Format::Html),
            "zip" => Ok(Format::Zip),
            _ => Err(format!("unknown format: {}", s)),
        }
    }
}

/// Expand targets string into Vec<String>.
/// Supports hostnames and CIDR ranges. Limits to `max_hosts` entries.
pub fn expand_targets(input: &str, max_hosts: usize) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for part in input.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        if let Ok(net) = part.parse::<IpNet>() {
            // ipnet::IpNet::hosts() yields IpAddr iterator
            for ip in net.hosts() {
                out.push(ip.to_string());
                if out.len() > max_hosts {
                    bail!("CIDR expansion too large (> {})", max_hosts);
                }
            }
        } else {
            out.push(part.to_string());
        }
    }
    if out.is_empty() {
        bail!("no targets provided");
    }
    Ok(out)
}
