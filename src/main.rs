use analogic_player::input::{Input, ReadBuffer};
use analogic_player::output::Output;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), anyhow::Error> {
    //Selecting host
    let host = cpal::default_host();
    println!("Default host selected: {}", host.id().name());

    //Selecting default input
    let input_device = Input::new(&host)?;
    println!("{}", input_device);

    //Selecting default output
    let output_device = Output::new(&host)?;
    println!("{}", output_device);

    // Read input device
    let buffer: ReadBuffer = Arc::new(Mutex::new(Vec::new()));
    input_device.read(&buffer)?;
    if let Ok(buffer_guard) = buffer.try_lock() {
        println!("{:?}", buffer_guard);
    }
    Ok(())
}
