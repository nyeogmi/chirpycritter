// TODO: Velocity per note
// TODO: Filters

mod content;
mod critter;
mod host;
mod math;
mod song;

pub(self) use content::sample_patch;
pub use critter::*;
pub use host::*;
pub use math::*;
pub(crate) use math::lerp;
pub use song::*;

// TODO: Host