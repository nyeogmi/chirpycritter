use crate::*;

#[derive(Clone, Copy)]
pub struct LFO<T> {
    pub sync: bool,
    pub period: T,

    pub pulse_width: f32,
    pub waveform: Waveform,
    pub adsr: Option<ADSR<T>>,  
}

impl LFO<f32> {
    pub(crate) fn apply_time(&self, config: TimeConfig) -> LFO<u64> {
        LFO { 
            sync: self.sync,
            period: (if self.sync { config.samples_per_beat as f32 } else { config.samples_per_second as f32 } * self.period as f32).floor() as u64,

            pulse_width: self.pulse_width,
            waveform: self.waveform,
            adsr: self.adsr.map(|a| a.apply_time(config)),
        }
    }

    pub fn none() -> LFO<f32> {
        LFO { sync: false, period: 0.0, pulse_width: 0.0, waveform: Waveform::Sine, adsr: None }
    }
}

impl LFO<u64> {
    pub(crate) fn at(&self, release_at: u64, t: u64) -> f32 {
        if self.period == 0 { return 0.0 };

        let mul = if let Some(adsr) = self.adsr {
            adsr.at(release_at, t)
        } else {
            1.0
        };

        // TODO: No F32 conversion, use the percentage operation
        let cycle_t = (t % self.period) as f32 / self.period as f32;

        let wf = self.waveform.at(self.pulse_width, cycle_t) * mul;
        wf
    }
}