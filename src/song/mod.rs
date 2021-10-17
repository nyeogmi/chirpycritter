use std::borrow::Cow;

use crate::PatchData;

pub const TRACKS: usize = 16;

#[derive(Clone, Debug)]
pub struct Song {
    pub tracks: [Track; TRACKS],
    pub ticks_per_second: u64,
    pub ticks_per_beat: u64,
    pub data: Cow<'static, [Packet]>,
}

#[derive(Clone, Copy, Debug)]
pub enum Packet {
    Play {
        track: u16,
        frequency: u16,
        duration: u16,
    },  // hertz, duration
    Wait(u16), // ticks
}

#[derive(Clone, Copy, Debug)]
pub struct Track {
    pub patch: PatchData<f32>,
}