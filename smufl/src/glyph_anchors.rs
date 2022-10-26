use crate::{Anchors, GlyphData};

/// Anchor data for glyphs.
///
/// See <https://w3c.github.io/smufl/latest/specification/glyphswithanchors.html>.
pub type GlyphAnchors = GlyphData<Anchors>;
