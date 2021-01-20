//std
use std::process;
use std::sync::{Arc, Mutex};
use std::time::Duration;

//UI include
pub mod ui;

// Stream
mod stream;
use stream::Stream;

// Tokio
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sample_for_ui: ui::SampleUiArcMutex = Arc::new(Mutex::new(Vec::new()));
    let sample_for_ui_clone = Arc::clone(&sample_for_ui);
    let (tx, mut rx) = oneshot::channel();

    let stream_task = tokio::spawn(async move {
        let err_msg = |err| {
            eprintln!("error stream: {}", err);
            process::exit(2);
        };
        let stream = Stream::new(sample_for_ui).unwrap_or_else(|err| err_msg(err));
        stream.play().unwrap_or_else(|err| {
            err_msg(err);
        });
        loop {
            match rx.try_recv() {
                Ok(resp) => {
                    if resp == true {
                        break;
                    }
                }
                Err(_) => std::thread::sleep(Duration::from_millis(150)),
            }
        }
    });

    let drawing_task = tokio::spawn(async move {
        ui::draw_it(sample_for_ui_clone).await;
        tx.send(true).unwrap();
    });

    drawing_task.await?;
    stream_task.await?;
    Ok(())
}
