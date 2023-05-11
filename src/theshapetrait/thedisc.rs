use crate::prelude::*;

pub struct TheDisc {
    id                              : u32,
    states                          : FxHashMap<TheState, TheStateProperties>,
    current_state                   : TheState,
    transition                      : Option<TheStateTransition>,

    shader                          : Box<dyn TheShaderTrait>,
}

impl TheShapeTrait for TheDisc {
    fn new(id: u32) -> Self {

        let mut normal = TheStateProperties::new(TheState::Normal);
        normal.set(TheProperty::X, vec![0.0]);
        normal.set(TheProperty::Y, vec![0.0]);
        normal.set(TheProperty::Radius, vec![100.0]);
        normal.set(TheProperty::Color, vec![0.0, 0.0, 0.0, 1.0]);
        //normal.set(P::B, vec![0.0]);
        //normal.set(P::BC, vec![0.0, 0.0, 0.0, 1.0]);

        let mut states = FxHashMap::default();
        states.insert(TheState::Normal, normal);

        Self {
            id,
            states,
            current_state           : TheState::Normal,
            transition              : None,

            shader                  : Box::new(TheColorShader::new()),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> String {
        "Disc".to_string()
    }

    // fn init(&mut self) {
    //     let r = self.get(P::R, State::Normal, &0).unwrap()[0];
    //     self.set(P::W, vec![r * 2.0], State::Normal);
    //     self.set(P::H, vec![r * 2.0], State::Normal);

    //     if let Some(state_props) = self.states.get_mut(&State::Normal) {
    //         state_props.gen_mask();
    //     }
    // }

    // fn test_mask(&self, key: P) -> bool {
    //     if let Some(state_props) = self.states.get(&State::Normal) {
    //         state_props.test_mask(key)
    //     } else {
    //         false
    //     }
    // }

    // fn layout(&mut self, rect: &Rect, _time: &u128) {
    //     self.set(P::X, vec![(rect.x + rect.width / 2) as f32], State::Normal);
    //     self.set(P::Y, vec![(rect.y + rect.height/ 2) as f32], State::Normal);
    //     //let w = (rect.width as f32 / 2.0).min(rect.height as f32 / 2.0);
    //     //self.set(P::R, vec![w], State::Normal);
    //     //self.set(P::H, vec![rect.height as f32], State::Normal);
    // }

    // fn set_state(&mut self, state: State, update: &mut EventUpdate) {
    //     let changed = state != self.current_state;

    //     if changed  {
    //         update.single = true;
    //         let source_props = self.states.get(&self.current_state);
    //         let dest_props = self.states.get(&state);
    //         if source_props.is_some() && dest_props.is_some() {
    //             let mut transition = StateTransition::new(source_props.unwrap().clone(), dest_props.unwrap().clone(), update.time);
    //             transition.precalc(vec![P::X, P::Y, P::R, P::C, P::B, P::BC]);
    //             self.transition = Some(transition);
    //             update.integrate_until(update.time + dest_props.unwrap().get_duration() as u128);
    //         } else {
    //             self.transition = None;
    //         }

    //         self.current_state = state;
    //     }
    // }

    // fn supported_properties(&self) -> FxHashMap<String, F> {
    //     let mut map = FxHashMap::default();
    //     map.insert("X".to_string(), 1.0);
    //     map.insert("Y".to_string(), 1.0);
    //     map.insert("C".to_string(), 4.0);
    //     map.insert("R".to_string(), 1.0);
    //     map.insert("DUR".to_string(), 1.0);
    //     map.insert("B".to_string(), 1.0);
    //     map.insert("BC".to_string(), 4.0);

    //     map
    // }

    // fn cleanup(&mut self, time: &u128) {
    //     if let Some(transition) = &self.transition {
    //         if transition.is_finished(time) {
    //             self.transition = None;
    //         }
    //     }
    // }

    /// Get the given property from the state
    #[inline(always)]
    fn get(&self, key: TheProperty, state: TheState, time: &u128) -> Option<&Vec<f32>> {
        //println!("Get {} {:?} {:?}", name, state, fallback);

        if let Some(transition) = &self.transition {
            return transition.get(key, time);
        } else
        if let Some(state_props) = self.states.get(&state) {
            if let Some(v) = state_props.get(key) {
                return Some(v);
            } else {
                if let Some(state_props) = self.states.get(&TheState::Normal) {
                    if let Some(v) = state_props.get(key) {
                        return Some(v);
                    }
                }
            }
        } else
        if let Some(state_props) = self.states.get(&TheState::Normal) {
            if let Some(v) = state_props.get(key) {
                return Some(v);
            }
        }
        None
    }

    fn get_current(&self, key: TheProperty, time: &u128) -> Option<&Vec<f32>> {
        self.get(key, self.current_state, time)
    }

    /// Set the given property for the state
    fn set(&mut self, key: TheProperty, value: Vec<f32>, state: TheState) {
        //println!("Set {:?} {:?} {:?}", key, value, state);
        if let Some(state_props) = self.states.get_mut(&state) {
            state_props.set(key, value);
        } else {
            let mut state_props = TheStateProperties::new(state);
            state_props.set(key, value);
            self.states.insert(state, state_props);
        }
    }

    #[inline(always)]
    fn distance(&self, pos: &Vec2f, rect: &mut Vec4f, time: &u128) -> f32 {
        let r = self.get(TheProperty::Radius, self.current_state, time).unwrap();
        let x = self.get(TheProperty::X, self.current_state, time).unwrap();
        let y = self.get(TheProperty::Y, self.current_state, time).unwrap();

        rect.x = x[0] - r[0];
        rect.y = y[0] - r[0];
        rect.z = 2.0 * r[0];
        rect.w = rect.z;

        length(vec2f(pos.x - x[0], pos.y - y[0])) - r[0]
    }

    fn get_shader(&self) -> &Box<dyn TheShaderTrait> {
        &self.shader
    }
}