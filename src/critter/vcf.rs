use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub(super) enum VCF {
    None,
    MoogLP(MoogLP),
}

impl VCF {
    pub fn process(&mut self, input: f32) -> f32 {
        match self {
            VCF::None => input,
            VCF::MoogLP(m) => m.process(input),
        }
    }
}

// Huovilainen's moog model
// https://github.com/ddiakopoulos/MoogLadders/blob/master/src/HuovilainenModel.h

#[derive(Clone, Copy, Debug)]
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

            thermal: 0.000025,
            tune: 0.0,
            acr: 0.0,
            res_quad: 0.0,
        };

        moog.set_cutoff(cutoff);
        moog.set_resonance(resonance);
        moog
    }

    pub(super) fn set_cutoff(&mut self, cutoff: f32) {
        if cutoff == self.cutoff { return; }

        self.cutoff = cutoff;

        let fc = cutoff / (self.sample_rate as f32);
        let f = fc * 0.5;
        let fc2 = fc * fc;
        let fc3 = fc * fc * fc;

        let fcr = 1.8730 * fc3 + 0.4955 * fc2 - 0.6490 * fc + 0.9988;
        self.acr = -3.9364 * fc2 + 1.8409 * fc + 0.9968;

        self.tune = (1.0 - (-2.0 * PI * f * fcr).exp()) / self.thermal;

        self.res_quad = 4.0 * self.resonance * self.acr;
    }

    pub(super) fn set_resonance(&mut self, resonance: f32) {
        if resonance == self.resonance { return; }

        self.resonance = resonance;
        self.res_quad = 4.0 * resonance * self.acr;
    }

    pub(super) fn process(&mut self, input: f32) -> f32 {
        for _ in 0..2 {
            let input = input - self.res_quad * self.delay[5];

            let new_delay = self.delay[0] + self.tune * ((input * self.thermal).tanh() - self.stage_tanh[0]);
            self.delay[0] = new_delay;
            self.stage[0] = new_delay;

            for k in 1..4 {
                let input = self.stage[k - 1];
                self.stage_tanh[k - 1] = (input * self.thermal).tanh();
                self.stage[k] = 
                    self.delay[k] + 
                    self.tune * (
                        self.stage_tanh[k - 1] -
                        (if k != 3 { self.stage_tanh[k]} else { (self.delay[k] * self.thermal).tanh() })
                    );
                self.delay[k] = self.stage[k]
            }
            self.delay[5] = (self.stage[3] + self.delay[4]) * 0.5;
            self.delay[4] = self.stage[3]
        }
        return self.delay[5]
    }
}