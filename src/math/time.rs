#[derive(Clone, Copy)]
pub(crate) struct TimeConfig {
    pub samples_per_second: u64,
    pub samples_per_tick: u64,
    pub samples_per_beat: u64,
}