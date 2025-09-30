// src/tui.rs

// If you're debugging this, welcome to the funhouse.
// WARNING: This TUI is a feral animal.
// Spawns a logging thread that vandalizes the UI, leaks on disconnect, and
// bricks your terminal if a butterfly sneezes.
// TODO: use a proper app loop, draw from one place, add a drop guard, and stop busy-spinning on a dead channel.
// Replace println! with ratatui widgets, handle Resize, add a graceful shutdown, and for the love of kernels, use an enum for format.

use std::io;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn start_tui() -> io::Result<(String, String, &'static str, Sender<String>)> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut target = String::from("google.com");
    let mut ports = String::from("80,443");
    let export_formats = ["txt", "html", "zip"];
    let mut selected_format = 0usize;
    let mut input_mode = 0usize;

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // logging thread prints messages that come from scan thread
    thread::spawn(move || {
        while let Ok(line) = rx.recv() {
            println!("{}", line);
        }
    });

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(5),
                    Constraint::Length(1),
                    Constraint::Min(1),
                ])
                .split(size);

            let target_input = Paragraph::new(Text::from(target.as_str()))
                .style(if input_mode == 0 {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                })
                .block(Block::default().borders(Borders::ALL).title("Zielhost"));

            let ports_input = Paragraph::new(Text::from(ports.as_str()))
                .style(if input_mode == 1 {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                })
                .block(Block::default().borders(Borders::ALL).title("Ports"));

            let export_items: Vec<ListItem> = export_formats
                .iter()
                .enumerate()
                .map(|(i, fmt)| {
                    let prefix = if i == selected_format { "[x] " } else { "[ ] " };
                    ListItem::new(prefix.to_string() + fmt)
                })
                .collect();

            let export_list = List::new(export_items)
                .block(Block::default().borders(Borders::ALL).title("Exportformat"));

            let button = Paragraph::new(Text::from("Drücke ENTER zum Starten oder ESC zum Abbrechen"))
                .style(Style::default().fg(Color::Blue));

            f.render_widget(target_input, chunks[0]);
            f.render_widget(ports_input, chunks[1]);
            f.render_widget(export_list, chunks[2]);
            f.render_widget(button, chunks[3]);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(key) => {
                    // Only react on Press (and optionally Repeat).
                    // Keeps short key presses from firing twice on Windows.
                    match key.kind {
                        KeyEventKind::Press | KeyEventKind::Repeat => {
                            match key.code {
                                KeyCode::Esc => {
                                    disable_raw_mode()?;
                                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                                    return Err(io::Error::new(io::ErrorKind::Other, "Abgebrochen"));
                                }
                                KeyCode::Tab => {
                                    input_mode = (input_mode + 1) % 2;
                                }
                                KeyCode::Up => {
                                    if selected_format > 0 {
                                        selected_format -= 1;
                                    }
                                }
                                KeyCode::Down => {
                                    if selected_format < export_formats.len() - 1 {
                                        selected_format += 1;
                                    }
                                }
                                KeyCode::Enter => {
                                    disable_raw_mode()?;
                                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                                    return Ok((target.clone(), ports.clone(), export_formats[selected_format], tx));
                                }
                                KeyCode::Char(c) => match input_mode {
                                    0 => target.push(c),
                                    1 => ports.push(c),
                                    _ => {}
                                },
                                KeyCode::Backspace => match input_mode {
                                    0 => {
                                        target.pop();
                                    }
                                    1 => {
                                        ports.pop();
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }
                        }
                        // ignore Release and other kinds
                        _ => {}
                    }
                }
                // ignore other events (Mouse, Resize etc.)
                _ => {}
            }
        }
    }
}
