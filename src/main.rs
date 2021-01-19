//std
use std::process;
use std::sync::{Arc, Mutex};

//UI include
mod ui;

// Stream include
mod stream;
use stream::Stream;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sample_for_ui: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));
    let sample_for_ui_2 = sample_for_ui.clone();

    tokio::spawn(async move {
        let err_msg = |err| {
            eprintln!("error stream: {}", err);
            process::exit(2);
        };
        let stream = Stream::new(sample_for_ui)
            .unwrap_or_else(|err| { err_msg(err) });
        stream.play()
            .unwrap_or_else(|err| { err_msg(err); });
        loop {} 
    });


    let drawing_task = tokio::spawn(async move {
        ui::draw_it(sample_for_ui_2).await;
    });

    drawing_task.await?;
    Ok(())
}
