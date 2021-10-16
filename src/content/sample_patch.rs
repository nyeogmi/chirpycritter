use crate::*;

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