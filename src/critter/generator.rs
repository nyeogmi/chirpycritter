use crate::*;
use super::*;

#[derive(Clone, Copy)]
pub(crate) struct Generator {
    osc1: Generator1,
    osc2: Option<Generator1>,
    modulators: Modulators<u64>,
    vcf1: VCFImpl,  
}

#[derive(Clone, Copy)]
pub(crate) struct Generator1 {
    pub patch: Patch1,
    pub waveform_progress: f32,
}

impl Generator {
    pub fn new_for(config: TimeConfig, patch: Patch<u64>) -> Generator {
        Generator {
            osc1: Generator1 { patch: patch.osc1, waveform_progress: 0.0 },
            osc2: patch.osc2.map(|p| { 
                Generator1 { patch: p, waveform_progress: 0.0 }
            }),
            modulators: patch.modulators,
            vcf1: VCFImpl::new(config.samples_per_second, patch.vcf1),
        }
    }

    pub(crate) fn is_playing(&self, trigger: Trigger) -> bool {
        if self.modulators.gain1.is_playing(trigger.released_at, trigger.sample) {
            return true
        }
        if let Some(_) = &self.osc2 {
            if self.modulators.gain2.is_playing(trigger.released_at, trigger.sample) {
                return true
            }
        }
        return false
    }

    pub(crate) fn sample(&mut self, trigger: Trigger) -> f32 {
        let snap = self.modulators.snap(trigger);
        let mut sum = self.osc1.sample(trigger, snap, snap.gain1);
        if let Some(osc2) = &mut self.osc2 {
            sum += osc2.sample(trigger, snap, snap.gain2);
        }

        let samp2 = self.vcf1.process(sum, snap);
        // println!("vcf: {:?}", self.vcf);
        samp2 
    }
}

impl Generator1 {
    pub(super) fn sample(&mut self, trigger: Trigger, snap: ModulatorSnapshot, gain: f32) -> f32 {
        let mut frequency = trigger.frequency as f32;
        frequency = transpose(frequency, self.patch.frequency_offset.over(snap));

        self.waveform_progress += frequency as f32 / trigger.config.samples_per_second as f32;
        self.waveform_progress = self.waveform_progress - self.waveform_progress.floor();

        let base_wave = self.patch.waveform.at(self.patch.pulse_width.over(snap), self.waveform_progress);

        base_wave * gain * self.patch.mul_gain.over(snap)
    }
}

fn transpose(frequency: f32, semitones: f32) -> f32 {
    frequency * 2.0_f32.powf(semitones/12.0)
}