use crate::*;

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
    pub(super) fn at(&self, dampen: f32, released_at: Option<f32>, t: Time) -> f32 {
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