use crate::*;

#[derive(Clone, Copy)]
pub struct Patch<T> {
    pub osc1: Patch1,
    pub osc2: Option<Patch1>,
    pub vcf1: VCF,
    pub modulators: Modulators<T>,
    pub spread: Spread,
}

#[derive(Clone, Copy)]
pub struct Patch1 {
    pub mul_gain: Modulated,
    pub frequency_offset: Modulated,  // TODO: Make sure this is in semitones
    pub waveform: Waveform,
    pub pulse_width: Modulated,
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
    pub(crate) fn left(self, config: TimeConfig) -> Patch<u64> {
        self.reexpose(config, -self.spread.frequency)
    }

    pub(crate) fn right(self, config: TimeConfig) -> Patch<u64> {
        self.reexpose(config, self.spread.frequency)
    }

    pub(crate) fn reexpose(self, config: TimeConfig, freq: f32) -> Patch<u64> {
        let mut osc1 = self.osc1;
        osc1.frequency_offset.value += freq;

        let osc2 = if let Some(mut o2) = self.osc2 {
            o2.frequency_offset.value += freq;
            Some(o2)
        } else {
            None
        };

        let spread = self.spread;

        Patch {
            osc1: osc1,
            osc2: osc2,
            vcf1: self.vcf1,
            modulators: self.modulators.apply_time(config),
            spread,
        }
    }
}