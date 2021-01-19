//std
use std::io;
use std::sync::{Arc, Mutex};

// termion
use termion::raw::IntoRawMode;

// tui
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Sparkline};
use tui::Terminal;

// Tokio
use tokio::time::{self, Duration};

pub async fn draw_it(sample_for_ui: Arc<Mutex<Vec<u64>>>) {
    let stdout = io::stdout().into_raw_mode().expect("Error stdout");
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("Error creating a new terminal");
        let mut interval = time::interval(Duration::from_millis(150));
        let mut data: Vec<u64> = Vec::new();
        loop {
            interval.tick().await;
            terminal.clear().unwrap();
            terminal
                .draw(|f| {
                    if let Ok(guard) = sample_for_ui.try_lock() {
                        data = guard.clone()
                    };
                    // Chuncks
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints([
                            Constraint::Percentage(10),
                            Constraint::Percentage(10),
                            Constraint::Percentage(10),
                        ])
                        .split(f.size());
                    // Sparkline
                    let sparkline_block = Block::default().title("Sparkline").borders(Borders::ALL);
                    let sparkline_style = Style::default().fg(Color::Magenta).bg(Color::Reset);
                    let sparkline = Sparkline::default()
                        .block(sparkline_block)
                        .data(&data)
                        .style(sparkline_style);
                    f.render_widget(sparkline, chunks[0]);
                })
                .unwrap();
        }
}