use crate::*;

#[derive(Clone, Copy)]
pub struct Echoes<T> {
    pub n_times: u64,

    pub sync: bool,  // NOTE: Ignored for unsigned T
    pub period: T,  
    pub decay: f32,
}

impl Echoes<f32> {
    pub(crate) fn apply_time(&self, config: TimeConfig) -> Echoes<u64> {
        Echoes { 
            n_times: self.n_times,

            sync: self.sync,
            period: (self.period * config.samples_per_second as f32).floor() as u64,
            decay: self.decay,
        }
    }
}