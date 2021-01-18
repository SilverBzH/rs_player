use analogic_player::input::Input;
use analogic_player::output::Output;
use analogic_player::StreamDevice;
use ringbuf::RingBuffer;

fn main() -> Result<(), anyhow::Error> {
    let (input_device, output_device) = init_stream()?;
    input_device.play()?;
    output_device.play()?;

    loop {}
}

fn init_stream() -> Result<(Input, Output), anyhow::Error> {
    let host = cpal::default_host();
    let latency = 100f32; //default, can be change
    println!("Default host selected: {}", host.id().name());

    //Selecting default input
    let mut input_device = Input::new(&host)?;
    let mut output_device = Output::new(&host)?;

    println!("{}", input_device);
    println!("{}", output_device);

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
    output_device.build_stream(consumer)?;
    println!("streams with `{}` milliseconds of latency.", latency);
    Ok((input_device, output_device))
}
