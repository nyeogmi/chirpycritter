mod ensemble;
mod generator;
mod main;
mod modulators;
mod patch;
mod voice;
mod trigger;
mod vcf;

pub use main::ChirpyCritter;
pub use modulators::{Modulators, Modulated, ModEnvelope, ModLfo};
pub use patch::{Patch, Patch1, Spread};

pub(self) use ensemble::Ensemble;
pub(self) use generator::Generator;
pub(self) use modulators::ModulatorSnapshot;
pub(self) use trigger::Trigger;
pub(self) use voice::Voice;
pub(self) use vcf::{MoogLP, VCF};