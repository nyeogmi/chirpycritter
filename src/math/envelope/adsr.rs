use crate::*;

#[derive(Clone, Copy)]
pub struct ADSR<T> {
    pub attack: T, // seconds
    pub decay: T, // seconds
    pub sustain: f32, // percent
    pub release: T, // seconds
}

impl ADSR<f32> {
    pub(crate) fn apply_time(&self, config: TimeConfig) -> ADSR<u64> {
        ADSR { 
            attack: (self.attack * config.samples_per_second as f32).floor() as u64,
            decay: (self.decay * config.samples_per_second as f32).floor() as u64,
            sustain: self.sustain,
            release: (self.release * config.samples_per_second as f32).floor() as u64,
        }
    }

    pub(crate) fn maxed() -> ADSR<f32> {
        ADSR { attack: 0.0, decay: 0.0, sustain: 1.0, release: 0.0 }
    }
}

impl ADSR<u64> {
    pub(crate) fn at(&self, released_at: Option<u64>, t: u64) -> f32 {
        if let Some(released_at) = released_at {
            if t > released_at {
                let prerelease = self.atperc_prerelease(released_at);
                let release_perc = percentage(t - released_at, self.release);
                return lerp(moog_decay(1.0 - release_perc), 0.0, prerelease)
            }
        }  

        self.atperc_prerelease(t)
    }

    fn atperc_prerelease(&self, t: u64) -> f32 {
        if t < self.attack {
            let attack_perc = percentage(t, self.attack);
            return moog_attack(attack_perc);
        }
        if t < self.attack + self.decay {
            let decay_perc = percentage(t - self.attack, self.decay);
            return lerp(moog_decay(1.0 - decay_perc), self.sustain, 1.0);
        }
        return self.sustain;
    }

    pub(crate) fn is_playing(&self, released_at: Option<u64>, sample: u64) -> bool {
        if let Some(ra) = released_at { 
            sample < ra + self.release
        } else {
            false
        }
    }
}

// TODO: Don't do any f32 conversions. Use arrays for this instead
// maps 0.0-1.0 to 0.0-1.0
fn moog_attack(x: f32) -> f32 {
    x.powf(0.33333333)
}

// maps 0.0-1.0 to 0.0-1.0
fn moog_decay(x: f32) -> f32 {
    x.powf(3.0)
}

fn percentage(x: u64, y: u64) -> f32 {
    // TODO: Allow overflow
    x as f32 / y as f32
}