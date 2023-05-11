use crate::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TheRenderer {

    /// The world space, usuall used as the background space. It will get automatically resized
    /// to the current canvas dimensions.
    pub world_space             : TheSpace,

    /// The color buffer all spaces draw into.
    pub buffer                  : TheColorBuffer,

    /// The counter for space ids
    pub space_id_counter        : u32,

    /// The counter for shape ids
    pub shape_id_counter        : u32,
}

/// The main rendering class.
impl TheRenderer {
    pub fn new() -> Self {
        Self {
            world_space         : TheSpace::new(0),
            buffer              : TheColorBuffer::new(100, 100),

            space_id_counter    : 1,
            shape_id_counter    : 0,
        }
    }

    pub fn draw(&mut self, pixels: &mut [u8], width: usize, height: usize) {

        // Resize the color buffer if necessary
        if self.buffer.width != width || self.buffer.height != height {
            self.buffer.width = width;
            self.buffer.height = height;
            self.buffer.pixels = vec![0.0; width * height * 4];
        }

        // Always keep the world space size fixed to the drawing size
        self.world_space.rect = TheRect::new(0, 0, width, height);

        let start_time = self.get_time();
        self.world_space.draw_mt(&mut self.buffer);
        let end_time = self.get_time();
        println!("Time: {}", end_time - start_time);

        self.buffer.convert_to_u8_mt(pixels);
    }

    ///
    pub fn add_shape(&mut self, space_id: u32, theshape: TheShapes) -> u32 {

        let shape;
        match theshape {
            TheShapes::Disc => {
                shape = Box::new(TheDisc::new(self.shape_id_counter));
            }
        }

        if self.world_space.id == space_id {
            self.world_space.shapes.push(shape);
        }

        let last_id = self.shape_id_counter;
        self.shape_id_counter += 1;
        last_id
    }

    /// Gets the current time in milliseconds
    fn get_time(&self) -> u128 {
        let stop = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
            stop.as_millis()
    }
}