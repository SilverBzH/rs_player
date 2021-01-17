use analogic_player::output::Output;

fn main() -> Result<(), String> {
    //Selecting host
    let host = cpal::default_host();
    println!("Default host selected: {}", host.id().name());

    //Selecting default output
    let output_device = Output::new(&host)?;
    println!("{}", output_device);

    //Play on output device
    if let Err(err) = output_device.run() {
        eprintln!("Error running output device: {}", err);
    }

    Ok(())
}
