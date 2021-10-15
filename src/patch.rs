use crate::{Waveform, envelope::Envelope};

#[derive(Clone, Copy)]
pub struct Patch {
    pub osc1: Patch1,
    pub osc2: Option<Patch1>,
    pub spread: Spread,
}

#[derive(Clone, Copy)]
pub struct Patch1 {
    pub gain: Envelope,
    pub frequency_offset: Envelope,  // TODO: Make sure this is in semitones
    pub frequency: u16,  // TODO: Drop this field, put it only in the oscillator
    pub waveform: Waveform,
    pub pulse_width: Envelope,
}

#[derive(Clone, Copy)]
pub struct Spread {
    pub frequency: f32,
    pub amount: f32,  // runs from 0.0 to 1.0
}

impl Patch {
    pub fn left(self, frequency: u16) -> Patch {
        let mut osc1 = self.osc1;
        osc1.frequency_offset.base -= self.spread.frequency;
        osc1.frequency = frequency;

        let osc2 = if let Some(mut o2) = self.osc2 {
            o2.frequency_offset.base -= self.spread.frequency;
            o2.frequency = frequency;
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

    pub fn right(self, frequency: u16) -> Patch {
        let mut osc1 = self.osc1;
        osc1.frequency_offset.base += self.spread.frequency;
        osc1.frequency = frequency;

        let osc2 = if let Some(mut o2) = self.osc2 {
            o2.frequency_offset.base += self.spread.frequency;
            o2.frequency = frequency;
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