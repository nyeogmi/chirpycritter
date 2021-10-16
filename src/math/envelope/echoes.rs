
#[derive(Clone, Copy)]
pub struct Echoes {
    pub n_times: usize,

    pub sync: bool,
    pub period: f32,  // beats
    pub decay: f32,
}