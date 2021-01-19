//std
use std::io;
use std::sync::{Arc, Mutex};

// termion
use termion::raw::IntoRawMode;

// tui
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Sparkline, BarChart};
use tui::Terminal;

// Tokio
use tokio::time::{self, Duration};

pub type SampleUi = (&'static str, u64);

pub type SampleUiArcMutex = Arc<Mutex<Vec<SampleUi>>>;

pub async fn draw_it(sample_for_ui: SampleUiArcMutex) {
    let stdout = io::stdout().into_raw_mode().expect("Error stdout");
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Error creating a new terminal");
    let mut interval = time::interval(Duration::from_millis(150));
    terminal.clear().unwrap();
    loop {
        interval.tick().await;

        // terminal.clear().unwrap();
        terminal
            .draw(|f| {
                let mut samples_ui: Vec<SampleUi> = Vec::new();
                let mut data: Vec<u64> = Vec::new();
                if let Ok(guard) = sample_for_ui.try_lock() {
                    samples_ui = guard.clone();
                    for sample in &samples_ui {
                        data.push(sample.1);
                    }
                };
                // Chuncks
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([
                        Constraint::Percentage(15),
                        Constraint::Percentage(85),
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
                
                // Bar Chart
                let barchart = BarChart::default()
                    .block(Block::default().title("BarChart").borders(Borders::ALL))
                    .bar_width(7)
                    .bar_gap(1)
                    .bar_style(Style::default().fg(Color::Blue).bg(Color::Reset))
                    .value_style(Style::default().fg(Color::Blue).bg(Color::Blue))
                    .data(&samples_ui)
                    .max(30);
                f.render_widget(barchart, chunks[1]);
            })
            .unwrap();
    }
}
