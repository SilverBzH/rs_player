//std
use std::io;
use std::process;
use std::sync::{Arc, Mutex};

// termion
use termion::event::Key;
use termion::raw::IntoRawMode;

// tui
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListItem, Sparkline};
use tui::Terminal;

// Event
pub mod events;
use events::{Event, Events};

// Log
use crate::log::{Log, LOGS};

pub type SampleUi = u64;

pub type SampleUiArcMutex = Arc<Mutex<Vec<SampleUi>>>;

pub async fn draw_it(sample_for_ui: SampleUiArcMutex) {
    let stdout = io::stdout().into_raw_mode().unwrap_or_else(|err| {
        Log::error(format!("error stdout: {}", err));
        process::exit(1)
    });
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap_or_else(|err| {
        Log::error(format!("Error creating a new terminal: {}", err));
        process::exit(1)
    });
    terminal.clear().unwrap();
    let event = Events::new();
    let mut samples_ui: Vec<u64> = Vec::new();
    loop {
        terminal
            .draw(|f| {
                if let Ok(guard) = sample_for_ui.try_lock() {
                    samples_ui.clear();
                    samples_ui = guard.clone();
                };
                // Chuncks
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(20), Constraint::Percentage(30)])
                    .split(f.size());

                // Sparkline
                let sparkline_block = Block::default().title("Audio output").borders(Borders::ALL);
                let sparkline_style = Style::default().fg(Color::Magenta).bg(Color::Reset);
                let sparkline = Sparkline::default()
                    .block(sparkline_block)
                    .data(&samples_ui)
                    .style(sparkline_style);
                f.render_widget(sparkline, chunks[0]);

                // Print Log
                let log_block = Block::default().title("Logs").borders(Borders::ALL);
                let mut log_items: Vec<ListItem> = Vec::new();
                if let Ok(log_guard) = LOGS.try_lock() {
                    log_items = log_guard.clone();
                }
                let list = List::new(log_items)
                    .block(log_block)
                    .highlight_style(Style::default().fg(Color::Blue))
                    .highlight_symbol("INFO");
                f.render_widget(list, chunks[1]);
            })
            .unwrap();

        match event.next().unwrap() {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    terminal.clear().unwrap();
                    break;
                }
                _ => {}
            },
            Event::Continue => continue,
        }
    }
}
