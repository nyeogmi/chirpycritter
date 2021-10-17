use crate::*;
use super::*;

use std::{borrow::Cow};

pub struct ChirpyCritter {
    config: SynthConfig,
    sample: u64,

    playback: Ensemble,
}

impl Synthesizer for ChirpyCritter {
    fn new(config: SynthConfig) -> Self {
        ChirpyCritter { 
            config, 
            sample: 0, 
            playback: Ensemble::start(config, 0, {
                Song {
                    ticks_per_second: 1,
                    ticks_per_beat: 1,
                    data: Cow::Borrowed(&[]),
                }
            })
        }
    }

    fn populate<const N: usize>(&mut self, buf: &mut FixedBuf<N>) {
        self.sample += 1;
        self.playback.populate(buf);
    }

    fn is_playing(&self, sample: u64) -> bool {
        return self.playback.is_playing(sample)
    }
}

impl ChirpyCritter {
    pub fn play(&mut self, song: Song) {
        self.playback = Ensemble::start(self.config, self.sample, song)
    }
}