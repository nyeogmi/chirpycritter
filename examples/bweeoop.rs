use std::{thread, time::Duration};

use chirpycritter::{SynthBuf, SynthConfig, SynthEnvironment, Synthesizer};

struct SineExample {
    config: SynthConfig,
    sample: u64,
}

impl Synthesizer for SineExample {
    fn new(config: SynthConfig) -> SineExample {
        SineExample { config, sample: 0 }
    }

    fn populate<Buf: SynthBuf>(&mut self, buf: &mut Buf) {
        for i in 0..buf.len() {
            self.sample += 1;
            let l = (self.sample as f32 * 440.0 * 2.0 * std::f32::consts::PI / self.config.sample_rate as f32).sin();
            let r = (self.sample as f32 * 440.0 * 2.0 * std::f32::consts::PI / self.config.sample_rate as f32).sin();
            buf.set(i, (l, r))
        }
    }

    fn is_playing(&self, sample: u64) -> bool {
        sample < self.config.sample_rate * 1
    }
}

fn main() {
    let sine = SynthEnvironment::<SineExample>::start();
    while sine.is_playing() {
        thread::sleep(Duration::from_millis(1000));
    }
}