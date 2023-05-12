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
}

/// The main rendering class.
impl TheRenderer {
    pub fn new() -> Self {
        Self {
            world_space         : TheSpace::new(0),
            buffer              : TheColorBuffer::new(100, 100),

            space_id_counter    : 1,
        }
    }

    /// Draws the space.
    pub fn draw(&mut self, pixels: &mut [u8], width: usize, height: usize) {

        // Resize the color buffer if necessary
        if self.buffer.width != width || self.buffer.height != height {
            self.buffer.width = width;
            self.buffer.height = height;
            self.buffer.pixels = vec![0.0; width * height * 4];
        }

        // Always keep the world space size fixed to the drawing size
        self.world_space.rect = TheRect::new(0, 0, width, height);

        let time = self.get_time();
        self.world_space.draw_mt(&mut self.buffer, &time);
        let end_time = self.get_time();
        println!("Time: {}", end_time - time);

        self.buffer.convert_to_u8_mt(pixels);
    }

    /// Returns the space for the given id
    pub fn get_space_mut(&mut self, space_id: u32) -> Option<&mut TheSpace> {
        if space_id == 0 {
            return Some(&mut self.world_space);
        }
        None
    }

    /// Returns true if the window needs an update
    pub fn needs_update(&mut self) -> bool {
        self.world_space.needs_update()
    }

    /// Gets the current time in milliseconds
    fn get_time(&self) -> u128 {
        let stop = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
            stop.as_millis()
    }
}