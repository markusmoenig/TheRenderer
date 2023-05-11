pub mod therenderer;
pub mod thespace;
pub mod therect;
pub mod theshadertrait;
pub mod thebuffer;

pub type TheColor = [f32; 4];

pub mod prelude {
    pub use crate::TheColor;

    pub use crate::therenderer::TheRenderer;
    pub use crate::thespace::TheSpace;
    pub use crate::therect::TheRect;
    pub use crate::thebuffer::TheColorBuffer;

    pub use maths_rs::prelude::*;
}