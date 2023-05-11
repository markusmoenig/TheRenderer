use crate::prelude::*;

pub mod thecolorshader;

#[allow(unused)]
pub trait TheShaderTrait : Sync + Send  {

    fn new() -> Self where Self: Sized;

    fn colorize(&self, uv: &Vec2f, size: &Vec2f, distance: &f32) -> TheColor {
        [1.0, 1.0, 1.0, 1.0]
    }
}