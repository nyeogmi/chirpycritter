use std::collections::BTreeMap;

use crate::*;

pub const INIT: &'static str = "INIT";

#[derive(Clone)]
pub struct Bank {
    patches: BTreeMap<String, Patch>
}

#[derive(Clone)]
pub struct Patch {
    pub name: String,
    pub author: String,
    pub data: PatchData<f32>,
}

impl Bank {
    pub fn new() -> Bank {
        Bank {
            patches: BTreeMap::new()
        }
    }

    pub fn add(&mut self, patch: Patch) -> Option<Patch> {
        // returns any patch that might have been pushed out
        self.patches.insert(patch.name.clone(), patch)
    }

    pub fn get(&self, name: &str) -> Option<&Patch> {
        self.patches.get(name)
    }
}