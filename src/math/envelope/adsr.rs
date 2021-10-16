use crate::*;

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

impl ADSRf {
    pub(super) fn at(&self, dampen: f32, released_at: Option<f32>, t: Time) -> f32 {
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
