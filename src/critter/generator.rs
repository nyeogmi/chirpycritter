use crate::{Patch, Patch1, Time};

#[derive(Clone, Copy)]
pub struct Generator {
    osc1: Generator1,
    osc2: Option<Generator1>,
}

#[derive(Clone, Copy)]
pub struct Generator1 {
    pub patch: Patch1,
    pub waveform_progress: f32,
}

impl Generator {
    pub fn new_for(patch: Patch) -> Generator {
        Generator {
            osc1: Generator1 { patch: patch.osc1, waveform_progress: 0.0 },
            osc2: patch.osc2.map(|p| { 
                Generator1 { patch: p, waveform_progress: 0.0 }
            }),
        }
    }

    pub fn is_playing(&self, released_at: Option<f32>, time: Time) -> bool {
        if self.osc1.is_playing(released_at, time) {
            return true
        }
        if let Some(osc2) = &self.osc2 {
            if osc2.is_playing(released_at, time) {
                return true
            }
        }
        return false
    }

    pub fn sample(&mut self, released_at: Option<f32>, delta_time: Time, time: Time) -> f32 {
        let mut sum = self.osc1.sample(released_at, delta_time, time);
        if let Some(osc2) = &mut self.osc2 {
            sum += osc2.sample(released_at, delta_time, time);
        }
        sum
    }
}

impl Generator1 {
    pub fn is_playing(&self, released_at: Option<f32>, time: Time) -> bool {
        self.patch.gain.is_playing(released_at, time)
    }

    pub fn sample(&mut self, released_at: Option<f32>, delta_time: Time, time: Time) -> f32 {
        let mut frequency = self.patch.frequency as f32;
        frequency = transpose(frequency, self.patch.frequency_offset.at(released_at, time));

        self.waveform_progress += delta_time.second as f32 * frequency as f32;
        self.waveform_progress = self.waveform_progress - self.waveform_progress.floor();

        let base_wave = self.patch.waveform.at(self.patch.pulse_width.at(released_at, time), self.waveform_progress);

        base_wave * self.patch.gain.at(released_at, time)
    }
}

fn transpose(frequency: f32, semitones: f32) -> f32 {
    frequency * 2.0_f32.powf(semitones/12.0)
}