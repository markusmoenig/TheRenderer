use crate::prelude::*;

pub struct TheColorShader {
}

impl TheShaderTrait for TheColorShader {
    fn new() -> Self {
        Self {

        }
    }

    fn colorize(&self, uv: &Vec2f, size: &Vec2f, distance: &f32) -> TheColor {
        let mask = 1.0 - smoothstep(-2.0, 0.0, *distance);
        [uv.x, uv.y, 0.0, mask]
    }
}