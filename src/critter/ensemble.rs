use crate::*;
use super::*;

use std::{borrow::Cow};

const VOICES: usize = 16;

pub struct Ensemble {
    config: SynthConfig,
    sample: u64,

    playback: Playback,
}

pub struct Playback {
    playing: [Option<Voice>; VOICES],
    started_at: u64,
    ended_at: Option<u64>,

    // metadata
    config: SongConfig,
    song_sample: u64,
    song_next_note: u64,

    ticks_to_wait: usize,
    cursor: usize,
    song: Song,
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
        if sample < self.playback.started_at { 
            return true 
        }
        if let Some(ended_at) = self.playback.ended_at {
            if sample - self.playback.started_at >= ended_at { 
                return false;
            }
        }
        return true
    }
}

impl Ensemble {
    pub fn play(&mut self, song: Song) {
        self.playback = Playback::start(self.config, self.sample, song)
    }
}

impl Playback {
    fn start(config: SynthConfig, started_at: u64, song: Song) -> Playback {
        let song_samples_per_tick = (config.sample_rate / song.ticks_per_second).max(1);
        let song_samples_per_beat = song_samples_per_tick * song.ticks_per_beat;

        let config = SongConfig {
            sample_rate: config.sample_rate,
            samples_per_tick: song_samples_per_tick,
            samples_per_beat: song_samples_per_beat,
        };

        Playback { 
            playing: [None; VOICES], 

            started_at,
            ended_at: None,

            config,
            song_sample: 0, 
            song_next_note: 0,

            ticks_to_wait: 0,
            cursor: 0, 
            song,
        }
    }

    fn next_sample(&mut self) -> (f32, f32) {
        if self.song_sample % self.config.samples_per_tick == 0 {
            self.on_tick();
        }

        let samp_result = self.render();

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

    fn render(&mut self) -> (f32, f32) {
        let mut sum_l = 0.0;
        let mut sum_r = 0.0;
        for v in self.playing.iter_mut() {
            if let Some(v) = v {
                let (l, r) = v.render(self.config);
                sum_l += l;
                sum_r += r;
            }
        }
        (sum_l, sum_r)
    }

    fn on_tick(&mut self) {
        if self.ticks_to_wait > 0 {
            self.ticks_to_wait -= 1;
        }
        self.degrade_voices();

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

        let patch = sample_patch(); // TODO: Use program

        let voice_to_use = Some(Voice { 
            note_ix, 

            duration_left: duration, 
            generator_l: Generator::new_for(patch.left(frequency)),
            generator_r: Generator::new_for(patch.right(frequency)),
            spread: patch.spread,

            sample: 0 ,
            released_at: None,
        });
        for v in self.playing.iter_mut() {
            if let None = v { *v = voice_to_use; return }
        }

        let (ix, _) = self.playing.iter().enumerate().min_by_key(|(_, x)| { x.unwrap().note_ix }).unwrap();
        self.playing[ix] = voice_to_use;
    }

    fn degrade_voices(&mut self) {
        for v in self.playing.iter_mut() {
            Voice::degrade(self.config, v);
        }
    }
}