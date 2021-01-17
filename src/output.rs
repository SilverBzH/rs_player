use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, StreamConfig, SupportedStreamConfig};
use std::fmt;

pub struct Output {
    device: Device,
    name: String,
    supported_stream_config: SupportedStreamConfig,
    stream_config: StreamConfig,
}

impl Output {
    pub fn new(host: &Host) -> Result<Output, anyhow::Error> {
        //Selecting default output
        let device = host
            .default_output_device()
            .expect("Default device not available.");
        let name = match device.name() {
            Ok(name) => name,
            Err(e) => {
                println!("Error getting name of output device: {}", e);
                "Default".to_string()
            }
        };

        //Selecting default output config
        let supported_stream_config = device.default_output_config()?;
        let supported_stream = supported_stream_config.clone();
        let stream_config: StreamConfig = supported_stream.clone().into();
        Ok(Output {
            device,
            name,
            supported_stream_config,
            stream_config,
        })
    }

    pub fn run(&self) -> Result<(), anyhow::Error> {
        match self.supported_stream_config.sample_format() {
            cpal::SampleFormat::F32 => self.running::<f32>(),
            cpal::SampleFormat::I16 => self.running::<i16>(),
            cpal::SampleFormat::U16 => self.running::<u16>(),
        }
    }

    fn running<T>(&self) -> Result<(), anyhow::Error>
    where
        T: cpal::Sample,
    {
        let sample_rate = self.stream_config.sample_rate.0 as f32;
        let channels = self.stream_config.channels as usize;

        // Produce a sinusoid of maximum amplitude.
        let mut sample_clock = 0f32;
        let mut next_value = move || -> f32 {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
        };

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        let stream = self.device.build_output_stream(
            &self.stream_config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                Output::write_data(data, channels, &mut next_value)
            },
            err_fn,
        )?;

        stream.play()?;

        std::thread::sleep(std::time::Duration::from_millis(1000));
        Ok(())
    }

    fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
    where
        T: cpal::Sample,
    {
        for frame in output.chunks_mut(channels) {
            let value: T = cpal::Sample::from::<f32>(&next_sample());
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = format!(
            "name: {}\nSupported stream config: {:?}\n",
            self.name, self.supported_stream_config
        );
        write!(f, "--- Output device---\n{}", info)
    }
}
