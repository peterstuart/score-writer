use std::collections::HashMap;

use serde::Deserialize;

use crate::{glyph_or_unknown::GlyphOrUnknown, Anchors, Glyph};

/// Anchor data for glyphs.
///
/// See <https://w3c.github.io/smufl/latest/specification/glyphswithanchors.html>.
#[derive(Debug, Default, Deserialize)]
#[serde(transparent)]
pub struct GlyphAnchors {
    anchors: HashMap<GlyphOrUnknown, Anchors>,
}

impl GlyphAnchors {
    pub fn get(&self, glyph: Glyph) -> Option<Anchors> {
        self.anchors.get(&GlyphOrUnknown::Glyph(glyph)).copied()
    }
}
