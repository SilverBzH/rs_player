use cpal::{Host, Device, SupportedStreamConfig};
use cpal::traits::{HostTrait, DeviceTrait};
use std::fmt;


pub struct Output {
    device: Device,
    name: String,
    config: SupportedStreamConfig,
}

impl Output {
    pub fn new(host: &Host) -> Result<Output, String> {
        //Selecting default output
        let device = host.default_output_device()
        .expect("Default device not available.");
        let name = match device.name() {
            Ok(name) => name,
            Err(e) => {
                println!("Error getting name of output device: {}", e);
                "Default".to_string()
            },
        };

        //Selecting default output config
        let config = match device.default_output_config() {
            Ok(config) => config,
            Err(e) => {
                return Err(String::from(format!("Error getting output config: {}", e)));
            }
        };

        Ok(Output {
            device,
            name,
            config,
        })
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = format!("name: {}\nconfig: {:?}\n", self.name, self.config);
        write!(f, "--- Output device---\n{}", info)
    }
}

