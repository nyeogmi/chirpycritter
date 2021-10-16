use crate::*;
use super::*;

#[derive(Clone, Copy)]
pub struct Modulators<T> {
    pub echoes: Echoes<T>,
    pub gain1: ADSR<T>, pub gain2: ADSR<T>,
    pub env1: ADSR<T>, pub env2: ADSR<T>, pub env3: ADSR<T>,
    pub lfo1: LFO<T>, pub lfo2: LFO<T>, pub lfo3: LFO<T>,
}

#[derive(Clone, Copy)]
pub(super) struct ModulatorSnapshot {
    pub(super) spread_pitch_offset: f32,  // TODO: Envelope or something for this?
    pub(super) echo: u64,
    pub(super) gain1: f32, pub(super) gain2: f32,
    pub(super) env1: f32, pub(super) env2: f32, pub(super) env3: f32,
    pub(super) lfo1: f32, pub(super) lfo2: f32, pub(super) lfo3: f32
}

impl Modulators<f32> {
    pub(crate) fn apply_time(&self, config: TimeConfig) -> Modulators<u64> {
        Modulators { 
            echoes: self.echoes.apply_time(config), 
            gain1: self.gain1.apply_time(config), 
            gain2: self.gain2.apply_time(config), 
            env1: self.env1.apply_time(config), 
            env2: self.env2.apply_time(config), 
            env3: self.env3.apply_time(config), 
            lfo1: self.lfo1.apply_time(config), 
            lfo2: self.lfo2.apply_time(config), 
            lfo3: self.lfo3.apply_time(config), 
        }
    }
}

impl Modulators<u64> {
    pub(super) fn snap(&self, trigger: Trigger) -> ModulatorSnapshot {
        let (echo, sample) = self.echoes.to_echo(trigger.sample);

        ModulatorSnapshot {
            spread_pitch_offset: 0.0,
            echo: echo,
            gain1: self.gain1.at(trigger.release_at, sample),
            gain2: self.gain2.at(trigger.release_at, sample),
            env1: self.env1.at(trigger.release_at, sample),
            env2: self.env2.at(trigger.release_at, sample),
            env3: self.env3.at(trigger.release_at, sample),
            lfo1: self.lfo1.at(trigger.release_at, sample),
            lfo2: self.lfo2.at(trigger.release_at, sample),
            lfo3: self.lfo3.at(trigger.release_at, sample),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Modulated {
    pub value: f32,
    pub value_echo_dampen: f32,
    pub env: ModEnvelope,
    pub env_amplitude: f32,
    pub env_echo_dampen: f32,
    pub lfo: ModLfo,
    pub lfo_amplitude: f32,
    pub lfo_echo_dampen: f32,
    pub sidechain: ModEnvelope,
}

#[derive(Clone, Copy)]
pub enum ModEnvelope { None, Env1, Env2, Env3, }

#[derive(Clone, Copy)]
pub enum ModLfo { None, Lfo1, Lfo2, Lfo3, }

impl Modulated {
    pub(crate) fn just(value: f32) -> Modulated {
        Modulated {
            value, value_echo_dampen: 1.0,
            env: ModEnvelope::None, env_amplitude: 0.0, env_echo_dampen: 1.0,
            lfo: ModLfo::None, lfo_amplitude: 0.0, lfo_echo_dampen: 1.0,
            sidechain: ModEnvelope::None,
        }
    }

    pub(super) fn over(&self, snap: ModulatorSnapshot) -> f32 {
        let mut val = self.value * self.value_echo_dampen.powf(snap.echo as f32);
        val += snap.get_env(self.env, 0.0) * self.env_amplitude * self.env_echo_dampen.powf(snap.echo as f32);
        val += snap.get_lfo(self.lfo) * self.lfo_amplitude * snap.get_env(self.sidechain, 1.0) * self.lfo_echo_dampen.powf(snap.echo as f32);
        val
    }
}
impl ModulatorSnapshot {
    fn get_env(&self, env: ModEnvelope, default: f32) -> f32 {
        match env {
            ModEnvelope::None => default,
            ModEnvelope::Env1 => self.env1,
            ModEnvelope::Env2 => self.env2,
            ModEnvelope::Env3 => self.env3
        }
    }

    fn get_lfo(&self, lfo: ModLfo) -> f32 {
        match lfo {
            ModLfo::None => 0.0,
            ModLfo::Lfo1 => self.lfo1,
            ModLfo::Lfo2 => self.lfo2,
            ModLfo::Lfo3 => self.lfo3,
        }
    }
}