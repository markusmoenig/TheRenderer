use crate::prelude::*;

/// The available state of shapes and widgets,
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum TheState {
    Undefined,

    // Application States
    Normal,
    Disabled,
    Hover,
    Selected,
    Checked,
    Open,
}

#[derive(PartialEq, Eq, Copy, Hash, Clone, Debug)]
pub enum TheProperty {
    X,
    Y,
    Width,
    Height,
    Radius,
    Color,
    B,
    BC,
    Duration,
    M,
    S
}

/// StateProperties store the properties for a state.
#[derive(PartialEq, Clone, Debug)]
pub struct TheStateProperties {
    pub state               : TheState,

    properties              : Vec<Option<Vec<f32>>>,
    mask                    : Vec<bool>,
}

impl TheStateProperties {

    pub fn new(state: TheState) -> Self {
        Self {
            state,
            properties      : vec![None; 20],
            mask            : vec![false; 20],
        }
    }

    pub fn contains(&self, key: TheProperty) -> bool {
        self.properties[key as usize].is_some()
    }

    pub fn get(&self, key: TheProperty) -> &Option<Vec<f32>> {
        &self.properties[key as usize]
    }

    pub fn set(&mut self, key: TheProperty, value: Vec<f32>) {
        self.properties[key as usize] = Some(value);
    }

    pub fn test_mask(&self, key: TheProperty) -> bool {
        self.mask[key as usize]
    }

    pub fn gen_mask(&mut self) {
        for i in 0..20 {
            self.mask[i] = self.properties[i].is_some();
        }
    }

    /// Returns the animation duration for this property state
    pub fn get_duration(&self) -> f32 {
        let mut duration = 100.0_f32;
        if let Some(dur) = self.get(TheProperty::Duration) {
            duration = dur[0] as f32
        }
        duration
    }
}

/// Transitions between states.
#[derive(PartialEq, Clone, Debug)]
pub struct TheStateTransition {
    pub source              : TheStateProperties,
    pub dest                : TheStateProperties,

    pub start_time          : u128,

    trans_values            : FxHashMap<TheProperty, Vec<Vec<f32>>>,
    duration                : usize,
}

impl TheStateTransition {

    pub fn new(source: TheStateProperties, dest: TheStateProperties, start_time: u128) -> Self {

        Self {
            source,
            dest,
            start_time,
            trans_values            : FxHashMap::default(),
            duration                : 0,
        }
    }

    pub fn precalc(&mut self, keys: Vec<TheProperty>) {
        self.duration = self.dest.get_duration() as usize;

        for key in keys {

            let mut values : Vec<Vec<f32>> = vec![];

            if let Some(source_value) = self.source.get(key) {
                if let Some(dest_value) = self.dest.get(key) {

                    let duration = self.duration;

                    for i in 0..=duration {

                        let t = ((i as f32) / duration as f32).clamp(0.0, 1.0);
                        let v = self.lerp(source_value, dest_value, t);
                        values.push(v);
                    }
                }
            }

            if values.len() > 0 {
                self.trans_values.insert(key, values);
            }
        }
    }

    pub fn get(&self, key: TheProperty, time: &u128) -> Option<&Vec<f32>> {

        if let Some(values) = self.trans_values.get(&key) {
            let t = ((time - self.start_time) as usize).min(self.duration);
            if t < values.len() {
                return Some(&values[t]);
            }
        }

        if let Some(source_value) = self.source.get(key) {
            return Some(source_value);
        } else
        if let Some(dest_value) = self.dest.get(key) {
            return Some(dest_value);
        }

        None
    }

    pub fn get_time_delta(&self, time: &u128) -> f32 {
        ((*time - self.start_time) as f32 / self.dest.get_duration()).clamp(0.0, 1.0)
    }

    pub fn is_finished(&self, time: &u128) -> bool {
        //self.get_time_delta(time) >= 1.0
        (time - self.start_time) as usize >= self.duration
    }

    fn lerp(&self, source: &Vec<f32>, dest: &Vec<f32>, t: f32) -> Vec<f32> {

        #[inline(always)]
        fn lerp_v(v0: f32, v1: f32, t: f32) -> f32 {
            return (1.0 - t) * v0 + t * v1;
        }

        #[inline(always)]
        fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
            let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
            return t * t * (3.0 - 2.0 * t);
        }

        let mut v = vec![];

        for i in 0..source.len() {
            v.push(lerp_v(source[i], dest[i], smoothstep(0.0, 1.0, t)));
        }

        v
    }
}

