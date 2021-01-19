use super::StreamDevice;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, Stream, StreamConfig, SupportedStreamConfig};
use ringbuf::Producer;
use std::fmt;
use std::sync::{Arc, Mutex};
pub type AudioBuffer = Arc<Mutex<Vec<f32>>>;

pub struct Input {
    device: Device,
    name: String,
    supported_stream_config: SupportedStreamConfig,
    pub stream_config: StreamConfig,
    stream: Option<Stream>,
}

impl Input {
    pub fn new(host: &Host) -> Result<Input, anyhow::Error> {
        let device = host
            .default_input_device()
            .expect("No input device available");
        let name = match device.name() {
            Ok(name) => name,
            Err(err) => {
                println!("Error getting input device name: {}", err);
                String::from("Default")
            }
        };
        let supported_stream_config = device.default_input_config()?;
        let supp_stream = supported_stream_config.clone();
        let stream_config: StreamConfig = supp_stream.into();

        Ok(Input {
            device,
            name,
            supported_stream_config,
            stream_config,
            stream: None,
        })
    }

    pub fn build_stream(&mut self, mut producer: Producer<f32>) -> Result<(), anyhow::Error> {
        let err_fn = |err: cpal::StreamError| {
            eprintln!("an error occurred on stream: {}", err);
        };
        let data_callback = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut output_fell_behind = false;
            for &sample in data {
                if producer.push(sample).is_err() {
                    output_fell_behind = true;
                }
            }
            if output_fell_behind {
                eprintln!("output stream fell behind: try increasing latency");
            }
        };
        self.stream = Some(self.device.build_input_stream(
            &self.stream_config,
            data_callback,
            err_fn,
        )?);
        Ok(())
    }
}

impl StreamDevice for Input {
    fn play(&self) -> Result<(), anyhow::Error> {
        match &self.stream {
            Some(s) => s.play()?,
            None => eprintln!("Stream not created"),
        }
        Ok(())
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = format!(
            "name: {}\nSupported stream config: {:?}\n",
            self.name, self.supported_stream_config
        );
        write!(f, "--- Output device---\n{}", info)
    }
}
