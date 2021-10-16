use crate::*;
use super::*;

#[derive(Clone, Copy)]
pub(super) struct Voice {
    pub(super) note_ix: u64,

    // TODO: Provide a constructor to populate these
    pub duration_left: u16,
    pub generator: Generator,  

    pub trigger: Trigger,
}

impl Voice {
    pub(crate) fn new(note_ix: u64, config: TimeConfig, patch: Patch<f32>, duration: u16, frequency: u16) -> Voice {
        Voice { 
            note_ix, 

            duration_left: duration, 
            generator: Generator::new_for(config, patch.apply_time(config)),

            trigger: Trigger { 
                config,
                sample: 0,
                frequency,
                released_at: None, 
            }
        }
    }

    pub(super) fn render(&mut self) -> (f32, f32) {
        self.trigger.sample += 1;
        self.generator.sample(self.trigger)
    }

    pub(super) fn degrade(v: &mut Option<Voice>) {
        if let Some(v2) = v {
            if v2.duration_left == 1 {
                v2.trigger.released_at = Some(v2.trigger.sample);
                v2.duration_left = 0;
            }
            else if v2.duration_left == 0 {
                // TODO: Only look at generator l? our spread feature can't make these diverge
                if !(v2.generator.is_playing(v2.trigger)) {
                    *v = None
                }
            }
            else {
                v2.duration_left -= 1
            }
        }
    }

}