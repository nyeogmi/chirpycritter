pub trait Synthesizer: 'static+Send {
    fn new(config: SynthConfig) -> Self;
    fn next_sample(&mut self) -> (f32, f32);
    fn is_playing(&self, sample: u64) -> bool;
}

#[derive(Clone, Copy)]
pub struct SynthConfig {
    pub sample_rate: u64
}