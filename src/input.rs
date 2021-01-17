use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, StreamConfig, SupportedStreamConfig};
use std::fmt;
use std::sync::{Arc, Mutex};

pub type ReadBuffer = Arc<Mutex<Vec<f32>>>;

pub struct Input {
    device: Device,
    name: String,
    supported_stream_config: SupportedStreamConfig,
    stream_config: StreamConfig,
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
        })
    }

    pub fn read(&self, buffer: &ReadBuffer) -> Result<(), anyhow::Error> {
        let buffer_2 = buffer.clone();
        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };

        let stream = match self.supported_stream_config.sample_format() {
            cpal::SampleFormat::F32 => self.device.build_input_stream(
                &self.stream_config,
                move |data, _: &_| Input::write_input_data::<f32>(data, &buffer_2),
                err_fn,
            )?,
            cpal::SampleFormat::I16 => self.device.build_input_stream(
                &self.stream_config,
                move |data, _: &_| Input::write_input_data::<i16>(data, &buffer_2),
                err_fn,
            )?,
            cpal::SampleFormat::U16 => self.device.build_input_stream(
                &self.stream_config,
                move |data, _: &_| Input::write_input_data::<u16>(data, &buffer_2),
                err_fn,
            )?,
        };
        stream.play()?;
        std::thread::sleep(std::time::Duration::from_secs(3));
        drop(stream);
        Ok(())
    }

    fn write_input_data<T>(input: &[T], buffer: &ReadBuffer)
    where
        T: cpal::Sample,
    {
        if let Ok(mut buffer_guard) = buffer.try_lock() {
            for &sample in input.iter() {
                buffer_guard.push(sample.to_f32());
            }
        }
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
