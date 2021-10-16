use crate::*;
use super::*;

const VOICES: usize = 16;

pub(super) struct Ensemble {
    playing: [Option<Voice>; VOICES],
    started_at: u64,
    ended_at: Option<u64>,

    // metadata
    config: TimeConfig,
    song_sample: u64,
    song_next_event_at: u64,
    song_next_note: u64,

    cursor: usize,
    song: Song,
}

impl Ensemble {
    pub(super) fn start(config: SynthConfig, started_at: u64, song: Song) -> Ensemble {
        let song_samples_per_tick = (config.sample_rate / song.ticks_per_second).max(1);
        let song_samples_per_beat = song_samples_per_tick * song.ticks_per_beat;

        let config = TimeConfig {
            samples_per_second: config.sample_rate,
            samples_per_tick: song_samples_per_tick,
            samples_per_beat: song_samples_per_beat,
        };

        Ensemble { 
            playing: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None], 

            started_at,
            ended_at: None,

            config,
            song_sample: 0, 
            song_next_event_at: 0,
            song_next_note: 0,

            cursor: 0, 
            song,
        }
    }

    pub(super) fn populate<Buf: SynthBuf>(&mut self, buf: &mut Buf) {
        let mut buf_i: usize = 0;

        for i in 0..buf.len() { buf.set(i, (0.0, 0.0)); }
        let mut spare_buf = StereoBuf::new();

        loop {
            let samples_needed = buf.len() - buf_i;
            if samples_needed <= 0 { break; }

            if self.song_sample == self.song_next_event_at { self.on_deadline_hit(); }

            let samples_available = (self.song_next_event_at - self.song_sample) as usize;
            let samples_to_take = samples_available.min(samples_needed);

            let mut cut_buf = spare_buf.up_to(samples_to_take);

            for v in self.playing.iter_mut() {
                if let Some(v2) = v {
                    let playing = v2.populate(&mut cut_buf);
                    if !playing { *v = None; }

                    for i in 0..cut_buf.len() {
                        let (old_l, old_r) = buf.get(buf_i + i as usize);
                        let (new_l, new_r) = cut_buf.get(i);
                        buf.set(buf_i + i as usize, (old_l + new_l, old_r + new_r));
                    }
                }
            }

            self.song_sample += samples_to_take as u64;
            buf_i += samples_to_take;

            if let None = self.ended_at {
                if self.song_over() {
                    if self.playing.iter().all(|i| i.is_none()) {
                        self.ended_at = Some(self.song_sample)
                    }
                }
            }
        }
    }

    pub(super) fn is_playing(&self, sample: u64) -> bool {
        if sample < self.started_at { 
            return true 
        }
        if let Some(ended_at) = self.ended_at {
            if sample - self.started_at >= ended_at { 
                return false;
            }
        }
        return true
    }

    fn on_deadline_hit(&mut self) {
        while !self.song_over() {
            match self.song.data[self.cursor] {
                crate::Packet::Play { channel, frequency, duration } => {
                    self.add_voice(channel, frequency, duration);
                    self.cursor += 1;
                }
                crate::Packet::Wait(ticks) => {
                    self.song_next_event_at = self.song_sample + ticks as u64 * self.config.samples_per_tick;
                    self.cursor += 1;
                    break;
                }
            }
        }

        if self.song_over() { self.song_next_event_at = u64::MAX; }
    }

    fn song_over(&self) -> bool {
        !(0..self.song.data.len()).contains(&self.cursor)
    }

    fn add_voice(&mut self, channel: u16, frequency: u16, duration: u16) {
        let note_ix = self.song_next_note;
        self.song_next_note += 1;

        let patch = if channel == 0 { sample_patch_1() } else { sample_patch_2() };

        let voice_to_use = Some(Voice::new(note_ix, self.config, patch, duration, frequency));
        for v in self.playing.iter_mut() {
            if let None = v { *v = voice_to_use; return }
        }

        let (ix, _) = self.playing.iter().enumerate().min_by_key(|(_, x)| { x.as_ref().unwrap().note_ix }).unwrap();
        self.playing[ix] = voice_to_use;
    }
}