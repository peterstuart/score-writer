use serde::Deserialize;

use crate::StaffSpaces;

/// Recommended defaults for line widths etc.
///
/// See <https://w3c.github.io/smufl/latest/specification/engravingdefaults.html>.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngravingDefaults {
    /// An array containing the text font family (or families, in descending
    /// order of preference) that are ideally paired with this music font; this
    /// list may also use the generic font family values defined in CSS, i.e.
    /// serif, sans-serif, cursive, fantasy, and monospace. Generic font family
    /// names should be listed after specific font families.
    pub text_font_family: Vec<String>,
    /// The thickness of each staff line
    pub staff_line_thickness: StaffSpaces,
    /// The thickness of a stem
    pub stem_thickness: StaffSpaces,
    /// The thickness of a beam
    pub beam_thickness: StaffSpaces,
    /// The distance between the inner edge of the primary and outer edge of
    /// subsequent secondary beams
    pub beam_spacing: StaffSpaces,
    /// The thickness of a leger line (normally somewhat thicker than a staff
    /// line)
    pub leger_line_thickness: StaffSpaces,
    /// The amount by which a leger line should extend either side of a
    /// notehead, scaled proportionally with the notehead's size, e.g. when
    /// scaled down as a grace note
    pub leger_line_extension: StaffSpaces,
    /// The thickness of the end of a slur
    pub slur_endpoint_thickness: StaffSpaces,
    /// The thickness of the mid-point of a slur (i.e. its thickest point)
    pub slur_midpoint_thickness: StaffSpaces,
    /// The thickness of the end of a tie
    pub tie_endpoint_thickness: StaffSpaces,
    /// The thickness of the mid-point of a tie
    pub tie_midpoint_thickness: StaffSpaces,
    /// The thickness of a thin barline, e.g. a normal barline, or each of the
    /// lines of a double barline
    pub thin_barline_thickness: StaffSpaces,
    /// The thickness of a thick barline, e.g. in a final barline or a repeat
    /// barline
    pub thick_barline_thickness: StaffSpaces,
    /// The thickness of a dashed barline
    pub dashed_barline_thickness: StaffSpaces,
    /// The length of the dashes to be used in a dashed barline
    pub dashed_barline_dash_length: StaffSpaces,
    /// The length of the gap between dashes in a dashed barline
    pub dashed_barline_gap_length: StaffSpaces,
    /// The default distance between multiple thin barlines when locked
    /// together, e.g. between two thin barlines making a double barline,
    /// measured from the right-hand edge of the left barline to the left-hand
    /// edge of the right barline.
    pub barline_separation: StaffSpaces,
    /// The default distance between a pair of thin and thick barlines when
    /// locked together, e.g. between the thin and thick barlines making a final
    /// barline, or between the thick and thin barlines making a start repeat
    /// barline.
    // TODO: This is missing in Bravura
    // thin_thick_barline_separation: StaffSpaces,
    /// The default horizontal distance between the dots and the inner barline
    /// of a repeat barline, measured from the edge of the dots to the edge of
    /// the barline.
    pub repeat_barline_dot_separation: StaffSpaces,
    /// The thickness of the vertical line of a bracket grouping staves together
    pub bracket_thickness: StaffSpaces,
    /// The thickness of the vertical line of a sub-bracket grouping staves
    /// belonging to the same instrument together
    pub sub_bracket_thickness: StaffSpaces,
    /// The thickness of a crescendo/diminuendo hairpin
    pub hairpin_thickness: StaffSpaces,
    /// The thickness of the dashed line used for an octave line
    pub octave_line_thickness: StaffSpaces,
    /// The thickness of the line used for piano pedaling
    pub pedal_line_thickness: StaffSpaces,
    /// The thickness of the brackets drawn to indicate repeat endings
    pub repeat_ending_line_thickness: StaffSpaces,
    /// The thickness of the line used for the shaft of an arrow
    pub arrow_shaft_thickness: StaffSpaces,
    /// The thickness of the lyric extension line to indicate a melisma in vocal
    /// music
    pub lyric_line_thickness: StaffSpaces,
    /// The thickness of a box drawn around text instructions (e.g. rehearsal
    /// marks)
    pub text_enclosure_thickness: StaffSpaces,
    /// The thickness of the brackets drawn either side of tuplet numbers
    pub tuplet_bracket_thickness: StaffSpaces,
    /// The thickness of the horizontal line drawn between two vertical lines,
    /// known as the H-bar, in a multi-bar rest
    pub h_bar_thickness: StaffSpaces,
}
