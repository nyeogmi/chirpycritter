use super::*;

#[derive(Clone, Copy)]
pub(crate) struct Generator {
    osc1: Generator1,
    osc2: Option<Generator1>,
}

#[derive(Clone, Copy)]
pub(crate) struct Generator1 {
    pub patch: Patch1<u64>,
    pub waveform_progress: f32,
}

impl Generator {
    pub fn new_for(patch: Patch<u64>) -> Generator {
        Generator {
            osc1: Generator1 { patch: patch.osc1, waveform_progress: 0.0 },
            osc2: patch.osc2.map(|p| { 
                Generator1 { patch: p, waveform_progress: 0.0 }
            }),
        }
    }

    pub(crate) fn is_playing(&self, trigger: Trigger) -> bool {
        if self.osc1.is_playing(trigger) {
            return true
        }
        if let Some(osc2) = &self.osc2 {
            if osc2.is_playing(trigger) {
                return true
            }
        }
        return false
    }

    pub(crate) fn sample(&mut self, trigger: Trigger) -> f32 {
        let mut sum = self.osc1.sample(trigger);
        if let Some(osc2) = &mut self.osc2 {
            sum += osc2.sample(trigger);
        }
        sum
    }
}

impl Generator1 {
    pub(crate) fn is_playing(&self, trigger: Trigger) -> bool {
        self.patch.gain.is_playing(trigger.released_at, trigger.sample)
    }

    pub(crate) fn sample(&mut self, trigger: Trigger) -> f32 {
        let mut frequency = trigger.frequency as f32;
        frequency = transpose(frequency, self.patch.frequency_offset.at(trigger.released_at, trigger.sample));

        self.waveform_progress += frequency as f32 / trigger.config.samples_per_second as f32;
        self.waveform_progress = self.waveform_progress - self.waveform_progress.floor();

        let base_wave = self.patch.waveform.at(self.patch.pulse_width.at(trigger.released_at, trigger.sample), self.waveform_progress);

        base_wave * self.patch.gain.at(trigger.released_at, trigger.sample)
    }
}

fn transpose(frequency: f32, semitones: f32) -> f32 {
    frequency * 2.0_f32.powf(semitones/12.0)
}