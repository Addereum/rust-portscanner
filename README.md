# portscanner

**A fast port scanner with TUI, parallel scanning, ZIP export, and colored output.**  
Rust · tokio (async) · clap for CLI · ratatui + crossterm for TUI · colored for terminal colors · zip for export.

---

## Features

- Parallel TCP connect scan
- Port lists and ranges (e.g. `22,80,8000-8100`)
- IPv4 / IPv6 Ready  
- Configurable Timeout and Threadcount
- Interactive TUI-View  
- Export in ZIP-Archive  
- Colored Terminaloutput  

---

## Installation

### Requirements
- Rust (actual stable version)  
- cargo ready  

### Clone Repository
    git clone https://github.com/Addereum/rust-portscanner
    cd portscanner

### Build (Debug)
    cargo build

### Build (Release)
    cargo build --release

### binary in target/release/portscanner

### Install (global)
    cargo install --path .

---

## Examples

### Show Help
    cargo run -- --help
    
### or after installation
    portscanner --help

### Simple Scan
    portscanner -t 192.0.2.1 -p 1-1024

### Multiple targets and ports
    portscanner --targets 192.0.2.1,example.com --ports 22,80,443,8000-8100

### Parallel Scan with Timeout and Threads
    portscanner -t example.com -p 1-65535 --timeout 200 --threads 200

### TUI mode (start)
    portscanner --tui --targets 192.0.2.1 --ports 1-1024

### Export results

    portscanner --targets example.com --ports 1-1024 --format zip

---

## Development & Debugging

### Tests
```bash
cargo test
```

Formatting:
```bash
cargo fmt
```

Lint / Clippy:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Debug example:
```bash
 cargo run -- -t 127.0.0.1 -p 22,80
 ```

---

## Packaging / Debian

With `cargo-deb`:
```bash
cargo install cargo-deb
cargo build --release
cargo deb --target x86_64-unknown-linux-gnu
```

---

## Security & Liability

Only scan networks for which you have explicit permission.  
Unauthorized scanning may be illegal in many jurisdictions.  
The author accepts no liability or warranty.

---

## License

GPL-3.0 (see `LICENSE` file).
- GPL-3.0: Copyleft — requires derivative works to remain open source when distributed.

---

## Contribution

1. Fork the repository
2. Create a branch named `feature/...` or `fix/...`
3. Commit with a clear message
4. Open a Pull Request

Please run `cargo fmt` and all tests before submitting.

---

## Contact

Lukas Roß <contact@lukas-ross.de>
