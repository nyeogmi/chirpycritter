mod ensemble;
mod generator;
mod main;
mod modulators;
mod patch_data;
mod voice;
mod trigger;
mod vcf;

pub use main::ChirpyCritter;
pub use modulators::{Modulators, Modulated, ModEnvelope, ModLfo};
pub use patch_data::{PatchData, Osc, Spread, VCF};

pub(self) use ensemble::Ensemble;
pub(self) use generator::Generator;
pub(self) use modulators::ModulatorSnapshot;
pub(self) use trigger::Trigger;
pub(self) use voice::Voice;
pub(self) use vcf::VCFImpl;