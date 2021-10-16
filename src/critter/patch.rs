use crate::*;

#[derive(Clone, Copy)]
pub struct Patch<T> {
    pub osc1: Osc,
    pub osc2: Option<Osc>,
    pub vcf1: VCF,
    pub modulators: Modulators<T>,
}

#[derive(Clone, Copy)]
pub struct Osc {
    pub mul_gain: Modulated,
    pub frequency_offset: Modulated,  // TODO: Make sure this is in semitones
    pub waveform: Waveform,
    pub pulse_width: Modulated,
    pub spread: Spread,
}

#[derive(Clone, Copy)]
pub struct VCF {
    pub cutoff: Modulated,  // TODO: Replace with an 0-1 knob
    pub resonance: Modulated,
}

#[derive(Clone, Copy)]
pub struct Spread {
    pub frequency: f32,
    pub amount: f32,  // runs from 0.0 to 1.0
}

impl Patch<f32> {
    pub(crate) fn apply_time(self, config: TimeConfig) -> Patch<u64> {
        Patch {
            osc1: self.osc1,
            osc2: self.osc2,
            vcf1: self.vcf1,
            modulators: self.modulators.apply_time(config),
        }
    }
}

impl Spread {
    pub(crate) fn needs_stereo(&self) -> bool {
        return self.amount > 0.0 && self.frequency > 0.0
    }
}