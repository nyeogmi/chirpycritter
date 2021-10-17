use std::{fs::File, thread, time::Duration};

use chirpycritter::{ChirpyCritter, SynthEnvironment, convert_midi, wavexport};

fn main() {
    let stock = SynthEnvironment::<ChirpyCritter>::start();

    let midi = std::fs::read("examples/idyll.mid").unwrap();
    let song = convert_midi(&midi);
    stock.setup(|synth| synth.play(song.clone()));

    println!("start export");
    let mut wav = File::create("examples/idyll.wav").unwrap();
    wavexport(|synth: &mut ChirpyCritter| synth.play(song.clone()), &mut wav);
    println!("finished export");

    while stock.is_playing() {
        thread::sleep(Duration::from_millis(1));
    }
}