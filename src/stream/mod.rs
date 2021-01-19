mod input;
mod output;

use input::Input;
use output::Output;
use ringbuf::RingBuffer;

use std::sync::{Mutex, Arc};

trait StreamDevice<T> {
    fn build_stream(
        &mut self,
        ringbuf: T,
        sample_for_ui: Option<Arc<Mutex<Vec<u64>>>>,
    ) -> Result<(), anyhow::Error>;

    fn play(&self) -> Result<(), anyhow::Error>;
}

pub struct Stream {
    input: Input,
    output: Output,
}

impl Stream {
    pub fn new(sample_for_ui: Arc<Mutex<Vec<u64>>>) -> Result<Stream, anyhow::Error> {
        let host = cpal::default_host();
        let latency = 100f32;

        //Selecting default input
        let mut input = Input::new(&host)?;
        let mut output = Output::new(&host)?;

        let config = &input.stream_config;

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

        input.build_stream(producer, None)?;
        output.build_stream(consumer, Some(sample_for_ui))?;
        Ok(Stream{
            input,
            output,
        })
    }

    pub fn play(&self) -> Result<(), anyhow::Error> {
        self.input.play()?;
        self.output.play()?;
        Ok(())
    }
}

