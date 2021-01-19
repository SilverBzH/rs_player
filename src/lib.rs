pub mod input;
pub mod output;

pub trait StreamDevice {
    fn play(&self) -> Result<(), anyhow::Error>;
}
