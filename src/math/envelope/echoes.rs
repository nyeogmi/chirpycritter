use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct Echoes<T> {
    pub n_times: u64,

    pub sync: bool,  // NOTE: Ignored for unsigned T
    pub period: T,  
}

impl Echoes<f32> {
    pub fn none() -> Echoes<f32> {
        Echoes {
            n_times: 0,

            sync: true,
            period: 0.0,
        }
    }

    pub(crate) fn apply_time(&self, config: TimeConfig) -> Echoes<u64> {
        let per = if self.sync { config.samples_per_beat } else { config.samples_per_second };
        Echoes { 
            n_times: self.n_times,

            sync: self.sync,
            period: (self.period * per as f32).floor() as u64,
        }
    }
}

impl Echoes<u64> {
    // return echo number, new sample
    pub(crate) fn to_echo(&self, sample: u64) -> (u64, u64) { 
        if self.n_times == 0 { return (0, sample) }
        let echo_n = (sample / self.period).min(self.n_times - 1);
        (echo_n, sample - echo_n * self.period)
    }
}