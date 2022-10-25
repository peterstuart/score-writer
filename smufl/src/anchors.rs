use serde::Deserialize;

use crate::Coord;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Anchors {
    /// The exact position at which the bottom right-hand (south-east) corner of
    /// an angled upward-pointing stem connecting the right-hand side of a
    /// notehead to a vertical stem to its left should start, relative to the
    /// glyph origin, expressed as Cartesian coordinates in staff spaces.
    #[serde(rename = "splitStemUpSE")]
    pub split_stem_up_se: Option<Coord>,
    /// The exact position at which the bottom left-hand (south-west) corner of
    /// an angled upward-pointing stem connecting the left-hand side of a
    /// notehead to a vertical stem to its right should start, relative to the
    /// glyph origin, expressed as Cartesian coordinates in staff spaces.
    #[serde(rename = "splitStemUpSW")]
    pub split_stem_up_sw: Option<Coord>,
    /// The exact position at which the top right-hand (north-east) corner of an
    /// angled downward-pointing stem connecting the right-hand side of a
    /// notehead to a vertical stem to its left should start, relative to the
    /// glyph origin, expressed as Cartesian coordinates in staff spaces.
    #[serde(rename = "splitStemDownNE")]
    pub split_stem_down_ne: Option<Coord>,
    /// The exact position at which the top left-hand (north-west) corner of an
    /// angled downward-pointing stem connecting the left-hand side of a
    /// notehead to a vertical stem to its right should start, relative to the
    /// glyph origin, expressed as Cartesian coordinates in staff spaces.
    #[serde(rename = "splitStemDownNW")]
    pub split_stem_down_nw: Option<Coord>,
    /// The exact position at which the bottom right-hand (south-east) corner of
    /// an upward-pointing stem rectangle should start, relative to the glyph
    /// origin, expressed as Cartesian coordinates in staff spaces.
    #[serde(rename = "stemUpSE")]
    pub stem_up_se: Option<Coord>,
    /// The exact position at which the top left-hand (north-west) corner of a
    /// downward-pointing stem rectangle should start, relative to the glyph
    /// origin, expressed as Cartesian coordinates in staff spaces.
    #[serde(rename = "stemDownNW")]
    pub stem_down_nw: Option<Coord>,
    /// The amount by which an up-stem should be lengthened from its nominal
    /// unmodified length in order to ensure a good connection with a flag, in
    /// spaces.1
    #[serde(rename = "stemUpNW")]
    pub stem_up_nw: Option<Coord>,
    /// The amount by which a down-stem should be lengthened from its nominal
    /// unmodified length in order to ensure a good connection with a flag, in
    /// spaces.
    #[serde(rename = "stemDownSW")]
    pub stem_down_sw: Option<Coord>,
    /// The width in staff spaces of a given glyph that should be used for e.g.
    /// positioning leger lines correctly.2
    pub nominal_width: Option<Coord>,
    /// The position in staff spaces that should be used to position numerals
    /// relative to clefs with ligated numbers where those numbers hang from the
    /// bottom of the clef, corresponding horizontally to the center of the
    /// numeral’s bounding box.
    pub numeral_top: Option<Coord>,
    /// The position in staff spaces that should be used to position numerals
    /// relative to clefs with ligatured numbers where those numbers sit on the
    /// baseline or at the north-east corner of the G clef, corresponding
    /// horizontally to the center of the numeral’s bounding box.
    pub numeral_bottom: Option<Coord>,
    /// The Cartesian coordinates in staff spaces of the bottom left corner of a
    /// nominal rectangle that intersects the top right corner of the glyph’s
    /// bounding box. This rectangle, together with those in the other four
    /// corners of the glyph’s bounding box, can be cut out to produce a more
    /// detailed bounding box (of abutting rectangles), useful for kerning or
    /// interlocking symbols such as accidentals.
    #[serde(rename = "cutOutNE")]
    pub cut_out_ne: Option<Coord>,
    /// The Cartesian coordinates in staff spaces of the top left corner of a
    /// nominal rectangle that intersects the bottom right corner of the glyph’s
    /// bounding box.
    #[serde(rename = "cutOutSE")]
    pub cut_out_se: Option<Coord>,
    /// The Cartesian coordinates in staff spaces of the top right corner of a
    /// nominal rectangle that intersects the bottom left corner of the glyph’s
    /// bounding box.
    #[serde(rename = "cutOutSW")]
    pub cut_out_sw: Option<Coord>,
    /// The Cartesian coordinates in staff spaces of the bottom right corner of
    /// a nominal rectangle that intersects the top left corner of the glyph’s
    /// bounding box.
    #[serde(rename = "cutOutNW")]
    pub cut_out_nw: Option<Coord>,
    /// The Cartesian coordinates in staff spaces of the position at which the
    /// glyph graceNoteSlashStemUp should be positioned relative to the stem-up
    /// flag of an unbeamed grace note; alternatively, the bottom left corner of
    /// a diagonal line drawn instead of using the above glyph.
    #[serde(rename = "graceNoteSlashSW")]
    pub grace_note_slash_sw: Option<Coord>,
    /// The Cartesian coordinates in staff spaces of the top right corner of a
    /// diagonal line drawn instead of using the glyph graceNoteSlashStemUp for
    /// a stem-up flag of an unbeamed grace note.
    #[serde(rename = "graceNoteSlashNE")]
    pub grace_note_slash_ne: Option<Coord>,
    /// The Cartesian coordinates in staff spaces of the position at which the
    /// glyph graceNoteSlashStemDown should be positioned relative to the
    /// stem-down flag of an unbeamed grace note; alternatively, the top left
    /// corner of a diagonal line drawn instead of using the above glyph.
    #[serde(rename = "graceNoteSlashNW")]
    pub grace_note_slash_nw: Option<Coord>,
    /// The Cartesian coordinates in staff spaces of the bottom right corner of
    /// a diagonal line drawn instead of using the glyph graceNoteSlashStemDown
    /// for a stem-down flag of an unbeamed grace note.
    #[serde(rename = "graceNoteSlashSE")]
    pub grace_note_slash_se: Option<Coord>,
    /// The Cartesian coordinates in staff spaces of the horizontal position at
    /// which a glyph repeats, i.e. the position at which the same glyph or
    /// another of the same group should be positioned to ensure correct
    /// tessellation. This is used for e.g. multi-segment lines and the
    /// component glyphs that make up trills and mordents.
    pub repeat_offset: Option<Coord>,
    /// The Cartesian coordinates in staff spaces of the left-hand edge of a
    /// notehead with a non-zero left-hand side bearing (e.g. a double whole, or
    /// breve, notehead with two vertical lines at each side), to assist in the
    /// correct horizontal alignment of these noteheads with other noteheads
    /// with zero-width left-side bearings.
    pub notehead_origin: Option<Coord>,
    /// The Cartesian coordinates in staff spaces of the optical center of the
    /// glyph, to assist in the correct horizontal alignment of the glyph
    /// relative to a notehead or stem. Currently recommended for use with
    /// glyphs in the *Dynamics* range.
    pub optical_center: Option<Coord>,
}
