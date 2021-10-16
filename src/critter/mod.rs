mod ensemble;
mod generator;
mod main;
mod patch;
mod voice;
mod trigger;

pub use main::ChirpyCritter;
pub use patch::{Patch, Patch1, Spread};

pub(self) use ensemble::Ensemble;
pub(self) use generator::Generator;
pub(self) use trigger::Trigger;
pub(self) use voice::Voice;