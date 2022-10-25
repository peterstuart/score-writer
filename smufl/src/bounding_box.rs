use serde::Deserialize;

use crate::Coord;

/// The glyph bounding box is defined as the smallest rectangle that encloses
/// every part of the glyph’s path.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct BoundingBox {
    #[serde(rename = "bBoxNE")]
    pub ne: Coord,

    #[serde(rename = "bBoxSW")]
    pub sw: Coord,
}
