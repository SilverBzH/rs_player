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

pub type SampleUi = u64;

pub type SampleUiArcMutex = Arc<Mutex<Vec<SampleUi>>>;

pub async fn draw_it(sample_for_ui: SampleUiArcMutex) {
    let stdout = io::stdout().into_raw_mode().expect("Error stdout");
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Error creating a new terminal");
    let mut interval = time::interval(Duration::from_millis(150));
    let mut samples_ui: Vec<SampleUi> = Vec::new();
    terminal.clear().unwrap();

    loop {
        interval.tick().await;

        // terminal.clear().unwrap();
        terminal
            .draw(|f| {
                if let Ok(guard) = sample_for_ui.try_lock() {
                    samples_ui = guard.clone();
                };
                // Chuncks
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([
                        Constraint::Percentage(20),
                        Constraint::Percentage(30),
                    ])
                    .split(f.size());

                // Sparkline
                let sparkline_block = Block::default().title("Sparkline").borders(Borders::ALL);
                let sparkline_style = Style::default().fg(Color::Magenta).bg(Color::Reset);
                let sparkline = Sparkline::default()
                    .block(sparkline_block)
                    .data(&samples_ui)
                    .style(sparkline_style);
                f.render_widget(sparkline, chunks[0]);

                // Print Log
                let log_block = Block::default().title("Log").borders(Borders::ALL);
                // let log_style = Style::default().fg(Color::Green).bg(Color::Reset);
                f.render_widget(log_block, chunks[1]);
            })
            .unwrap();
    }
}
