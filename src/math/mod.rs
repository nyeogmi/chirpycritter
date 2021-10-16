mod envelope;
mod interpolation;
mod time;
mod waveform;

pub use envelope::{
    ADSR, Echoes, Envelope, LFO
};

pub(crate) use interpolation::lerp;
pub(crate) use time::TimeConfig;
pub use waveform::Waveform;