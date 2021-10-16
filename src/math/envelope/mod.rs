mod adsr;
mod echoes;
mod lfo;

pub use adsr::ADSRf;
pub use echoes::Echoes;
pub use lfo::LFOf;

use crate::*;

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