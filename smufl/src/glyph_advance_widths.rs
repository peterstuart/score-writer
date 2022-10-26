use crate::{GlyphData, StaffSpaces};

/// Advance width of each glyph.
///
/// The advance width is defined as the width of a glyph as measured from its
/// origin to the origin of the next glyph on the line. In text fonts for many
/// languages, glyphs normally have positive left and right side-bearings, i.e.
/// space to either side of the glyph, so that a string of glyphs will show the
/// expected letter spacing. The advance width includes these side-bearing
/// values. If a glyph's path protrudes beyond the width defined for the glyph
/// in the font, however, these protrusions to the left or the right – which can
/// be termed negative side-bearings – are not included in the advance width.
///
/// In SMuFL fonts, glyphs typically have zero left and right side-bearings, and
/// some glyphs may have negative side-bearings. For example,
/// [`StemSulPonticello`](Glyph::StemSulPonticello) has a very narrow width, and
/// large negative side-bearings to accommodate the sul ponticello sign that is
/// centered on the stem.
///
/// See <https://w3c.github.io/smufl/latest/specification/glyphadvancewidths.html>.
pub type GlyphAdvanceWidths = GlyphData<StaffSpaces>;
