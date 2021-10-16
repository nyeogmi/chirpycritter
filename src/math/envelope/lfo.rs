use crate::*;

#[derive(Clone, Copy)]
pub struct LFO<T> {
    pub low: f32,
    pub high: f32,

    pub sync: bool,
    pub period: T,

    pub pulse_width: f32,
    pub waveform: Waveform,
    pub adsr: Option<ADSR<T>>,
}

impl LFO<f32> {
    pub(crate) fn apply_time(&self, config: TimeConfig) -> LFO<u64> {
        LFO { 
            low: self.low,
            high: self.high,

            sync: self.sync,
            period: (if self.sync { config.samples_per_beat as f32 } else { config.samples_per_second as f32 } * self.period as f32).floor() as u64,

            pulse_width: self.pulse_width,
            waveform: self.waveform,
            adsr: self.adsr.map(|a| a.apply_time(config)),
        }
    }
}

impl LFO<u64> {
    pub(super) fn at(&self, dampen: f32, released_at: Option<u64>, t: u64) -> f32 {
        let mul = dampen * if let Some(adsr) = self.adsr {
            adsr.at(1.0, released_at, t)
        } else {
            1.0
        };

        // TODO: No F32 conversion, use the percentage operation
        let cycle_t = (t % self.period) as f32 / self.period as f32;

        let wf = self.waveform.at(self.pulse_width, cycle_t) * mul;
        lerp((wf + 1.0) / 2.0, self.low, self.high)
    }
}