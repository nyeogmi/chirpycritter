use super::tables;

#[derive(Clone, Copy, Debug)]
pub enum Waveform {
    Sine,
    Square,
    Triangle,
    Saw,
}

// TODO: Nearly all of this code can probably be done way faster
impl Waveform {
    pub fn at(&self, pwm: f32, pos: u32) -> f32 {
        let cycle_width = ((1.0 - pwm / 2.0) * u32::MAX as f32) as u64;  // it's more natural to have this go up from 0.0 to 1.0, where 1.0 is most intense
        let cycle_width = cycle_width.max((u32::MAX/2) as u64).min(u32::MAX as u64);

        let pos = ((pos as u64 * u32::MAX as u64) / cycle_width) as u32;
        let pos2 = ((pos >> 22) & 0x3ff) as usize;  // from 0 to 1024

        let table = match self {
            Waveform::Sine => { tables::SIN }
            Waveform::Square => { tables::SQUARE }
            Waveform::Triangle => { tables::TRIANGLE }
            Waveform::Saw => { tables::SAW }
        };
        table[pos2]
    }
}