use std::f32::consts::PI;

use crate::*;
use super::*;

use fastapprox::faster::exp as fast_exp;
use fastapprox::faster::tanh as fast_tanh;

#[derive(Clone)]
pub(super) struct VCFImpl {
    patch: VCF,
    lp: MoogLP,
}

impl VCFImpl {
    pub fn new(sample_rate: u64, patch: VCF) -> VCFImpl {
        VCFImpl {
            patch,
            lp: MoogLP::new(sample_rate, 1000.0, 0.0),
        }
    }

    pub fn process<Buf: MonoBuf>(&mut self, snap: &ModulatorSnapshot, buf: &mut Buf) {
        let (cutoff, key_tracking) = self.patch.cutoff.over(snap);
        self.lp.set_cutoff(cutoff, key_tracking);
        self.lp.set_resonance(self.patch.resonance.over(snap).0);
        self.lp.process(buf)
    }
}

// Huovilainen's moog model
// https://github.com/ddiakopoulos/MoogLadders/blob/master/src/HuovilainenModel.h

#[derive(Clone, Copy)]
pub(super) struct MoogLP {
    sample_rate: u64,
    cutoff: f32,
    resonance: f32,

    stage: [f32; 4],
    stage_tanh: [f32; 3],
    delay: [f32; 6],

    thermal: f32,
    tune: f32,
    acr: f32,
    res_quad: f32,
    
}

impl MoogLP {
    // TODO: Store cutoff in samples?
    pub(super) fn new(sample_rate: u64, cutoff: f32, resonance: f32) -> MoogLP {
        let mut moog = MoogLP {
            sample_rate,
            cutoff: 0.0,
            resonance: 0.0,

            stage: [0.0; 4],
            stage_tanh: [0.0; 3],
            delay: [0.0; 6],

            thermal: 800.0 * 0.000025,  // TODO: Does this matter? TODO2: Consider a faster moog filter
            tune: 0.0,
            acr: 0.0,
            res_quad: 0.0,
        };

        moog.set_cutoff(cutoff, 0.0);
        moog.set_resonance(resonance);
        moog
    }

    pub(super) fn set_cutoff(&mut self, cutoff: f32, key_tracking_offset: f32) {
        let cutoff = cutoff.max(0.0).min(1.0);
        if cutoff == self.cutoff { return; }

        self.cutoff = cutoff;

        // basically, 0 should be something really low and 1 should be 24000 hz
        let cutoff_freq = 47.0 * (512.0_f32).powf(cutoff) + key_tracking_offset;
        let cutoff_freq = cutoff_freq.min(24000.0);

        let fc = cutoff_freq / (self.sample_rate as f32);
        let f = fc * 0.5;
        let fc2 = fc * fc;
        let fc3 = fc * fc * fc;

        let fcr = 1.8730 * fc3 + 0.4955 * fc2 - 0.6490 * fc + 0.9988;
        self.acr = -3.9364 * fc2 + 1.8409 * fc + 0.9968;

        self.tune = (1.0 - fast_exp(-2.0 * PI * f * fcr)) / self.thermal;

        self.res_quad = 4.0 * self.resonance * self.acr;
    }

    pub(super) fn set_resonance(&mut self, resonance: f32) {
        let resonance = resonance.min(1.0).max(0.0);
        if resonance == self.resonance { return; }

        self.resonance = resonance;

        // was originally 4.0, adjust to a cap of .90
        // self.res_quad = 4.0 * resonance * self.acr;
        self.res_quad = (3.2 * resonance + 0.4) * self.acr;
    }

    pub(super) fn process<Buf: MonoBuf>(&mut self, buf: &mut Buf) {
        for i in 0..buf.len() {
            let input = buf.get(i);
            for _ in 0..2 {
                let input = input - self.res_quad * self.delay[5];

                let new_delay = self.delay[0] + self.tune * (fast_tanh(input * self.thermal) - self.stage_tanh[0]);
                self.delay[0] = new_delay;
                self.stage[0] = new_delay;

                for k in 1..4 {
                    let input = self.stage[k - 1];
                    self.stage_tanh[k - 1] = fast_tanh(input * self.thermal);
                    self.stage[k] = 
                        self.delay[k] + 
                        self.tune * (
                            self.stage_tanh[k - 1] -
                            (if k != 3 { self.stage_tanh[k]} else { fast_tanh(self.delay[k] * self.thermal) })
                        );
                    self.delay[k] = self.stage[k]
                }
                self.delay[5] = (self.stage[3] + self.delay[4]) * 0.5;
                self.delay[4] = self.stage[3]
            }
            buf.set(i, self.delay[5].max(-1.0).min(1.0))
        }
    }
}