use crate::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TheRenderer {

    /// The world space, usuall used as the background space. It will get automatically resized
    /// to the current canvas dimensions.
    pub world_space             : TheSpace,

    /// The color buffer all spaces draw into.
    pub buffer                  : TheColorBuffer,
}

/// The main rendering class.
impl TheRenderer {
    pub fn new() -> Self {
        Self {
            world_space         : TheSpace::new(),
            buffer              : TheColorBuffer::new(100, 100)
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
        self.buffer.convert_to_u8_mt(pixels);
        println!("Time: {}", end_time - start_time);
    }

    /// Gets the current time in milliseconds
    fn get_time(&self) -> u128 {
        let stop = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
            stop.as_millis()
    }
}