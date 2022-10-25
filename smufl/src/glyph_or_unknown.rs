use serde::Deserialize;

use crate::Glyph;

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(untagged)]
pub enum GlyphOrUnknown {
    Glyph(Glyph),
    Unknown(String),
}
