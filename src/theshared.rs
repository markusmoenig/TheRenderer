pub use crate::prelude::*;
use fontdue::Font;
pub use crate::Embedded;

pub struct TheShared {
    pub font                : Option<Font>,

}

impl TheShared {
    pub fn new() -> Self {

        let mut font : Option<Font> = None;

        for file in Embedded::iter() {
            let name = file.as_ref();
            if name.starts_with("fonts/") {
                if let Some(font_bytes) = Embedded::get(name) {
                    if let Some(f) = Font::from_bytes(font_bytes.data, fontdue::FontSettings::default()).ok() {
                        font = Some(f);
                    }
                }
            }
        }

        Self {
            font
        }
    }
}