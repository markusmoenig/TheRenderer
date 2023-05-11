use crate::prelude::*;

pub mod thecolorshader;

#[allow(unused)]
pub trait TheShaderTrait : Sync + Send  {

    fn new() -> Self where Self: Sized;

    /// Returns a list of the required properties for this shader. The properties
    /// will be passed to colorize.
    fn get_properties(&self) -> &Vec<TheProperty>;

    /// Return the color for the given shape distance.
    fn colorize(&self, uv: &Vec2f, size: &Vec2f, distance: &f32, props: &Vec<&Vec<f32>>) -> TheColor {
        [1.0, 1.0, 1.0, 1.0]
    }
}