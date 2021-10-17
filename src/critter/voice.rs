use crate::*;
use super::*;

#[derive(Clone)]
pub(super) struct Voice {
    pub(super) note_ix: u64,

    // TODO: Provide a constructor to populate these
    pub generator: Generator,  
    pub trigger: Trigger,
    pub sample: u64,
}

impl Voice {
    pub(crate) fn new(note_ix: u64, config: TimeConfig, patch: PatchData<f32>, duration: u16, frequency: u16) -> Voice {
        Voice { 
            note_ix, 

            generator: Generator::new_for(config, patch.apply_time(config)),

            trigger: Trigger { 
                config,
                frequency,
                release_at: duration as u64 * config.samples_per_tick, 
            },
            sample: 0,
        }
    }

    pub(super) fn populate<Buf: StereoBuf>(&mut self, buf: &mut Buf) -> bool {
        self.generator.populate(self.trigger, self.sample..self.sample + (buf.len() as u64), buf);
        self.sample += buf.len() as u64;
        self.generator.is_playing(self.trigger, self.sample)
    }
}