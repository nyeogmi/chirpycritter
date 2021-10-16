use crate::TimeConfig;

#[derive(Clone, Copy)]
pub(crate) struct Trigger {
    pub config: TimeConfig,
    pub sample: u64,
    pub frequency: u16,
    pub released_at: Option<u64>,

    /*
    pub n_retriggers: usize,
    pub retrigger_every: u64,
    */
}