use crate::*;
use super::*;

#[derive(Clone, Copy)]
pub(super) struct Voice {
    pub(super) note_ix: u64,

    // TODO: Provide a constructor to populate these
    pub duration_left: u16,
    pub generator_l: Generator,  
    pub generator_r: Generator,  
    pub spread: Spread,

    pub trigger: Trigger,
}

impl Voice {
    pub(crate) fn new(note_ix: u64, config: TimeConfig, patch: Patch<f32>, duration: u16, frequency: u16) -> Voice {
        Voice { 
            note_ix, 

            duration_left: duration, 
            generator_l: Generator::new_for(patch.left(config)),
            generator_r: Generator::new_for(patch.right(config)),
            spread: patch.spread,

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

        let pure_l = self.generator_l.sample(self.trigger);
        let pure_r = self.generator_r.sample(self.trigger);

        // Move closer
        // TODO: Use a real panning function for this
        let (l, r) = (
            lerp(lerp(self.spread.amount, 0.5, 0.0), pure_l, pure_r), 
            lerp(lerp(self.spread.amount, 0.5, 0.0), pure_r, pure_l)
        );
        (l, r)
    }

    pub(super) fn degrade(v: &mut Option<Voice>) {
        if let Some(v2) = v {
            if v2.duration_left == 1 {
                v2.trigger.released_at = Some(v2.trigger.sample);
                v2.duration_left = 0;
            }
            else if v2.duration_left == 0 {
                // TODO: Only look at generator l? our spread feature can't make these diverge
                if !(v2.generator_l.is_playing(v2.trigger) || v2.generator_r.is_playing(v2.trigger)) {
                    *v = None
                }
            }
            else {
                v2.duration_left -= 1
            }
        }
    }

}