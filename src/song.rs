use std::borrow::Cow;

#[derive(Debug)]
pub struct Song {
    pub ticks_per_second: u64,
    pub data: Cow<'static, [Packet]>,
}

#[derive(Clone, Copy, Debug)]
pub enum Packet {
    Play(u16, u16),  // hertz, duration
    Wait(u16), // ticks
}