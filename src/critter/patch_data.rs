use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct PatchData<T> {
    pub osc1: Osc,
    pub osc2: Option<Osc>,
    pub vcf1: VCF,
    pub modulators: Modulators<T>,
}

#[derive(Clone, Copy, Debug)]
pub struct Osc {
    pub mul_gain: Modulated,
    pub frequency_offset: Modulated,  // TODO: Make sure this is in semitones
    pub waveform: Waveform,
    pub pulse_width: Modulated,
    pub spread: Spread,
}

#[derive(Clone, Copy, Debug)]
pub struct VCF {
    pub cutoff: Modulated,  // TODO: Replace with an 0-1 knob
    pub resonance: Modulated,
}

#[derive(Clone, Copy, Debug)]
pub struct Spread {
    pub frequency: f32,
    pub amount: f32,  // runs from 0.0 to 1.0
}

impl PatchData<f32> {
    pub(crate) fn apply_time(self, config: TimeConfig) -> PatchData<u64> {
        PatchData {
            osc1: self.osc1,
            osc2: self.osc2,
            vcf1: self.vcf1,
            modulators: self.modulators.apply_time(config),
        }
    }

    pub(crate) fn init() -> PatchData<f32> {
        return PatchData { 
            osc1: Osc { 
                mul_gain: Modulated::just(0.8), 
                frequency_offset: Modulated::just(0.0),
                waveform: Waveform::Sine, 
                pulse_width: Modulated::just(0.0),
                spread: Spread { frequency: 0.0, amount: 0.0 },
            }, 
            osc2: None ,
            vcf1: VCF {
                cutoff: Modulated::just(1.0),
                resonance: Modulated::just(0.0),
            }, 
            modulators: Modulators {
                echoes: Echoes::none(),
                gain1: ADSR::init(),
                gain2: ADSR::init(),
                env1: ADSR::init(),
                env2: ADSR::init(),
                env3: ADSR::init(),
                lfo1: LFO::init(),
                lfo2: LFO::init(),
                lfo3: LFO::init(),
            } 
        }
    }
}

impl Spread {
    pub(crate) fn needs_stereo(&self) -> bool {
        return self.amount > 0.0 && self.frequency > 0.0
    }
}