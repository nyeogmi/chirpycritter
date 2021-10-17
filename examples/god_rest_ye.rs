use std::{thread, time::Duration};

use chirpycritter::*;

fn main() {
    let stock = SynthEnvironment::<ChirpyCritter>::start();

    let presets = presets::load();

    let god_rest_ye = std::fs::read("examples/god_rest_ye.mid").unwrap();
    let song = convert_midi(&presets, &god_rest_ye);
    stock.setup(|synth| synth.play(song));

    while stock.is_playing() {
        thread::sleep(Duration::from_millis(1));
    }
}