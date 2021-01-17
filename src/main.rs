use analogic_player::output::Output;

fn main() -> Result<(), String>{
    //Selecting host
    let host = cpal::default_host();
    println!("Default host selected: {}", host.id().name());

    //Selecting default output
    let output_device = Output::new(&host)?;
    println!("{}", output_device);
    Ok(())
    // match output_config.sample_format() {
    //     cpal::SampleFormat::F32 => run::<f32>(&output_device, &output_config.into()),
    //     cpal::SampleFormat::I16 => run::<i16>(&output_device, &output_config.into()),
    //     cpal::SampleFormat::U16 => run::<u16>(&output_device, &output_config.into()),
    // }
}
