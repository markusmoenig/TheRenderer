pub mod therenderer;
pub mod thespace;
pub mod therect;
pub mod theshadertrait;
pub mod thebuffer;
pub mod theshapetrait;
pub mod thestate;
pub mod theevent;
pub mod theshared;

pub type TheColor = [f32; 4];

use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "embedded/"]
#[exclude = ".txt"]
#[exclude = ".DS_Store"]
pub struct Embedded;

pub mod prelude {
    pub use crate::TheColor;

    pub use crate::therenderer::TheRenderer;
    pub use crate::thespace::{TheSpace, TheSpaceCoordinate, TheSpaceCoordinate::*};
    pub use crate::therect::TheRect;
    pub use crate::thebuffer::TheColorBuffer;
    pub use crate::thestate::{*, TheProperty::*, TheState::*};
    pub use crate::theevent::TheEventUpdate;
    pub use crate::theshared::TheShared;

    pub use crate::theshapetrait::{TheShapeTrait, TheShapes, TheShapes::*};
    pub use crate::theshapetrait::thedisc::TheDisc;

    pub use crate::theshadertrait::TheShaderTrait;
    pub use crate::theshadertrait::thecolorshader::TheColorShader;

    pub use maths_rs::prelude::*;
    pub use rustc_hash::FxHashMap;
}