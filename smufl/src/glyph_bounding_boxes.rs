use std::collections::HashMap;

use serde::Deserialize;

use crate::{glyph_or_unknown::GlyphOrUnknown, BoundingBox, Glyph};

#[derive(Debug, Default, Deserialize)]
#[serde(transparent)]
pub struct GlyphBoundingBoxes {
    bounding_boxes: HashMap<GlyphOrUnknown, BoundingBox>,
}

impl GlyphBoundingBoxes {
    pub fn get(&self, glyph: Glyph) -> Option<BoundingBox> {
        self.bounding_boxes
            .get(&GlyphOrUnknown::Glyph(glyph))
            .copied()
    }
}
