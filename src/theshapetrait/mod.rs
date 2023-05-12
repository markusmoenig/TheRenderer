pub mod thedisc;

use crate::prelude::*;

pub enum TheShapes {
    Disc,
}

#[allow(unused)]
pub trait TheShapeTrait : Sync + Send {
    fn new(id: u32) -> Self where Self: Sized;

    fn id(&self) -> u32;
    fn name(&self) -> String {"".to_string()}

    // fn init(&mut self) {}

    fn set_state(&mut self, state: TheState, update: &mut TheEventUpdate);

    fn get(&self, key: TheProperty, state: TheState, time: &u128) -> Option<&Vec<f32>>;
    fn get_current(&self, key: TheProperty, time: &u128) -> Option<&Vec<f32>>;

    fn set(&mut self, key: TheProperty, value: Vec<f32>, state: TheState);

    // fn test_mask(&self, key: P) -> bool { false }

    // fn supported_properties(&self) -> FxHashMap<String, F>;

    fn distance(&self, pos: &Vec2f, rect: &mut Vec4f, time: &u128) -> f32 {
        f32::MAX
    }

    fn get_shader(&self) -> &Box<dyn TheShaderTrait>;

    // #[inline(always)]
    // fn colorize(&self, pos_x: &F, pos_y: &F, time: &u128, color: &mut [F; 4]) {
    //     let dist = self.distance(pos_x, pos_y, time);

    //     if dist < 1.0 {
    //         let mask = fill_mask(dist);
    //         let c =self.get_current(P::C, time).unwrap();

    //         let b_size =self.get_current(P::B, time).unwrap();
    //         let bc =self.get_current(P::BC, time).unwrap();

    //         let mut c = mix_color(color, &c, mask * c[3]);

    //         let b = border_mask(dist, b_size[0]);
    //         c = mix_color(&c, &bc, b);

    //         color.copy_from_slice(&c);
    //     }
    // }

    // fn cleanup(&mut self, time: &u128);

    // // layout support

    // fn add_child(&mut self, shape: Box<dyn Shape>) -> bool{ false }
    // fn children(&mut self) -> Option<&mut Vec<Box<dyn Shape>>> { None }

    // fn layout(&mut self, rect: &Rect, time: &u128) {}

}