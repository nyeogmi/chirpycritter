use crate::*;
use super::*;

use std::{borrow::Cow};

pub struct Ensemble {
    config: SynthConfig,
    sample: u64,

    playback: Playback,
}

impl Synthesizer for Ensemble {
    fn new(config: SynthConfig) -> Self {
        Ensemble { 
            config, 
            sample: 0, 
            playback: Playback::start(config, 0, {
                Song {
                    ticks_per_second: 1,
                    ticks_per_beat: 1,
                    data: Cow::Borrowed(&[]),
                }
            })
        }
    }

    fn next_sample(&mut self) -> (f32, f32) {
        self.sample += 1;
        self.playback.next_sample()
    }

    fn is_playing(&self, sample: u64) -> bool {
        return self.playback.is_playing(sample)
    }
}

impl Ensemble {
    pub fn play(&mut self, song: Song) {
        self.playback = Playback::start(self.config, self.sample, song)
    }
}