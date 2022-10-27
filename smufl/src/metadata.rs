use std::io::Read;

use itertools::Itertools;
use serde::Deserialize;
use tracing::{debug, instrument};

use crate::{EngravingDefaults, GlyphAdvanceWidths, GlyphAnchors, GlyphBoundingBoxes};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub font_name: String,

    pub engraving_defaults: Option<EngravingDefaults>,

    #[serde(default, rename = "glyphAdvanceWidths")]
    pub advance_widths: GlyphAdvanceWidths,

    #[serde(default, rename = "glyphsWithAnchors")]
    pub anchors: GlyphAnchors,

    #[serde(default, rename = "glyphBBoxes")]
    pub boxes: GlyphBoundingBoxes,
}

impl Metadata {
    #[instrument(skip(reader), err(Debug))]
    pub fn from_reader(reader: impl Read) -> Result<Self, serde_json::Error> {
        let metadata: Self = serde_json::from_reader(reader)?;

        metadata.log_unknowns();

        Ok(metadata)
    }

    fn log_unknowns(&self) {
        let unknowns = self
            .advance_widths
            .unknown_glyphs()
            .chain(self.anchors.unknown_glyphs())
            .chain(self.boxes.unknown_glyphs())
            .unique()
            .sorted()
            .collect::<Vec<_>>();

        if unknowns.is_empty() {
            debug!("No unknowns glyphs found");
        } else {
            debug!(?unknowns, "Unknown glyphs found");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn from_json_bravura_metadata() -> anyhow::Result<()> {
        let file = File::open("../submodules/bravura/redist/bravura_metadata.json")?;
        let reader = BufReader::new(file);
        let metadata = Metadata::from_reader(reader);

        assert!(metadata.is_ok());

        Ok(())
    }
}
