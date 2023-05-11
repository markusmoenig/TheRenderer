use crate::prelude::*;

use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};

/// The coordinate system for spaces
pub enum TheSpaceCoordinate {
    LeftTop,
    Center,
    LeftBottom,
}

use TheSpaceCoordinate::*;

pub struct TheSpace {
    pub id                      : u32,
    pub rect                    : TheRect,

    pub coord_system            : TheSpaceCoordinate,

    pub shapes                  : Vec<Box<dyn TheShapeTrait>>,

    /// The counter for shape ids
    pub shape_id_counter        : u32,
}

impl TheSpace {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            rect                : TheRect::empty(),

            coord_system        : LeftTop,

            shapes              : vec![],

            shape_id_counter    : 0,
        }
    }

    pub fn draw_mt(&mut self, buffer: &mut TheColorBuffer) {
        let width = buffer.width;
        let height = buffer.height as f32;

        buffer.pixels
            .par_rchunks_exact_mut(width * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {
                    let i = j * width + i;

                    let x = (i % width) as f32;
                    let y = (i / width) as f32;

                    let uv = vec2f(x / width as f32, y / height);

                    let mut color = [uv.x, uv.y, 0.0, 1.0];

                    for shape in &self.shapes {

                        let mut rect = Vec4f::zero();

                        let pos = match self.coord_system {
                            LeftTop => vec2f(x, height - y),
                            Center => vec2f(x - width as f32 / 2.0, y - height / 2.0),
                            LeftBottom => vec2f(x, y),
                        };

                        #[inline(always)]
                        pub fn mix_color(a: &TheColor, b: &TheColor, v: &f32) -> TheColor {
                            [
                                (1.0 - v) * a[0] + b[0] * v,
                                (1.0 - v) * a[1] + b[1] * v,
                                (1.0 - v) * a[2] + b[2] * v,
                                1.0
                            ]
                        }

                        let distance = shape.distance(&pos, &mut rect, &0);
                        if distance <= 0.0 {
                            let shader = shape.get_shader();

                            let uv = vec2f((pos.x - rect.x) / rect.z, 1.0 - (pos.y - rect.y) / rect.w);

                            let needed = shader.get_properties();
                            let mut properties : Vec<&Vec<f32>> = vec![];

                            for prop in needed {
                                if let Some(p) = shape.get_current(*prop, &0) {
                                    properties.push(p);
                                }
                            }

                            let c = shader.colorize(&uv, &vec2f(rect.z, rect.w), &distance, &properties);

                            color = mix_color(&color, &c, &c[3]);
                        }
                    }

                    pixel.copy_from_slice(&color);
                }
        });
    }

    /// Sets the coordinate system for the space.
    pub fn set_coord_system(&mut self, coord_system: TheSpaceCoordinate) {
        self.coord_system = coord_system;
    }

    /// Sets the coordinate system for the space.
    pub fn set_shape_property(&mut self, shape_id: u32, state: TheState, property: TheProperty, value: Vec<f32>) {
        for shape in &mut self.shapes {
            if shape.id() == shape_id {
                shape.set(property, value, state);
                break;
            }
        }
    }

    /// Adds a new shape to the space
    pub fn add_shape(&mut self, theshape: TheShapes) -> u32 {

        let shape;
        match theshape {
            TheShapes::Disc => {
                shape = Box::new(TheDisc::new(self.shape_id_counter));
            }
        }

        self.shapes.push(shape);

        let last_id = self.shape_id_counter;
        self.shape_id_counter += 1;
        last_id
    }
}