use super::*;

pub trait Synthesizer: 'static+Send {
    fn new(config: SynthConfig) -> Self;
    fn populate<const N: usize>(&mut self, out: &mut FixedBuf<N>);
    fn is_playing(&self, sample: u64) -> bool;
}

#[derive(Clone, Copy)]
pub struct SynthConfig {
    pub sample_rate: u64,
}