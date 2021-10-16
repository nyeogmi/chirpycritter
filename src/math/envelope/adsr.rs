use crate::*;

#[derive(Clone, Copy)]
pub struct ADSR<T> {
    pub low: f32,
    pub high: f32,

    pub attack: T, // seconds
    pub decay: T, // seconds
    pub sustain: f32, // percent
    pub release: T, // seconds
}

impl ADSR<f32> {
    pub(crate) fn apply_time(&self, config: TimeConfig) -> ADSR<u64> {
        ADSR { 
            low: self.low,
            high: self.high,

            attack: (self.attack * config.samples_per_second as f32).floor() as u64,
            decay: (self.decay * config.samples_per_second as f32).floor() as u64,
            sustain: self.sustain,
            release: (self.release * config.samples_per_second as f32).floor() as u64,
        }
    }
}

impl ADSR<u64> {
    pub(super) fn at(&self, dampen: f32, released_at: Option<u64>, t: u64) -> f32 {
        if let Some(released_at) = released_at {
            if t > released_at {
                let prerelease = self.atperc_prerelease(released_at);
                let release_perc = percentage(t - released_at, self.release);
                let base = lerp(moog_decay(255 - release_perc), 0.0, prerelease);
                let dampen_base = lerp(base, 0.0, dampen);
                return lerp(dampen_base, self.low, self.high);
            }
        }  

        let base = self.atperc_prerelease(t);
        let dampen_base = lerp(base, 0.0, dampen);
        return lerp(dampen_base, self.low, self.high);
    }

    fn atperc_prerelease(&self, t: u64) -> f32 {
        if t < self.attack {
            let attack_perc = percentage(t, self.attack);
            return moog_attack(attack_perc);
        }
        if t < self.attack + self.decay {
            let decay_perc = percentage(t - self.attack, self.decay);
            return lerp(moog_decay(255 - decay_perc), self.sustain, 1.0);
        }
        return self.sustain;
    }
}

// TODO: Don't do any f32 conversions. Use arrays for this instead
// maps 0.0-1.0 to 0.0-1.0
fn moog_attack(x: u8) -> f32 {
    (x as f32 / 255.0).powf(0.33333333)
}

// maps 0.0-1.0 to 0.0-1.0
fn moog_decay(x: u8) -> f32 {
    (x as f32 / 255.0).powf(3.0)
}

fn percentage(x: u64, y: u64) -> u8 {
    // TODO: Allow overflow
    ((255 * x / y) & 0xff) as u8
}