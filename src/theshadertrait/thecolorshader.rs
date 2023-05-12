use crate::prelude::*;

pub struct TheColorShader {
    pub properties              : Vec<TheProperty>,
}

impl TheShaderTrait for TheColorShader {
    fn new() -> Self {
        Self {
            properties          : vec!(Color),
        }
    }

    fn get_properties(&self) -> &Vec<TheProperty> {
        &self.properties
    }

    fn colorize(&self, _uv: &Vec2f, _size: &Vec2f, distance: &f32, props: &Vec<&Vec<f32>>) -> TheColor {
        let mask = 1.0 - smoothstep(-2.0, 0.0, *distance);
        //let mask = clamp(0.5 - *distance / 3.0, 0.0, 1.0);

        let color = props[0];
        [color[0], color[1], color[2], mask]
    }
}