// TODO: Velocity per note
// TODO: Filters

mod content;
mod critter;
mod host;
mod math;
mod song;

pub(self) use content::{sample_patch_1, sample_patch_2};
pub use critter::*;
pub use host::*;
pub use math::*;
pub(crate) use math::lerp;
pub use song::*;

// TODO: Host