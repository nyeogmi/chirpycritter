use std::{borrow::Cow};

use crate::{Generator, Time};
use crate::{Song, SynthConfig, Synthesizer};

const VOICES: usize = 16;

pub struct Stock {
    config: SynthConfig,
    sample: u64,

    song_state: SongState,
}

pub struct SongState {
    playing: [Option<Voice>; VOICES],
    started_at: u64,
    ended_at: Option<u64>,

    // metadata
    song_sample: u64,
    song_samples_per_tick: u64,
    song_samples_per_beat: u64,
    song_next_note: u64,

    ticks_to_wait: usize,
    cursor: usize,
    song: Song,
}

#[derive(Clone, Copy)]
pub struct Voice {
    note_ix: u64,

    duration_left: u16,
    generator: Generator,  // TODO: Program struct

    sample: u64,
    released_at: Option<u64>,
}

impl Synthesizer for Stock {
    fn new(config: SynthConfig) -> Self {
        Stock { 
            config, 
            sample: 0, 
            song_state: SongState::start(config, 0, {
                Song {
                    ticks_per_second: 1,
                    ticks_per_beat: 1,
                    data: Cow::Borrowed(&[]),
                }
            })
        }
    }

    fn next_sample(&mut self) -> f32 {
        self.sample += 1;
        self.song_state.next_sample(self.config)
    }

    fn is_playing(&self, sample: u64) -> bool {
        if sample < self.song_state.started_at { 
            return true 
        }
        if let Some(ended_at) = self.song_state.ended_at {
            if sample - self.song_state.started_at >= ended_at { 
                return false;
            }
        }
        return true
    }
}

impl Stock {
    pub fn play(&mut self, song: Song) {
        self.song_state = SongState::start(self.config, self.sample, song)
    }
}

impl SongState {
    fn start(config: SynthConfig, started_at: u64, song: Song) -> SongState {
        let song_samples_per_tick = (config.sample_rate / song.ticks_per_second).max(1);
        let song_samples_per_beat = song_samples_per_tick * song.ticks_per_beat;

        SongState { 
            playing: [None; VOICES], 

            started_at,
            ended_at: None,

            song_sample: 0, 
            song_samples_per_tick,
            song_samples_per_beat,
            song_next_note: 0,

            ticks_to_wait: 0,
            cursor: 0, 
            song,
        }
    }

    fn next_sample(&mut self, config: SynthConfig) -> f32 {
        if self.song_sample % self.song_samples_per_tick == 0 {
            self.on_tick(config);
        }

        let samp_result = self.render(config);

        self.song_sample += 1;

        if let None = self.ended_at {
            if self.song_over() {
                if self.playing.iter().all(|i| i.is_none()) {
                    self.ended_at = Some(self.song_sample)
                }
            }
        }
        
        samp_result
    }

    fn render(&mut self, config: SynthConfig) -> f32 {
        let mut sum = 0.0;
        for v in self.playing.iter_mut() {
            if let Some(v) = v {
                sum += v.render(config, self.song_samples_per_beat);
            }
        }
        sum
    }

    fn on_tick(&mut self, config: SynthConfig) {
        if self.ticks_to_wait > 0 {
            self.ticks_to_wait -= 1;
        }
        self.degrade_voices(config);

        while !self.song_over() && self.ticks_to_wait == 0 {
            match self.song.data[self.cursor] {
                crate::Packet::Play { program, frequency, duration } => {
                    self.add_voice(program, frequency, duration)
                }
                crate::Packet::Wait(ticks) => {
                    self.ticks_to_wait += ticks as usize
                }
            }
            self.cursor += 1;
        }
    }

    fn song_over(&self) -> bool {
        !(0..self.song.data.len()).contains(&self.cursor)
    }

    fn add_voice(&mut self, program: u16, frequency: u16, duration: u16) {
        let note_ix = self.song_next_note;
        self.song_next_note += 1;

        let voice_to_use = Some(Voice { 
            note_ix, 

            duration_left: duration, 
            generator: Generator::new_for(program, frequency), 

            sample: 0 ,
            released_at: None,
        });
        for v in self.playing.iter_mut() {
            if let None = v { *v = voice_to_use; return }
        }

        let (ix, _) = self.playing.iter().enumerate().min_by_key(|(_, x)| { x.unwrap().note_ix }).unwrap();
        self.playing[ix] = voice_to_use;
    }

    fn degrade_voices(&mut self, config: SynthConfig) {
        for v in self.playing.iter_mut() {
            if let Some(v2) = v {
                if v2.duration_left == 1 {
                    v2.released_at = Some(v2.sample);
                    v2.duration_left = 0;
                }
                else if v2.duration_left == 0 {
                    if !v2.generator.is_playing(
                        v2.released_at.map(|x| x as f32 / config.sample_rate as f32), 
                        Time { 
                            second: v2.sample as f32 / config.sample_rate as f32,
                            beat: v2.sample as f32 / self.song_samples_per_beat as f32,
                            sample: v2.sample,
                        }
                    ) {
                        *v = None
                    }
                }
                else {
                    v2.duration_left -= 1
                }
            }
        }
    }
}
impl Voice {
    fn render(&mut self, config: SynthConfig, samples_per_beat: u64) -> f32 {
        self.sample += 1;

        self.generator.sample(
            self.released_at.map(|x| x as f32 / config.sample_rate as f32), 
            Time { 
                second: self.sample as f32 / config.sample_rate as f32,
                beat: self.sample as f32 / samples_per_beat as f32,
                sample: self.sample,
            }
        )
    }
}