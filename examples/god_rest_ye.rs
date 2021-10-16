use std::{thread, time::Duration};

use chirpycritter::{convert_midi, ChirpyCritter, SynthEnvironment};

fn main() {
    let stock = SynthEnvironment::<ChirpyCritter>::start();

    let god_rest_ye = std::fs::read("examples/god_rest_ye.mid").unwrap();
    let song = convert_midi(&god_rest_ye);
    stock.setup(|synth| synth.play(song));

    while stock.is_playing() {
        thread::sleep(Duration::from_millis(1));
    }
}