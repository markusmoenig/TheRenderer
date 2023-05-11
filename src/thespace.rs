use crate::prelude::*;

use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};

pub struct TheSpace {
    pub rect                : TheRect,
}

impl TheSpace {
    pub fn new() -> Self {
        Self {
            rect            : TheRect::empty(),
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

                    let color = [uv.x, uv.y, 0.0, 1.0];

                    pixel.copy_from_slice(&color);
                }
        });
    }
}