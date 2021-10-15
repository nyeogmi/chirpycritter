use crate::{Time, Waveform};

#[derive(Clone, Copy)]
pub struct Envelope {
    pub base: f32,
    pub adsr: Option<ADSRf>,
    pub lfo: Option<LFOf>,
}

impl Envelope {
    pub fn at(&self, released_at: Option<f32>, t: Time) -> f32 {
        let mut value = self.base;
        if let Some(adsr) = self.adsr {
            value += adsr.at(released_at, t);
        }
        if let Some(lfo) = self.lfo {
            value += lfo.at(released_at, t);
        }
        value
    }

    pub fn is_playing(&self, released_at: Option<f32>, t: Time) -> bool {
        if let Some(ra) = released_at { 
            if let Some(adsr) = self.adsr {
                return t.second < ra + adsr.release
            } else {
                return t.second < ra
            }
        }
        return true
    }
}

// TODO: Optimized impl based on samples
#[derive(Clone, Copy)]
pub struct ADSRf {
    pub low: f32,
    pub high: f32,

    pub attack: f32, // seconds
    pub decay: f32, // seconds
    pub sustain: f32, // percent
    pub release: f32, // seconds
}

// TODO: Use arrays for this instead
// maps 0.0-1.0 to 0.0-1.0
fn moog_attack(x: f32) -> f32 {
    x.powf(0.33333333)
}

// maps 0.0-1.0 to 0.0-1.0
fn moog_decay(x: f32) -> f32 {
    x.powf(3.0)
}

fn lerp(amt: f32, x0: f32, x1: f32) -> f32 {
    if amt <= 0.0 { return x0; }
    if amt >= 1.0 { return x1; }
    return x0 + (x1 - x0) * amt;
}

impl ADSRf {
    fn at(&self, released_at: Option<f32>, t: Time) -> f32 {
        if let Some(released_at) = released_at {
            if t.second > released_at {
                let prerelease = self.atperc_prerelease(released_at);
                let release_perc = (t.second - released_at) / self.release;
                let base = lerp(moog_decay(1.0 - release_perc), 0.0, prerelease);
                return lerp(base, self.low, self.high);
            }
        }  

        let base = self.atperc_prerelease(t.second);
        return lerp(base, self.low, self.high);
    }

    fn atperc_prerelease(&self, t: f32) -> f32 {
        if t < self.attack {
            let attack_perc = t / self.attack;
            return moog_attack(attack_perc);
        }
        if t < self.attack + self.decay {
            let decay_perc = (t - self.attack) / self.decay;
            return lerp(moog_decay(1.0 - decay_perc), self.sustain, 1.0);
        }
        return self.sustain;
    }
}

// TODO: Optimized impl based on samples
#[derive(Clone, Copy)]
pub struct LFOf {
    pub low: f32,
    pub high: f32,

    pub sync: bool,
    pub frequency: f32,

    pub pulse_width: f32,
    pub waveform: Waveform,
    pub adsr: Option<ADSRf>,
}

impl LFOf {
    fn at(&self, released_at: Option<f32>, t: Time) -> f32 {
        let mul = if let Some(adsr) = self.adsr {
            adsr.at(released_at, t)
        } else {
            1.0
        };

        let cycle_t = if self.sync {
            t.beat * self.frequency
        } else {
            t.second * self.frequency
        };

        let wf = self.waveform.at(self.pulse_width, cycle_t - cycle_t.floor()) * mul;
        lerp((wf + 1.0) / 2.0, self.low, self.high)
    }
}