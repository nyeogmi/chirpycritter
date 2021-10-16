mod adsr;
mod echoes;
mod lfo;

pub use adsr::ADSR;
pub use echoes::Echoes;
pub use lfo::LFO;

use super::*;

#[derive(Clone, Copy)]
pub struct Envelope<T> {
    pub base: f32,
    pub adsr: Option<ADSR<T>>,
    pub lfo: Option<LFO<T>>,
    pub echoes: Option<Echoes<T>>,
}

impl Envelope<f32> {
    pub(crate) fn apply_time(&self, config: TimeConfig) -> Envelope<u64> {
        Envelope { 
            base: self.base,
            adsr: self.adsr.map(|adsr| adsr.apply_time(config)), 
            lfo: self.lfo.map(|lfo| lfo.apply_time(config)) ,
            echoes: self.echoes.map(|echoes| echoes.apply_time(config)),
        }
    }
}

impl Envelope<u64> {
    pub fn at(&self, released_at: Option<u64>, mut t: u64) -> f32 {
        let mut decay = 1.0;
        if let Some(e) = self.echoes {
            let echo_n = (t / e.period).min(e.n_times);
            t -= echo_n * e.period;
            decay = e.decay.powf(echo_n as f32);  // TODO: Faster way to do this
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

    pub fn is_playing(&self, released_at: Option<u64>, t: u64) -> bool {
        if let Some(ra) = released_at { 
            let echo_time = if let Some(e) = self.echoes {
                e.n_times * e.period
            } else {
                0
            };

            if let Some(adsr) = self.adsr {
                return t < ra + adsr.release + echo_time
            } else {
                return t < ra + echo_time
            }
        }
        return true
    }
}