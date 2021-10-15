use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub enum Waveform {
    Sine,
    Square,
    Saw,
}

impl Waveform {
    pub fn at(&self, pulse_width: f32, pos: f32) -> f32 {
        let pulse_width = 1.0 - pulse_width;  // it's more natural to have this go up from 0.0 to 1.0, where 1.0 is most intense
        match self {
            Waveform::Sine => {
                let cycle_width = 0.5 + pulse_width * 0.5;
                let pos2 = pos / cycle_width;
                let pos2 = pos2 - pos2.floor();
                (pos2 * 2.0 * PI).sin()
            }
            Waveform::Square => if pos < (0.5 * pulse_width) { -1.0 } else { 1.0 },
            Waveform::Saw => {
                let cycle_width = 0.5 + pulse_width * 0.5;
                let pos2 = pos / cycle_width;
                let pos2 = pos2 - pos2.floor();
                pos2 * 2.0 - 1.0
            }
        }
    }
}
