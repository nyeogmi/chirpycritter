// TODO: Velocity per note
// TODO: Filters

mod bank;
pub mod presets;
mod critter;
mod host;
mod math;
mod song;

pub use bank::*;
pub use critter::*;
pub use host::*;
pub use math::*;
pub(crate) use math::lerp;
pub use song::*;