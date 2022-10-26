use std::collections::HashMap;

use serde::Deserialize;

use crate::{glyph_or_unknown::GlyphOrUnknown, Glyph};

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GlyphData<T> {
    data: HashMap<GlyphOrUnknown, T>,
}

impl<T> Default for GlyphData<T> {
    fn default() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
}

impl<T: Copy> GlyphData<T> {
    pub fn get(&self, glyph: Glyph) -> Option<T> {
        self.data.get(&GlyphOrUnknown::Glyph(glyph)).copied()
    }
}
