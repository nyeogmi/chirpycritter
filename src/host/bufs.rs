use std::ops::Range;

use wide::f32x8;

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

pub struct FixedBuf { 
    pub left: [f32x8; 64],
    pub right: [f32x8; 64], 
}
pub struct BorrowedBuf<'a> { 
    pub range: Range<usize>,
    pub left: &'a mut [f32x8; 64],
    pub right: &'a mut [f32x8; 64],
}

pub struct BorrowedChannel<'a, T: StereoBuf> { pub basis: &'a mut T, offset: usize }

impl FixedBuf {
    pub fn new() -> FixedBuf {
        FixedBuf { 
            left: [f32x8::ZERO; 64],
            right: [f32x8::ZERO; 64],
        }
    }

    pub fn range<'a>(&'a mut self, range: Range<usize>) -> BorrowedBuf<'a> {
        BorrowedBuf { 
            range,
            left: &mut self.left,
            right: &mut self.right,
        }
    }

    pub const fn n_raw_samples(&self) -> usize {
        self.left.len() * 8 * 2
    }

    // TODO: This is probably not very performant!!
    pub fn raw_sample(&self, buf_ix: usize) -> f32 {
        let table_ix = buf_ix % 2;
        let in_table_ix = buf_ix / 2;
        let cell_ix = in_table_ix / 8;
        let in_cell_ix = in_table_ix % 8;

        let c: [f32; 8] = if table_ix == 0 { self.left } else { self.right }[cell_ix].into();
        c[in_cell_ix]
    }

    pub fn add_from(&mut self, other: &BorrowedBuf) {
        let cell_range_low = other.range.start / 8;
        let cell_range_high = (other.range.end + 7) / 8;

        for i in cell_range_low..cell_range_high {
            self.left[i] += other.left[i];
            self.right[i] += other.right[i];
        }
    }
}

// TODO: Do these with unsafe?
impl StereoBuf for FixedBuf {
    fn len(&self) -> usize { self.left.len() * 8 }

    fn get(&self, ix: usize) -> [f32; 2] {
        let (ix0, ix1) = (ix / 8, ix % 8);
        let l: [f32; 8] = self.left[ix0].into();
        let r: [f32; 8] = self.right[ix0].into();
        [l[ix1], r[ix1]]
    }

    fn set(&mut self, ix: usize, samp: [f32; 2]) {
        let (ix0, ix1) = (ix / 8, ix % 8);
        let mut l: [f32; 8] = self.left[ix0].into();
        let mut r: [f32; 8] = self.right[ix0].into();
        l[ix1] = samp[0];
        r[ix1] = samp[1];
        self.left[ix0] = l.into();
        self.right[ix0] = r.into();
    }
}

impl<'a> StereoBuf for BorrowedBuf<'a> {
    fn len(&self) -> usize { self.range.len() }

    #[track_caller]
    fn get(&self, mut ix: usize) -> [f32; 2] {
        ix += self.range.start;
        assert!(self.range.contains(&ix));

        let (ix0, ix1) = (ix / 8, ix % 8);
        let l: [f32; 8] = self.left[ix0].into();
        let r: [f32; 8] = self.right[ix0].into();
        [l[ix1], r[ix1]]
    }

    #[track_caller]
    fn set(&mut self, mut ix: usize, samp: [f32; 2]) {
        ix += self.range.start;
        assert!(self.range.contains(&ix));

        let (ix0, ix1) = (ix / 8, ix % 8);
        let mut l: [f32; 8] = self.left[ix0].into();
        let mut r: [f32; 8] = self.right[ix0].into();
        l[ix1] = samp[0];
        r[ix1] = samp[1];
        self.left[ix0] = l.into();
        self.right[ix0] = r.into();
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