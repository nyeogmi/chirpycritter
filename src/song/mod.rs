use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Song {
    pub ticks_per_second: u64,
    pub ticks_per_beat: u64,
    pub data: Cow<'static, [Packet]>,
}

#[derive(Clone, Copy, Debug)]
pub enum Packet {
    Play {
        channel: u16,
        frequency: u16,
        duration: u16,
    },  // hertz, duration
    Wait(u16), // ticks
}