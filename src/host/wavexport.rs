use std::io::{Seek, Write};

use super::*;

pub fn wavexport<S: Synthesizer, W: Write + Seek>(setup: impl Fn(&mut S), file: &mut W) {
    let sample_rate = 44100;
    let wav_header = wav::Header::new(
        wav::header::WAV_FORMAT_PCM,
        2,
        sample_rate as u32,
        16,
    );
    let mut s = S::new(SynthConfig { sample_rate });
    setup(&mut s);

    let mut buf = FixedBuf::<1>::new();
    let mut samples: Vec<i16> = Vec::new();
    let mut i: u64 = 0;
    'bigloop: loop {
        s.populate(&mut buf);

        for i2 in 0..buf.len() {
            if !s.is_playing(i) {
                break 'bigloop;
            }
            let [l, r] = buf.get(i2);
            i += 1;
            samples.push(cpal::Sample::from(&l));
            samples.push(cpal::Sample::from(&r));
        }
    }

    wav::write(wav_header, &wav::BitDepth::Sixteen(samples), file).unwrap()
}