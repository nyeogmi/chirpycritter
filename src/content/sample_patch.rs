use crate::*;

/*
pub fn sample_patch() -> Patch<f32> {
    Patch { 
        osc1: Patch1 {
            gain: Envelope { 
                base: 0.0, 
                adsr: Some(ADSR { low: 0.0, high: 0.3, attack: 0.0, decay: 0.8, sustain: 0.4, release: 0.4 }) ,
                lfo: None,
                /*
                echoes: Some(Echoes {
                    n_times: 4,
                    sync: true,
                    period: 0.25,
                    decay: 0.8,
                }),
                */
                echoes: None,
            },
            frequency_offset: Envelope { 
                base: 0.0, 
                adsr: None,
                lfo: None,
                echoes: None,
            }, 
            waveform: Waveform::Square,
            pulse_width: Envelope { 
                base: 0.0, 
                adsr: Some(ADSR { low: 0.0, high: 0.3, attack: 0.3, decay: 0.2, sustain: 0.05, release: 0.1}),
                lfo: None,
                echoes: None,
            },
        },

        osc2: None,
        spread: Spread {
            frequency: 0.1,
            amount: 0.1,
        }
        /*
        osc2: Some(Generator1 {
            gain: Envelope { 
                base: 0.0, 
                adsr: Some(ADSRf { low: 0.0, high: 0.1, attack: 0.0, decay: 0.2, sustain: 0.2, release: 0.1 }) ,
                lfo: None,
                echoes: None,
            },
            frequency_offset: Envelope { 
                base: 0.0, 
                adsr: None ,
                lfo: Some(LFOf {
                    adsr: None,

                    low: -0.5,
                    high: 0.5,

                    sync: true,
                    period: 1.0,
                    pulse_width: 0.0,
                    waveform: Waveform::Sine,
                }),
                echoes: None,
            }, 
            frequency,
            waveform: Waveform::Square,
            pulse_width: Envelope { 
                base: 0.0, 
                adsr: None,
                lfo: None,
                echoes: None,
            },

            waveform_progress: 0.0,
        }),
        */
    }
}
*/
pub fn sample_patch_1() -> Patch<f32> {
    Patch { 
        osc1: Osc {
            mul_gain: Modulated { 
                value_echo_dampen: 0.3,
                ..Modulated::just(0.1)
            },
            frequency_offset: Modulated {
                env: ModEnvelope::Env1, env_amplitude: 12.0, env_echo_dampen: 0.0,
                lfo: ModLfo::Lfo1,
                lfo_amplitude: 1.0,
                lfo_echo_dampen: 0.15,
                ..Modulated::just(0.1)
            },
            waveform: Waveform::Square,
            pulse_width: Modulated::just(0.0),
            spread: Spread { frequency: 0.2, amount: 0.1 },
        }, 
        osc2: Some(Osc {
            mul_gain: Modulated { 
                value_echo_dampen: 0.3,
                ..Modulated::just(0.4)
            },
            frequency_offset: Modulated::just(0.0), // -12.0),
            waveform: Waveform::Square,
            pulse_width: Modulated::just(0.0),
            spread: Spread { frequency: 0.2, amount: 0.1 },
        }), 
        vcf1: VCF {
            cutoff: Modulated { 
                env: ModEnvelope::Env2, env_amplitude: 0.4,
                ..Modulated::just(0.4)
            },
            resonance: Modulated::just(0.7),
        },
        modulators: Modulators {
            echoes: Echoes { n_times: 4, sync: true, period: 0.25 },
            gain1: ADSR { attack: 0.0, decay: 0.5, sustain: 0.2, release: 0.2 },
            gain2: ADSR { attack: 0.0, decay: 0.6, sustain: 0.5, release: 0.2 },
            env1: ADSR { attack: 0.0, decay: 0.1, sustain: 0.0, release: 0.0 },
            env2: ADSR { attack: 0.1, decay: 0.1, sustain: 0.0, release: 0.0 },
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
                lfo_echo_dampen: 0.15,
                ..Modulated::just(0.0)
            },
            waveform: Waveform::Sine,
            pulse_width: Modulated::just(0.8),
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
            env1: ADSR::maxed(),
            env2: ADSR::maxed(),
            env3: ADSR::maxed(),
            lfo1: LFO { sync: true, period: 0.125, pulse_width: 0.0, waveform: Waveform::Sine, adsr: Some(ADSR { attack: 0.1, decay: 0.1, sustain: 0.025, release: 0.1 }) },
            lfo2: LFO::none(),
            lfo3: LFO::none(),
        }
    }
}