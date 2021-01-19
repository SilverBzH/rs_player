use super::StreamDevice;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, Stream, StreamConfig, SupportedStreamConfig};
use ringbuf::Consumer;
use std::fmt;

use super::SampleUiArcMutex;

pub struct Output {
    pub device: Device,
    name: String,
    supported_stream_config: SupportedStreamConfig,
    stream_config: StreamConfig,
    stream: Option<Stream>,
}

impl Output {
    pub(super) fn new(host: &Host) -> Result<Output, anyhow::Error> {
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

    pub fn build_stream(
        &mut self,
        mut consumer: Consumer<f32>,
        sample_for_ui: Option<SampleUiArcMutex>,
    ) -> Result<(), anyhow::Error> {
        let err_fn = |err: cpal::StreamError| {
            eprintln!("an error occurred on stream: {}", err);
        };

        let data_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut input_fell_behind = false;
            let feed_sample_ui = |sample_for_ui: &Option<SampleUiArcMutex>, sample: f32| {
                if let Some(some_sample) = sample_for_ui {
                    if let Ok(mut guard) = some_sample.try_lock() {
                        if guard.len() > 1000 {
                            guard.remove(0);
                        }
                        let sample: f32 = sample.abs() * 100f32;
                        guard.push(("", sample as u64));
                    }
                }
            };
            for sample in data {
                *sample = match consumer.pop() {
                    Some(s) => {
                        feed_sample_ui(&sample_for_ui, s);
                        s
                    }
                    None => {
                        input_fell_behind = true;
                        feed_sample_ui(&sample_for_ui, 0.0);
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
}

impl StreamDevice for Output {
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
