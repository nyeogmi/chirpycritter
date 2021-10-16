use crate::*;

#[derive(Clone, Copy)]
pub struct Patch<T> {
    pub osc1: Patch1<T>,
    pub osc2: Option<Patch1<T>>,
    pub spread: Spread,
}

#[derive(Clone, Copy)]
pub struct Patch1<T> {
    pub gain: Envelope<T>,
    pub frequency_offset: Envelope<T>,  // TODO: Make sure this is in semitones
    pub waveform: Waveform,
    pub pulse_width: Envelope<T>,
}

#[derive(Clone, Copy)]
pub struct Spread {
    pub frequency: f32,
    pub amount: f32,  // runs from 0.0 to 1.0
}

impl Patch<f32> {
    pub(crate) fn left(self, config: TimeConfig) -> Patch<u64> {
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
            osc1: osc1.apply_time(config), 
            osc2: osc2.map(|o2| o2.apply_time(config)), 
            spread
        }
    }

    pub(crate) fn right(self, config: TimeConfig) -> Patch<u64> {
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
            osc1: osc1.apply_time(config), 
            osc2: osc2.map(|o2| o2.apply_time(config)), 
            spread
        }
    }
}

impl Patch1<f32> {
    fn apply_time(&self, config: TimeConfig) -> Patch1<u64> {
        Patch1 { 
            gain: self.gain.apply_time(config),
            frequency_offset: self.frequency_offset.apply_time(config),
            waveform: self.waveform,
            pulse_width: self.pulse_width.apply_time(config),
        }
    }
}