// NYEO NOTE: This is a wrapper for the scaffolding in the beep demo from CPAL.
// I've simplified a few things and will likely de-simplify them later

// TODO: get rid of unwrap()

use std::{sync::{Arc, Mutex}, time::Instant};

use cpal::{Stream, traits::{DeviceTrait, HostTrait, StreamTrait}};

use super::*;

pub struct SynthEnvironment<S: Synthesizer> {
    first_sample_instant: Instant,
    synthesizer: Arc<Mutex<SynthState<S>>>,
    config: SynthConfig,
    #[allow(dead_code)]
    stream: Stream,  // just keep this so it doesn't get dealloc'ed
}

impl<S: Synthesizer> SynthEnvironment<S> {
    pub fn is_playing(&self) -> bool {
        // TODO: estimate the current sample: then ask the synthesizer
        let estimated_nanos = self.first_sample_instant.elapsed().as_nanos();
        let estimated_sample = (estimated_nanos * self.config.sample_rate as u128) / 1_000_000_000u128;
        let synth = self.synthesizer.lock().unwrap();
        synth.synth.is_playing(estimated_sample as u64)
    }

    pub fn start() -> SynthEnvironment<S> {
        let host = cpal::default_host();

        let device = host.default_output_device().unwrap();
        let config = device.default_output_config().unwrap();

        println!("device: {:?}", device.name());
        println!("config: {:?}", config);

        let sample_format = config.sample_format();
        let stream_config: cpal::StreamConfig = config.into();

        let sample_rate = stream_config.sample_rate.0 as u64;
        let channels = stream_config.channels as usize;

        let synth_config = SynthConfig { sample_rate };

        let synth_ref = Arc::new(Mutex::new(SynthState::new(synth_config)));

        let sr = synth_ref.clone();
        let stream = match (sample_format, channels) {
            (cpal::SampleFormat::F32, 2) => Self::run_stereo::<f32>(sr, &device, &stream_config),
            (cpal::SampleFormat::I16, 2) => Self::run_stereo::<i16>(sr, &device, &stream_config),
            (cpal::SampleFormat::U16, 2) => Self::run_stereo::<u16>(sr, &device, &stream_config),
            _ => panic!("don't know how to run in mono yet")
        };

        SynthEnvironment {
            first_sample_instant: Instant::now(),
            synthesizer: synth_ref,
            config: synth_config,
            stream,
        }
    }

    pub fn setup(&self, f: impl FnOnce(&mut S)) {
        let mut s = self.synthesizer.lock().unwrap();
        f(&mut s.synth)
    }

    fn run_stereo<T>(synth_ref: Arc<Mutex<SynthState<S>>>, device: &cpal::Device, config: &cpal::StreamConfig) -> Stream
    where
        T: cpal::Sample,
    {
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                let mut s = synth_ref.lock().unwrap(); // TODO: What the fuck is a poisonerror
                s.write_data_stereo(data)
            },
            err_fn,
        ).unwrap();
        stream.play().unwrap();
        stream
    }

}

pub struct SynthState<S: Synthesizer> {
    // TODO: Reset buf state in any cases? (End of song, etc.)
    buf: FixedBuf<1>,
    buf_ix: usize,
    synth: S,
}

impl<S: Synthesizer> SynthState<S> {
    pub(crate) fn new(synth_config: SynthConfig) -> SynthState<S> {
        let buf = FixedBuf::new();
        let buf_ix = buf.len();
        Self {
            buf, buf_ix,
            synth: S::new(synth_config)
        }
    }

    pub fn write_data_stereo<T>(&mut self, output: &mut [T])
    where
        T: cpal::Sample,
    {
        let mut output_i = 0;
        loop {
            let frames_needed = output.len() - output_i;
            if frames_needed <= 0 { break; }

            if self.buf_ix >= self.buf.n_raw_samples() {
                self.buf_ix = 0;
                self.synth.populate(&mut self.buf)
            }

            let frames_available = self.buf.n_raw_samples() - self.buf_ix;

            let frames_to_take = frames_available.min(frames_needed);
            for frame in 0..frames_to_take {
                let samp = self.buf.raw_sample(self.buf_ix + frame);
                output[output_i + frame] = cpal::Sample::from::<f32>(&samp);
            }
            output_i += frames_to_take;
            self.buf_ix += frames_to_take;
        }
    }
}