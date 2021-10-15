use std::{thread, time::Duration};

use chirpycritter::{Stock, SynthEnvironment, midi};
fn main() {
    let stock = SynthEnvironment::<Stock>::start();

    let god_rest_ye = std::fs::read("examples/god_rest_ye.mid").unwrap();
    let song = midi::convert_midi(&god_rest_ye);
    stock.setup(|synth| synth.play(song));

    while stock.is_playing() {
        thread::sleep(Duration::from_millis(1));
    }
}