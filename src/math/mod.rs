mod envelope;
mod interpolation;
mod time;
mod waveform;

pub use envelope::{ADSRf, Echoes, Envelope, LFOf};
pub(crate) use interpolation::lerp;
pub use time::Time;
pub use waveform::Waveform;