
//use crate::prelude::*;

use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};

/// A color buffer holding an array of f32 pixels.
#[derive(PartialEq, Debug, Clone)]
pub struct TheColorBuffer {

    pub width               : usize,
    pub height              : usize,

    pub pixels              : Vec<f32>,

    pub frames              : usize,
}

impl TheColorBuffer {

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,

            pixels      : vec![0.0; width * height * 4],
            frames      : 0,
        }
    }

    #[inline(always)]
    pub fn at(&self, x: usize, y: usize) -> [f32;4] {
        let i = y * self.width * 4 + x * 4;
        [self.pixels[i], self.pixels[i + 1], self.pixels[i + 2], self.pixels[i + 3]]
    }

    /// Convert the frame to an u8 vec
    pub fn to_u8_vec(&self) -> Vec<u8> {

        let source = &self.pixels[..];
        let mut out : Vec<u8> = vec![0; self.width * self.height * 4];

        for y in 0..self.height {
            for x in 0..self.width {
                let d = x * 4 + y * self.width * 4;
                let c = [(source[d] * 255.0) as u8, (source[d+1] * 255.0) as u8,  (source[d+2] * 255.0) as u8,  (source[d+3] * 255.0) as u8];
                // let c = [(source[d].powf(0.4545) * 255.0) as u8, (source[d+1].powf(0.4545) * 255.0) as u8, (source[d+2].powf(0.4545) * 255.0) as u8, (source[d+3] * 255.0) as u8];
                out[d..d + 4].copy_from_slice(&c);
            }
        }
        out
    }

    /// Convert the pixel buffer to an Vec<u8>.
    pub fn convert_to_u8(&self, frame: &mut [u8]) {
        for y in 0..self.height {
            for x in 0..self.width {
                let o = x * 4 + y * self.width * 4;
                // let c = [(self.pixels[o].powf(0.4545) * 255.0) as u8, (self.pixels[o+1].powf(0.4545) * 255.0) as u8, (self.pixels[o+2].powf(0.4545) * 255.0) as u8, (self.pixels[o+3] * 255.0) as u8];
                let c = [(self.pixels[o]* 255.0) as u8, (self.pixels[o+1] * 255.0) as u8, (self.pixels[o+2]* 255.0) as u8, (self.pixels[o+3] * 255.0) as u8];
                frame[o..o + 4].copy_from_slice(&c);
            }
        }
    }

    /// Convert the pixel buffer multi-threaded to an Vec<u8>.
    pub fn convert_to_u8_mt(&self, frame: &mut [u8]) {

        let width = self.width;
        let height: usize = self.height;

        frame
            .par_rchunks_exact_mut(width * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {
                    let i = j * width + i;

                    let x = i % width;
                    let y = height - i / width - 1;

                    let o = x * 4 + y * width * 4;
                    let c = [(self.pixels[o] * 255.0) as u8, (self.pixels[o+1] * 255.0) as u8, (self.pixels[o+2] * 255.0) as u8, (self.pixels[o+3] * 255.0) as u8];
                    pixel.copy_from_slice(&c);
                }
            });
    }
}