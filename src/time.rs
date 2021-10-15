#[derive(Clone, Copy)]
pub struct Time {
    pub beat: f32,
    pub second: f32,
    pub sample: u64,
}
impl Time {
    pub fn delta(&self, old_time: Time) -> Time {
        return Time { beat: self.beat - old_time.beat, second: self.second - old_time.second, sample: self.sample - old_time.sample }
    }

    pub const ZERO: Time = Time { beat: 0.0, second: 0.0, sample: 0 };
}