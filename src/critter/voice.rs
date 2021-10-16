use crate::*;

use super::SongConfig;

#[derive(Clone, Copy)]
pub(super) struct Voice {
    pub(super) note_ix: u64,

    // TODO: Provide a constructor to populate these
    pub duration_left: u16,
    pub generator_l: Generator,  
    pub generator_r: Generator,  
    pub spread: Spread,

    pub sample: u64,
    pub released_at: Option<u64>,
}

impl Voice {
    pub(crate) fn new(note_ix: u64, patch: Patch, duration: u16, frequency: u16) -> Voice {
        Voice { 
            note_ix, 

            duration_left: duration, 
            generator_l: Generator::new_for(patch.left(frequency)),
            generator_r: Generator::new_for(patch.right(frequency)),
            spread: patch.spread,

            sample: 0 ,
            released_at: None,
        }
    }

    pub(super) fn render(&mut self, config: SongConfig) -> (f32, f32) {
        self.sample += 1;

        let beats_per_second = config.sample_rate as f32 / config.samples_per_beat as f32;

        let released_at = self.released_at.map(|x| x as f32 / config.sample_rate as f32);
        let delta_time = Time { 
            second: 1.0 / config.sample_rate as f32,
            beat: 1.0 / config.samples_per_beat as f32,
            beats_per_second,
        };
        let time = Time { 
            second: self.sample as f32 / config.sample_rate as f32,
            beat: self.sample as f32 / config.samples_per_beat as f32,
            beats_per_second,
        };

        let pure_l = self.generator_l.sample(released_at, delta_time, time);
        let pure_r = self.generator_r.sample(released_at, delta_time, time);

        // Move closer
        // TODO: Use a real panning function for this
        let (l, r) = (
            lerp(lerp(self.spread.amount, 0.5, 0.0), pure_l, pure_r), 
            lerp(lerp(self.spread.amount, 0.5, 0.0), pure_r, pure_l)
        );
        (l, r)
    }

    pub(super) fn degrade(config: SongConfig, v: &mut Option<Voice>) {
        if let Some(v2) = v {
            if v2.duration_left == 1 {
                v2.released_at = Some(v2.sample);
                v2.duration_left = 0;
            }
            else if v2.duration_left == 0 {
                let released_at = v2.released_at.map(|x| x as f32 / config.sample_rate as f32);
                let time = Time { 
                    second: v2.sample as f32 / config.sample_rate as f32,
                    beat: v2.sample as f32 / config.samples_per_beat as f32,
                    beats_per_second: config.sample_rate as f32 / config.samples_per_beat as f32,
                };

                // TODO: Only look at generator l? our spread feature can't make these diverge
                if !(v2.generator_l.is_playing(released_at, time) || v2.generator_r.is_playing(released_at, time)) {
                    *v = None
                }
            }
            else {
                v2.duration_left -= 1
            }
        }
    }

}