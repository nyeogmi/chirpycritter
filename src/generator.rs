use std::f32::consts::PI;

use crate::{Time, envelope::{ADSRf, Envelope, LFOf}};

#[derive(Clone, Copy)]
pub struct Generator {
    osc1: Generator1,
    osc2: Option<Generator1>,
}

#[derive(Clone, Copy)]
pub struct Generator1 {
    pub gain: Envelope,
    pub frequency_offset: Envelope,  // TODO: Make sure this is in semitones
    pub frequency: u16, 
    pub waveform: Waveform,
    pub pulse_width: Envelope,

    pub time: Time,
    pub waveform_progress: f32,
}

#[derive(Clone, Copy)]
pub enum Waveform {
    Sine,
    Square,
    Saw,
}

impl Generator {
    pub fn new_for(_program: u16, frequency: u16) -> Generator {
        Generator { 
            osc1: Generator1 {
                gain: Envelope { 
                    base: 0.0, 
                    adsr: Some(ADSRf { low: 0.0, high: 0.3, attack: 0.0, decay: 0.2, sustain: 0.2, release: 0.4 }) ,
                    lfo: None,
                },
                frequency_offset: Envelope { 
                    base: 0.0, 
                    adsr: None,
                    lfo: None,
                }, 
                frequency,
                waveform: Waveform::Sine,
                pulse_width: Envelope { 
                    base: 0.0, 
                    adsr: Some(ADSRf { low: 0.0, high: 0.3, attack: 0.3, decay: 0.2, sustain: 0.05, release: 0.1}),
                    lfo: None,
                },

                time: Time::ZERO,
                waveform_progress: 0.0,
            },
            osc2: Some(Generator1 {
                gain: Envelope { 
                    base: 0.0, 
                    adsr: Some(ADSRf { low: 0.0, high: 0.1, attack: 0.0, decay: 0.2, sustain: 0.2, release: 0.1 }) ,
                    lfo: None,
                },
                frequency_offset: Envelope { 
                    base: 0.0, 
                    adsr: None ,
                    lfo: Some(LFOf {
                        adsr: None,

                        low: -0.5,
                        high: 0.5,

                        sync: true,
                        frequency: 1.0,
                        pulse_width: 0.0,
                        waveform: Waveform::Sine,
                    }),
                }, 
                frequency,
                waveform: Waveform::Square,
                pulse_width: Envelope { 
                    base: 0.0, 
                    adsr: None,
                    lfo: None,
                },

                time: Time::ZERO,
                waveform_progress: 0.0,
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

    pub fn sample(&mut self, released_at: Option<f32>, time: Time) -> f32 {
        let mut sum = self.osc1.sample(released_at, time);
        if let Some(osc2) = &mut self.osc2 {
            sum += osc2.sample(released_at, time);
        }
        sum
    }
}

impl Generator1 {
    pub fn is_playing(&self, released_at: Option<f32>, time: Time) -> bool {
        self.gain.is_playing(released_at, time)
    }

    pub fn sample(&mut self, released_at: Option<f32>, time: Time) -> f32 {
        let delta_time = time.delta(self.time);
        self.time = time;

        let mut frequency = self.frequency as f32;
        frequency = transpose(frequency, self.frequency_offset.at(released_at, time));

        self.waveform_progress += delta_time.second as f32 * frequency as f32;
        self.waveform_progress = self.waveform_progress - self.waveform_progress.floor();

        let base_wave = self.waveform.at(self.pulse_width.at(released_at, time), self.waveform_progress);

        base_wave * self.gain.at(released_at, time)
    }
}

impl Waveform {
    pub fn at(&self, unpulse_width: f32, pos: f32) -> f32 {
        let pulse_width = 1.0 - unpulse_width;
        match self {
            Waveform::Sine => {
                let cycle_width = 0.5 + pulse_width * 0.5;
                let pos2 = pos / cycle_width;
                let pos2 = pos2 - pos2.floor();
                (pos2 * 2.0 * PI).sin()
            }
            Waveform::Square => if pos < (0.5 * pulse_width) { -1.0 } else { 1.0 },
            Waveform::Saw => {
                let cycle_width = 0.5 + pulse_width * 0.5;
                let pos2 = pos / cycle_width;
                let pos2 = pos2 - pos2.floor();
                pos2 * 2.0 - 1.0
            }
        }
    }
}

fn transpose(frequency: f32, semitones: f32) -> f32 {
    frequency * 2.0_f32.powf(semitones/12.0)
}