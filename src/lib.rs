pub mod input;
pub mod output;

pub trait StreamDevice<T> {
    fn build_stream(&mut self, ring_buffer: T) -> Result<(), anyhow::Error>;
    fn play(&self) -> Result<(), anyhow::Error>;
}
