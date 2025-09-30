# portscanner

**Ein schneller Portscanner mit TUI, parallelem Scan, Export (ZIP) und farbiger Ausgabe.**  
Rust · rayon für Parallelität · ratatui + crossterm für TUI · clap für CLI · colored für Terminalfarben · zip für Export.

---

## Features

- Paralleler TCP-Connect-Scan  
- Portlisten und Bereiche (z. B. `22,80,8000-8100`)  
- IPv4 / IPv6 Unterstützung  
- Konfigurierbares Timeout und Threadanzahl  
- Interaktive TUI-Ansicht  
- Export (CSV/JSON) in ZIP-Archiv  
- Farbige Terminalausgabe  

---

## Installation

### Voraussetzungen
- Rust (aktuelle stable Toolchain)  
- cargo verfügbar  

### Repository klonen
    git clone <repo-url>
    cd portscanner

### Build (Debug)
    cargo build

### Build (Release)
    cargo build --release
    # Binär liegt in target/release/portscanner

### Installieren (global)
    cargo install --path .

---

## Nutzung / Beispiele

### Hilfe anzeigen
    cargo run -- --help
    # oder nach Installation
    portscanner --help

### Einfacher Scan
    portscanner -t 192.0.2.1 -p 1-1024

### Mehrere Ziele und Ports
    portscanner -t 192.0.2.1,example.com -p 22,80,443,8000-8100

### Paralleler Scan mit Timeout und Threads
    portscanner -t example.com -p 1-65535 --timeout 200 --threads 200

### TUI starten
    portscanner --tui -t 192.0.2.1 -p 1-1024

### Ergebnis exportieren
    portscanner -t example.com -p 1-1024 --export results.zip

---

## Entwickeln & Debugging

Tests:
    cargo test

Formatierung:
    cargo fmt

Lint / Clippy:
    cargo clippy --all-targets --all-features -- -D warnings

Debug-Beispiel:
    cargo run -- -t 127.0.0.1 -p 22,80

---

## Packaging / Debian

Mit cargo-deb:
    cargo install cargo-deb
    cargo build --release
    cargo deb --target x86_64-unknown-linux-gnu

---

## Sicherheit & Haftung

Scanne nur Netzwerke, für die du eine ausdrückliche Erlaubnis hast.  
Unautorisierte Scans sind in vielen Jurisdiktionen rechtswidrig.  
Keine Haftung durch den Autor.

---

## License

GPL-3.0 (Datei LICENSE).
- GPL-3.0: Copyleft, zwingt abgeleitete Werke bei Verteilung offen zu bleiben  

---

## Contribution

1. Fork  
2. Branch `feature/...` oder `fix/...`  
3. Commit mit klarer Nachricht  
4. Pull Request öffnen  

Bitte cargo fmt und Tests ausführen.

---

## Kontakt

Lukas Roß <contact@lukas-ross.de>
