
#[derive(Clone, Copy)]
pub struct SongConfig {
    pub sample_rate: u64,
    pub samples_per_tick: u64,
    pub samples_per_beat: u64,
}