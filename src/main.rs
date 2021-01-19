//std
use std::process;
use std::sync::{Arc, Mutex};

//UI include
use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Sparkline};
use tui::Terminal;

// Stream include
use analogic_player::input::Input;
use analogic_player::output::Output;
use analogic_player::StreamDevice;
use ringbuf::RingBuffer;

//Tokio include
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sample_for_ui: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));
    let sample_for_ui_2 = sample_for_ui.clone();

    tokio::spawn(async move {
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
                    if let Ok(guard) = sample_for_ui_2.try_lock() {
                        data = guard.clone()
                    };
                    // println!("data: {:?}", data);
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints([
                            Constraint::Percentage(10),
                            Constraint::Percentage(10),
                            Constraint::Percentage(10),
                        ])
                        .split(f.size());
                    let block = Block::default().title("Block").borders(Borders::ALL);
                    f.render_widget(block, chunks[0]);
                    let sparkline_block = Block::default().title("Sparkline").borders(Borders::ALL);
                    let sparkline_style = Style::default().fg(Color::Magenta).bg(Color::Reset);
                    let sparkline = Sparkline::default()
                        .block(sparkline_block)
                        .data(&data)
                        .style(sparkline_style);
                    f.render_widget(sparkline, chunks[1]);
                })
                .unwrap();
        }
    });

    let stream_task = tokio::spawn(async move {
        //Handle Stream
        let err_msg = |err, stream_io| {
            eprintln!("Error playing {} stream: {}", stream_io, err);
            process::exit(2);
        };
        let (input_device, output_device) = init_stream(sample_for_ui).unwrap_or_else(|err| {
            eprintln!("Error initiating the stream: {}", err);
            process::exit(2);
        });
        input_device.play().unwrap_or_else(|err| {
            err_msg(err, "input");
        });
        output_device.play().unwrap_or_else(|err| {
            err_msg(err, "output");
        });
        loop {}
    });

    stream_task.await?;
    Ok(())
}

fn init_stream(sample_for_ui: Arc<Mutex<Vec<u64>>>) -> Result<(Input, Output), anyhow::Error> {
    let host = cpal::default_host();
    let latency = 100f32; //default, can be change
                          // println!("Default host selected: {}", host.id().name());

    //Selecting default input
    let mut input_device = Input::new(&host)?;
    let mut output_device = Output::new(&host)?;

    // println!("{}", input_device);
    // println!("{}", output_device);

    let config = &input_device.stream_config;

    // Create a delay in case the input and output devices aren't synced.
    let latency_frames = (latency / 1_000.0) * config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;

    // Read input device
    // The buffer to share samples
    let ring = RingBuffer::new(latency_samples * 2);
    let (mut producer, consumer) = ring.split();
    // Fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.push(0.0).unwrap();
    }

    input_device.build_stream(producer)?;
    output_device.build_stream(consumer, sample_for_ui)?;
    // println!("streams with `{}` milliseconds of latency.", latency);
    Ok((input_device, output_device))
}
