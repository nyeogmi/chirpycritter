use crate::*;

pub fn sample_patch_1() -> Patch<f32> {
    Patch { 
        osc1: Osc {
            mul_gain: Modulated::just(0.8),
            frequency_offset: Modulated::just(0.0),
            waveform: Waveform::Square,
            pulse_width: Modulated::just(0.0),
            spread: Spread { frequency: 0.2, amount: 0.1 },
        }, 
        osc2: None,
        vcf1: VCF {
            cutoff: Modulated { env: ModEnvelope::Env1, env_amplitude: 0.4, ..Modulated::just(0.4) },
            resonance: Modulated::just(0.7),
        },
        modulators: Modulators {
            echoes: Echoes { n_times: 0, sync: true, period: 0.25 },
            gain1: ADSR { attack: 0.005, decay: 0.5, sustain: 0.2, release: 0.2 },
            gain2: ADSR::maxed(),
            env1: ADSR { attack: 0.005, decay: 0.4, sustain: 0.0, release: 0.4 },
            env2: ADSR::maxed(),
            env3: ADSR::maxed(),
            lfo1: LFO { sync: true, period: 0.125, pulse_width: 0.0, waveform: Waveform::Sine, adsr: Some(ADSR { attack: 0.1, decay: 0.1, sustain: 0.025, release: 0.1 }) },
            lfo2: LFO::none(),
            lfo3: LFO::none(),
        }
    }
}

pub fn sample_patch_2() -> Patch<f32> {
    Patch { 
        osc1: Osc {
            mul_gain: Modulated { 
                value_echo_dampen: 0.25,
                ..Modulated::just(1.0)
            },
            frequency_offset: Modulated {
                lfo: ModLfo::Lfo1,
                lfo_amplitude: 1.0,
                lfo_echo_dampen: 0.65,
                ..Modulated::just(0.0)
            },
            waveform: Waveform::Sine,
            pulse_width: Modulated {
                env: ModEnvelope::Env1, env_amplitude: 0.3,
                ..Modulated::just(0.3)
            },
            spread: Spread { frequency: 0.2, amount: 0.1 },
        }, 
        osc2: None,
        vcf1: VCF {
            cutoff: Modulated { ..Modulated::just(1.0) },
            resonance: Modulated::just(0.0),
        },
        modulators: Modulators {
            echoes: Echoes { n_times: 16, sync: true, period: 0.25 },
            gain1: ADSR { attack: 0.3, decay: 0.0, sustain: 0.1, release: 0.2 },
            gain2: ADSR::maxed(),
            env1: ADSR { attack: 0.02, decay: 0.02, sustain: 0.0, release: 0.0 },
            env2: ADSR::maxed(),
            env3: ADSR::maxed(),
            lfo1: LFO { sync: true, period: 0.125, pulse_width: 0.0, waveform: Waveform::Sine, adsr: Some(ADSR { attack: 0.1, decay: 0.1, sustain: 0.025, release: 0.1 }) },
            lfo2: LFO::none(),
            lfo3: LFO::none(),
        }
    }
}