pub trait Synthesizer: 'static+Send {
    fn new(config: SynthConfig) -> Self;
    fn populate<Buf: SynthBuf>(&mut self, out: &mut Buf);
    fn is_playing(&self, sample: u64) -> bool;
}

pub trait SynthBuf {
    fn len(&self) -> usize;
    fn get(&self, ix: usize) -> (f32, f32);
    fn set(&mut self, ix: usize, samp: (f32, f32));
}

pub struct StereoBuf { pub values: [f32; 1024] }

impl StereoBuf {
    pub(crate) fn new() -> StereoBuf {
        StereoBuf { values: [0.0; 1024] }
    }
}

impl SynthBuf for StereoBuf {
    fn len(&self) -> usize { self.values.len() / 2 }

    fn get(&self, ix: usize) -> (f32, f32) {
        (self.values[ix * 2], self.values[ix * 2 + 1])
    }

    fn set(&mut self, ix: usize, samp: (f32, f32)) {
        self.values[ix * 2] = samp.0;
        self.values[ix * 2] = samp.1;
    }
}

#[derive(Clone, Copy)]
pub struct SynthConfig {
    pub sample_rate: u64,
}