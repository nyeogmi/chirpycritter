mod envelope;
mod interpolation;
pub(self) mod tables;
mod time;
mod waveform;

pub use envelope::{
    ADSR, Echoes, LFO
};

pub(crate) use interpolation::lerp;
pub(crate) use time::TimeConfig;
pub use waveform::Waveform;