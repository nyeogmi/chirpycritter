// NYEO NOTE: This is a wrapper for the scaffolding in the beep demo from CPAL.
// I've simplified a few things and will likely de-simplify them later

// TODO: get rid of unwrap()

use std::{sync::{Arc, Mutex}, time::Instant};

use cpal::{Stream, traits::{DeviceTrait, HostTrait, StreamTrait}};

pub trait Synthesizer: 'static+Send {
    fn new(config: SynthConfig) -> Self;
    fn next_sample(&mut self) -> f32;
    fn is_playing(&self, sample: u64) -> bool;
}

pub struct SynthEnvironment<S: Synthesizer> {
    first_sample_instant: Instant,
    synthesizer: Arc<Mutex<S>>,
    config: SynthConfig,
    #[allow(dead_code)]
    stream: Stream,  // just keep this so it doesn't get dealloc'ed
}

#[derive(Clone, Copy)]
pub struct SynthConfig {
    pub sample_rate: u64
}

impl<S: Synthesizer> SynthEnvironment<S> {
    pub fn is_playing(&self) -> bool {
        // TODO: estimate the current sample: then ask the synthesizer
        let estimated_nanos = self.first_sample_instant.elapsed().as_nanos();
        let estimated_sample = (estimated_nanos * self.config.sample_rate as u128) / 1_000_000_000u128;
        let synth = self.synthesizer.lock().unwrap();
        synth.is_playing(estimated_sample as u64)
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

        let synth_ref = Arc::new(Mutex::new(S::new(synth_config)));

        let sr = synth_ref.clone();
        let stream = match sample_format {
            cpal::SampleFormat::F32 => Self::run::<f32>(sr, &device, &stream_config, channels),
            cpal::SampleFormat::I16 => Self::run::<i16>(sr, &device, &stream_config, channels),
            cpal::SampleFormat::U16 => Self::run::<u16>(sr, &device, &stream_config, channels),
        };

        SynthEnvironment {
            first_sample_instant: Instant::now(),
            synthesizer: synth_ref,
            config: synth_config,
            stream,
        }
    }

    fn run<T>(synth_ref: Arc<Mutex<S>>, device: &cpal::Device, config: &cpal::StreamConfig, channels: usize) -> Stream
    where
        T: cpal::Sample,
    {
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                let mut s = synth_ref.lock().unwrap(); // TODO: What the fuck is a poisonerror
                write_data(data, channels, &mut *s)
            },
            err_fn,
        ).unwrap();
        stream.play().unwrap();
        stream
    }

}

fn write_data<S, T>(output: &mut [T], channels: usize, synth: &mut S)
where
    S: Synthesizer,
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&synth.next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}