use crate::{Envelope, Waveform};

#[derive(Clone, Copy)]
pub struct SamplePatch {
    pub osc1: Patch1,
    pub osc2: Option<Patch1>,
    pub spread: Spread,
}

#[derive(Clone, Copy)]
pub struct SamplePatch1 {
    pub gain: Envelope,
    pub frequency_offset: SampleEnvelope,  // TODO: Make sure this is in semitones
    pub waveform: Waveform,
    pub pulse_width: SampleEnvelope,
}

#[derive(Clone, Copy)]
pub struct Spread {
    pub frequency: f32,
    pub amount: f32,  // runs from 0.0 to 1.0
}

impl Patch {
    pub fn left(self) -> Patch {
        let mut osc1 = self.osc1;
        osc1.frequency_offset.base -= self.spread.frequency;

        let osc2 = if let Some(mut o2) = self.osc2 {
            o2.frequency_offset.base -= self.spread.frequency;
            Some(o2)
        } else {
            None
        };

        let spread = self.spread;

        Patch {
            osc1, 
            osc2, 
            spread
        }
    }

    pub fn right(self) -> Patch {
        let mut osc1 = self.osc1;
        osc1.frequency_offset.base += self.spread.frequency;

        let osc2 = if let Some(mut o2) = self.osc2 {
            o2.frequency_offset.base += self.spread.frequency;
            Some(o2)
        } else {
            None
        };

        let spread = self.spread;

        Patch {
            osc1, 
            osc2, 
            spread
        }
    }
}