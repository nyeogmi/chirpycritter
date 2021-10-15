use crate::{Time, Waveform};

#[derive(Clone, Copy)]
pub struct Envelope {
    pub base: f32,
    pub adsr: Option<ADSRf>,
    pub lfo: Option<LFOf>,
    pub echoes: Option<Echoes>,
}

impl Envelope {
    pub fn at(&self, released_at: Option<f32>, mut t: Time) -> f32 {
        let mut decay = 1.0;
        if let Some(e) = self.echoes {
            // NOTE: For fun and ease, echoes only affect the second and not the beat
            if e.sync {
                let echo_n = (t.beat / e.period).floor();
                if echo_n < e.n_times as f32 {
                    t.shift_back_beats(echo_n * e.period)
                }
                decay = e.decay.powf(echo_n)
            }
            else {
                let echo_n = (t.second / e.period).floor();
                if echo_n < e.n_times as f32 {
                    t.shift_back_seconds(echo_n * e.period)
                }
                decay = e.decay.powf(echo_n)
            }
        }

        let mut value = self.base;
        if let Some(adsr) = self.adsr {
            value += adsr.at(decay, released_at, t);
        }
        if let Some(lfo) = self.lfo {
            value += lfo.at(decay, released_at, t);
        }
        value
    }

    pub fn is_playing(&self, released_at: Option<f32>, mut t: Time) -> bool {
        if let Some(e) = self.echoes {
            if e.sync {
                t.shift_back_beats(e.n_times as f32 * e.period)
            }
            else {
                t.shift_back_seconds(e.n_times as f32 * e.period)
            }
        }
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

// TODO: Utils
pub(crate) fn lerp(amt: f32, x0: f32, x1: f32) -> f32 {
    if amt <= 0.0 { return x0; }
    if amt >= 1.0 { return x1; }
    return x0 + (x1 - x0) * amt;
}

impl ADSRf {
    fn at(&self, dampen: f32, released_at: Option<f32>, t: Time) -> f32 {
        if let Some(released_at) = released_at {
            if t.second > released_at {
                let prerelease = self.atperc_prerelease(released_at);
                let release_perc = (t.second - released_at) / self.release;
                let base = lerp(moog_decay(1.0 - release_perc), 0.0, prerelease);
                let dampen_base = lerp(base, 0.0, dampen);
                return lerp(dampen_base, self.low, self.high);
            }
        }  

        let base = self.atperc_prerelease(t.second);
        let dampen_base = lerp(base, 0.0, dampen);
        return lerp(dampen_base, self.low, self.high);
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
    pub period: f32,

    pub pulse_width: f32,
    pub waveform: Waveform,
    pub adsr: Option<ADSRf>,
}

impl LFOf {
    fn at(&self, dampen: f32, released_at: Option<f32>, t: Time) -> f32 {
        let mul = dampen * if let Some(adsr) = self.adsr {
            adsr.at(1.0, released_at, t)
        } else {
            1.0
        };

        let cycle_t = if self.sync {
            t.beat / self.period
        } else {
            t.second / self.period
        };

        let wf = self.waveform.at(self.pulse_width, cycle_t - cycle_t.floor()) * mul;
        lerp((wf + 1.0) / 2.0, self.low, self.high)
    }
}

#[derive(Clone, Copy)]
pub struct Echoes {
    pub n_times: usize,

    pub sync: bool,
    pub period: f32,  // beats
    pub decay: f32,
}