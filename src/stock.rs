use std::{borrow::Cow, f32::consts::PI};

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
    song_next_note: u64,

    ticks_to_wait: usize,
    cursor: usize,
    song: Song,
}

#[derive(Clone, Copy)]
pub struct Voice {
    note_ix: u64,
    hertz: u16,
    duration: u16,
    sample: u64,
}

impl Synthesizer for Stock {
    fn new(config: SynthConfig) -> Self {
        Stock { 
            config, 
            sample: 0, 
            song_state: SongState::start(config, 0, {
                Song {
                    ticks_per_second: 1,
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
        SongState { 
            playing: [None; VOICES], 

            started_at,
            ended_at: None,

            song_sample: 0, 
            song_samples_per_tick: (config.sample_rate / song.ticks_per_second).max(1), 
            song_next_note: 0,

            ticks_to_wait: 0,
            cursor: 0, 
            song,
        }
    }

    fn next_sample(&mut self, config: SynthConfig) -> f32 {
        if self.song_sample % self.song_samples_per_tick == 0 {
            self.on_tick();
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
                sum += v.render(config);
            }
        }
        sum
    }

    fn on_tick(&mut self) {
        if self.ticks_to_wait > 0 {
            self.ticks_to_wait -= 1;
        }
        self.degrade_voices();

        while !self.song_over() && self.ticks_to_wait == 0 {
            match self.song.data[self.cursor] {
                crate::Packet::Play(hertz, duration) => {
                    self.add_voice(hertz, duration)
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

    fn add_voice(&mut self, hertz: u16, duration: u16) {
        let note_ix = self.song_next_note;
        self.song_next_note += 1;

        let voice_to_use = Some(Voice { note_ix, hertz, duration, sample: 0 });
        for v in self.playing.iter_mut() {
            if let None = v { *v = voice_to_use; return }
        }

        let (ix, _) = self.playing.iter().enumerate().min_by_key(|(_, x)| { x.unwrap().note_ix }).unwrap();
        self.playing[ix] = voice_to_use;
    }

    fn degrade_voices(&mut self) {
        for v in self.playing.iter_mut() {
            if let Some(v2) = v {
                if v2.duration == 1 {
                    *v = None;
                }
                else {
                    v2.duration -= 1
                }
            }
        }
    }
}
impl Voice {
    fn render(&mut self, config: SynthConfig) -> f32 {
        let samp = self.sample;

        let result = (samp as f32 * self.hertz as f32 * 2.0 * PI / config.sample_rate as f32).sin();
        self.sample += 1;

        result
    }
}