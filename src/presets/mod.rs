use crate::*;

pub fn load() -> Bank {
    let mut bank = Bank::new();
    add(&mut bank);
    bank
}

pub fn add(bank: &mut Bank) {
    // NOTE: These are placeholders
    bank.add(Patch {
        name: INIT.to_string(),
        author: "Nyeogmi".to_string(),
        data: PatchData::init(),
    });

    bank.add(Patch {
        name: "trombone".to_string(),
        author: "Nyeogmi".to_string(),
        data: {
            let mut p = bank.get(INIT).unwrap().data;
            p.osc1.waveform = Waveform::Triangle;
            p.osc1.mul_gain.value = 0.4;
            p.osc1.pulse_width = Modulated::just(0.2);
            p.osc1.pulse_width.lfo = ModLfo::Lfo1;
            p.osc1.pulse_width.lfo_amplitude = 0.2;

            p.osc2 = Some({
                let mut o2 = p.osc1;
                o2.waveform = Waveform::Saw;
                o2.mul_gain.value = 0.3;
                o2.pulse_width = Modulated::just(0.2);
                o2.pulse_width.lfo = ModLfo::Lfo1;
                o2.pulse_width.lfo_amplitude = 0.2;
                o2
            });

            p.modulators.gain1.decay = 0.2;
            p.modulators.gain1.sustain = 0.8;
            p.modulators.gain2.attack = 0.1;
            p.modulators.gain2.decay = 0.7;
            p.modulators.gain2.sustain = 0.2;

            p.modulators.lfo1.sync = true;
            p.modulators.lfo1.period = 1.0;

            p
        }
    });

    bank.add(Patch {
        name: "beetle".to_string(),
        author: "Nyeogmi".to_string(),
        data: {
            let mut p = bank.get("trombone").unwrap().data;
            p.osc1.waveform = Waveform::Square;
            (&mut p.osc2).unwrap().waveform = Waveform::Square;
            p
        }
    });

    bank.add(Patch {
        name: "chiffoff".to_string(),
        author: "Nyeogmi".to_string(),
        data: bank.get("trombone").unwrap().data,
    });

    bank.add(Patch {
        name: "tenor".to_string(),
        author: "Nyeogmi".to_string(),
        data: bank.get("trombone").unwrap().data,
    });
}