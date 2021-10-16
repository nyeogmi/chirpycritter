use crate::*;
use super::*;

#[derive(Clone)]
pub(super) struct Voice {
    pub(super) note_ix: u64,

    // TODO: Provide a constructor to populate these
    pub generator: Generator,  
    pub trigger: Trigger,
}

impl Voice {
    pub(crate) fn new(note_ix: u64, config: TimeConfig, patch: Patch<f32>, duration: u16, frequency: u16) -> Voice {
        Voice { 
            note_ix, 

            generator: Generator::new_for(config, patch.apply_time(config)),

            trigger: Trigger { 
                config,
                sample: 0,
                frequency,
                release_at: duration as u64 * config.samples_per_tick, 
            }
        }
    }

    pub(super) fn populate<Buf: SynthBuf>(&mut self, buf: &mut Buf) -> bool {
        for i in 0..buf.len() {
            // println!("sampling for: {:?}", self.trigger.sample);
            self.trigger.sample += 1;
            let (l, r) = self.generator.sample(self.trigger);
            buf.set(i, (l, r))
        }
        self.generator.is_playing(self.trigger)
    }
}