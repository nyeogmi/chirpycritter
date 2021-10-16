use crate::*;
use super::*;

#[derive(Clone, Copy)]
pub(crate) struct Generator {
    osc1: StereoOscImpl,
    osc2: Option<StereoOscImpl>,
    modulators: Modulators<u64>,
    vcf1_l: VCFImpl,  
    vcf1_r: VCFImpl,  
}

#[derive(Clone, Copy)]
enum StereoOscImpl {
    Mono(OscImpl),
    Stereo(Spread, OscImpl, OscImpl),
}

#[derive(Clone, Copy)]
pub(crate) struct OscImpl {
    pub patch: Osc,
    pub waveform_progress: f32,
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

    pub(crate) fn is_playing(&self, trigger: Trigger) -> bool {
        if self.modulators.gain1.is_playing(trigger.released_at, trigger.sample) {
            return true
        }
        if let Some(_) = &self.osc2 {
            if self.modulators.gain2.is_playing(trigger.released_at, trigger.sample) {
                return true
            }
        }
        return false
    }

    pub(crate) fn sample(&mut self, trigger: Trigger) -> (f32, f32) {
        let snap = self.modulators.snap(trigger);

        match (&mut self.osc1, &mut self.osc2) {
            // avoid right filter if we're in mono
            (StereoOscImpl::Mono(o1), None) => {
                let x = o1.sample(trigger, snap, snap.gain1);
                let processed = self.vcf1_l.process(x, snap);
                (processed, processed)
            }
            (StereoOscImpl::Mono(o1), Some(StereoOscImpl::Mono(o2))) => {
                let x = o1.sample(trigger, snap, snap.gain1);
                let y = o2.sample(trigger, snap, snap.gain1);
                let processed = self.vcf1_l.process(x + y, snap);
                (processed, processed)
            }
            (osc1, osc2) => {
                let (mut l, mut r) = osc1.sample(trigger, snap, snap.gain1);
                if let Some(osc2) = osc2 {
                    let (l2, r2) = osc2.sample(trigger, snap, snap.gain2);
                    l += l2;
                    r += r2;
                }
                (self.vcf1_l.process(l, snap), self.vcf1_r.process(r, snap))
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

    pub(super) fn sample(&mut self, trigger: Trigger, snap: ModulatorSnapshot, gain: f32) -> (f32, f32) {
        match self {
            StereoOscImpl::Mono(o) => {
                let c = o.sample(trigger, snap, gain);
                (c, c)
            }
            StereoOscImpl::Stereo(spread, l, r) => {
                let mut snap_l = snap;
                let mut snap_r = snap;

                snap_l.spread_pitch_offset -= spread.frequency;
                snap_r.spread_pitch_offset += spread.frequency;

                let pure_l = l.sample(trigger, snap_l, gain);
                let pure_r = r.sample(trigger, snap_r, gain);

                // Move closer
                // TODO: Use a real panning function for this
                let (l, r) = (
                    lerp(lerp(spread.amount, 0.5, 0.0), pure_l, pure_r), 
                    lerp(lerp(spread.amount, 0.5, 0.0), pure_r, pure_l)
                );
                (l, r)
            }
        }
    }
}

impl OscImpl {
    pub(super) fn new(osc: Osc) -> OscImpl {
        OscImpl { 
            patch: osc,
            waveform_progress: 0.0,
        }
    }

    pub(super) fn sample(&mut self, trigger: Trigger, snap: ModulatorSnapshot, gain: f32) -> f32 {
        let mut frequency = trigger.frequency as f32;
        frequency = transpose(frequency, self.patch.frequency_offset.over(snap) + snap.spread_pitch_offset);

        self.waveform_progress += frequency as f32 / trigger.config.samples_per_second as f32;
        self.waveform_progress = self.waveform_progress - self.waveform_progress.floor();

        let base_wave = self.patch.waveform.at(self.patch.pulse_width.over(snap), self.waveform_progress);

        base_wave * gain * self.patch.mul_gain.over(snap)
    }
}

fn transpose(frequency: f32, semitones: f32) -> f32 {
    frequency * 2.0_f32.powf(semitones/12.0)
}