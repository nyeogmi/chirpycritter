#[derive(Clone, Copy)]
pub struct Time {
    pub beat: f32,
    pub second: f32,
    pub beats_per_second: f32,
}
impl Time {
    pub fn delta(&self, old_time: Time) -> Time {
        return Time { 
            beat: self.beat - old_time.beat, 
            second: self.second - old_time.second,
            beats_per_second: self.beats_per_second,
        }
    }

    pub fn zero(beats_per_second: f32) -> Time {
        Time { beat: 0.0, second: 0.0, beats_per_second }
    }

    pub fn shift_back_beats(&mut self, beats: f32) {
        self.beat -= beats;
        self.second -= beats / self.beats_per_second;
    } 

    pub fn shift_back_seconds(&mut self, seconds: f32) {
        self.beat -= seconds * self.beats_per_second;
        self.second -= seconds;
    } 
}