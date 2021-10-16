pub trait Synthesizer: 'static+Send {
    fn new(config: SynthConfig) -> Self;
    fn populate<Buf: StereoBuf>(&mut self, out: &mut Buf);
    fn is_playing(&self, sample: u64) -> bool;
}

pub trait StereoBuf: Sized {
    fn len(&self) -> usize;
    fn get(&self, ix: usize) -> [f32; 2];
    fn set(&mut self, ix: usize, samp: [f32; 2]);

    fn left<'a>(&'a mut self) -> BorrowedChannel<'a, Self> { BorrowedChannel { basis: self, offset: 0 } }
    fn right<'a>(&'a mut self) -> BorrowedChannel<'a, Self> { BorrowedChannel { basis: self, offset: 1 } }
}

pub trait MonoBuf: Sized {
    fn len(&self) -> usize;
    fn get(&self, ix: usize) -> f32;
    fn set(&mut self, ix: usize, samp: f32);
}

pub struct FixedBuf { pub values: [f32; 128] }
pub struct BorrowedBuf<'a> { pub values: &'a mut [f32] }
pub struct BorrowedChannel<'a, T: StereoBuf> { pub basis: &'a mut T, offset: usize }

impl FixedBuf {
    pub fn new() -> FixedBuf {
        FixedBuf { values: [0.0; 128] }
    }

    pub fn up_to<'a>(&'a mut self, len: usize) -> BorrowedBuf<'a> {
        BorrowedBuf { values: &mut self.values[..len * 2] }
    }
}

impl StereoBuf for FixedBuf {
    fn len(&self) -> usize { self.values.len() / 2 }

    fn get(&self, ix: usize) -> [f32; 2] {
        [self.values[ix * 2], self.values[ix * 2 + 1]]
    }

    fn set(&mut self, ix: usize, samp: [f32; 2]) {
        self.values[ix * 2] = samp[0];
        self.values[ix * 2 + 1] = samp[1];
    }
}

impl<'a> StereoBuf for BorrowedBuf<'a> {
    fn len(&self) -> usize { self.values.len() / 2 }

    fn get(&self, ix: usize) -> [f32; 2] {
        [self.values[ix * 2], self.values[ix * 2 + 1]]
    }

    fn set(&mut self, ix: usize, samp: [f32; 2]) {
        self.values[ix * 2] = samp[0];
        self.values[ix * 2 + 1] = samp[1];
    }
}

impl <'a, T: StereoBuf> MonoBuf for BorrowedChannel<'a, T> {
    fn len(&self) -> usize { self.basis.len() }

    fn get(&self, ix: usize) -> f32 {
        self.basis.get(ix)[self.offset]
    }

    fn set(&mut self, ix: usize, samp: f32) {
        let mut v = self.basis.get(ix);
        v[self.offset] = samp;
        self.basis.set(ix, v);
    }
}

#[derive(Clone, Copy)]
pub struct SynthConfig {
    pub sample_rate: u64,
}