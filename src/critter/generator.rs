use std::{num::Wrapping, ops::Range};

use crate::*;
use super::*;

use fastapprox::faster::pow2 as fast_pow2;

#[derive(Clone)]
pub(crate) struct Generator {
    osc1: StereoOscImpl,
    osc2: Option<StereoOscImpl>,
    modulators: Modulators<u64>,
    vcf1_l: VCFImpl,  
    vcf1_r: VCFImpl,  
}

#[derive(Clone)]
enum StereoOscImpl {
    Mono(OscImpl),
    Stereo(Spread, OscImpl, OscImpl),
}

#[derive(Clone)]
pub(crate) struct OscImpl {
    pub patch: Osc,
    pub waveform_progress: Wrapping<u32>,
}

impl Generator {
    pub fn new_for(config: TimeConfig, patch: Patch<u64>) -> Generator {
        let osc1 = StereoOscImpl::new(patch.osc1);
        let osc2 = patch.osc2.map(StereoOscImpl::new);

        Generator {
            osc1,
            osc2,
            modulators: patch.modulators,
            vcf1_l: VCFImpl::new(config.samples_per_second, patch.vcf1),
            vcf1_r: VCFImpl::new(config.samples_per_second, patch.vcf1),
        }
    }

    pub(crate) fn is_playing(&self, trigger: Trigger, sample: u64) -> bool {
        if self.modulators.gain1.is_playing(trigger.release_at, sample) {
            return true
        }
        if let Some(_) = &self.osc2 {
            if self.modulators.gain2.is_playing(trigger.release_at, sample) {
                return true
            }
        }
        return false
    }

    pub(crate) fn populate<Buf: StereoBuf>(&mut self, trigger: Trigger, samples: Range<u64>, buf: &mut Buf) {
        // TODO: Take this more than once
        assert!(samples.end - samples.start == buf.len() as u64);
        let mut snap = self.modulators.snap(trigger, samples.start);
        let gain1 = snap.gain1;
        let gain2 = snap.gain2;

        match (&mut self.osc1, &mut self.osc2) {
            // TODO: avoid right filter if we're in mono
            (osc1, osc2) => {
                osc1.populate(trigger, samples.clone(), &mut snap, gain1, |i, (l, r)| { buf.set(i, [l, r]) });
                if let Some(osc2) = osc2 {
                    osc2.populate(trigger, samples.clone(), &mut snap, gain2, |i, (l, r)| { 
                        let [old_l, old_r] = buf.get(i);
                        buf.set(i, [old_l + l, old_r + r]) 
                    });
                }
                self.vcf1_l.process(&snap, &mut buf.left());
                self.vcf1_r.process(&snap, &mut buf.right());
            }
        }
    }
}

impl StereoOscImpl {
    pub(super) fn new(osc: Osc) -> StereoOscImpl {
        if osc.spread.needs_stereo() {
            StereoOscImpl::Stereo(osc.spread, OscImpl::new(osc), OscImpl::new(osc))
        } else {
            StereoOscImpl::Mono(OscImpl::new(osc))
        }
    }

    pub(super) fn populate(&mut self, trigger: Trigger, samples: Range<u64>, snap: &mut ModulatorSnapshot, gain: f32, mut write: impl FnMut(usize, (f32, f32))) {
        let mut samp = |osc: &mut OscImpl, offset: f32 | {
            let mut frequency = trigger.frequency as f32;
            frequency = transpose(frequency, osc.patch.frequency_offset.over(snap).0 + offset);
            snap.true_frequency = frequency;
            // TODO: Do with doubles instead?
            snap.waveform_progress = ((snap.true_frequency as f32 / trigger.config.samples_per_second as f32) * u32::MAX as f32) as u32;
            osc.sample(snap, gain)
        };

        for (i, _) in samples.enumerate() {
            match self {
                StereoOscImpl::Mono(o) => {
                    let c = samp(o, 0.0);
                    write(i, (c, c))
                }
                StereoOscImpl::Stereo(spread, l, r) => {
                    let pure_l = samp(l, -spread.frequency);
                    let pure_r = samp(r, spread.frequency);

                    // Move closer
                    // TODO: Use a real panning function for this
                    let (l, r) = (
                        lerp(lerp(spread.amount, 0.5, 0.0), pure_l, pure_r), 
                        lerp(lerp(spread.amount, 0.5, 0.0), pure_r, pure_l)
                    );
                    write(i, (l, r))
                }
            }
        }
    }
}

impl OscImpl {
    pub(super) fn new(osc: Osc) -> OscImpl {
        OscImpl { 
            patch: osc,
            waveform_progress: Wrapping(0u32),
        }
    }

    pub(super) fn sample(&mut self, snap: &ModulatorSnapshot, gain: f32) -> f32 {
        self.waveform_progress += Wrapping(snap.waveform_progress); 

        let base_wave = self.patch.waveform.at(self.patch.pulse_width.over(snap).0, self.waveform_progress.0);

        base_wave * gain * self.patch.mul_gain.over(snap).0
    }
}

fn transpose(frequency: f32, semitones: f32) -> f32 {
    frequency * fast_pow2(semitones/12.0)
}