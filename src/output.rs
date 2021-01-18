use super::StreamDevice;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, Stream, StreamConfig, SupportedStreamConfig};
use ringbuf::Consumer;
use std::fmt;

pub struct Output {
    pub device: Device,
    name: String,
    supported_stream_config: SupportedStreamConfig,
    stream_config: StreamConfig,
    stream: Option<Stream>,
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
            stream: None,
        })
    }
}

impl StreamDevice<Consumer<f32>> for Output {
    fn build_stream(&mut self, mut consumer: Consumer<f32>) -> Result<(), anyhow::Error> {
        let err_fn = |err: cpal::StreamError| {
            eprintln!("an error occurred on stream: {}", err);
        };
        let data_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut input_fell_behind = false;
            for sample in data {
                *sample = match consumer.pop() {
                    Some(s) => s,
                    None => {
                        input_fell_behind = true;
                        0.0
                    }
                };
            }
            if input_fell_behind {
                eprintln!("input stream fell behind: try increasing latency");
            }
        };
        self.stream = Some(self.device.build_output_stream(
            &self.stream_config,
            data_callback,
            err_fn,
        )?);
        Ok(())
    }

    fn play(&self) -> Result<(), anyhow::Error> {
        match &self.stream {
            Some(s) => s.play()?,
            None => eprintln!("Stream not created"),
        }
        Ok(())
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
