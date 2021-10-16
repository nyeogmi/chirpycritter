use crate::TimeConfig;

#[derive(Clone, Copy)]
pub(crate) struct Trigger {
    pub config: TimeConfig,
    pub frequency: u16,
    pub release_at: u64,

    /*
    pub n_retriggers: usize,
    pub retrigger_every: u64,
    */
}