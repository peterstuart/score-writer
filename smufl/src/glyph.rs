use serde::Deserialize;

/// SMuFL glyphs
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum Glyph {
    /// 11 large diesis down, 3° down \[46 EDO\]
    #[serde(rename = "accSagittal11LargeDiesisDown")]
    AccSagittal11LargeDiesisDown,
    /// 11 large diesis up, (11L), (sharp less 11M), 3° up \[46 EDO\]
    #[serde(rename = "accSagittal11LargeDiesisUp")]
    AccSagittal11LargeDiesisUp,
    /// 11 medium diesis down, 1°\[17 31\] 2°46 down, 1/4-tone down
    #[serde(rename = "accSagittal11MediumDiesisDown")]
    AccSagittal11MediumDiesisDown,
    /// 11 medium diesis up, (11M), 1°\[17 31\] 2°46 up, 1/4-tone up
    #[serde(rename = "accSagittal11MediumDiesisUp")]
    AccSagittal11MediumDiesisUp,
    /// 11:19 large diesis down
    #[serde(rename = "accSagittal11v19LargeDiesisDown")]
    AccSagittal11V19LargeDiesisDown,
    /// 11:19 large diesis up, (11:19L, apotome less 11:19M)
    #[serde(rename = "accSagittal11v19LargeDiesisUp")]
    AccSagittal11V19LargeDiesisUp,
    /// 11:19 medium diesis down
    #[serde(rename = "accSagittal11v19MediumDiesisDown")]
    AccSagittal11V19MediumDiesisDown,
    /// 11:19 medium diesis up, (11:19M, 11M plus 19s)
    #[serde(rename = "accSagittal11v19MediumDiesisUp")]
    AccSagittal11V19MediumDiesisUp,
    /// 11:49 comma down
    #[serde(rename = "accSagittal11v49CommaDown")]
    AccSagittal11V49CommaDown,
    /// 11:49 comma up, (11:49C, 11M less 49C)
    #[serde(rename = "accSagittal11v49CommaUp")]
    AccSagittal11V49CommaUp,
    /// 143 comma down
    #[serde(rename = "accSagittal143CommaDown")]
    AccSagittal143CommaDown,
    /// 143 comma up, (143C, 13L less 11M)
    #[serde(rename = "accSagittal143CommaUp")]
    AccSagittal143CommaUp,
    /// 17 comma down
    #[serde(rename = "accSagittal17CommaDown")]
    AccSagittal17CommaDown,
    /// 17 comma up, (17C)
    #[serde(rename = "accSagittal17CommaUp")]
    AccSagittal17CommaUp,
    /// 17 kleisma down
    #[serde(rename = "accSagittal17KleismaDown")]
    AccSagittal17KleismaDown,
    /// 17 kleisma up, (17k)
    #[serde(rename = "accSagittal17KleismaUp")]
    AccSagittal17KleismaUp,
    /// 19 comma down
    #[serde(rename = "accSagittal19CommaDown")]
    AccSagittal19CommaDown,
    /// 19 comma up, (19C)
    #[serde(rename = "accSagittal19CommaUp")]
    AccSagittal19CommaUp,
    /// 19 schisma down
    #[serde(rename = "accSagittal19SchismaDown")]
    AccSagittal19SchismaDown,
    /// 19 schisma up, (19s)
    #[serde(rename = "accSagittal19SchismaUp")]
    AccSagittal19SchismaUp,
    /// 1 mina down, 1/(5⋅7⋅13)-schismina down, 0.42 cents down
    #[serde(rename = "accSagittal1MinaDown")]
    AccSagittal1MinaDown,
    /// 1 mina up, 1/(5⋅7⋅13)-schismina up, 0.42 cents up
    #[serde(rename = "accSagittal1MinaUp")]
    AccSagittal1MinaUp,
    /// 1 tina down, 7²⋅11⋅19/5-schismina down, 0.17 cents down
    #[serde(rename = "accSagittal1TinaDown")]
    AccSagittal1TinaDown,
    /// 1 tina up, 7²⋅11⋅19/5-schismina up, 0.17 cents up
    #[serde(rename = "accSagittal1TinaUp")]
    AccSagittal1TinaUp,
    /// 23 comma down, 2° down \[96 EDO\], 1/8-tone down
    #[serde(rename = "accSagittal23CommaDown")]
    AccSagittal23CommaDown,
    /// 23 comma up, (23C), 2° up \[96 EDO\], 1/8-tone up
    #[serde(rename = "accSagittal23CommaUp")]
    AccSagittal23CommaUp,
    /// 23 small diesis down
    #[serde(rename = "accSagittal23SmallDiesisDown")]
    AccSagittal23SmallDiesisDown,
    /// 23 small diesis up, (23S)
    #[serde(rename = "accSagittal23SmallDiesisUp")]
    AccSagittal23SmallDiesisUp,
    /// 25 small diesis down, 2° down \[53 EDO\]
    #[serde(rename = "accSagittal25SmallDiesisDown")]
    AccSagittal25SmallDiesisDown,
    /// 25 small diesis up, (25S, ~5:13S, ~37S, 5C plus 5C), 2° up \[53 EDO\]
    #[serde(rename = "accSagittal25SmallDiesisUp")]
    AccSagittal25SmallDiesisUp,
    /// 2 minas down, 65/77-schismina down, 0.83 cents down
    #[serde(rename = "accSagittal2MinasDown")]
    AccSagittal2MinasDown,
    /// 2 minas up, 65/77-schismina up, 0.83 cents up
    #[serde(rename = "accSagittal2MinasUp")]
    AccSagittal2MinasUp,
    /// 2 tinas down, 1/(7³⋅17)-schismina down, 0.30 cents down
    #[serde(rename = "accSagittal2TinasDown")]
    AccSagittal2TinasDown,
    /// 2 tinas up, 1/(7³⋅17)-schismina up, 0.30 cents up
    #[serde(rename = "accSagittal2TinasUp")]
    AccSagittal2TinasUp,
    /// 35 large diesis down, 2° down \[50 EDO\], 5/18-tone down
    #[serde(rename = "accSagittal35LargeDiesisDown")]
    AccSagittal35LargeDiesisDown,
    /// 35 large diesis up, (35L, ~13L, ~125L, sharp less 35M), 2°50 up
    #[serde(rename = "accSagittal35LargeDiesisUp")]
    AccSagittal35LargeDiesisUp,
    /// 35 medium diesis down, 1°\[50\] 2°\[27\] down, 2/9-tone down
    #[serde(rename = "accSagittal35MediumDiesisDown")]
    AccSagittal35MediumDiesisDown,
    /// 35 medium diesis up, (35M, ~13M, ~125M, 5C plus 7C), 2/9-tone up
    #[serde(rename = "accSagittal35MediumDiesisUp")]
    AccSagittal35MediumDiesisUp,
    /// 3 tinas down, 1 mina down, 1/(5⋅7⋅13)-schismina down, 0.42 cents down
    #[serde(rename = "accSagittal3TinasDown")]
    AccSagittal3TinasDown,
    /// 3 tinas up, 1 mina up, 1/(5⋅7⋅13)-schismina up, 0.42 cents up
    #[serde(rename = "accSagittal3TinasUp")]
    AccSagittal3TinasUp,
    /// 49 large diesis down
    #[serde(rename = "accSagittal49LargeDiesisDown")]
    AccSagittal49LargeDiesisDown,
    /// 49 large diesis up, (49L, ~31L, apotome less 49M)
    #[serde(rename = "accSagittal49LargeDiesisUp")]
    AccSagittal49LargeDiesisUp,
    /// 49 medium diesis down
    #[serde(rename = "accSagittal49MediumDiesisDown")]
    AccSagittal49MediumDiesisDown,
    /// 49 medium diesis up, (49M, ~31M, 7C plus 7C)
    #[serde(rename = "accSagittal49MediumDiesisUp")]
    AccSagittal49MediumDiesisUp,
    /// 49 small diesis down
    #[serde(rename = "accSagittal49SmallDiesisDown")]
    AccSagittal49SmallDiesisDown,
    /// 49 small diesis up, (49S, ~31S)
    #[serde(rename = "accSagittal49SmallDiesisUp")]
    AccSagittal49SmallDiesisUp,
    /// 4 tinas down, 5²⋅11²/7-schismina down, 0.57 cents down
    #[serde(rename = "accSagittal4TinasDown")]
    AccSagittal4TinasDown,
    /// 4 tinas up, 5²⋅11²/7-schismina up, 0.57 cents up
    #[serde(rename = "accSagittal4TinasUp")]
    AccSagittal4TinasUp,
    /// 55 comma down, 3° down \[96 EDO\], 3/16-tone down
    #[serde(rename = "accSagittal55CommaDown")]
    AccSagittal55CommaDown,
    /// 55 comma up, (55C, 11M less 5C), 3°up \[96 EDO\], 3/16-tone up
    #[serde(rename = "accSagittal55CommaUp")]
    AccSagittal55CommaUp,
    /// 5 comma down, 1° down \[22 27 29 34 41 46 53 96 EDOs\], 1/12-tone down
    #[serde(rename = "accSagittal5CommaDown")]
    AccSagittal5CommaDown,
    /// 5 comma up, (5C), 1° up \[22 27 29 34 41 46 53 96 EDOs\], 1/12-tone up
    #[serde(rename = "accSagittal5CommaUp")]
    AccSagittal5CommaUp,
    /// 5 tinas down, 7⁴/25-schismina down, 0.72 cents down
    #[serde(rename = "accSagittal5TinasDown")]
    AccSagittal5TinasDown,
    /// 5 tinas up, 7⁴/25-schismina up, 0.72 cents up
    #[serde(rename = "accSagittal5TinasUp")]
    AccSagittal5TinasUp,
    /// 5:11 small diesis down
    #[serde(rename = "accSagittal5v11SmallDiesisDown")]
    AccSagittal5V11SmallDiesisDown,
    /// 5:11 small diesis up, (5:11S, ~7:13S, ~11:17S, 5:7k plus 7:11C)
    #[serde(rename = "accSagittal5v11SmallDiesisUp")]
    AccSagittal5V11SmallDiesisUp,
    /// 5:13 large diesis down
    #[serde(rename = "accSagittal5v13LargeDiesisDown")]
    AccSagittal5V13LargeDiesisDown,
    /// 5:13 large diesis up, (5:13L, ~37L, apotome less 5:13M)
    #[serde(rename = "accSagittal5v13LargeDiesisUp")]
    AccSagittal5V13LargeDiesisUp,
    /// 5:13 medium diesis down
    #[serde(rename = "accSagittal5v13MediumDiesisDown")]
    AccSagittal5V13MediumDiesisDown,
    /// 5:13 medium diesis up, (5:13M, ~37M, 5C plus 13C)
    #[serde(rename = "accSagittal5v13MediumDiesisUp")]
    AccSagittal5V13MediumDiesisUp,
    /// 5:19 comma down, 1/20-tone down
    #[serde(rename = "accSagittal5v19CommaDown")]
    AccSagittal5V19CommaDown,
    /// 5:19 comma up, (5:19C, 5C plus 19s), 1/20-tone up
    #[serde(rename = "accSagittal5v19CommaUp")]
    AccSagittal5V19CommaUp,
    /// 5:23 small diesis down, 2° down \[60 EDO\], 1/5-tone down
    #[serde(rename = "accSagittal5v23SmallDiesisDown")]
    AccSagittal5V23SmallDiesisDown,
    /// 5:23 small diesis up, (5:23S, 5C plus 23C), 2° up \[60 EDO\], 1/5-tone
    /// up
    #[serde(rename = "accSagittal5v23SmallDiesisUp")]
    AccSagittal5V23SmallDiesisUp,
    /// 5:49 medium diesis down
    #[serde(rename = "accSagittal5v49MediumDiesisDown")]
    AccSagittal5V49MediumDiesisDown,
    /// 5:49 medium diesis up, (5:49M, half apotome)
    #[serde(rename = "accSagittal5v49MediumDiesisUp")]
    AccSagittal5V49MediumDiesisUp,
    /// 5:7 kleisma down
    #[serde(rename = "accSagittal5v7KleismaDown")]
    AccSagittal5V7KleismaDown,
    /// 5:7 kleisma up, (5:7k, ~11:13k, 7C less 5C)
    #[serde(rename = "accSagittal5v7KleismaUp")]
    AccSagittal5V7KleismaUp,
    /// 6 tinas down, 2 minas down, 65/77-schismina down, 0.83 cents down
    #[serde(rename = "accSagittal6TinasDown")]
    AccSagittal6TinasDown,
    /// 6 tinas up, 2 minas up, 65/77-schismina up, 0.83 cents up
    #[serde(rename = "accSagittal6TinasUp")]
    AccSagittal6TinasUp,
    /// 7 comma down, 1° down \[43 EDO\], 2° down \[72 EDO\], 1/6-tone down
    #[serde(rename = "accSagittal7CommaDown")]
    AccSagittal7CommaDown,
    /// 7 comma up, (7C), 1° up \[43 EDO\], 2° up \[72 EDO\], 1/6-tone up
    #[serde(rename = "accSagittal7CommaUp")]
    AccSagittal7CommaUp,
    /// 7 tinas down, 7/(5²⋅17)-schismina down, 1.02 cents down
    #[serde(rename = "accSagittal7TinasDown")]
    AccSagittal7TinasDown,
    /// 7 tinas up, 7/(5²⋅17)-schismina up, 1.02 cents up
    #[serde(rename = "accSagittal7TinasUp")]
    AccSagittal7TinasUp,
    /// 7:11 comma down, 1° down \[60 EDO\], 1/10-tone down
    #[serde(rename = "accSagittal7v11CommaDown")]
    AccSagittal7V11CommaDown,
    /// 7:11 comma up, (7:11C, ~13:17S, ~29S, 11L less 7C), 1° up \[60 EDO\]
    #[serde(rename = "accSagittal7v11CommaUp")]
    AccSagittal7V11CommaUp,
    /// 7:11 kleisma down
    #[serde(rename = "accSagittal7v11KleismaDown")]
    AccSagittal7V11KleismaDown,
    /// 7:11 kleisma up, (7:11k, ~29k)
    #[serde(rename = "accSagittal7v11KleismaUp")]
    AccSagittal7V11KleismaUp,
    /// 7:19 comma down
    #[serde(rename = "accSagittal7v19CommaDown")]
    AccSagittal7V19CommaDown,
    /// 7:19 comma up, (7:19C, 7C less 19s)
    #[serde(rename = "accSagittal7v19CommaUp")]
    AccSagittal7V19CommaUp,
    /// 8 tinas down, 11⋅17/(5²⋅7)-schismina down, 1.14 cents down
    #[serde(rename = "accSagittal8TinasDown")]
    AccSagittal8TinasDown,
    /// 8 tinas up, 11⋅17/(5²⋅7)-schismina up, 1.14 cents up
    #[serde(rename = "accSagittal8TinasUp")]
    AccSagittal8TinasUp,
    /// 9 tinas down, 1/(7²⋅11)-schismina down, 1.26 cents down
    #[serde(rename = "accSagittal9TinasDown")]
    AccSagittal9TinasDown,
    /// 9 tinas up, 1/(7²⋅11)-schismina up, 1.26 cents up
    #[serde(rename = "accSagittal9TinasUp")]
    AccSagittal9TinasUp,
    /// Acute, 5 schisma up (5s), 2 cents up
    #[serde(rename = "accSagittalAcute")]
    AccSagittalAcute,
    /// Double flat, (2 apotomes down)\[almost all EDOs\], whole-tone down
    #[serde(rename = "accSagittalDoubleFlat")]
    AccSagittalDoubleFlat,
    /// Double flat 11:49C-up
    #[serde(rename = "accSagittalDoubleFlat11v49CUp")]
    AccSagittalDoubleFlat11V49CUp,
    /// Double flat 143C-up
    #[serde(rename = "accSagittalDoubleFlat143CUp")]
    AccSagittalDoubleFlat143CUp,
    /// Double flat 17C-up
    #[serde(rename = "accSagittalDoubleFlat17CUp")]
    AccSagittalDoubleFlat17CUp,
    /// Double flat 17k-up
    #[serde(rename = "accSagittalDoubleFlat17kUp")]
    AccSagittalDoubleFlat17KUp,
    /// Double flat 19C-up
    #[serde(rename = "accSagittalDoubleFlat19CUp")]
    AccSagittalDoubleFlat19CUp,
    /// Double flat 19s-up
    #[serde(rename = "accSagittalDoubleFlat19sUp")]
    AccSagittalDoubleFlat19SUp,
    /// Double flat 23C-up, 14° down \[96 EDO\], 7/8-tone down
    #[serde(rename = "accSagittalDoubleFlat23CUp")]
    AccSagittalDoubleFlat23CUp,
    /// Double flat 23S-up
    #[serde(rename = "accSagittalDoubleFlat23SUp")]
    AccSagittalDoubleFlat23SUp,
    /// Double flat 25S-up, 8°down \[53 EDO\]
    #[serde(rename = "accSagittalDoubleFlat25SUp")]
    AccSagittalDoubleFlat25SUp,
    /// Double flat 49S-up
    #[serde(rename = "accSagittalDoubleFlat49SUp")]
    AccSagittalDoubleFlat49SUp,
    /// Double flat 55C-up, 13° down \[96 EDO\], 13/16-tone down
    #[serde(rename = "accSagittalDoubleFlat55CUp")]
    AccSagittalDoubleFlat55CUp,
    /// Double flat 5C-up, 5°\[22 29\] 7°\[34 41\] 9°53 down, 11/12 tone down
    #[serde(rename = "accSagittalDoubleFlat5CUp")]
    AccSagittalDoubleFlat5CUp,
    /// Double flat 5:11S-up
    #[serde(rename = "accSagittalDoubleFlat5v11SUp")]
    AccSagittalDoubleFlat5V11SUp,
    /// Double flat 5:19C-up, 19/20-tone down
    #[serde(rename = "accSagittalDoubleFlat5v19CUp")]
    AccSagittalDoubleFlat5V19CUp,
    /// Double flat 5:23S-up, 8° down \[60 EDO\], 4/5-tone down
    #[serde(rename = "accSagittalDoubleFlat5v23SUp")]
    AccSagittalDoubleFlat5V23SUp,
    /// Double flat 5:7k-up
    #[serde(rename = "accSagittalDoubleFlat5v7kUp")]
    AccSagittalDoubleFlat5V7KUp,
    /// Double flat 7C-up, 5° down \[43 EDO\], 10° down \[72 EDO\], 5/6-tone
    /// down
    #[serde(rename = "accSagittalDoubleFlat7CUp")]
    AccSagittalDoubleFlat7CUp,
    /// Double flat 7:11C-up, 9° down \[60 EDO\], 9/10-tone down
    #[serde(rename = "accSagittalDoubleFlat7v11CUp")]
    AccSagittalDoubleFlat7V11CUp,
    /// Double flat 7:11k-up
    #[serde(rename = "accSagittalDoubleFlat7v11kUp")]
    AccSagittalDoubleFlat7V11KUp,
    /// Double flat 7:19C-up
    #[serde(rename = "accSagittalDoubleFlat7v19CUp")]
    AccSagittalDoubleFlat7V19CUp,
    /// Double sharp, (2 apotomes up)\[almost all EDOs\], whole-tone up
    #[serde(rename = "accSagittalDoubleSharp")]
    AccSagittalDoubleSharp,
    /// Double sharp 11:49C-down
    #[serde(rename = "accSagittalDoubleSharp11v49CDown")]
    AccSagittalDoubleSharp11V49CDown,
    /// Double sharp 143C-down
    #[serde(rename = "accSagittalDoubleSharp143CDown")]
    AccSagittalDoubleSharp143CDown,
    /// Double sharp 17C-down
    #[serde(rename = "accSagittalDoubleSharp17CDown")]
    AccSagittalDoubleSharp17CDown,
    /// Double sharp 17k-down
    #[serde(rename = "accSagittalDoubleSharp17kDown")]
    AccSagittalDoubleSharp17KDown,
    /// Double sharp 19C-down
    #[serde(rename = "accSagittalDoubleSharp19CDown")]
    AccSagittalDoubleSharp19CDown,
    /// Double sharp 19s-down
    #[serde(rename = "accSagittalDoubleSharp19sDown")]
    AccSagittalDoubleSharp19SDown,
    /// Double sharp 23C-down, 14°up \[96 EDO\], 7/8-tone up
    #[serde(rename = "accSagittalDoubleSharp23CDown")]
    AccSagittalDoubleSharp23CDown,
    /// Double sharp 23S-down
    #[serde(rename = "accSagittalDoubleSharp23SDown")]
    AccSagittalDoubleSharp23SDown,
    /// Double sharp 25S-down, 8°up \[53 EDO\]
    #[serde(rename = "accSagittalDoubleSharp25SDown")]
    AccSagittalDoubleSharp25SDown,
    /// Double sharp 49S-down
    #[serde(rename = "accSagittalDoubleSharp49SDown")]
    AccSagittalDoubleSharp49SDown,
    /// Double sharp 55C-down, 13° up \[96 EDO\], 13/16-tone up
    #[serde(rename = "accSagittalDoubleSharp55CDown")]
    AccSagittalDoubleSharp55CDown,
    /// Double sharp 5C-down, 5°\[22 29\] 7°\[34 41\] 9°53 up, 11/12 tone up
    #[serde(rename = "accSagittalDoubleSharp5CDown")]
    AccSagittalDoubleSharp5CDown,
    /// Double sharp 5:11S-down
    #[serde(rename = "accSagittalDoubleSharp5v11SDown")]
    AccSagittalDoubleSharp5V11SDown,
    /// Double sharp 5:19C-down, 19/20-tone up
    #[serde(rename = "accSagittalDoubleSharp5v19CDown")]
    AccSagittalDoubleSharp5V19CDown,
    /// Double sharp 5:23S-down, 8° up \[60 EDO\], 4/5-tone up
    #[serde(rename = "accSagittalDoubleSharp5v23SDown")]
    AccSagittalDoubleSharp5V23SDown,
    /// Double sharp 5:7k-down
    #[serde(rename = "accSagittalDoubleSharp5v7kDown")]
    AccSagittalDoubleSharp5V7KDown,
    /// Double sharp 7C-down, 5°\[43\] 10°\[72\] up, 5/6-tone up
    #[serde(rename = "accSagittalDoubleSharp7CDown")]
    AccSagittalDoubleSharp7CDown,
    /// Double sharp 7:11C-down, 9° up \[60 EDO\], 9/10-tone up
    #[serde(rename = "accSagittalDoubleSharp7v11CDown")]
    AccSagittalDoubleSharp7V11CDown,
    /// Double sharp 7:11k-down
    #[serde(rename = "accSagittalDoubleSharp7v11kDown")]
    AccSagittalDoubleSharp7V11KDown,
    /// Double sharp 7:19C-down
    #[serde(rename = "accSagittalDoubleSharp7v19CDown")]
    AccSagittalDoubleSharp7V19CDown,
    /// Flat, (apotome down)\[almost all EDOs\], 1/2-tone down
    #[serde(rename = "accSagittalFlat")]
    AccSagittalFlat,
    /// Flat 11L-down, 8° up \[46 EDO\]
    #[serde(rename = "accSagittalFlat11LDown")]
    AccSagittalFlat11LDown,
    /// Flat 11M-down, 3° down \[17 31 EDOs\], 7° down \[46 EDO\], 3/4-tone down
    #[serde(rename = "accSagittalFlat11MDown")]
    AccSagittalFlat11MDown,
    /// Flat 11:19L-down
    #[serde(rename = "accSagittalFlat11v19LDown")]
    AccSagittalFlat11V19LDown,
    /// Flat 11:19M-down
    #[serde(rename = "accSagittalFlat11v19MDown")]
    AccSagittalFlat11V19MDown,
    /// Flat 11:49C-down
    #[serde(rename = "accSagittalFlat11v49CDown")]
    AccSagittalFlat11V49CDown,
    /// Flat 11:49C-up
    #[serde(rename = "accSagittalFlat11v49CUp")]
    AccSagittalFlat11V49CUp,
    /// Flat 143C-down
    #[serde(rename = "accSagittalFlat143CDown")]
    AccSagittalFlat143CDown,
    /// Flat 143C-up
    #[serde(rename = "accSagittalFlat143CUp")]
    AccSagittalFlat143CUp,
    /// Flat 17C-down
    #[serde(rename = "accSagittalFlat17CDown")]
    AccSagittalFlat17CDown,
    /// Flat 17C-up
    #[serde(rename = "accSagittalFlat17CUp")]
    AccSagittalFlat17CUp,
    /// Flat 17k-down
    #[serde(rename = "accSagittalFlat17kDown")]
    AccSagittalFlat17KDown,
    /// Flat 17k-up
    #[serde(rename = "accSagittalFlat17kUp")]
    AccSagittalFlat17KUp,
    /// Flat 19C-down
    #[serde(rename = "accSagittalFlat19CDown")]
    AccSagittalFlat19CDown,
    /// Flat 19C-up
    #[serde(rename = "accSagittalFlat19CUp")]
    AccSagittalFlat19CUp,
    /// Flat 19s-down
    #[serde(rename = "accSagittalFlat19sDown")]
    AccSagittalFlat19SDown,
    /// Flat 19s-up
    #[serde(rename = "accSagittalFlat19sUp")]
    AccSagittalFlat19SUp,
    /// Flat 23C-down, 10° down \[96 EDO\], 5/8-tone down
    #[serde(rename = "accSagittalFlat23CDown")]
    AccSagittalFlat23CDown,
    /// Flat 23C-up, 6° down \[96 EDO\], 3/8-tone down
    #[serde(rename = "accSagittalFlat23CUp")]
    AccSagittalFlat23CUp,
    /// Flat 23S-down
    #[serde(rename = "accSagittalFlat23SDown")]
    AccSagittalFlat23SDown,
    /// Flat 23S-up
    #[serde(rename = "accSagittalFlat23SUp")]
    AccSagittalFlat23SUp,
    /// Flat 25S-down, 7° down \[53 EDO\]
    #[serde(rename = "accSagittalFlat25SDown")]
    AccSagittalFlat25SDown,
    /// Flat 25S-up, 3° down \[53 EDO\]
    #[serde(rename = "accSagittalFlat25SUp")]
    AccSagittalFlat25SUp,
    /// Flat 35L-down, 5° down \[50 EDO\]
    #[serde(rename = "accSagittalFlat35LDown")]
    AccSagittalFlat35LDown,
    /// Flat 35M-down, 4° down \[50 EDO\], 6° down \[27 EDO\], 13/18-tone down
    #[serde(rename = "accSagittalFlat35MDown")]
    AccSagittalFlat35MDown,
    /// Flat 49L-down
    #[serde(rename = "accSagittalFlat49LDown")]
    AccSagittalFlat49LDown,
    /// Flat 49M-down
    #[serde(rename = "accSagittalFlat49MDown")]
    AccSagittalFlat49MDown,
    /// Flat 49S-down
    #[serde(rename = "accSagittalFlat49SDown")]
    AccSagittalFlat49SDown,
    /// Flat 49S-up
    #[serde(rename = "accSagittalFlat49SUp")]
    AccSagittalFlat49SUp,
    /// Flat 55C-down, 11° down \[96 EDO\], 11/16-tone down
    #[serde(rename = "accSagittalFlat55CDown")]
    AccSagittalFlat55CDown,
    /// Flat 55C-up, 5° down \[96 EDO\], 5/16-tone down
    #[serde(rename = "accSagittalFlat55CUp")]
    AccSagittalFlat55CUp,
    /// Flat 5C-down, 4°\[22 29\] 5°\[27 34 41\] 6°\[39 46 53\] down, 7/12-tone
    /// down
    #[serde(rename = "accSagittalFlat5CDown")]
    AccSagittalFlat5CDown,
    /// Flat 5C-up, 2°\[22 29\] 3°\[27 34 41\] 4°\[39 46 53\] 5°72 7°\[96\]
    /// down, 5/12-tone down
    #[serde(rename = "accSagittalFlat5CUp")]
    AccSagittalFlat5CUp,
    /// Flat 5:11S-down
    #[serde(rename = "accSagittalFlat5v11SDown")]
    AccSagittalFlat5V11SDown,
    /// Flat 5:11S-up
    #[serde(rename = "accSagittalFlat5v11SUp")]
    AccSagittalFlat5V11SUp,
    /// Flat 5:13L-down
    #[serde(rename = "accSagittalFlat5v13LDown")]
    AccSagittalFlat5V13LDown,
    /// Flat 5:13M-down
    #[serde(rename = "accSagittalFlat5v13MDown")]
    AccSagittalFlat5V13MDown,
    /// Flat 5:19C-down, 11/20-tone down
    #[serde(rename = "accSagittalFlat5v19CDown")]
    AccSagittalFlat5V19CDown,
    /// Flat 5:19C-up, 9/20-tone down
    #[serde(rename = "accSagittalFlat5v19CUp")]
    AccSagittalFlat5V19CUp,
    /// Flat 5:23S-down, 7° down \[60 EDO\], 7/10-tone down
    #[serde(rename = "accSagittalFlat5v23SDown")]
    AccSagittalFlat5V23SDown,
    /// Flat 5:23S-up, 3° down \[60 EDO\], 3/10-tone down
    #[serde(rename = "accSagittalFlat5v23SUp")]
    AccSagittalFlat5V23SUp,
    /// Flat 5:49M-down
    #[serde(rename = "accSagittalFlat5v49MDown")]
    AccSagittalFlat5V49MDown,
    /// Flat 5:7k-down
    #[serde(rename = "accSagittalFlat5v7kDown")]
    AccSagittalFlat5V7KDown,
    /// Flat 5:7k-up
    #[serde(rename = "accSagittalFlat5v7kUp")]
    AccSagittalFlat5V7KUp,
    /// Flat 7C-down, 4° down \[43 EDO\], 8° down \[72 EDO\], 2/3-tone down
    #[serde(rename = "accSagittalFlat7CDown")]
    AccSagittalFlat7CDown,
    /// Flat 7C-up, 2° down \[43 EDO\], 4° down \[72 EDO\], 1/3-tone down
    #[serde(rename = "accSagittalFlat7CUp")]
    AccSagittalFlat7CUp,
    /// Flat 7:11C-down, 6° down \[60 EDO\], 3/5- tone down
    #[serde(rename = "accSagittalFlat7v11CDown")]
    AccSagittalFlat7V11CDown,
    /// Flat 7:11C-up, 4° down \[60 EDO\], 2/5-tone down
    #[serde(rename = "accSagittalFlat7v11CUp")]
    AccSagittalFlat7V11CUp,
    /// Flat 7:11k-down
    #[serde(rename = "accSagittalFlat7v11kDown")]
    AccSagittalFlat7V11KDown,
    /// Flat 7:11k-up
    #[serde(rename = "accSagittalFlat7v11kUp")]
    AccSagittalFlat7V11KUp,
    /// Flat 7:19C-down
    #[serde(rename = "accSagittalFlat7v19CDown")]
    AccSagittalFlat7V19CDown,
    /// Flat 7:19C-up
    #[serde(rename = "accSagittalFlat7v19CUp")]
    AccSagittalFlat7V19CUp,
    /// Fractional tina down, 77/(5⋅37)-schismina down, 0.08 cents down
    #[serde(rename = "accSagittalFractionalTinaDown")]
    AccSagittalFractionalTinaDown,
    /// Fractional tina up, 77/(5⋅37)-schismina up, 0.08 cents up
    #[serde(rename = "accSagittalFractionalTinaUp")]
    AccSagittalFractionalTinaUp,
    /// Grave, 5 schisma down, 2 cents down
    #[serde(rename = "accSagittalGrave")]
    AccSagittalGrave,
    /// Shaft down, (natural for use with only diacritics down)
    #[serde(rename = "accSagittalShaftDown")]
    AccSagittalShaftDown,
    /// Shaft up, (natural for use with only diacritics up)
    #[serde(rename = "accSagittalShaftUp")]
    AccSagittalShaftUp,
    /// Sharp, (apotome up)\[almost all EDOs\], 1/2-tone up
    #[serde(rename = "accSagittalSharp")]
    AccSagittalSharp,
    /// Sharp 11L-up, 8° up \[46 EDO\]
    #[serde(rename = "accSagittalSharp11LUp")]
    AccSagittalSharp11LUp,
    /// Sharp 11M-up, 3° up \[17 31 EDOs\], 7° up \[46 EDO\], 3/4-tone up
    #[serde(rename = "accSagittalSharp11MUp")]
    AccSagittalSharp11MUp,
    /// Sharp 11:19L-up
    #[serde(rename = "accSagittalSharp11v19LUp")]
    AccSagittalSharp11V19LUp,
    /// Sharp 11:19M-up
    #[serde(rename = "accSagittalSharp11v19MUp")]
    AccSagittalSharp11V19MUp,
    /// Sharp 11:49C-down
    #[serde(rename = "accSagittalSharp11v49CDown")]
    AccSagittalSharp11V49CDown,
    /// Sharp 11:49C-up
    #[serde(rename = "accSagittalSharp11v49CUp")]
    AccSagittalSharp11V49CUp,
    /// Sharp 143C-down
    #[serde(rename = "accSagittalSharp143CDown")]
    AccSagittalSharp143CDown,
    /// Sharp 143C-up
    #[serde(rename = "accSagittalSharp143CUp")]
    AccSagittalSharp143CUp,
    /// Sharp 17C-down
    #[serde(rename = "accSagittalSharp17CDown")]
    AccSagittalSharp17CDown,
    /// Sharp 17C-up
    #[serde(rename = "accSagittalSharp17CUp")]
    AccSagittalSharp17CUp,
    /// Sharp 17k-down
    #[serde(rename = "accSagittalSharp17kDown")]
    AccSagittalSharp17KDown,
    /// Sharp 17k-up
    #[serde(rename = "accSagittalSharp17kUp")]
    AccSagittalSharp17KUp,
    /// Sharp 19C-down
    #[serde(rename = "accSagittalSharp19CDown")]
    AccSagittalSharp19CDown,
    /// Sharp 19C-up
    #[serde(rename = "accSagittalSharp19CUp")]
    AccSagittalSharp19CUp,
    /// Sharp 19s-down
    #[serde(rename = "accSagittalSharp19sDown")]
    AccSagittalSharp19SDown,
    /// Sharp 19s-up
    #[serde(rename = "accSagittalSharp19sUp")]
    AccSagittalSharp19SUp,
    /// Sharp 23C-down, 6° up \[96 EDO\], 3/8-tone up
    #[serde(rename = "accSagittalSharp23CDown")]
    AccSagittalSharp23CDown,
    /// Sharp 23C-up, 10° up \[96 EDO\], 5/8-tone up
    #[serde(rename = "accSagittalSharp23CUp")]
    AccSagittalSharp23CUp,
    /// Sharp 23S-down
    #[serde(rename = "accSagittalSharp23SDown")]
    AccSagittalSharp23SDown,
    /// Sharp 23S-up
    #[serde(rename = "accSagittalSharp23SUp")]
    AccSagittalSharp23SUp,
    /// Sharp 25S-down, 3° up \[53 EDO\]
    #[serde(rename = "accSagittalSharp25SDown")]
    AccSagittalSharp25SDown,
    /// Sharp 25S-up, 7° up \[53 EDO\]
    #[serde(rename = "accSagittalSharp25SUp")]
    AccSagittalSharp25SUp,
    /// Sharp 35L-up, 5° up \[50 EDO\]
    #[serde(rename = "accSagittalSharp35LUp")]
    AccSagittalSharp35LUp,
    /// Sharp 35M-up, 4° up \[50 EDO\], 6° up \[27 EDO\], 13/18-tone up
    #[serde(rename = "accSagittalSharp35MUp")]
    AccSagittalSharp35MUp,
    /// Sharp 49L-up
    #[serde(rename = "accSagittalSharp49LUp")]
    AccSagittalSharp49LUp,
    /// Sharp 49M-up
    #[serde(rename = "accSagittalSharp49MUp")]
    AccSagittalSharp49MUp,
    /// Sharp 49S-down
    #[serde(rename = "accSagittalSharp49SDown")]
    AccSagittalSharp49SDown,
    /// Sharp 49S-up
    #[serde(rename = "accSagittalSharp49SUp")]
    AccSagittalSharp49SUp,
    /// Sharp 55C-down, 5° up \[96 EDO\], 5/16-tone up
    #[serde(rename = "accSagittalSharp55CDown")]
    AccSagittalSharp55CDown,
    /// Sharp 55C-up, 11° up \[96 EDO\], 11/16-tone up
    #[serde(rename = "accSagittalSharp55CUp")]
    AccSagittalSharp55CUp,
    /// Sharp 5C-down, 2°\[22 29\] 3°\[27 34 41\] 4°\[39 46 53\] 5°\[72\]
    /// 7°\[96\] up, 5/12-tone up
    #[serde(rename = "accSagittalSharp5CDown")]
    AccSagittalSharp5CDown,
    /// Sharp 5C-up, 4°\[22 29\] 5°\[27 34 41\] 6°\[39 46 53\] up, 7/12-tone up
    #[serde(rename = "accSagittalSharp5CUp")]
    AccSagittalSharp5CUp,
    /// Sharp 5:11S-down
    #[serde(rename = "accSagittalSharp5v11SDown")]
    AccSagittalSharp5V11SDown,
    /// Sharp 5:11S-up
    #[serde(rename = "accSagittalSharp5v11SUp")]
    AccSagittalSharp5V11SUp,
    /// Sharp 5:13L-up
    #[serde(rename = "accSagittalSharp5v13LUp")]
    AccSagittalSharp5V13LUp,
    /// Sharp 5:13M-up
    #[serde(rename = "accSagittalSharp5v13MUp")]
    AccSagittalSharp5V13MUp,
    /// Sharp 5:19C-down, 9/20-tone up
    #[serde(rename = "accSagittalSharp5v19CDown")]
    AccSagittalSharp5V19CDown,
    /// Sharp 5:19C-up, 11/20-tone up
    #[serde(rename = "accSagittalSharp5v19CUp")]
    AccSagittalSharp5V19CUp,
    /// Sharp 5:23S-down, 3° up \[60 EDO\], 3/10-tone up
    #[serde(rename = "accSagittalSharp5v23SDown")]
    AccSagittalSharp5V23SDown,
    /// Sharp 5:23S-up, 7° up \[60 EDO\], 7/10-tone up
    #[serde(rename = "accSagittalSharp5v23SUp")]
    AccSagittalSharp5V23SUp,
    /// Sharp 5:49M-up, (one and a half apotomes)
    #[serde(rename = "accSagittalSharp5v49MUp")]
    AccSagittalSharp5V49MUp,
    /// Sharp 5:7k-down
    #[serde(rename = "accSagittalSharp5v7kDown")]
    AccSagittalSharp5V7KDown,
    /// Sharp 5:7k-up
    #[serde(rename = "accSagittalSharp5v7kUp")]
    AccSagittalSharp5V7KUp,
    /// Sharp 7C-down, 2° up \[43 EDO\], 4° up \[72 EDO\], 1/3-tone up
    #[serde(rename = "accSagittalSharp7CDown")]
    AccSagittalSharp7CDown,
    /// Sharp 7C-up, 4° up \[43 EDO\], 8° up \[72 EDO\], 2/3-tone up
    #[serde(rename = "accSagittalSharp7CUp")]
    AccSagittalSharp7CUp,
    /// Sharp 7:11C-down, 4° up \[60 EDO\], 2/5-tone up
    #[serde(rename = "accSagittalSharp7v11CDown")]
    AccSagittalSharp7V11CDown,
    /// Sharp 7:11C-up, 6° up \[60 EDO\], 3/5- tone up
    #[serde(rename = "accSagittalSharp7v11CUp")]
    AccSagittalSharp7V11CUp,
    /// Sharp 7:11k-down
    #[serde(rename = "accSagittalSharp7v11kDown")]
    AccSagittalSharp7V11KDown,
    /// Sharp 7:11k-up
    #[serde(rename = "accSagittalSharp7v11kUp")]
    AccSagittalSharp7V11KUp,
    /// Sharp 7:19C-down
    #[serde(rename = "accSagittalSharp7v19CDown")]
    AccSagittalSharp7V19CDown,
    /// Sharp 7:19C-up
    #[serde(rename = "accSagittalSharp7v19CUp")]
    AccSagittalSharp7V19CUp,
    /// Unused
    #[serde(rename = "accSagittalUnused1")]
    AccSagittalUnused1,
    /// Unused
    #[serde(rename = "accSagittalUnused2")]
    AccSagittalUnused2,
    /// Unused
    #[serde(rename = "accSagittalUnused3")]
    AccSagittalUnused3,
    /// Unused
    #[serde(rename = "accSagittalUnused4")]
    AccSagittalUnused4,
    /// Combining accordion coupler dot
    #[serde(rename = "accdnCombDot")]
    AccdnCombDot,
    /// Combining left hand, 2 ranks, empty
    #[serde(rename = "accdnCombLH2RanksEmpty")]
    AccdnCombLh2RanksEmpty,
    /// Combining left hand, 3 ranks, empty (square)
    #[serde(rename = "accdnCombLH3RanksEmptySquare")]
    AccdnCombLh3RanksEmptySquare,
    /// Combining right hand, 3 ranks, empty
    #[serde(rename = "accdnCombRH3RanksEmpty")]
    AccdnCombRh3RanksEmpty,
    /// Combining right hand, 4 ranks, empty
    #[serde(rename = "accdnCombRH4RanksEmpty")]
    AccdnCombRh4RanksEmpty,
    /// Diatonic accordion clef
    #[serde(rename = "accdnDiatonicClef")]
    AccdnDiatonicClef,
    /// Left hand, 2 ranks, 16' stop (round)
    #[serde(rename = "accdnLH2Ranks16Round")]
    AccdnLh2Ranks16Round,
    /// Left hand, 2 ranks, 8' stop + 16' stop (round)
    #[serde(rename = "accdnLH2Ranks8Plus16Round")]
    AccdnLh2Ranks8Plus16Round,
    /// Left hand, 2 ranks, 8' stop (round)
    #[serde(rename = "accdnLH2Ranks8Round")]
    AccdnLh2Ranks8Round,
    /// Left hand, 2 ranks, full master (round)
    #[serde(rename = "accdnLH2RanksFullMasterRound")]
    AccdnLh2RanksFullMasterRound,
    /// Left hand, 2 ranks, master + 16' stop (round)
    #[serde(rename = "accdnLH2RanksMasterPlus16Round")]
    AccdnLh2RanksMasterPlus16Round,
    /// Left hand, 2 ranks, master (round)
    #[serde(rename = "accdnLH2RanksMasterRound")]
    AccdnLh2RanksMasterRound,
    /// Left hand, 3 ranks, 2' stop + 8' stop (square)
    #[serde(rename = "accdnLH3Ranks2Plus8Square")]
    AccdnLh3Ranks2Plus8Square,
    /// Left hand, 3 ranks, 2' stop (square)
    #[serde(rename = "accdnLH3Ranks2Square")]
    AccdnLh3Ranks2Square,
    /// Left hand, 3 ranks, 8' stop (square)
    #[serde(rename = "accdnLH3Ranks8Square")]
    AccdnLh3Ranks8Square,
    /// Left hand, 3 ranks, double 8' stop (square)
    #[serde(rename = "accdnLH3RanksDouble8Square")]
    AccdnLh3RanksDouble8Square,
    /// Left hand, 3 ranks, 2' stop + double 8' stop (tutti) (square)
    #[serde(rename = "accdnLH3RanksTuttiSquare")]
    AccdnLh3RanksTuttiSquare,
    /// Pull
    #[serde(rename = "accdnPull")]
    AccdnPull,
    /// Push
    #[serde(rename = "accdnPush")]
    AccdnPush,
    /// Right hand, 3 ranks, 8' stop + upper tremolo 8' stop + 16' stop
    /// (accordion)
    #[serde(rename = "accdnRH3RanksAccordion")]
    AccdnRh3RanksAccordion,
    /// Right hand, 3 ranks, lower tremolo 8' stop + 8' stop + upper tremolo 8'
    /// stop (authentic musette)
    #[serde(rename = "accdnRH3RanksAuthenticMusette")]
    AccdnRh3RanksAuthenticMusette,
    /// Right hand, 3 ranks, 8' stop + 16' stop (bandoneón)
    #[serde(rename = "accdnRH3RanksBandoneon")]
    AccdnRh3RanksBandoneon,
    /// Right hand, 3 ranks, 16' stop (bassoon)
    #[serde(rename = "accdnRH3RanksBassoon")]
    AccdnRh3RanksBassoon,
    /// Right hand, 3 ranks, 8' stop (clarinet)
    #[serde(rename = "accdnRH3RanksClarinet")]
    AccdnRh3RanksClarinet,
    /// Right hand, 3 ranks, lower tremolo 8' stop + 8' stop + upper tremolo 8'
    /// stop + 16' stop
    #[serde(rename = "accdnRH3RanksDoubleTremoloLower8ve")]
    AccdnRh3RanksDoubleTremoloLower8Ve,
    /// Right hand, 3 ranks, 4' stop + lower tremolo 8' stop + 8' stop + upper
    /// tremolo 8' stop
    #[serde(rename = "accdnRH3RanksDoubleTremoloUpper8ve")]
    AccdnRh3RanksDoubleTremoloUpper8Ve,
    /// Right hand, 3 ranks, 4' stop + lower tremolo 8' stop + 8' stop + upper
    /// tremolo 8' stop + 16' stop
    #[serde(rename = "accdnRH3RanksFullFactory")]
    AccdnRh3RanksFullFactory,
    /// Right hand, 3 ranks, 4' stop + 8' stop + 16' stop (harmonium)
    #[serde(rename = "accdnRH3RanksHarmonium")]
    AccdnRh3RanksHarmonium,
    /// Right hand, 3 ranks, 4' stop + 8' stop + upper tremolo 8' stop
    /// (imitation musette)
    #[serde(rename = "accdnRH3RanksImitationMusette")]
    AccdnRh3RanksImitationMusette,
    /// Right hand, 3 ranks, lower tremolo 8' stop
    #[serde(rename = "accdnRH3RanksLowerTremolo8")]
    AccdnRh3RanksLowerTremolo8,
    /// Right hand, 3 ranks, 4' stop + lower tremolo 8' stop + upper tremolo 8'
    /// stop + 16' stop (master)
    #[serde(rename = "accdnRH3RanksMaster")]
    AccdnRh3RanksMaster,
    /// Right hand, 3 ranks, 4' stop + 8' stop (oboe)
    #[serde(rename = "accdnRH3RanksOboe")]
    AccdnRh3RanksOboe,
    /// Right hand, 3 ranks, 4' stop + 16' stop (organ)
    #[serde(rename = "accdnRH3RanksOrgan")]
    AccdnRh3RanksOrgan,
    /// Right hand, 3 ranks, 4' stop (piccolo)
    #[serde(rename = "accdnRH3RanksPiccolo")]
    AccdnRh3RanksPiccolo,
    /// Right hand, 3 ranks, lower tremolo 8' stop + upper tremolo 8' stop + 16'
    /// stop
    #[serde(rename = "accdnRH3RanksTremoloLower8ve")]
    AccdnRh3RanksTremoloLower8Ve,
    /// Right hand, 3 ranks, 4' stop + lower tremolo 8' stop + upper tremolo 8'
    /// stop
    #[serde(rename = "accdnRH3RanksTremoloUpper8ve")]
    AccdnRh3RanksTremoloUpper8Ve,
    /// Right hand, 3 ranks, lower tremolo 8' stop + upper tremolo 8' stop
    #[serde(rename = "accdnRH3RanksTwoChoirs")]
    AccdnRh3RanksTwoChoirs,
    /// Right hand, 3 ranks, upper tremolo 8' stop
    #[serde(rename = "accdnRH3RanksUpperTremolo8")]
    AccdnRh3RanksUpperTremolo8,
    /// Right hand, 3 ranks, 8' stop + upper tremolo 8' stop (violin)
    #[serde(rename = "accdnRH3RanksViolin")]
    AccdnRh3RanksViolin,
    /// Right hand, 4 ranks, alto
    #[serde(rename = "accdnRH4RanksAlto")]
    AccdnRh4RanksAlto,
    /// Right hand, 4 ranks, bass/alto
    #[serde(rename = "accdnRH4RanksBassAlto")]
    AccdnRh4RanksBassAlto,
    /// Right hand, 4 ranks, master
    #[serde(rename = "accdnRH4RanksMaster")]
    AccdnRh4RanksMaster,
    /// Right hand, 4 ranks, soft bass
    #[serde(rename = "accdnRH4RanksSoftBass")]
    AccdnRh4RanksSoftBass,
    /// Right hand, 4 ranks, soft tenor
    #[serde(rename = "accdnRH4RanksSoftTenor")]
    AccdnRh4RanksSoftTenor,
    /// Right hand, 4 ranks, soprano
    #[serde(rename = "accdnRH4RanksSoprano")]
    AccdnRh4RanksSoprano,
    /// Right hand, 4 ranks, tenor
    #[serde(rename = "accdnRH4RanksTenor")]
    AccdnRh4RanksTenor,
    /// Ricochet (2 tones)
    #[serde(rename = "accdnRicochet2")]
    AccdnRicochet2,
    /// Ricochet (3 tones)
    #[serde(rename = "accdnRicochet3")]
    AccdnRicochet3,
    /// Ricochet (4 tones)
    #[serde(rename = "accdnRicochet4")]
    AccdnRicochet4,
    /// Ricochet (5 tones)
    #[serde(rename = "accdnRicochet5")]
    AccdnRicochet5,
    /// Ricochet (6 tones)
    #[serde(rename = "accdnRicochet6")]
    AccdnRicochet6,
    /// Combining ricochet for stem (2 tones)
    #[serde(rename = "accdnRicochetStem2")]
    AccdnRicochetStem2,
    /// Combining ricochet for stem (3 tones)
    #[serde(rename = "accdnRicochetStem3")]
    AccdnRicochetStem3,
    /// Combining ricochet for stem (4 tones)
    #[serde(rename = "accdnRicochetStem4")]
    AccdnRicochetStem4,
    /// Combining ricochet for stem (5 tones)
    #[serde(rename = "accdnRicochetStem5")]
    AccdnRicochetStem5,
    /// Combining ricochet for stem (6 tones)
    #[serde(rename = "accdnRicochetStem6")]
    AccdnRicochetStem6,
    /// 1-comma flat
    #[serde(rename = "accidental1CommaFlat")]
    Accidental1CommaFlat,
    /// 1-comma sharp
    #[serde(rename = "accidental1CommaSharp")]
    Accidental1CommaSharp,
    /// 2-comma flat
    #[serde(rename = "accidental2CommaFlat")]
    Accidental2CommaFlat,
    /// 2-comma sharp
    #[serde(rename = "accidental2CommaSharp")]
    Accidental2CommaSharp,
    /// 3-comma flat
    #[serde(rename = "accidental3CommaFlat")]
    Accidental3CommaFlat,
    /// 3-comma sharp
    #[serde(rename = "accidental3CommaSharp")]
    Accidental3CommaSharp,
    /// 4-comma flat
    #[serde(rename = "accidental4CommaFlat")]
    Accidental4CommaFlat,
    /// 5-comma sharp
    #[serde(rename = "accidental5CommaSharp")]
    Accidental5CommaSharp,
    /// Arrow down (lower by one quarter-tone)
    #[serde(rename = "accidentalArrowDown")]
    AccidentalArrowDown,
    /// Arrow up (raise by one quarter-tone)
    #[serde(rename = "accidentalArrowUp")]
    AccidentalArrowUp,
    /// Bakiye (flat)
    #[serde(rename = "accidentalBakiyeFlat")]
    AccidentalBakiyeFlat,
    /// Bakiye (sharp)
    #[serde(rename = "accidentalBakiyeSharp")]
    AccidentalBakiyeSharp,
    /// Accidental bracket, left
    #[serde(rename = "accidentalBracketLeft")]
    AccidentalBracketLeft,
    /// Accidental bracket, right
    #[serde(rename = "accidentalBracketRight")]
    AccidentalBracketRight,
    /// Büyük mücenneb (flat)
    #[serde(rename = "accidentalBuyukMucennebFlat")]
    AccidentalBuyukMucennebFlat,
    /// Büyük mücenneb (sharp)
    #[serde(rename = "accidentalBuyukMucennebSharp")]
    AccidentalBuyukMucennebSharp,
    /// Combining close curly brace
    #[serde(rename = "accidentalCombiningCloseCurlyBrace")]
    AccidentalCombiningCloseCurlyBrace,
    /// Combining lower by one 17-limit schisma
    #[serde(rename = "accidentalCombiningLower17Schisma")]
    AccidentalCombiningLower17Schisma,
    /// Combining lower by one 19-limit schisma
    #[serde(rename = "accidentalCombiningLower19Schisma")]
    AccidentalCombiningLower19Schisma,
    /// Combining lower by one 23-limit comma
    #[serde(rename = "accidentalCombiningLower23Limit29LimitComma")]
    AccidentalCombiningLower23Limit29LimitComma,
    /// Combining lower by one 29-limit comma
    #[serde(rename = "accidentalCombiningLower29LimitComma")]
    AccidentalCombiningLower29LimitComma,
    /// Combining lower by one 31-limit schisma
    #[serde(rename = "accidentalCombiningLower31Schisma")]
    AccidentalCombiningLower31Schisma,
    /// Combining lower by one 37-limit quartertone
    #[serde(rename = "accidentalCombiningLower37Quartertone")]
    AccidentalCombiningLower37Quartertone,
    /// Combining lower by one 41-limit comma
    #[serde(rename = "accidentalCombiningLower41Comma")]
    AccidentalCombiningLower41Comma,
    /// Combining lower by one 43-limit comma
    #[serde(rename = "accidentalCombiningLower43Comma")]
    AccidentalCombiningLower43Comma,
    /// Combining lower by one 47-limit quartertone
    #[serde(rename = "accidentalCombiningLower47Quartertone")]
    AccidentalCombiningLower47Quartertone,
    /// Combining lower by one 53-limit comma
    #[serde(rename = "accidentalCombiningLower53LimitComma")]
    AccidentalCombiningLower53LimitComma,
    /// Combining open curly brace
    #[serde(rename = "accidentalCombiningOpenCurlyBrace")]
    AccidentalCombiningOpenCurlyBrace,
    /// Combining raise by one 17-limit schisma
    #[serde(rename = "accidentalCombiningRaise17Schisma")]
    AccidentalCombiningRaise17Schisma,
    /// Combining raise by one 19-limit schisma
    #[serde(rename = "accidentalCombiningRaise19Schisma")]
    AccidentalCombiningRaise19Schisma,
    /// Combining raise by one 23-limit comma
    #[serde(rename = "accidentalCombiningRaise23Limit29LimitComma")]
    AccidentalCombiningRaise23Limit29LimitComma,
    /// Combining raise by one 29-limit comma
    #[serde(rename = "accidentalCombiningRaise29LimitComma")]
    AccidentalCombiningRaise29LimitComma,
    /// Combining raise by one 31-limit schisma
    #[serde(rename = "accidentalCombiningRaise31Schisma")]
    AccidentalCombiningRaise31Schisma,
    /// Combining raise by one 37-limit quartertone
    #[serde(rename = "accidentalCombiningRaise37Quartertone")]
    AccidentalCombiningRaise37Quartertone,
    /// Combining raise by one 41-limit comma
    #[serde(rename = "accidentalCombiningRaise41Comma")]
    AccidentalCombiningRaise41Comma,
    /// Combining raise by one 43-limit comma
    #[serde(rename = "accidentalCombiningRaise43Comma")]
    AccidentalCombiningRaise43Comma,
    /// Combining raise by one 47-limit quartertone
    #[serde(rename = "accidentalCombiningRaise47Quartertone")]
    AccidentalCombiningRaise47Quartertone,
    /// Combining raise by one 53-limit comma
    #[serde(rename = "accidentalCombiningRaise53LimitComma")]
    AccidentalCombiningRaise53LimitComma,
    /// Syntonic/Didymus comma (80:81) down (Bosanquet)
    #[serde(rename = "accidentalCommaSlashDown")]
    AccidentalCommaSlashDown,
    /// Syntonic/Didymus comma (80:81) up (Bosanquet)
    #[serde(rename = "accidentalCommaSlashUp")]
    AccidentalCommaSlashUp,
    /// Double flat
    #[serde(rename = "accidentalDoubleFlat")]
    AccidentalDoubleFlat,
    /// Arabic double flat
    #[serde(rename = "accidentalDoubleFlatArabic")]
    AccidentalDoubleFlatArabic,
    /// Double flat equal tempered semitone
    #[serde(rename = "accidentalDoubleFlatEqualTempered")]
    AccidentalDoubleFlatEqualTempered,
    /// Double flat lowered by one syntonic comma
    #[serde(rename = "accidentalDoubleFlatOneArrowDown")]
    AccidentalDoubleFlatOneArrowDown,
    /// Double flat raised by one syntonic comma
    #[serde(rename = "accidentalDoubleFlatOneArrowUp")]
    AccidentalDoubleFlatOneArrowUp,
    /// Reversed double flat
    #[serde(rename = "accidentalDoubleFlatReversed")]
    AccidentalDoubleFlatReversed,
    /// Double flat lowered by three syntonic commas
    #[serde(rename = "accidentalDoubleFlatThreeArrowsDown")]
    AccidentalDoubleFlatThreeArrowsDown,
    /// Double flat raised by three syntonic commas
    #[serde(rename = "accidentalDoubleFlatThreeArrowsUp")]
    AccidentalDoubleFlatThreeArrowsUp,
    /// Turned double flat
    #[serde(rename = "accidentalDoubleFlatTurned")]
    AccidentalDoubleFlatTurned,
    /// Double flat lowered by two syntonic commas
    #[serde(rename = "accidentalDoubleFlatTwoArrowsDown")]
    AccidentalDoubleFlatTwoArrowsDown,
    /// Double flat raised by two syntonic commas
    #[serde(rename = "accidentalDoubleFlatTwoArrowsUp")]
    AccidentalDoubleFlatTwoArrowsUp,
    /// Double sharp
    #[serde(rename = "accidentalDoubleSharp")]
    AccidentalDoubleSharp,
    /// Arabic double sharp
    #[serde(rename = "accidentalDoubleSharpArabic")]
    AccidentalDoubleSharpArabic,
    /// Double sharp equal tempered semitone
    #[serde(rename = "accidentalDoubleSharpEqualTempered")]
    AccidentalDoubleSharpEqualTempered,
    /// Double sharp lowered by one syntonic comma
    #[serde(rename = "accidentalDoubleSharpOneArrowDown")]
    AccidentalDoubleSharpOneArrowDown,
    /// Double sharp raised by one syntonic comma
    #[serde(rename = "accidentalDoubleSharpOneArrowUp")]
    AccidentalDoubleSharpOneArrowUp,
    /// Double sharp lowered by three syntonic commas
    #[serde(rename = "accidentalDoubleSharpThreeArrowsDown")]
    AccidentalDoubleSharpThreeArrowsDown,
    /// Double sharp raised by three syntonic commas
    #[serde(rename = "accidentalDoubleSharpThreeArrowsUp")]
    AccidentalDoubleSharpThreeArrowsUp,
    /// Double sharp lowered by two syntonic commas
    #[serde(rename = "accidentalDoubleSharpTwoArrowsDown")]
    AccidentalDoubleSharpTwoArrowsDown,
    /// Double sharp raised by two syntonic commas
    #[serde(rename = "accidentalDoubleSharpTwoArrowsUp")]
    AccidentalDoubleSharpTwoArrowsUp,
    /// Enharmonically reinterpret accidental almost equal to
    #[serde(rename = "accidentalEnharmonicAlmostEqualTo")]
    AccidentalEnharmonicAlmostEqualTo,
    /// Enharmonically reinterpret accidental equals
    #[serde(rename = "accidentalEnharmonicEquals")]
    AccidentalEnharmonicEquals,
    /// Enharmonically reinterpret accidental tilde
    #[serde(rename = "accidentalEnharmonicTilde")]
    AccidentalEnharmonicTilde,
    /// Filled reversed flat and flat
    #[serde(rename = "accidentalFilledReversedFlatAndFlat")]
    AccidentalFilledReversedFlatAndFlat,
    /// Filled reversed flat and flat with arrow down
    #[serde(rename = "accidentalFilledReversedFlatAndFlatArrowDown")]
    AccidentalFilledReversedFlatAndFlatArrowDown,
    /// Filled reversed flat and flat with arrow up
    #[serde(rename = "accidentalFilledReversedFlatAndFlatArrowUp")]
    AccidentalFilledReversedFlatAndFlatArrowUp,
    /// Filled reversed flat with arrow down
    #[serde(rename = "accidentalFilledReversedFlatArrowDown")]
    AccidentalFilledReversedFlatArrowDown,
    /// Filled reversed flat with arrow up
    #[serde(rename = "accidentalFilledReversedFlatArrowUp")]
    AccidentalFilledReversedFlatArrowUp,
    /// Five-quarter-tones flat
    #[serde(rename = "accidentalFiveQuarterTonesFlatArrowDown")]
    AccidentalFiveQuarterTonesFlatArrowDown,
    /// Five-quarter-tones sharp
    #[serde(rename = "accidentalFiveQuarterTonesSharpArrowUp")]
    AccidentalFiveQuarterTonesSharpArrowUp,
    /// Flat
    #[serde(rename = "accidentalFlat")]
    AccidentalFlat,
    /// Arabic half-tone flat
    #[serde(rename = "accidentalFlatArabic")]
    AccidentalFlatArabic,
    /// Flat equal tempered semitone
    #[serde(rename = "accidentalFlatEqualTempered")]
    AccidentalFlatEqualTempered,
    /// Lowered flat (Stockhausen)
    #[serde(rename = "accidentalFlatLoweredStockhausen")]
    AccidentalFlatLoweredStockhausen,
    /// Flat lowered by one syntonic comma
    #[serde(rename = "accidentalFlatOneArrowDown")]
    AccidentalFlatOneArrowDown,
    /// Flat raised by one syntonic comma
    #[serde(rename = "accidentalFlatOneArrowUp")]
    AccidentalFlatOneArrowUp,
    /// Raised flat (Stockhausen)
    #[serde(rename = "accidentalFlatRaisedStockhausen")]
    AccidentalFlatRaisedStockhausen,
    /// Repeated flat, note on line (Stockhausen)
    #[serde(rename = "accidentalFlatRepeatedLineStockhausen")]
    AccidentalFlatRepeatedLineStockhausen,
    /// Repeated flat, note in space (Stockhausen)
    #[serde(rename = "accidentalFlatRepeatedSpaceStockhausen")]
    AccidentalFlatRepeatedSpaceStockhausen,
    /// Flat lowered by three syntonic commas
    #[serde(rename = "accidentalFlatThreeArrowsDown")]
    AccidentalFlatThreeArrowsDown,
    /// Flat raised by three syntonic commas
    #[serde(rename = "accidentalFlatThreeArrowsUp")]
    AccidentalFlatThreeArrowsUp,
    /// Turned flat
    #[serde(rename = "accidentalFlatTurned")]
    AccidentalFlatTurned,
    /// Flat lowered by two syntonic commas
    #[serde(rename = "accidentalFlatTwoArrowsDown")]
    AccidentalFlatTwoArrowsDown,
    /// Flat raised by two syntonic commas
    #[serde(rename = "accidentalFlatTwoArrowsUp")]
    AccidentalFlatTwoArrowsUp,
    /// Quarter-tone higher (Alois Hába)
    #[serde(rename = "accidentalHabaFlatQuarterToneHigher")]
    AccidentalHabaFlatQuarterToneHigher,
    /// Three quarter-tones lower (Alois Hába)
    #[serde(rename = "accidentalHabaFlatThreeQuarterTonesLower")]
    AccidentalHabaFlatThreeQuarterTonesLower,
    /// Quarter-tone higher (Alois Hába)
    #[serde(rename = "accidentalHabaQuarterToneHigher")]
    AccidentalHabaQuarterToneHigher,
    /// Quarter-tone lower (Alois Hába)
    #[serde(rename = "accidentalHabaQuarterToneLower")]
    AccidentalHabaQuarterToneLower,
    /// Quarter-tone lower (Alois Hába)
    #[serde(rename = "accidentalHabaSharpQuarterToneLower")]
    AccidentalHabaSharpQuarterToneLower,
    /// Three quarter-tones higher (Alois Hába)
    #[serde(rename = "accidentalHabaSharpThreeQuarterTonesHigher")]
    AccidentalHabaSharpThreeQuarterTonesHigher,
    /// Half sharp with arrow down
    #[serde(rename = "accidentalHalfSharpArrowDown")]
    AccidentalHalfSharpArrowDown,
    /// Half sharp with arrow up
    #[serde(rename = "accidentalHalfSharpArrowUp")]
    AccidentalHalfSharpArrowUp,
    /// Thirteen (raise by 65:64)
    #[serde(rename = "accidentalJohnston13")]
    AccidentalJohnston13,
    /// Inverted 13 (lower by 65:64)
    #[serde(rename = "accidentalJohnston31")]
    AccidentalJohnston31,
    /// Down arrow (lower by 33:32)
    #[serde(rename = "accidentalJohnstonDown")]
    AccidentalJohnstonDown,
    /// Inverted seven (raise by 36:35)
    #[serde(rename = "accidentalJohnstonEl")]
    AccidentalJohnstonEl,
    /// Minus (lower by 81:80)
    #[serde(rename = "accidentalJohnstonMinus")]
    AccidentalJohnstonMinus,
    /// Plus (raise by 81:80)
    #[serde(rename = "accidentalJohnstonPlus")]
    AccidentalJohnstonPlus,
    /// Seven (lower by 36:35)
    #[serde(rename = "accidentalJohnstonSeven")]
    AccidentalJohnstonSeven,
    /// Up arrow (raise by 33:32)
    #[serde(rename = "accidentalJohnstonUp")]
    AccidentalJohnstonUp,
    /// Koma (flat)
    #[serde(rename = "accidentalKomaFlat")]
    AccidentalKomaFlat,
    /// Koma (sharp)
    #[serde(rename = "accidentalKomaSharp")]
    AccidentalKomaSharp,
    /// Koron (quarter tone flat)
    #[serde(rename = "accidentalKoron")]
    AccidentalKoron,
    /// Küçük mücenneb (flat)
    #[serde(rename = "accidentalKucukMucennebFlat")]
    AccidentalKucukMucennebFlat,
    /// Küçük mücenneb (sharp)
    #[serde(rename = "accidentalKucukMucennebSharp")]
    AccidentalKucukMucennebSharp,
    /// Large double sharp
    #[serde(rename = "accidentalLargeDoubleSharp")]
    AccidentalLargeDoubleSharp,
    /// Lower by one septimal comma
    #[serde(rename = "accidentalLowerOneSeptimalComma")]
    AccidentalLowerOneSeptimalComma,
    /// Lower by one tridecimal quartertone
    #[serde(rename = "accidentalLowerOneTridecimalQuartertone")]
    AccidentalLowerOneTridecimalQuartertone,
    /// Lower by one undecimal quartertone
    #[serde(rename = "accidentalLowerOneUndecimalQuartertone")]
    AccidentalLowerOneUndecimalQuartertone,
    /// Lower by two septimal commas
    #[serde(rename = "accidentalLowerTwoSeptimalCommas")]
    AccidentalLowerTwoSeptimalCommas,
    /// Lowered (Stockhausen)
    #[serde(rename = "accidentalLoweredStockhausen")]
    AccidentalLoweredStockhausen,
    /// Narrow reversed flat(quarter-tone flat)
    #[serde(rename = "accidentalNarrowReversedFlat")]
    AccidentalNarrowReversedFlat,
    /// Narrow reversed flat and flat(three-quarter-tones flat)
    #[serde(rename = "accidentalNarrowReversedFlatAndFlat")]
    AccidentalNarrowReversedFlatAndFlat,
    /// Natural
    #[serde(rename = "accidentalNatural")]
    AccidentalNatural,
    /// Arabic natural
    #[serde(rename = "accidentalNaturalArabic")]
    AccidentalNaturalArabic,
    /// Natural equal tempered semitone
    #[serde(rename = "accidentalNaturalEqualTempered")]
    AccidentalNaturalEqualTempered,
    /// Natural flat
    #[serde(rename = "accidentalNaturalFlat")]
    AccidentalNaturalFlat,
    /// Lowered natural (Stockhausen)
    #[serde(rename = "accidentalNaturalLoweredStockhausen")]
    AccidentalNaturalLoweredStockhausen,
    /// Natural lowered by one syntonic comma
    #[serde(rename = "accidentalNaturalOneArrowDown")]
    AccidentalNaturalOneArrowDown,
    /// Natural raised by one syntonic comma
    #[serde(rename = "accidentalNaturalOneArrowUp")]
    AccidentalNaturalOneArrowUp,
    /// Raised natural (Stockhausen)
    #[serde(rename = "accidentalNaturalRaisedStockhausen")]
    AccidentalNaturalRaisedStockhausen,
    /// Reversed natural
    #[serde(rename = "accidentalNaturalReversed")]
    AccidentalNaturalReversed,
    /// Natural sharp
    #[serde(rename = "accidentalNaturalSharp")]
    AccidentalNaturalSharp,
    /// Natural lowered by three syntonic commas
    #[serde(rename = "accidentalNaturalThreeArrowsDown")]
    AccidentalNaturalThreeArrowsDown,
    /// Natural raised by three syntonic commas
    #[serde(rename = "accidentalNaturalThreeArrowsUp")]
    AccidentalNaturalThreeArrowsUp,
    /// Natural lowered by two syntonic commas
    #[serde(rename = "accidentalNaturalTwoArrowsDown")]
    AccidentalNaturalTwoArrowsDown,
    /// Natural raised by two syntonic commas
    #[serde(rename = "accidentalNaturalTwoArrowsUp")]
    AccidentalNaturalTwoArrowsUp,
    /// One and a half sharps with arrow down
    #[serde(rename = "accidentalOneAndAHalfSharpsArrowDown")]
    AccidentalOneAndAHalfSharpsArrowDown,
    /// One and a half sharps with arrow up
    #[serde(rename = "accidentalOneAndAHalfSharpsArrowUp")]
    AccidentalOneAndAHalfSharpsArrowUp,
    /// One-quarter-tone flat (Ferneyhough)
    #[serde(rename = "accidentalOneQuarterToneFlatFerneyhough")]
    AccidentalOneQuarterToneFlatFerneyhough,
    /// One-quarter-tone flat (Stockhausen)
    #[serde(rename = "accidentalOneQuarterToneFlatStockhausen")]
    AccidentalOneQuarterToneFlatStockhausen,
    /// One-quarter-tone sharp (Ferneyhough)
    #[serde(rename = "accidentalOneQuarterToneSharpFerneyhough")]
    AccidentalOneQuarterToneSharpFerneyhough,
    /// One-quarter-tone sharp (Stockhausen)
    #[serde(rename = "accidentalOneQuarterToneSharpStockhausen")]
    AccidentalOneQuarterToneSharpStockhausen,
    /// One-third-tone flat (Ferneyhough)
    #[serde(rename = "accidentalOneThirdToneFlatFerneyhough")]
    AccidentalOneThirdToneFlatFerneyhough,
    /// One-third-tone sharp (Ferneyhough)
    #[serde(rename = "accidentalOneThirdToneSharpFerneyhough")]
    AccidentalOneThirdToneSharpFerneyhough,
    /// Accidental parenthesis, left
    #[serde(rename = "accidentalParensLeft")]
    AccidentalParensLeft,
    /// Accidental parenthesis, right
    #[serde(rename = "accidentalParensRight")]
    AccidentalParensRight,
    /// Lower by one equal tempered quarter-tone
    #[serde(rename = "accidentalQuarterFlatEqualTempered")]
    AccidentalQuarterFlatEqualTempered,
    /// Raise by one equal tempered quarter tone
    #[serde(rename = "accidentalQuarterSharpEqualTempered")]
    AccidentalQuarterSharpEqualTempered,
    /// Quarter-tone flat
    #[serde(rename = "accidentalQuarterToneFlat4")]
    AccidentalQuarterToneFlat4,
    /// Arabic quarter-tone flat
    #[serde(rename = "accidentalQuarterToneFlatArabic")]
    AccidentalQuarterToneFlatArabic,
    /// Quarter-tone flat
    #[serde(rename = "accidentalQuarterToneFlatArrowUp")]
    AccidentalQuarterToneFlatArrowUp,
    /// Filled reversed flat (quarter-tone flat)
    #[serde(rename = "accidentalQuarterToneFlatFilledReversed")]
    AccidentalQuarterToneFlatFilledReversed,
    /// Quarter-tone flat
    #[serde(rename = "accidentalQuarterToneFlatNaturalArrowDown")]
    AccidentalQuarterToneFlatNaturalArrowDown,
    /// Quarter tone flat (Penderecki)
    #[serde(rename = "accidentalQuarterToneFlatPenderecki")]
    AccidentalQuarterToneFlatPenderecki,
    /// Reversed flat (quarter-tone flat) (Stein)
    #[serde(rename = "accidentalQuarterToneFlatStein")]
    AccidentalQuarterToneFlatStein,
    /// Quarter-tone flat (van Blankenburg)
    #[serde(rename = "accidentalQuarterToneFlatVanBlankenburg")]
    AccidentalQuarterToneFlatVanBlankenburg,
    /// Quarter-tone sharp
    #[serde(rename = "accidentalQuarterToneSharp4")]
    AccidentalQuarterToneSharp4,
    /// Arabic quarter-tone sharp
    #[serde(rename = "accidentalQuarterToneSharpArabic")]
    AccidentalQuarterToneSharpArabic,
    /// Quarter-tone sharp
    #[serde(rename = "accidentalQuarterToneSharpArrowDown")]
    AccidentalQuarterToneSharpArrowDown,
    /// Quarter tone sharp (Bussotti)
    #[serde(rename = "accidentalQuarterToneSharpBusotti")]
    AccidentalQuarterToneSharpBusotti,
    /// Quarter-tone sharp
    #[serde(rename = "accidentalQuarterToneSharpNaturalArrowUp")]
    AccidentalQuarterToneSharpNaturalArrowUp,
    /// Half sharp (quarter-tone sharp) (Stein)
    #[serde(rename = "accidentalQuarterToneSharpStein")]
    AccidentalQuarterToneSharpStein,
    /// Quarter tone sharp with wiggly tail
    #[serde(rename = "accidentalQuarterToneSharpWiggle")]
    AccidentalQuarterToneSharpWiggle,
    /// Raise by one septimal comma
    #[serde(rename = "accidentalRaiseOneSeptimalComma")]
    AccidentalRaiseOneSeptimalComma,
    /// Raise by one tridecimal quartertone
    #[serde(rename = "accidentalRaiseOneTridecimalQuartertone")]
    AccidentalRaiseOneTridecimalQuartertone,
    /// Raise by one undecimal quartertone
    #[serde(rename = "accidentalRaiseOneUndecimalQuartertone")]
    AccidentalRaiseOneUndecimalQuartertone,
    /// Raise by two septimal commas
    #[serde(rename = "accidentalRaiseTwoSeptimalCommas")]
    AccidentalRaiseTwoSeptimalCommas,
    /// Raised (Stockhausen)
    #[serde(rename = "accidentalRaisedStockhausen")]
    AccidentalRaisedStockhausen,
    /// Reversed flat and flat with arrow down
    #[serde(rename = "accidentalReversedFlatAndFlatArrowDown")]
    AccidentalReversedFlatAndFlatArrowDown,
    /// Reversed flat and flat with arrow up
    #[serde(rename = "accidentalReversedFlatAndFlatArrowUp")]
    AccidentalReversedFlatAndFlatArrowUp,
    /// Reversed flat with arrow down
    #[serde(rename = "accidentalReversedFlatArrowDown")]
    AccidentalReversedFlatArrowDown,
    /// Reversed flat with arrow up
    #[serde(rename = "accidentalReversedFlatArrowUp")]
    AccidentalReversedFlatArrowUp,
    /// Sharp
    #[serde(rename = "accidentalSharp")]
    AccidentalSharp,
    /// Arabic half-tone sharp
    #[serde(rename = "accidentalSharpArabic")]
    AccidentalSharpArabic,
    /// Sharp equal tempered semitone
    #[serde(rename = "accidentalSharpEqualTempered")]
    AccidentalSharpEqualTempered,
    /// Lowered sharp (Stockhausen)
    #[serde(rename = "accidentalSharpLoweredStockhausen")]
    AccidentalSharpLoweredStockhausen,
    /// Sharp lowered by one syntonic comma
    #[serde(rename = "accidentalSharpOneArrowDown")]
    AccidentalSharpOneArrowDown,
    /// Sharp raised by one syntonic comma
    #[serde(rename = "accidentalSharpOneArrowUp")]
    AccidentalSharpOneArrowUp,
    /// One or three quarter tones sharp
    #[serde(rename = "accidentalSharpOneHorizontalStroke")]
    AccidentalSharpOneHorizontalStroke,
    /// Raised sharp (Stockhausen)
    #[serde(rename = "accidentalSharpRaisedStockhausen")]
    AccidentalSharpRaisedStockhausen,
    /// Repeated sharp, note on line (Stockhausen)
    #[serde(rename = "accidentalSharpRepeatedLineStockhausen")]
    AccidentalSharpRepeatedLineStockhausen,
    /// Repeated sharp, note in space (Stockhausen)
    #[serde(rename = "accidentalSharpRepeatedSpaceStockhausen")]
    AccidentalSharpRepeatedSpaceStockhausen,
    /// Reversed sharp
    #[serde(rename = "accidentalSharpReversed")]
    AccidentalSharpReversed,
    /// Sharp sharp
    #[serde(rename = "accidentalSharpSharp")]
    AccidentalSharpSharp,
    /// Sharp lowered by three syntonic commas
    #[serde(rename = "accidentalSharpThreeArrowsDown")]
    AccidentalSharpThreeArrowsDown,
    /// Sharp raised by three syntonic commas
    #[serde(rename = "accidentalSharpThreeArrowsUp")]
    AccidentalSharpThreeArrowsUp,
    /// Sharp lowered by two syntonic commas
    #[serde(rename = "accidentalSharpTwoArrowsDown")]
    AccidentalSharpTwoArrowsDown,
    /// Sharp raised by two syntonic commas
    #[serde(rename = "accidentalSharpTwoArrowsUp")]
    AccidentalSharpTwoArrowsUp,
    /// 1/12 tone low
    #[serde(rename = "accidentalSims12Down")]
    AccidentalSims12Down,
    /// 1/12 tone high
    #[serde(rename = "accidentalSims12Up")]
    AccidentalSims12Up,
    /// 1/4 tone low
    #[serde(rename = "accidentalSims4Down")]
    AccidentalSims4Down,
    /// 1/4 tone high
    #[serde(rename = "accidentalSims4Up")]
    AccidentalSims4Up,
    /// 1/6 tone low
    #[serde(rename = "accidentalSims6Down")]
    AccidentalSims6Down,
    /// 1/6 tone high
    #[serde(rename = "accidentalSims6Up")]
    AccidentalSims6Up,
    /// Sori (quarter tone sharp)
    #[serde(rename = "accidentalSori")]
    AccidentalSori,
    /// Byzantine-style Bakiye flat (Tavener)
    #[serde(rename = "accidentalTavenerFlat")]
    AccidentalTavenerFlat,
    /// Byzantine-style Büyük mücenneb sharp (Tavener)
    #[serde(rename = "accidentalTavenerSharp")]
    AccidentalTavenerSharp,
    /// Arabic three-quarter-tones flat
    #[serde(rename = "accidentalThreeQuarterTonesFlatArabic")]
    AccidentalThreeQuarterTonesFlatArabic,
    /// Three-quarter-tones flat
    #[serde(rename = "accidentalThreeQuarterTonesFlatArrowDown")]
    AccidentalThreeQuarterTonesFlatArrowDown,
    /// Three-quarter-tones flat
    #[serde(rename = "accidentalThreeQuarterTonesFlatArrowUp")]
    AccidentalThreeQuarterTonesFlatArrowUp,
    /// Three-quarter-tones flat (Couper)
    #[serde(rename = "accidentalThreeQuarterTonesFlatCouper")]
    AccidentalThreeQuarterTonesFlatCouper,
    /// Three-quarter-tones flat (Grisey)
    #[serde(rename = "accidentalThreeQuarterTonesFlatGrisey")]
    AccidentalThreeQuarterTonesFlatGrisey,
    /// Three-quarter-tones flat (Tartini)
    #[serde(rename = "accidentalThreeQuarterTonesFlatTartini")]
    AccidentalThreeQuarterTonesFlatTartini,
    /// Reversed flat and flat (three-quarter-tones flat) (Zimmermann)
    #[serde(rename = "accidentalThreeQuarterTonesFlatZimmermann")]
    AccidentalThreeQuarterTonesFlatZimmermann,
    /// Arabic three-quarter-tones sharp
    #[serde(rename = "accidentalThreeQuarterTonesSharpArabic")]
    AccidentalThreeQuarterTonesSharpArabic,
    /// Three-quarter-tones sharp
    #[serde(rename = "accidentalThreeQuarterTonesSharpArrowDown")]
    AccidentalThreeQuarterTonesSharpArrowDown,
    /// Three-quarter-tones sharp
    #[serde(rename = "accidentalThreeQuarterTonesSharpArrowUp")]
    AccidentalThreeQuarterTonesSharpArrowUp,
    /// Three quarter tones sharp (Bussotti)
    #[serde(rename = "accidentalThreeQuarterTonesSharpBusotti")]
    AccidentalThreeQuarterTonesSharpBusotti,
    /// One and a half sharps (three-quarter-tones sharp) (Stein)
    #[serde(rename = "accidentalThreeQuarterTonesSharpStein")]
    AccidentalThreeQuarterTonesSharpStein,
    /// Three-quarter-tones sharp (Stockhausen)
    #[serde(rename = "accidentalThreeQuarterTonesSharpStockhausen")]
    AccidentalThreeQuarterTonesSharpStockhausen,
    /// Triple flat
    #[serde(rename = "accidentalTripleFlat")]
    AccidentalTripleFlat,
    /// Triple sharp
    #[serde(rename = "accidentalTripleSharp")]
    AccidentalTripleSharp,
    /// Two-third-tones flat (Ferneyhough)
    #[serde(rename = "accidentalTwoThirdTonesFlatFerneyhough")]
    AccidentalTwoThirdTonesFlatFerneyhough,
    /// Two-third-tones sharp (Ferneyhough)
    #[serde(rename = "accidentalTwoThirdTonesSharpFerneyhough")]
    AccidentalTwoThirdTonesSharpFerneyhough,
    /// Accidental down
    #[serde(rename = "accidentalUpsAndDownsDown")]
    AccidentalUpsAndDownsDown,
    /// Accidental less
    #[serde(rename = "accidentalUpsAndDownsLess")]
    AccidentalUpsAndDownsLess,
    /// Accidental more
    #[serde(rename = "accidentalUpsAndDownsMore")]
    AccidentalUpsAndDownsMore,
    /// Accidental up
    #[serde(rename = "accidentalUpsAndDownsUp")]
    AccidentalUpsAndDownsUp,
    /// Wilson minus (5 comma down)
    #[serde(rename = "accidentalWilsonMinus")]
    AccidentalWilsonMinus,
    /// Wilson plus (5 comma up)
    #[serde(rename = "accidentalWilsonPlus")]
    AccidentalWilsonPlus,
    /// 5/6 tone flat
    #[serde(rename = "accidentalWyschnegradsky10TwelfthsFlat")]
    AccidentalWyschnegradsky10TwelfthsFlat,
    /// 5/6 tone sharp
    #[serde(rename = "accidentalWyschnegradsky10TwelfthsSharp")]
    AccidentalWyschnegradsky10TwelfthsSharp,
    /// 11/12 tone flat
    #[serde(rename = "accidentalWyschnegradsky11TwelfthsFlat")]
    AccidentalWyschnegradsky11TwelfthsFlat,
    /// 11/12 tone sharp
    #[serde(rename = "accidentalWyschnegradsky11TwelfthsSharp")]
    AccidentalWyschnegradsky11TwelfthsSharp,
    /// 1/12 tone flat
    #[serde(rename = "accidentalWyschnegradsky1TwelfthsFlat")]
    AccidentalWyschnegradsky1TwelfthsFlat,
    /// 1/12 tone sharp
    #[serde(rename = "accidentalWyschnegradsky1TwelfthsSharp")]
    AccidentalWyschnegradsky1TwelfthsSharp,
    /// 1/6 tone flat
    #[serde(rename = "accidentalWyschnegradsky2TwelfthsFlat")]
    AccidentalWyschnegradsky2TwelfthsFlat,
    /// 1/6 tone sharp
    #[serde(rename = "accidentalWyschnegradsky2TwelfthsSharp")]
    AccidentalWyschnegradsky2TwelfthsSharp,
    /// 1/4 tone flat
    #[serde(rename = "accidentalWyschnegradsky3TwelfthsFlat")]
    AccidentalWyschnegradsky3TwelfthsFlat,
    /// 1/4 tone sharp
    #[serde(rename = "accidentalWyschnegradsky3TwelfthsSharp")]
    AccidentalWyschnegradsky3TwelfthsSharp,
    /// 1/3 tone flat
    #[serde(rename = "accidentalWyschnegradsky4TwelfthsFlat")]
    AccidentalWyschnegradsky4TwelfthsFlat,
    /// 1/3 tone sharp
    #[serde(rename = "accidentalWyschnegradsky4TwelfthsSharp")]
    AccidentalWyschnegradsky4TwelfthsSharp,
    /// 5/12 tone flat
    #[serde(rename = "accidentalWyschnegradsky5TwelfthsFlat")]
    AccidentalWyschnegradsky5TwelfthsFlat,
    /// 5/12 tone sharp
    #[serde(rename = "accidentalWyschnegradsky5TwelfthsSharp")]
    AccidentalWyschnegradsky5TwelfthsSharp,
    /// 1/2 tone flat
    #[serde(rename = "accidentalWyschnegradsky6TwelfthsFlat")]
    AccidentalWyschnegradsky6TwelfthsFlat,
    /// 1/2 tone sharp
    #[serde(rename = "accidentalWyschnegradsky6TwelfthsSharp")]
    AccidentalWyschnegradsky6TwelfthsSharp,
    /// 7/12 tone flat
    #[serde(rename = "accidentalWyschnegradsky7TwelfthsFlat")]
    AccidentalWyschnegradsky7TwelfthsFlat,
    /// 7/12 tone sharp
    #[serde(rename = "accidentalWyschnegradsky7TwelfthsSharp")]
    AccidentalWyschnegradsky7TwelfthsSharp,
    /// 2/3 tone flat
    #[serde(rename = "accidentalWyschnegradsky8TwelfthsFlat")]
    AccidentalWyschnegradsky8TwelfthsFlat,
    /// 2/3 tone sharp
    #[serde(rename = "accidentalWyschnegradsky8TwelfthsSharp")]
    AccidentalWyschnegradsky8TwelfthsSharp,
    /// 3/4 tone flat
    #[serde(rename = "accidentalWyschnegradsky9TwelfthsFlat")]
    AccidentalWyschnegradsky9TwelfthsFlat,
    /// 3/4 tone sharp
    #[serde(rename = "accidentalWyschnegradsky9TwelfthsSharp")]
    AccidentalWyschnegradsky9TwelfthsSharp,
    /// One-third-tone sharp (Xenakis)
    #[serde(rename = "accidentalXenakisOneThirdToneSharp")]
    AccidentalXenakisOneThirdToneSharp,
    /// Two-third-tones sharp (Xenakis)
    #[serde(rename = "accidentalXenakisTwoThirdTonesSharp")]
    AccidentalXenakisTwoThirdTonesSharp,
    /// Choralmelodie (Berg)
    #[serde(rename = "analyticsChoralmelodie")]
    AnalyticsChoralmelodie,
    /// End of stimme
    #[serde(rename = "analyticsEndStimme")]
    AnalyticsEndStimme,
    /// Hauptrhythmus (Berg)
    #[serde(rename = "analyticsHauptrhythmus")]
    AnalyticsHauptrhythmus,
    /// Hauptstimme
    #[serde(rename = "analyticsHauptstimme")]
    AnalyticsHauptstimme,
    /// Inversion 1
    #[serde(rename = "analyticsInversion1")]
    AnalyticsInversion1,
    /// Nebenstimme
    #[serde(rename = "analyticsNebenstimme")]
    AnalyticsNebenstimme,
    /// Start of stimme
    #[serde(rename = "analyticsStartStimme")]
    AnalyticsStartStimme,
    /// Theme
    #[serde(rename = "analyticsTheme")]
    AnalyticsTheme,
    /// Theme 1
    #[serde(rename = "analyticsTheme1")]
    AnalyticsTheme1,
    /// Inversion of theme
    #[serde(rename = "analyticsThemeInversion")]
    AnalyticsThemeInversion,
    /// Retrograde of theme
    #[serde(rename = "analyticsThemeRetrograde")]
    AnalyticsThemeRetrograde,
    /// Retrograde inversion of theme
    #[serde(rename = "analyticsThemeRetrogradeInversion")]
    AnalyticsThemeRetrogradeInversion,
    /// Arpeggiato
    #[serde(rename = "arpeggiato")]
    Arpeggiato,
    /// Arpeggiato down
    #[serde(rename = "arpeggiatoDown")]
    ArpeggiatoDown,
    /// Arpeggiato up
    #[serde(rename = "arpeggiatoUp")]
    ArpeggiatoUp,
    /// Black arrow down (S)
    #[serde(rename = "arrowBlackDown")]
    ArrowBlackDown,
    /// Black arrow down-left (SW)
    #[serde(rename = "arrowBlackDownLeft")]
    ArrowBlackDownLeft,
    /// Black arrow down-right (SE)
    #[serde(rename = "arrowBlackDownRight")]
    ArrowBlackDownRight,
    /// Black arrow left (W)
    #[serde(rename = "arrowBlackLeft")]
    ArrowBlackLeft,
    /// Black arrow right (E)
    #[serde(rename = "arrowBlackRight")]
    ArrowBlackRight,
    /// Black arrow up (N)
    #[serde(rename = "arrowBlackUp")]
    ArrowBlackUp,
    /// Black arrow up-left (NW)
    #[serde(rename = "arrowBlackUpLeft")]
    ArrowBlackUpLeft,
    /// Black arrow up-right (NE)
    #[serde(rename = "arrowBlackUpRight")]
    ArrowBlackUpRight,
    /// Open arrow down (S)
    #[serde(rename = "arrowOpenDown")]
    ArrowOpenDown,
    /// Open arrow down-left (SW)
    #[serde(rename = "arrowOpenDownLeft")]
    ArrowOpenDownLeft,
    /// Open arrow down-right (SE)
    #[serde(rename = "arrowOpenDownRight")]
    ArrowOpenDownRight,
    /// Open arrow left (W)
    #[serde(rename = "arrowOpenLeft")]
    ArrowOpenLeft,
    /// Open arrow right (E)
    #[serde(rename = "arrowOpenRight")]
    ArrowOpenRight,
    /// Open arrow up (N)
    #[serde(rename = "arrowOpenUp")]
    ArrowOpenUp,
    /// Open arrow up-left (NW)
    #[serde(rename = "arrowOpenUpLeft")]
    ArrowOpenUpLeft,
    /// Open arrow up-right (NE)
    #[serde(rename = "arrowOpenUpRight")]
    ArrowOpenUpRight,
    /// White arrow down (S)
    #[serde(rename = "arrowWhiteDown")]
    ArrowWhiteDown,
    /// White arrow down-left (SW)
    #[serde(rename = "arrowWhiteDownLeft")]
    ArrowWhiteDownLeft,
    /// White arrow down-right (SE)
    #[serde(rename = "arrowWhiteDownRight")]
    ArrowWhiteDownRight,
    /// White arrow left (W)
    #[serde(rename = "arrowWhiteLeft")]
    ArrowWhiteLeft,
    /// White arrow right (E)
    #[serde(rename = "arrowWhiteRight")]
    ArrowWhiteRight,
    /// White arrow up (N)
    #[serde(rename = "arrowWhiteUp")]
    ArrowWhiteUp,
    /// White arrow up-left (NW)
    #[serde(rename = "arrowWhiteUpLeft")]
    ArrowWhiteUpLeft,
    /// White arrow up-right (NE)
    #[serde(rename = "arrowWhiteUpRight")]
    ArrowWhiteUpRight,
    /// Black arrowhead down (S)
    #[serde(rename = "arrowheadBlackDown")]
    ArrowheadBlackDown,
    /// Black arrowhead down-left (SW)
    #[serde(rename = "arrowheadBlackDownLeft")]
    ArrowheadBlackDownLeft,
    /// Black arrowhead down-right (SE)
    #[serde(rename = "arrowheadBlackDownRight")]
    ArrowheadBlackDownRight,
    /// Black arrowhead left (W)
    #[serde(rename = "arrowheadBlackLeft")]
    ArrowheadBlackLeft,
    /// Black arrowhead right (E)
    #[serde(rename = "arrowheadBlackRight")]
    ArrowheadBlackRight,
    /// Black arrowhead up (N)
    #[serde(rename = "arrowheadBlackUp")]
    ArrowheadBlackUp,
    /// Black arrowhead up-left (NW)
    #[serde(rename = "arrowheadBlackUpLeft")]
    ArrowheadBlackUpLeft,
    /// Black arrowhead up-right (NE)
    #[serde(rename = "arrowheadBlackUpRight")]
    ArrowheadBlackUpRight,
    /// Open arrowhead down (S)
    #[serde(rename = "arrowheadOpenDown")]
    ArrowheadOpenDown,
    /// Open arrowhead down-left (SW)
    #[serde(rename = "arrowheadOpenDownLeft")]
    ArrowheadOpenDownLeft,
    /// Open arrowhead down-right (SE)
    #[serde(rename = "arrowheadOpenDownRight")]
    ArrowheadOpenDownRight,
    /// Open arrowhead left (W)
    #[serde(rename = "arrowheadOpenLeft")]
    ArrowheadOpenLeft,
    /// Open arrowhead right (E)
    #[serde(rename = "arrowheadOpenRight")]
    ArrowheadOpenRight,
    /// Open arrowhead up (N)
    #[serde(rename = "arrowheadOpenUp")]
    ArrowheadOpenUp,
    /// Open arrowhead up-left (NW)
    #[serde(rename = "arrowheadOpenUpLeft")]
    ArrowheadOpenUpLeft,
    /// Open arrowhead up-right (NE)
    #[serde(rename = "arrowheadOpenUpRight")]
    ArrowheadOpenUpRight,
    /// White arrowhead down (S)
    #[serde(rename = "arrowheadWhiteDown")]
    ArrowheadWhiteDown,
    /// White arrowhead down-left (SW)
    #[serde(rename = "arrowheadWhiteDownLeft")]
    ArrowheadWhiteDownLeft,
    /// White arrowhead down-right (SE)
    #[serde(rename = "arrowheadWhiteDownRight")]
    ArrowheadWhiteDownRight,
    /// White arrowhead left (W)
    #[serde(rename = "arrowheadWhiteLeft")]
    ArrowheadWhiteLeft,
    /// White arrowhead right (E)
    #[serde(rename = "arrowheadWhiteRight")]
    ArrowheadWhiteRight,
    /// White arrowhead up (N)
    #[serde(rename = "arrowheadWhiteUp")]
    ArrowheadWhiteUp,
    /// White arrowhead up-left (NW)
    #[serde(rename = "arrowheadWhiteUpLeft")]
    ArrowheadWhiteUpLeft,
    /// White arrowhead up-right (NE)
    #[serde(rename = "arrowheadWhiteUpRight")]
    ArrowheadWhiteUpRight,
    /// Accent above
    #[serde(rename = "articAccentAbove")]
    ArticAccentAbove,
    /// Accent below
    #[serde(rename = "articAccentBelow")]
    ArticAccentBelow,
    /// Accent-staccato above
    #[serde(rename = "articAccentStaccatoAbove")]
    ArticAccentStaccatoAbove,
    /// Accent-staccato below
    #[serde(rename = "articAccentStaccatoBelow")]
    ArticAccentStaccatoBelow,
    /// Laissez vibrer (l.v.) above
    #[serde(rename = "articLaissezVibrerAbove")]
    ArticLaissezVibrerAbove,
    /// Laissez vibrer (l.v.) below
    #[serde(rename = "articLaissezVibrerBelow")]
    ArticLaissezVibrerBelow,
    /// Marcato above
    #[serde(rename = "articMarcatoAbove")]
    ArticMarcatoAbove,
    /// Marcato below
    #[serde(rename = "articMarcatoBelow")]
    ArticMarcatoBelow,
    /// Marcato-staccato above
    #[serde(rename = "articMarcatoStaccatoAbove")]
    ArticMarcatoStaccatoAbove,
    /// Marcato-staccato below
    #[serde(rename = "articMarcatoStaccatoBelow")]
    ArticMarcatoStaccatoBelow,
    /// Marcato-tenuto above
    #[serde(rename = "articMarcatoTenutoAbove")]
    ArticMarcatoTenutoAbove,
    /// Marcato-tenuto below
    #[serde(rename = "articMarcatoTenutoBelow")]
    ArticMarcatoTenutoBelow,
    /// Soft accent above
    #[serde(rename = "articSoftAccentAbove")]
    ArticSoftAccentAbove,
    /// Soft accent below
    #[serde(rename = "articSoftAccentBelow")]
    ArticSoftAccentBelow,
    /// Soft accent-staccato above
    #[serde(rename = "articSoftAccentStaccatoAbove")]
    ArticSoftAccentStaccatoAbove,
    /// Soft accent-staccato below
    #[serde(rename = "articSoftAccentStaccatoBelow")]
    ArticSoftAccentStaccatoBelow,
    /// Soft accent-tenuto above
    #[serde(rename = "articSoftAccentTenutoAbove")]
    ArticSoftAccentTenutoAbove,
    /// Soft accent-tenuto below
    #[serde(rename = "articSoftAccentTenutoBelow")]
    ArticSoftAccentTenutoBelow,
    /// Soft accent-tenuto-staccato above
    #[serde(rename = "articSoftAccentTenutoStaccatoAbove")]
    ArticSoftAccentTenutoStaccatoAbove,
    /// Soft accent-tenuto-staccato below
    #[serde(rename = "articSoftAccentTenutoStaccatoBelow")]
    ArticSoftAccentTenutoStaccatoBelow,
    /// Staccatissimo above
    #[serde(rename = "articStaccatissimoAbove")]
    ArticStaccatissimoAbove,
    /// Staccatissimo below
    #[serde(rename = "articStaccatissimoBelow")]
    ArticStaccatissimoBelow,
    /// Staccatissimo stroke above
    #[serde(rename = "articStaccatissimoStrokeAbove")]
    ArticStaccatissimoStrokeAbove,
    /// Staccatissimo stroke below
    #[serde(rename = "articStaccatissimoStrokeBelow")]
    ArticStaccatissimoStrokeBelow,
    /// Staccatissimo wedge above
    #[serde(rename = "articStaccatissimoWedgeAbove")]
    ArticStaccatissimoWedgeAbove,
    /// Staccatissimo wedge below
    #[serde(rename = "articStaccatissimoWedgeBelow")]
    ArticStaccatissimoWedgeBelow,
    /// Staccato above
    #[serde(rename = "articStaccatoAbove")]
    ArticStaccatoAbove,
    /// Staccato below
    #[serde(rename = "articStaccatoBelow")]
    ArticStaccatoBelow,
    /// Stress above
    #[serde(rename = "articStressAbove")]
    ArticStressAbove,
    /// Stress below
    #[serde(rename = "articStressBelow")]
    ArticStressBelow,
    /// Tenuto above
    #[serde(rename = "articTenutoAbove")]
    ArticTenutoAbove,
    /// Tenuto-accent above
    #[serde(rename = "articTenutoAccentAbove")]
    ArticTenutoAccentAbove,
    /// Tenuto-accent below
    #[serde(rename = "articTenutoAccentBelow")]
    ArticTenutoAccentBelow,
    /// Tenuto below
    #[serde(rename = "articTenutoBelow")]
    ArticTenutoBelow,
    /// Louré (tenuto-staccato) above
    #[serde(rename = "articTenutoStaccatoAbove")]
    ArticTenutoStaccatoAbove,
    /// Louré (tenuto-staccato) below
    #[serde(rename = "articTenutoStaccatoBelow")]
    ArticTenutoStaccatoBelow,
    /// Unstress above
    #[serde(rename = "articUnstressAbove")]
    ArticUnstressAbove,
    /// Unstress below
    #[serde(rename = "articUnstressBelow")]
    ArticUnstressBelow,
    /// Augmentation dot
    #[serde(rename = "augmentationDot")]
    AugmentationDot,
    /// Dashed barline
    #[serde(rename = "barlineDashed")]
    BarlineDashed,
    /// Dotted barline
    #[serde(rename = "barlineDotted")]
    BarlineDotted,
    /// Double barline
    #[serde(rename = "barlineDouble")]
    BarlineDouble,
    /// Final barline
    #[serde(rename = "barlineFinal")]
    BarlineFinal,
    /// Heavy barline
    #[serde(rename = "barlineHeavy")]
    BarlineHeavy,
    /// Heavy double barline
    #[serde(rename = "barlineHeavyHeavy")]
    BarlineHeavyHeavy,
    /// Reverse final barline
    #[serde(rename = "barlineReverseFinal")]
    BarlineReverseFinal,
    /// Short barline
    #[serde(rename = "barlineShort")]
    BarlineShort,
    /// Single barline
    #[serde(rename = "barlineSingle")]
    BarlineSingle,
    /// Tick barline
    #[serde(rename = "barlineTick")]
    BarlineTick,
    /// Accel./rit. beam 1 (widest)
    #[serde(rename = "beamAccelRit1")]
    BeamAccelRit1,
    /// Accel./rit. beam 10
    #[serde(rename = "beamAccelRit10")]
    BeamAccelRit10,
    /// Accel./rit. beam 11
    #[serde(rename = "beamAccelRit11")]
    BeamAccelRit11,
    /// Accel./rit. beam 12
    #[serde(rename = "beamAccelRit12")]
    BeamAccelRit12,
    /// Accel./rit. beam 13
    #[serde(rename = "beamAccelRit13")]
    BeamAccelRit13,
    /// Accel./rit. beam 14
    #[serde(rename = "beamAccelRit14")]
    BeamAccelRit14,
    /// Accel./rit. beam 15 (narrowest)
    #[serde(rename = "beamAccelRit15")]
    BeamAccelRit15,
    /// Accel./rit. beam 2
    #[serde(rename = "beamAccelRit2")]
    BeamAccelRit2,
    /// Accel./rit. beam 3
    #[serde(rename = "beamAccelRit3")]
    BeamAccelRit3,
    /// Accel./rit. beam 4
    #[serde(rename = "beamAccelRit4")]
    BeamAccelRit4,
    /// Accel./rit. beam 5
    #[serde(rename = "beamAccelRit5")]
    BeamAccelRit5,
    /// Accel./rit. beam 6
    #[serde(rename = "beamAccelRit6")]
    BeamAccelRit6,
    /// Accel./rit. beam 7
    #[serde(rename = "beamAccelRit7")]
    BeamAccelRit7,
    /// Accel./rit. beam 8
    #[serde(rename = "beamAccelRit8")]
    BeamAccelRit8,
    /// Accel./rit. beam 9
    #[serde(rename = "beamAccelRit9")]
    BeamAccelRit9,
    /// Accel./rit. beam terminating line
    #[serde(rename = "beamAccelRitFinal")]
    BeamAccelRitFinal,
    /// Brace
    #[serde(rename = "brace")]
    Brace,
    /// Bracket
    #[serde(rename = "bracket")]
    Bracket,
    /// Bracket bottom
    #[serde(rename = "bracketBottom")]
    BracketBottom,
    /// Bracket top
    #[serde(rename = "bracketTop")]
    BracketTop,
    /// Bend
    #[serde(rename = "brassBend")]
    BrassBend,
    /// Doit, long
    #[serde(rename = "brassDoitLong")]
    BrassDoitLong,
    /// Doit, medium
    #[serde(rename = "brassDoitMedium")]
    BrassDoitMedium,
    /// Doit, short
    #[serde(rename = "brassDoitShort")]
    BrassDoitShort,
    /// Lip fall, long
    #[serde(rename = "brassFallLipLong")]
    BrassFallLipLong,
    /// Lip fall, medium
    #[serde(rename = "brassFallLipMedium")]
    BrassFallLipMedium,
    /// Lip fall, short
    #[serde(rename = "brassFallLipShort")]
    BrassFallLipShort,
    /// Rough fall, long
    #[serde(rename = "brassFallRoughLong")]
    BrassFallRoughLong,
    /// Rough fall, medium
    #[serde(rename = "brassFallRoughMedium")]
    BrassFallRoughMedium,
    /// Rough fall, short
    #[serde(rename = "brassFallRoughShort")]
    BrassFallRoughShort,
    /// Smooth fall, long
    #[serde(rename = "brassFallSmoothLong")]
    BrassFallSmoothLong,
    /// Smooth fall, medium
    #[serde(rename = "brassFallSmoothMedium")]
    BrassFallSmoothMedium,
    /// Smooth fall, short
    #[serde(rename = "brassFallSmoothShort")]
    BrassFallSmoothShort,
    /// Flip
    #[serde(rename = "brassFlip")]
    BrassFlip,
    /// Harmon mute, stem in
    #[serde(rename = "brassHarmonMuteClosed")]
    BrassHarmonMuteClosed,
    /// Harmon mute, stem extended, left
    #[serde(rename = "brassHarmonMuteStemHalfLeft")]
    BrassHarmonMuteStemHalfLeft,
    /// Harmon mute, stem extended, right
    #[serde(rename = "brassHarmonMuteStemHalfRight")]
    BrassHarmonMuteStemHalfRight,
    /// Harmon mute, stem out
    #[serde(rename = "brassHarmonMuteStemOpen")]
    BrassHarmonMuteStemOpen,
    /// Jazz turn
    #[serde(rename = "brassJazzTurn")]
    BrassJazzTurn,
    /// Lift, long
    #[serde(rename = "brassLiftLong")]
    BrassLiftLong,
    /// Lift, medium
    #[serde(rename = "brassLiftMedium")]
    BrassLiftMedium,
    /// Lift, short
    #[serde(rename = "brassLiftShort")]
    BrassLiftShort,
    /// Smooth lift, long
    #[serde(rename = "brassLiftSmoothLong")]
    BrassLiftSmoothLong,
    /// Smooth lift, medium
    #[serde(rename = "brassLiftSmoothMedium")]
    BrassLiftSmoothMedium,
    /// Smooth lift, short
    #[serde(rename = "brassLiftSmoothShort")]
    BrassLiftSmoothShort,
    /// Muted (closed)
    #[serde(rename = "brassMuteClosed")]
    BrassMuteClosed,
    /// Half-muted (half-closed)
    #[serde(rename = "brassMuteHalfClosed")]
    BrassMuteHalfClosed,
    /// Open
    #[serde(rename = "brassMuteOpen")]
    BrassMuteOpen,
    /// Plop
    #[serde(rename = "brassPlop")]
    BrassPlop,
    /// Scoop
    #[serde(rename = "brassScoop")]
    BrassScoop,
    /// Smear
    #[serde(rename = "brassSmear")]
    BrassSmear,
    /// Valve trill
    #[serde(rename = "brassValveTrill")]
    BrassValveTrill,
    /// Breath mark (comma)
    #[serde(rename = "breathMarkComma")]
    BreathMarkComma,
    /// Breath mark (Salzedo)
    #[serde(rename = "breathMarkSalzedo")]
    BreathMarkSalzedo,
    /// Breath mark (tick-like)
    #[serde(rename = "breathMarkTick")]
    BreathMarkTick,
    /// Breath mark (upbow-like)
    #[serde(rename = "breathMarkUpbow")]
    BreathMarkUpbow,
    /// Bridge clef
    #[serde(rename = "bridgeClef")]
    BridgeClef,
    /// Buzz roll
    #[serde(rename = "buzzRoll")]
    BuzzRoll,
    /// C clef
    #[serde(rename = "cClef")]
    CClef,
    /// C clef ottava bassa
    #[serde(rename = "cClef8vb")]
    CClef8Vb,
    /// C clef, arrow down
    #[serde(rename = "cClefArrowDown")]
    CClefArrowDown,
    /// C clef, arrow up
    #[serde(rename = "cClefArrowUp")]
    CClefArrowUp,
    /// C clef change
    #[serde(rename = "cClefChange")]
    CClefChange,
    /// Combining C clef
    #[serde(rename = "cClefCombining")]
    CClefCombining,
    /// Reversed C clef
    #[serde(rename = "cClefReversed")]
    CClefReversed,
    /// C clef (19th century)
    #[serde(rename = "cClefSquare")]
    CClefSquare,
    /// Caesura
    #[serde(rename = "caesura")]
    Caesura,
    /// Curved caesura
    #[serde(rename = "caesuraCurved")]
    CaesuraCurved,
    /// Short caesura
    #[serde(rename = "caesuraShort")]
    CaesuraShort,
    /// Single stroke caesura
    #[serde(rename = "caesuraSingleStroke")]
    CaesuraSingleStroke,
    /// Thick caesura
    #[serde(rename = "caesuraThick")]
    CaesuraThick,
    /// Accentus above
    #[serde(rename = "chantAccentusAbove")]
    ChantAccentusAbove,
    /// Accentus below
    #[serde(rename = "chantAccentusBelow")]
    ChantAccentusBelow,
    /// Punctum auctum, ascending
    #[serde(rename = "chantAuctumAsc")]
    ChantAuctumAsc,
    /// Punctum auctum, descending
    #[serde(rename = "chantAuctumDesc")]
    ChantAuctumDesc,
    /// Augmentum (mora)
    #[serde(rename = "chantAugmentum")]
    ChantAugmentum,
    /// Caesura
    #[serde(rename = "chantCaesura")]
    ChantCaesura,
    /// Plainchant C clef
    #[serde(rename = "chantCclef")]
    ChantCclef,
    /// Circulus above
    #[serde(rename = "chantCirculusAbove")]
    ChantCirculusAbove,
    /// Circulus below
    #[serde(rename = "chantCirculusBelow")]
    ChantCirculusBelow,
    /// Connecting line, ascending 2nd
    #[serde(rename = "chantConnectingLineAsc2nd")]
    ChantConnectingLineAsc2Nd,
    /// Connecting line, ascending 3rd
    #[serde(rename = "chantConnectingLineAsc3rd")]
    ChantConnectingLineAsc3Rd,
    /// Connecting line, ascending 4th
    #[serde(rename = "chantConnectingLineAsc4th")]
    ChantConnectingLineAsc4Th,
    /// Connecting line, ascending 5th
    #[serde(rename = "chantConnectingLineAsc5th")]
    ChantConnectingLineAsc5Th,
    /// Connecting line, ascending 6th
    #[serde(rename = "chantConnectingLineAsc6th")]
    ChantConnectingLineAsc6Th,
    /// Plainchant custos, stem down, high position
    #[serde(rename = "chantCustosStemDownPosHigh")]
    ChantCustosStemDownPosHigh,
    /// Plainchant custos, stem down, highest position
    #[serde(rename = "chantCustosStemDownPosHighest")]
    ChantCustosStemDownPosHighest,
    /// Plainchant custos, stem down, middle position
    #[serde(rename = "chantCustosStemDownPosMiddle")]
    ChantCustosStemDownPosMiddle,
    /// Plainchant custos, stem up, low position
    #[serde(rename = "chantCustosStemUpPosLow")]
    ChantCustosStemUpPosLow,
    /// Plainchant custos, stem up, lowest position
    #[serde(rename = "chantCustosStemUpPosLowest")]
    ChantCustosStemUpPosLowest,
    /// Plainchant custos, stem up, middle position
    #[serde(rename = "chantCustosStemUpPosMiddle")]
    ChantCustosStemUpPosMiddle,
    /// Punctum deminutum, lower
    #[serde(rename = "chantDeminutumLower")]
    ChantDeminutumLower,
    /// Punctum deminutum, upper
    #[serde(rename = "chantDeminutumUpper")]
    ChantDeminutumUpper,
    /// Divisio finalis
    #[serde(rename = "chantDivisioFinalis")]
    ChantDivisioFinalis,
    /// Divisio maior
    #[serde(rename = "chantDivisioMaior")]
    ChantDivisioMaior,
    /// Divisio maxima
    #[serde(rename = "chantDivisioMaxima")]
    ChantDivisioMaxima,
    /// Divisio minima
    #[serde(rename = "chantDivisioMinima")]
    ChantDivisioMinima,
    /// Entry line, ascending 2nd
    #[serde(rename = "chantEntryLineAsc2nd")]
    ChantEntryLineAsc2Nd,
    /// Entry line, ascending 3rd
    #[serde(rename = "chantEntryLineAsc3rd")]
    ChantEntryLineAsc3Rd,
    /// Entry line, ascending 4th
    #[serde(rename = "chantEntryLineAsc4th")]
    ChantEntryLineAsc4Th,
    /// Entry line, ascending 5th
    #[serde(rename = "chantEntryLineAsc5th")]
    ChantEntryLineAsc5Th,
    /// Entry line, ascending 6th
    #[serde(rename = "chantEntryLineAsc6th")]
    ChantEntryLineAsc6Th,
    /// Episema
    #[serde(rename = "chantEpisema")]
    ChantEpisema,
    /// Plainchant F clef
    #[serde(rename = "chantFclef")]
    ChantFclef,
    /// Ictus above
    #[serde(rename = "chantIctusAbove")]
    ChantIctusAbove,
    /// Ictus below
    #[serde(rename = "chantIctusBelow")]
    ChantIctusBelow,
    /// Ligated stroke, descending 2nd
    #[serde(rename = "chantLigaturaDesc2nd")]
    ChantLigaturaDesc2Nd,
    /// Ligated stroke, descending 3rd
    #[serde(rename = "chantLigaturaDesc3rd")]
    ChantLigaturaDesc3Rd,
    /// Ligated stroke, descending 4th
    #[serde(rename = "chantLigaturaDesc4th")]
    ChantLigaturaDesc4Th,
    /// Ligated stroke, descending 5th
    #[serde(rename = "chantLigaturaDesc5th")]
    ChantLigaturaDesc5Th,
    /// Oriscus ascending
    #[serde(rename = "chantOriscusAscending")]
    ChantOriscusAscending,
    /// Oriscus descending
    #[serde(rename = "chantOriscusDescending")]
    ChantOriscusDescending,
    /// Oriscus liquescens
    #[serde(rename = "chantOriscusLiquescens")]
    ChantOriscusLiquescens,
    /// Podatus, lower
    #[serde(rename = "chantPodatusLower")]
    ChantPodatusLower,
    /// Podatus, upper
    #[serde(rename = "chantPodatusUpper")]
    ChantPodatusUpper,
    /// Punctum
    #[serde(rename = "chantPunctum")]
    ChantPunctum,
    /// Punctum cavum
    #[serde(rename = "chantPunctumCavum")]
    ChantPunctumCavum,
    /// Punctum deminutum
    #[serde(rename = "chantPunctumDeminutum")]
    ChantPunctumDeminutum,
    /// Punctum inclinatum
    #[serde(rename = "chantPunctumInclinatum")]
    ChantPunctumInclinatum,
    /// Punctum inclinatum auctum
    #[serde(rename = "chantPunctumInclinatumAuctum")]
    ChantPunctumInclinatumAuctum,
    /// Punctum inclinatum deminutum
    #[serde(rename = "chantPunctumInclinatumDeminutum")]
    ChantPunctumInclinatumDeminutum,
    /// Punctum linea
    #[serde(rename = "chantPunctumLinea")]
    ChantPunctumLinea,
    /// Punctum linea cavum
    #[serde(rename = "chantPunctumLineaCavum")]
    ChantPunctumLineaCavum,
    /// Punctum virga
    #[serde(rename = "chantPunctumVirga")]
    ChantPunctumVirga,
    /// Punctum virga, reversed
    #[serde(rename = "chantPunctumVirgaReversed")]
    ChantPunctumVirgaReversed,
    /// Quilisma
    #[serde(rename = "chantQuilisma")]
    ChantQuilisma,
    /// Semicirculus above
    #[serde(rename = "chantSemicirculusAbove")]
    ChantSemicirculusAbove,
    /// Semicirculus below
    #[serde(rename = "chantSemicirculusBelow")]
    ChantSemicirculusBelow,
    /// Plainchant staff
    #[serde(rename = "chantStaff")]
    ChantStaff,
    /// Plainchant staff (narrow)
    #[serde(rename = "chantStaffNarrow")]
    ChantStaffNarrow,
    /// Plainchant staff (wide)
    #[serde(rename = "chantStaffWide")]
    ChantStaffWide,
    /// Strophicus
    #[serde(rename = "chantStrophicus")]
    ChantStrophicus,
    /// Strophicus auctus
    #[serde(rename = "chantStrophicusAuctus")]
    ChantStrophicusAuctus,
    /// Strophicus liquescens, 2nd
    #[serde(rename = "chantStrophicusLiquescens2nd")]
    ChantStrophicusLiquescens2Nd,
    /// Strophicus liquescens, 3rd
    #[serde(rename = "chantStrophicusLiquescens3rd")]
    ChantStrophicusLiquescens3Rd,
    /// Strophicus liquescens, 4th
    #[serde(rename = "chantStrophicusLiquescens4th")]
    ChantStrophicusLiquescens4Th,
    /// Strophicus liquescens, 5th
    #[serde(rename = "chantStrophicusLiquescens5th")]
    ChantStrophicusLiquescens5Th,
    /// Virgula
    #[serde(rename = "chantVirgula")]
    ChantVirgula,
    /// 15 for clefs
    #[serde(rename = "clef15")]
    Clef15,
    /// 8 for clefs
    #[serde(rename = "clef8")]
    Clef8,
    /// Combining clef change
    #[serde(rename = "clefChangeCombining")]
    ClefChangeCombining,
    /// Coda
    #[serde(rename = "coda")]
    Coda,
    /// Square coda
    #[serde(rename = "codaSquare")]
    CodaSquare,
    /// Beat 2, compound time
    #[serde(rename = "conductorBeat2Compound")]
    ConductorBeat2Compound,
    /// Beat 2, simple time
    #[serde(rename = "conductorBeat2Simple")]
    ConductorBeat2Simple,
    /// Beat 3, compound time
    #[serde(rename = "conductorBeat3Compound")]
    ConductorBeat3Compound,
    /// Beat 3, simple time
    #[serde(rename = "conductorBeat3Simple")]
    ConductorBeat3Simple,
    /// Beat 4, compound time
    #[serde(rename = "conductorBeat4Compound")]
    ConductorBeat4Compound,
    /// Beat 4, simple time
    #[serde(rename = "conductorBeat4Simple")]
    ConductorBeat4Simple,
    /// Left-hand beat or cue
    #[serde(rename = "conductorLeftBeat")]
    ConductorLeftBeat,
    /// Right-hand beat or cue
    #[serde(rename = "conductorRightBeat")]
    ConductorRightBeat,
    /// Strong beat or cue
    #[serde(rename = "conductorStrongBeat")]
    ConductorStrongBeat,
    /// Unconducted/free passages
    #[serde(rename = "conductorUnconducted")]
    ConductorUnconducted,
    /// Weak beat or cue
    #[serde(rename = "conductorWeakBeat")]
    ConductorWeakBeat,
    /// Begin beam
    #[serde(rename = "controlBeginBeam")]
    ControlBeginBeam,
    /// Begin phrase
    #[serde(rename = "controlBeginPhrase")]
    ControlBeginPhrase,
    /// Begin slur
    #[serde(rename = "controlBeginSlur")]
    ControlBeginSlur,
    /// Begin tie
    #[serde(rename = "controlBeginTie")]
    ControlBeginTie,
    /// End beam
    #[serde(rename = "controlEndBeam")]
    ControlEndBeam,
    /// End phrase
    #[serde(rename = "controlEndPhrase")]
    ControlEndPhrase,
    /// End slur
    #[serde(rename = "controlEndSlur")]
    ControlEndSlur,
    /// End tie
    #[serde(rename = "controlEndTie")]
    ControlEndTie,
    /// Double flat
    #[serde(rename = "csymAccidentalDoubleFlat")]
    CsymAccidentalDoubleFlat,
    /// Double sharp
    #[serde(rename = "csymAccidentalDoubleSharp")]
    CsymAccidentalDoubleSharp,
    /// Flat
    #[serde(rename = "csymAccidentalFlat")]
    CsymAccidentalFlat,
    /// Natural
    #[serde(rename = "csymAccidentalNatural")]
    CsymAccidentalNatural,
    /// Sharp
    #[serde(rename = "csymAccidentalSharp")]
    CsymAccidentalSharp,
    /// Triple flat
    #[serde(rename = "csymAccidentalTripleFlat")]
    CsymAccidentalTripleFlat,
    /// Triple sharp
    #[serde(rename = "csymAccidentalTripleSharp")]
    CsymAccidentalTripleSharp,
    /// Slash for altered bass note
    #[serde(rename = "csymAlteredBassSlash")]
    CsymAlteredBassSlash,
    /// Augmented
    #[serde(rename = "csymAugmented")]
    CsymAugmented,
    /// Double-height left bracket
    #[serde(rename = "csymBracketLeftTall")]
    CsymBracketLeftTall,
    /// Double-height right bracket
    #[serde(rename = "csymBracketRightTall")]
    CsymBracketRightTall,
    /// Slash for chord symbols arranged diagonally
    #[serde(rename = "csymDiagonalArrangementSlash")]
    CsymDiagonalArrangementSlash,
    /// Diminished
    #[serde(rename = "csymDiminished")]
    CsymDiminished,
    /// Half-diminished
    #[serde(rename = "csymHalfDiminished")]
    CsymHalfDiminished,
    /// Major seventh
    #[serde(rename = "csymMajorSeventh")]
    CsymMajorSeventh,
    /// Minor
    #[serde(rename = "csymMinor")]
    CsymMinor,
    /// Double-height left parenthesis
    #[serde(rename = "csymParensLeftTall")]
    CsymParensLeftTall,
    /// Triple-height left parenthesis
    #[serde(rename = "csymParensLeftVeryTall")]
    CsymParensLeftVeryTall,
    /// Double-height right parenthesis
    #[serde(rename = "csymParensRightTall")]
    CsymParensRightTall,
    /// Triple-height right parenthesis
    #[serde(rename = "csymParensRightVeryTall")]
    CsymParensRightVeryTall,
    /// Curlew (Britten)
    #[serde(rename = "curlewSign")]
    CurlewSign,
    /// Da capo
    #[serde(rename = "daCapo")]
    DaCapo,
    /// Dal segno
    #[serde(rename = "dalSegno")]
    DalSegno,
    /// Daseian excellentes 1
    #[serde(rename = "daseianExcellentes1")]
    DaseianExcellentes1,
    /// Daseian excellentes 2
    #[serde(rename = "daseianExcellentes2")]
    DaseianExcellentes2,
    /// Daseian excellentes 3
    #[serde(rename = "daseianExcellentes3")]
    DaseianExcellentes3,
    /// Daseian excellentes 4
    #[serde(rename = "daseianExcellentes4")]
    DaseianExcellentes4,
    /// Daseian finales 1
    #[serde(rename = "daseianFinales1")]
    DaseianFinales1,
    /// Daseian finales 2
    #[serde(rename = "daseianFinales2")]
    DaseianFinales2,
    /// Daseian finales 3
    #[serde(rename = "daseianFinales3")]
    DaseianFinales3,
    /// Daseian finales 4
    #[serde(rename = "daseianFinales4")]
    DaseianFinales4,
    /// Daseian graves 1
    #[serde(rename = "daseianGraves1")]
    DaseianGraves1,
    /// Daseian graves 2
    #[serde(rename = "daseianGraves2")]
    DaseianGraves2,
    /// Daseian graves 3
    #[serde(rename = "daseianGraves3")]
    DaseianGraves3,
    /// Daseian graves 4
    #[serde(rename = "daseianGraves4")]
    DaseianGraves4,
    /// Daseian residua 1
    #[serde(rename = "daseianResidua1")]
    DaseianResidua1,
    /// Daseian residua 2
    #[serde(rename = "daseianResidua2")]
    DaseianResidua2,
    /// Daseian superiores 1
    #[serde(rename = "daseianSuperiores1")]
    DaseianSuperiores1,
    /// Daseian superiores 2
    #[serde(rename = "daseianSuperiores2")]
    DaseianSuperiores2,
    /// Daseian superiores 3
    #[serde(rename = "daseianSuperiores3")]
    DaseianSuperiores3,
    /// Daseian superiores 4
    #[serde(rename = "daseianSuperiores4")]
    DaseianSuperiores4,
    /// Double lateral roll (Stevens)
    #[serde(rename = "doubleLateralRollStevens")]
    DoubleLateralRollStevens,
    /// Double-tongue above
    #[serde(rename = "doubleTongueAbove")]
    DoubleTongueAbove,
    /// Double-tongue below
    #[serde(rename = "doubleTongueBelow")]
    DoubleTongueBelow,
    /// Colon separator for combined dynamics
    #[serde(rename = "dynamicCombinedSeparatorColon")]
    DynamicCombinedSeparatorColon,
    /// Hyphen separator for combined dynamics
    #[serde(rename = "dynamicCombinedSeparatorHyphen")]
    DynamicCombinedSeparatorHyphen,
    /// Slash separator for combined dynamics
    #[serde(rename = "dynamicCombinedSeparatorSlash")]
    DynamicCombinedSeparatorSlash,
    /// Space separator for combined dynamics
    #[serde(rename = "dynamicCombinedSeparatorSpace")]
    DynamicCombinedSeparatorSpace,
    /// Crescendo
    #[serde(rename = "dynamicCrescendoHairpin")]
    DynamicCrescendoHairpin,
    /// Diminuendo
    #[serde(rename = "dynamicDiminuendoHairpin")]
    DynamicDiminuendoHairpin,
    /// ff
    #[serde(rename = "dynamicFF")]
    DynamicFf,
    /// fff
    #[serde(rename = "dynamicFFF")]
    DynamicFff,
    /// ffff
    #[serde(rename = "dynamicFFFF")]
    DynamicFfff,
    /// fffff
    #[serde(rename = "dynamicFFFFF")]
    DynamicFffff,
    /// ffffff
    #[serde(rename = "dynamicFFFFFF")]
    DynamicFfffff,
    /// Forte
    #[serde(rename = "dynamicForte")]
    DynamicForte,
    /// Forte-piano
    #[serde(rename = "dynamicFortePiano")]
    DynamicFortePiano,
    /// Forzando
    #[serde(rename = "dynamicForzando")]
    DynamicForzando,
    /// Left bracket (for hairpins)
    #[serde(rename = "dynamicHairpinBracketLeft")]
    DynamicHairpinBracketLeft,
    /// Right bracket (for hairpins)
    #[serde(rename = "dynamicHairpinBracketRight")]
    DynamicHairpinBracketRight,
    /// Left parenthesis (for hairpins)
    #[serde(rename = "dynamicHairpinParenthesisLeft")]
    DynamicHairpinParenthesisLeft,
    /// Right parenthesis (for hairpins)
    #[serde(rename = "dynamicHairpinParenthesisRight")]
    DynamicHairpinParenthesisRight,
    /// Messa di voce
    #[serde(rename = "dynamicMessaDiVoce")]
    DynamicMessaDiVoce,
    /// Mezzo
    #[serde(rename = "dynamicMezzo")]
    DynamicMezzo,
    /// mf
    #[serde(rename = "dynamicMF")]
    DynamicMf,
    /// mp
    #[serde(rename = "dynamicMP")]
    DynamicMp,
    /// Niente
    #[serde(rename = "dynamicNiente")]
    DynamicNiente,
    /// Niente (for hairpins)
    #[serde(rename = "dynamicNienteForHairpin")]
    DynamicNienteForHairpin,
    /// pf
    #[serde(rename = "dynamicPF")]
    DynamicPf,
    /// Piano
    #[serde(rename = "dynamicPiano")]
    DynamicPiano,
    /// pp
    #[serde(rename = "dynamicPP")]
    DynamicPp,
    /// ppp
    #[serde(rename = "dynamicPPP")]
    DynamicPpp,
    /// pppp
    #[serde(rename = "dynamicPPPP")]
    DynamicPppp,
    /// ppppp
    #[serde(rename = "dynamicPPPPP")]
    DynamicPpppp,
    /// pppppp
    #[serde(rename = "dynamicPPPPPP")]
    DynamicPppppp,
    /// Rinforzando
    #[serde(rename = "dynamicRinforzando")]
    DynamicRinforzando,
    /// Rinforzando 1
    #[serde(rename = "dynamicRinforzando1")]
    DynamicRinforzando1,
    /// Rinforzando 2
    #[serde(rename = "dynamicRinforzando2")]
    DynamicRinforzando2,
    /// Sforzando
    #[serde(rename = "dynamicSforzando")]
    DynamicSforzando,
    /// Sforzando 1
    #[serde(rename = "dynamicSforzando1")]
    DynamicSforzando1,
    /// Sforzando-pianissimo
    #[serde(rename = "dynamicSforzandoPianissimo")]
    DynamicSforzandoPianissimo,
    /// Sforzando-piano
    #[serde(rename = "dynamicSforzandoPiano")]
    DynamicSforzandoPiano,
    /// Sforzato
    #[serde(rename = "dynamicSforzato")]
    DynamicSforzato,
    /// Sforzatissimo
    #[serde(rename = "dynamicSforzatoFF")]
    DynamicSforzatoFf,
    /// Sforzato-piano
    #[serde(rename = "dynamicSforzatoPiano")]
    DynamicSforzatoPiano,
    /// Z
    #[serde(rename = "dynamicZ")]
    DynamicZ,
    /// Eight channels (7.1 surround)
    #[serde(rename = "elecAudioChannelsEight")]
    ElecAudioChannelsEight,
    /// Five channels
    #[serde(rename = "elecAudioChannelsFive")]
    ElecAudioChannelsFive,
    /// Four channels
    #[serde(rename = "elecAudioChannelsFour")]
    ElecAudioChannelsFour,
    /// One channel (mono)
    #[serde(rename = "elecAudioChannelsOne")]
    ElecAudioChannelsOne,
    /// Seven channels
    #[serde(rename = "elecAudioChannelsSeven")]
    ElecAudioChannelsSeven,
    /// Six channels (5.1 surround)
    #[serde(rename = "elecAudioChannelsSix")]
    ElecAudioChannelsSix,
    /// Three channels (frontal)
    #[serde(rename = "elecAudioChannelsThreeFrontal")]
    ElecAudioChannelsThreeFrontal,
    /// Three channels (surround)
    #[serde(rename = "elecAudioChannelsThreeSurround")]
    ElecAudioChannelsThreeSurround,
    /// Two channels (stereo)
    #[serde(rename = "elecAudioChannelsTwo")]
    ElecAudioChannelsTwo,
    /// Audio in
    #[serde(rename = "elecAudioIn")]
    ElecAudioIn,
    /// Mono audio setup
    #[serde(rename = "elecAudioMono")]
    ElecAudioMono,
    /// Audio out
    #[serde(rename = "elecAudioOut")]
    ElecAudioOut,
    /// Stereo audio setup
    #[serde(rename = "elecAudioStereo")]
    ElecAudioStereo,
    /// Camera
    #[serde(rename = "elecCamera")]
    ElecCamera,
    /// Data in
    #[serde(rename = "elecDataIn")]
    ElecDataIn,
    /// Data out
    #[serde(rename = "elecDataOut")]
    ElecDataOut,
    /// Disc
    #[serde(rename = "elecDisc")]
    ElecDisc,
    /// Download
    #[serde(rename = "elecDownload")]
    ElecDownload,
    /// Eject
    #[serde(rename = "elecEject")]
    ElecEject,
    /// Fast-forward
    #[serde(rename = "elecFastForward")]
    ElecFastForward,
    /// Headphones
    #[serde(rename = "elecHeadphones")]
    ElecHeadphones,
    /// Headset
    #[serde(rename = "elecHeadset")]
    ElecHeadset,
    /// Line in
    #[serde(rename = "elecLineIn")]
    ElecLineIn,
    /// Line out
    #[serde(rename = "elecLineOut")]
    ElecLineOut,
    /// Loop
    #[serde(rename = "elecLoop")]
    ElecLoop,
    /// Loudspeaker
    #[serde(rename = "elecLoudspeaker")]
    ElecLoudspeaker,
    /// Microphone
    #[serde(rename = "elecMicrophone")]
    ElecMicrophone,
    /// Mute microphone
    #[serde(rename = "elecMicrophoneMute")]
    ElecMicrophoneMute,
    /// Unmute microphone
    #[serde(rename = "elecMicrophoneUnmute")]
    ElecMicrophoneUnmute,
    /// MIDI controller 0%
    #[serde(rename = "elecMIDIController0")]
    ElecMidiController0,
    /// MIDI controller 100%
    #[serde(rename = "elecMIDIController100")]
    ElecMidiController100,
    /// MIDI controller 20%
    #[serde(rename = "elecMIDIController20")]
    ElecMidiController20,
    /// MIDI controller 40%
    #[serde(rename = "elecMIDIController40")]
    ElecMidiController40,
    /// MIDI controller 60%
    #[serde(rename = "elecMIDIController60")]
    ElecMidiController60,
    /// MIDI controller 80%
    #[serde(rename = "elecMIDIController80")]
    ElecMidiController80,
    /// MIDI in
    #[serde(rename = "elecMIDIIn")]
    ElecMidiIn,
    /// MIDI out
    #[serde(rename = "elecMIDIOut")]
    ElecMidiOut,
    /// Mixing console
    #[serde(rename = "elecMixingConsole")]
    ElecMixingConsole,
    /// Monitor
    #[serde(rename = "elecMonitor")]
    ElecMonitor,
    /// Mute
    #[serde(rename = "elecMute")]
    ElecMute,
    /// Pause
    #[serde(rename = "elecPause")]
    ElecPause,
    /// Play
    #[serde(rename = "elecPlay")]
    ElecPlay,
    /// Power on/off
    #[serde(rename = "elecPowerOnOff")]
    ElecPowerOnOff,
    /// Projector
    #[serde(rename = "elecProjector")]
    ElecProjector,
    /// Replay
    #[serde(rename = "elecReplay")]
    ElecReplay,
    /// Rewind
    #[serde(rename = "elecRewind")]
    ElecRewind,
    /// Shuffle
    #[serde(rename = "elecShuffle")]
    ElecShuffle,
    /// Skip backwards
    #[serde(rename = "elecSkipBackwards")]
    ElecSkipBackwards,
    /// Skip forwards
    #[serde(rename = "elecSkipForwards")]
    ElecSkipForwards,
    /// Stop
    #[serde(rename = "elecStop")]
    ElecStop,
    /// Tape
    #[serde(rename = "elecTape")]
    ElecTape,
    /// Unmute
    #[serde(rename = "elecUnmute")]
    ElecUnmute,
    /// Upload
    #[serde(rename = "elecUpload")]
    ElecUpload,
    /// USB connection
    #[serde(rename = "elecUSB")]
    ElecUsb,
    /// Video camera
    #[serde(rename = "elecVideoCamera")]
    ElecVideoCamera,
    /// Video in
    #[serde(rename = "elecVideoIn")]
    ElecVideoIn,
    /// Video out
    #[serde(rename = "elecVideoOut")]
    ElecVideoOut,
    /// Combining volume fader
    #[serde(rename = "elecVolumeFader")]
    ElecVolumeFader,
    /// Combining volume fader thumb
    #[serde(rename = "elecVolumeFaderThumb")]
    ElecVolumeFaderThumb,
    /// Volume level 0%
    #[serde(rename = "elecVolumeLevel0")]
    ElecVolumeLevel0,
    /// Volume level 100%
    #[serde(rename = "elecVolumeLevel100")]
    ElecVolumeLevel100,
    /// Volume level 20%
    #[serde(rename = "elecVolumeLevel20")]
    ElecVolumeLevel20,
    /// Volume level 40%
    #[serde(rename = "elecVolumeLevel40")]
    ElecVolumeLevel40,
    /// Volume level 60%
    #[serde(rename = "elecVolumeLevel60")]
    ElecVolumeLevel60,
    /// Volume level 80%
    #[serde(rename = "elecVolumeLevel80")]
    ElecVolumeLevel80,
    /// F clef
    #[serde(rename = "fClef")]
    FClef,
    /// F clef quindicesima alta
    #[serde(rename = "fClef15ma")]
    FClef15Ma,
    /// F clef quindicesima bassa
    #[serde(rename = "fClef15mb")]
    FClef15Mb,
    /// F clef ottava alta
    #[serde(rename = "fClef8va")]
    FClef8Va,
    /// F clef ottava bassa
    #[serde(rename = "fClef8vb")]
    FClef8Vb,
    /// F clef, arrow down
    #[serde(rename = "fClefArrowDown")]
    FClefArrowDown,
    /// F clef, arrow up
    #[serde(rename = "fClefArrowUp")]
    FClefArrowUp,
    /// F clef change
    #[serde(rename = "fClefChange")]
    FClefChange,
    /// Reversed F clef
    #[serde(rename = "fClefReversed")]
    FClefReversed,
    /// Turned F clef
    #[serde(rename = "fClefTurned")]
    FClefTurned,
    /// Fermata above
    #[serde(rename = "fermataAbove")]
    FermataAbove,
    /// Fermata below
    #[serde(rename = "fermataBelow")]
    FermataBelow,
    /// Long fermata above
    #[serde(rename = "fermataLongAbove")]
    FermataLongAbove,
    /// Long fermata below
    #[serde(rename = "fermataLongBelow")]
    FermataLongBelow,
    /// Long fermata (Henze) above
    #[serde(rename = "fermataLongHenzeAbove")]
    FermataLongHenzeAbove,
    /// Long fermata (Henze) below
    #[serde(rename = "fermataLongHenzeBelow")]
    FermataLongHenzeBelow,
    /// Short fermata above
    #[serde(rename = "fermataShortAbove")]
    FermataShortAbove,
    /// Short fermata below
    #[serde(rename = "fermataShortBelow")]
    FermataShortBelow,
    /// Short fermata (Henze) above
    #[serde(rename = "fermataShortHenzeAbove")]
    FermataShortHenzeAbove,
    /// Short fermata (Henze) below
    #[serde(rename = "fermataShortHenzeBelow")]
    FermataShortHenzeBelow,
    /// Very long fermata above
    #[serde(rename = "fermataVeryLongAbove")]
    FermataVeryLongAbove,
    /// Very long fermata below
    #[serde(rename = "fermataVeryLongBelow")]
    FermataVeryLongBelow,
    /// Very short fermata above
    #[serde(rename = "fermataVeryShortAbove")]
    FermataVeryShortAbove,
    /// Very short fermata below
    #[serde(rename = "fermataVeryShortBelow")]
    FermataVeryShortBelow,
    /// Figured bass 0
    #[serde(rename = "figbass0")]
    Figbass0,
    /// Figured bass 1
    #[serde(rename = "figbass1")]
    Figbass1,
    /// Figured bass 2
    #[serde(rename = "figbass2")]
    Figbass2,
    /// Figured bass 2 raised by half-step
    #[serde(rename = "figbass2Raised")]
    Figbass2Raised,
    /// Figured bass 3
    #[serde(rename = "figbass3")]
    Figbass3,
    /// Figured bass 4
    #[serde(rename = "figbass4")]
    Figbass4,
    /// Figured bass 4 raised by half-step
    #[serde(rename = "figbass4Raised")]
    Figbass4Raised,
    /// Figured bass 5
    #[serde(rename = "figbass5")]
    Figbass5,
    /// Figured bass 5 raised by half-step
    #[serde(rename = "figbass5Raised1")]
    Figbass5Raised1,
    /// Figured bass 5 raised by half-step 2
    #[serde(rename = "figbass5Raised2")]
    Figbass5Raised2,
    /// Figured bass diminished 5
    #[serde(rename = "figbass5Raised3")]
    Figbass5Raised3,
    /// Figured bass 6
    #[serde(rename = "figbass6")]
    Figbass6,
    /// Figured bass 6 raised by half-step
    #[serde(rename = "figbass6Raised")]
    Figbass6Raised,
    /// Figured bass 6 raised by half-step 2
    #[serde(rename = "figbass6Raised2")]
    Figbass6Raised2,
    /// Figured bass 7
    #[serde(rename = "figbass7")]
    Figbass7,
    /// Figured bass 7 diminished
    #[serde(rename = "figbass7Diminished")]
    Figbass7Diminished,
    /// Figured bass 7 raised by half-step
    #[serde(rename = "figbass7Raised1")]
    Figbass7Raised1,
    /// Figured bass 7 lowered by a half-step
    #[serde(rename = "figbass7Raised2")]
    Figbass7Raised2,
    /// Figured bass 8
    #[serde(rename = "figbass8")]
    Figbass8,
    /// Figured bass 9
    #[serde(rename = "figbass9")]
    Figbass9,
    /// Figured bass 9 raised by half-step
    #[serde(rename = "figbass9Raised")]
    Figbass9Raised,
    /// Figured bass \[
    #[serde(rename = "figbassBracketLeft")]
    FigbassBracketLeft,
    /// Figured bass \]
    #[serde(rename = "figbassBracketRight")]
    FigbassBracketRight,
    /// Combining lower
    #[serde(rename = "figbassCombiningLowering")]
    FigbassCombiningLowering,
    /// Combining raise
    #[serde(rename = "figbassCombiningRaising")]
    FigbassCombiningRaising,
    /// Figured bass double flat
    #[serde(rename = "figbassDoubleFlat")]
    FigbassDoubleFlat,
    /// Figured bass double sharp
    #[serde(rename = "figbassDoubleSharp")]
    FigbassDoubleSharp,
    /// Figured bass flat
    #[serde(rename = "figbassFlat")]
    FigbassFlat,
    /// Figured bass natural
    #[serde(rename = "figbassNatural")]
    FigbassNatural,
    /// Figured bass (
    #[serde(rename = "figbassParensLeft")]
    FigbassParensLeft,
    /// Figured bass )
    #[serde(rename = "figbassParensRight")]
    FigbassParensRight,
    /// Figured bass +
    #[serde(rename = "figbassPlus")]
    FigbassPlus,
    /// Figured bass sharp
    #[serde(rename = "figbassSharp")]
    FigbassSharp,
    /// Figured bass triple flat
    #[serde(rename = "figbassTripleFlat")]
    FigbassTripleFlat,
    /// Figured bass triple sharp
    #[serde(rename = "figbassTripleSharp")]
    FigbassTripleSharp,
    /// Fingering 0 (open string)
    #[serde(rename = "fingering0")]
    Fingering0,
    /// Fingering 0 italic (open string)
    #[serde(rename = "fingering0Italic")]
    Fingering0Italic,
    /// Fingering 1 (thumb)
    #[serde(rename = "fingering1")]
    Fingering1,
    /// Fingering 1 italic (thumb)
    #[serde(rename = "fingering1Italic")]
    Fingering1Italic,
    /// Fingering 2 (index finger)
    #[serde(rename = "fingering2")]
    Fingering2,
    /// Fingering 2 italic (index finger)
    #[serde(rename = "fingering2Italic")]
    Fingering2Italic,
    /// Fingering 3 (middle finger)
    #[serde(rename = "fingering3")]
    Fingering3,
    /// Fingering 3 italic (middle finger)
    #[serde(rename = "fingering3Italic")]
    Fingering3Italic,
    /// Fingering 4 (ring finger)
    #[serde(rename = "fingering4")]
    Fingering4,
    /// Fingering 4 italic (ring finger)
    #[serde(rename = "fingering4Italic")]
    Fingering4Italic,
    /// Fingering 5 (little finger)
    #[serde(rename = "fingering5")]
    Fingering5,
    /// Fingering 5 italic (little finger)
    #[serde(rename = "fingering5Italic")]
    Fingering5Italic,
    /// Fingering 6
    #[serde(rename = "fingering6")]
    Fingering6,
    /// Fingering 6 italic
    #[serde(rename = "fingering6Italic")]
    Fingering6Italic,
    /// Fingering 7
    #[serde(rename = "fingering7")]
    Fingering7,
    /// Fingering 7 italic
    #[serde(rename = "fingering7Italic")]
    Fingering7Italic,
    /// Fingering 8
    #[serde(rename = "fingering8")]
    Fingering8,
    /// Fingering 8 italic
    #[serde(rename = "fingering8Italic")]
    Fingering8Italic,
    /// Fingering 9
    #[serde(rename = "fingering9")]
    Fingering9,
    /// Fingering 9 italic
    #[serde(rename = "fingering9Italic")]
    Fingering9Italic,
    /// Fingering a (anular; right-hand ring finger for guitar)
    #[serde(rename = "fingeringALower")]
    FingeringALower,
    /// Fingering c (right-hand little finger for guitar)
    #[serde(rename = "fingeringCLower")]
    FingeringCLower,
    /// Fingering e (right-hand little finger for guitar)
    #[serde(rename = "fingeringELower")]
    FingeringELower,
    /// Fingering i (indicio; right-hand index finger for guitar)
    #[serde(rename = "fingeringILower")]
    FingeringILower,
    /// Fingering left bracket
    #[serde(rename = "fingeringLeftBracket")]
    FingeringLeftBracket,
    /// Fingering left bracket italic
    #[serde(rename = "fingeringLeftBracketItalic")]
    FingeringLeftBracketItalic,
    /// Fingering left parenthesis
    #[serde(rename = "fingeringLeftParenthesis")]
    FingeringLeftParenthesis,
    /// Fingering left parenthesis italic
    #[serde(rename = "fingeringLeftParenthesisItalic")]
    FingeringLeftParenthesisItalic,
    /// Fingering m (medio; right-hand middle finger for guitar)
    #[serde(rename = "fingeringMLower")]
    FingeringMLower,
    /// Multiple notes played by thumb or single finger
    #[serde(rename = "fingeringMultipleNotes")]
    FingeringMultipleNotes,
    /// Fingering o (right-hand little finger for guitar)
    #[serde(rename = "fingeringOLower")]
    FingeringOLower,
    /// Fingering p (pulgar; right-hand thumb for guitar)
    #[serde(rename = "fingeringPLower")]
    FingeringPLower,
    /// Fingering q (right-hand little finger for guitar)
    #[serde(rename = "fingeringQLower")]
    FingeringQLower,
    /// Fingering right bracket
    #[serde(rename = "fingeringRightBracket")]
    FingeringRightBracket,
    /// Fingering right bracket italic
    #[serde(rename = "fingeringRightBracketItalic")]
    FingeringRightBracketItalic,
    /// Fingering right parenthesis
    #[serde(rename = "fingeringRightParenthesis")]
    FingeringRightParenthesis,
    /// Fingering right parenthesis italic
    #[serde(rename = "fingeringRightParenthesisItalic")]
    FingeringRightParenthesisItalic,
    /// Fingering s (right-hand little finger for guitar)
    #[serde(rename = "fingeringSLower")]
    FingeringSLower,
    /// Fingering middle dot separator
    #[serde(rename = "fingeringSeparatorMiddleDot")]
    FingeringSeparatorMiddleDot,
    /// Fingering white middle dot separator
    #[serde(rename = "fingeringSeparatorMiddleDotWhite")]
    FingeringSeparatorMiddleDotWhite,
    /// Fingering forward slash separator
    #[serde(rename = "fingeringSeparatorSlash")]
    FingeringSeparatorSlash,
    /// Finger substitution above
    #[serde(rename = "fingeringSubstitutionAbove")]
    FingeringSubstitutionAbove,
    /// Finger substitution below
    #[serde(rename = "fingeringSubstitutionBelow")]
    FingeringSubstitutionBelow,
    /// Finger substitution dash
    #[serde(rename = "fingeringSubstitutionDash")]
    FingeringSubstitutionDash,
    /// Fingering t (right-hand thumb for guitar)
    #[serde(rename = "fingeringTLower")]
    FingeringTLower,
    /// Fingering T (left-hand thumb for guitar)
    #[serde(rename = "fingeringTUpper")]
    FingeringTUpper,
    /// Fingering x (right-hand little finger for guitar)
    #[serde(rename = "fingeringXLower")]
    FingeringXLower,
    /// Combining flag 8 (1024th) below
    #[serde(rename = "flag1024thDown")]
    Flag1024ThDown,
    /// Combining flag 8 (1024th) above
    #[serde(rename = "flag1024thUp")]
    Flag1024ThUp,
    /// Combining flag 5 (128th) below
    #[serde(rename = "flag128thDown")]
    Flag128ThDown,
    /// Combining flag 5 (128th) above
    #[serde(rename = "flag128thUp")]
    Flag128ThUp,
    /// Combining flag 2 (16th) below
    #[serde(rename = "flag16thDown")]
    Flag16ThDown,
    /// Combining flag 2 (16th) above
    #[serde(rename = "flag16thUp")]
    Flag16ThUp,
    /// Combining flag 6 (256th) below
    #[serde(rename = "flag256thDown")]
    Flag256ThDown,
    /// Combining flag 6 (256th) above
    #[serde(rename = "flag256thUp")]
    Flag256ThUp,
    /// Combining flag 3 (32nd) below
    #[serde(rename = "flag32ndDown")]
    Flag32NdDown,
    /// Combining flag 3 (32nd) above
    #[serde(rename = "flag32ndUp")]
    Flag32NdUp,
    /// Combining flag 7 (512th) below
    #[serde(rename = "flag512thDown")]
    Flag512ThDown,
    /// Combining flag 7 (512th) above
    #[serde(rename = "flag512thUp")]
    Flag512ThUp,
    /// Combining flag 4 (64th) below
    #[serde(rename = "flag64thDown")]
    Flag64ThDown,
    /// Combining flag 4 (64th) above
    #[serde(rename = "flag64thUp")]
    Flag64ThUp,
    /// Combining flag 1 (8th) below
    #[serde(rename = "flag8thDown")]
    Flag8ThDown,
    /// Combining flag 1 (8th) above
    #[serde(rename = "flag8thUp")]
    Flag8ThUp,
    /// Internal combining flag below
    #[serde(rename = "flagInternalDown")]
    FlagInternalDown,
    /// Internal combining flag above
    #[serde(rename = "flagInternalUp")]
    FlagInternalUp,
    /// 3-string fretboard
    #[serde(rename = "fretboard3String")]
    Fretboard3String,
    /// 3-string fretboard at nut
    #[serde(rename = "fretboard3StringNut")]
    Fretboard3StringNut,
    /// 4-string fretboard
    #[serde(rename = "fretboard4String")]
    Fretboard4String,
    /// 4-string fretboard at nut
    #[serde(rename = "fretboard4StringNut")]
    Fretboard4StringNut,
    /// 5-string fretboard
    #[serde(rename = "fretboard5String")]
    Fretboard5String,
    /// 5-string fretboard at nut
    #[serde(rename = "fretboard5StringNut")]
    Fretboard5StringNut,
    /// 6-string fretboard
    #[serde(rename = "fretboard6String")]
    Fretboard6String,
    /// 6-string fretboard at nut
    #[serde(rename = "fretboard6StringNut")]
    Fretboard6StringNut,
    /// Fingered fret (filled circle)
    #[serde(rename = "fretboardFilledCircle")]
    FretboardFilledCircle,
    /// Open string (O)
    #[serde(rename = "fretboardO")]
    FretboardO,
    /// String not played (X)
    #[serde(rename = "fretboardX")]
    FretboardX,
    /// Function theory angle bracket left
    #[serde(rename = "functionAngleLeft")]
    FunctionAngleLeft,
    /// Function theory angle bracket right
    #[serde(rename = "functionAngleRight")]
    FunctionAngleRight,
    /// Function theory bracket left
    #[serde(rename = "functionBracketLeft")]
    FunctionBracketLeft,
    /// Function theory bracket right
    #[serde(rename = "functionBracketRight")]
    FunctionBracketRight,
    /// Function theory minor dominant
    #[serde(rename = "functionDLower")]
    FunctionDLower,
    /// Function theory major dominant
    #[serde(rename = "functionDUpper")]
    FunctionDUpper,
    /// Function theory dominant of dominant
    #[serde(rename = "functionDD")]
    FunctionDd,
    /// Function theory 8
    #[serde(rename = "functionEight")]
    FunctionEight,
    /// Function theory F
    #[serde(rename = "functionFUpper")]
    FunctionFUpper,
    /// Function theory 5
    #[serde(rename = "functionFive")]
    FunctionFive,
    /// Function theory 4
    #[serde(rename = "functionFour")]
    FunctionFour,
    /// Function theory g
    #[serde(rename = "functionGLower")]
    FunctionGLower,
    /// Function theory G
    #[serde(rename = "functionGUpper")]
    FunctionGUpper,
    /// Function theory greater than
    #[serde(rename = "functionGreaterThan")]
    FunctionGreaterThan,
    /// Function theory i
    #[serde(rename = "functionILower")]
    FunctionILower,
    /// Function theory I
    #[serde(rename = "functionIUpper")]
    FunctionIUpper,
    /// Function theory k
    #[serde(rename = "functionKLower")]
    FunctionKLower,
    /// Function theory K
    #[serde(rename = "functionKUpper")]
    FunctionKUpper,
    /// Function theory l
    #[serde(rename = "functionLLower")]
    FunctionLLower,
    /// Function theory L
    #[serde(rename = "functionLUpper")]
    FunctionLUpper,
    /// Function theory less than
    #[serde(rename = "functionLessThan")]
    FunctionLessThan,
    /// Function theory m
    #[serde(rename = "functionMLower")]
    FunctionMLower,
    /// Function theory M
    #[serde(rename = "functionMUpper")]
    FunctionMUpper,
    /// Function theory minus
    #[serde(rename = "functionMinus")]
    FunctionMinus,
    /// Function theory n
    #[serde(rename = "functionNLower")]
    FunctionNLower,
    /// Function theory N
    #[serde(rename = "functionNUpper")]
    FunctionNUpper,
    /// Function theory superscript N
    #[serde(rename = "functionNUpperSuperscript")]
    FunctionNUpperSuperscript,
    /// Function theory 9
    #[serde(rename = "functionNine")]
    FunctionNine,
    /// Function theory 1
    #[serde(rename = "functionOne")]
    FunctionOne,
    /// Function theory p
    #[serde(rename = "functionPLower")]
    FunctionPLower,
    /// Function theory P
    #[serde(rename = "functionPUpper")]
    FunctionPUpper,
    /// Function theory parenthesis left
    #[serde(rename = "functionParensLeft")]
    FunctionParensLeft,
    /// Function theory parenthesis right
    #[serde(rename = "functionParensRight")]
    FunctionParensRight,
    /// Function theory prefix plus
    #[serde(rename = "functionPlus")]
    FunctionPlus,
    /// Function theory r
    #[serde(rename = "functionRLower")]
    FunctionRLower,
    /// Function theory repetition 1
    #[serde(rename = "functionRepetition1")]
    FunctionRepetition1,
    /// Function theory repetition 2
    #[serde(rename = "functionRepetition2")]
    FunctionRepetition2,
    /// Function theory prefix ring
    #[serde(rename = "functionRing")]
    FunctionRing,
    /// Function theory minor subdominant
    #[serde(rename = "functionSLower")]
    FunctionSLower,
    /// Function theory major subdominant
    #[serde(rename = "functionSUpper")]
    FunctionSUpper,
    /// Function theory 7
    #[serde(rename = "functionSeven")]
    FunctionSeven,
    /// Function theory 6
    #[serde(rename = "functionSix")]
    FunctionSix,
    /// Function theory double dominant seventh
    #[serde(rename = "functionSlashedDD")]
    FunctionSlashedDd,
    /// Function theory minor subdominant of subdominant
    #[serde(rename = "functionSSLower")]
    FunctionSsLower,
    /// Function theory major subdominant of subdominant
    #[serde(rename = "functionSSUpper")]
    FunctionSsUpper,
    /// Function theory minor tonic
    #[serde(rename = "functionTLower")]
    FunctionTLower,
    /// Function theory tonic
    #[serde(rename = "functionTUpper")]
    FunctionTUpper,
    /// Function theory 3
    #[serde(rename = "functionThree")]
    FunctionThree,
    /// Function theory 2
    #[serde(rename = "functionTwo")]
    FunctionTwo,
    /// Function theory v
    #[serde(rename = "functionVLower")]
    FunctionVLower,
    /// Function theory V
    #[serde(rename = "functionVUpper")]
    FunctionVUpper,
    /// Function theory 0
    #[serde(rename = "functionZero")]
    FunctionZero,
    /// G clef
    #[serde(rename = "gClef")]
    GClef,
    /// G clef quindicesima alta
    #[serde(rename = "gClef15ma")]
    GClef15Ma,
    /// G clef quindicesima bassa
    #[serde(rename = "gClef15mb")]
    GClef15Mb,
    /// G clef ottava alta
    #[serde(rename = "gClef8va")]
    GClef8Va,
    /// G clef ottava bassa
    #[serde(rename = "gClef8vb")]
    GClef8Vb,
    /// G clef ottava bassa with C clef
    #[serde(rename = "gClef8vbCClef")]
    GClef8VbCClef,
    /// G clef ottava bassa (old style)
    #[serde(rename = "gClef8vbOld")]
    GClef8VbOld,
    /// G clef, optionally ottava bassa
    #[serde(rename = "gClef8vbParens")]
    GClef8VbParens,
    /// G clef, arrow down
    #[serde(rename = "gClefArrowDown")]
    GClefArrowDown,
    /// G clef, arrow up
    #[serde(rename = "gClefArrowUp")]
    GClefArrowUp,
    /// G clef change
    #[serde(rename = "gClefChange")]
    GClefChange,
    /// Combining G clef, number above
    #[serde(rename = "gClefLigatedNumberAbove")]
    GClefLigatedNumberAbove,
    /// Combining G clef, number below
    #[serde(rename = "gClefLigatedNumberBelow")]
    GClefLigatedNumberBelow,
    /// Reversed G clef
    #[serde(rename = "gClefReversed")]
    GClefReversed,
    /// Turned G clef
    #[serde(rename = "gClefTurned")]
    GClefTurned,
    /// Glissando down
    #[serde(rename = "glissandoDown")]
    GlissandoDown,
    /// Glissando up
    #[serde(rename = "glissandoUp")]
    GlissandoUp,
    /// Slashed grace note stem down
    #[serde(rename = "graceNoteAcciaccaturaStemDown")]
    GraceNoteAcciaccaturaStemDown,
    /// Slashed grace note stem up
    #[serde(rename = "graceNoteAcciaccaturaStemUp")]
    GraceNoteAcciaccaturaStemUp,
    /// Grace note stem down
    #[serde(rename = "graceNoteAppoggiaturaStemDown")]
    GraceNoteAppoggiaturaStemDown,
    /// Grace note stem up
    #[serde(rename = "graceNoteAppoggiaturaStemUp")]
    GraceNoteAppoggiaturaStemUp,
    /// Slash for stem down grace note
    #[serde(rename = "graceNoteSlashStemDown")]
    GraceNoteSlashStemDown,
    /// Slash for stem up grace note
    #[serde(rename = "graceNoteSlashStemUp")]
    GraceNoteSlashStemUp,
    /// Full barré
    #[serde(rename = "guitarBarreFull")]
    GuitarBarreFull,
    /// Half barré
    #[serde(rename = "guitarBarreHalf")]
    GuitarBarreHalf,
    /// Closed wah/volume pedal
    #[serde(rename = "guitarClosePedal")]
    GuitarClosePedal,
    /// Fade in
    #[serde(rename = "guitarFadeIn")]
    GuitarFadeIn,
    /// Fade out
    #[serde(rename = "guitarFadeOut")]
    GuitarFadeOut,
    /// Golpe (tapping the pick guard)
    #[serde(rename = "guitarGolpe")]
    GuitarGolpe,
    /// Half-open wah/volume pedal
    #[serde(rename = "guitarHalfOpenPedal")]
    GuitarHalfOpenPedal,
    /// Left-hand tapping
    #[serde(rename = "guitarLeftHandTapping")]
    GuitarLeftHandTapping,
    /// Open wah/volume pedal
    #[serde(rename = "guitarOpenPedal")]
    GuitarOpenPedal,
    /// Right-hand tapping
    #[serde(rename = "guitarRightHandTapping")]
    GuitarRightHandTapping,
    /// Guitar shake
    #[serde(rename = "guitarShake")]
    GuitarShake,
    /// String number 0
    #[serde(rename = "guitarString0")]
    GuitarString0,
    /// String number 1
    #[serde(rename = "guitarString1")]
    GuitarString1,
    /// String number 10
    #[serde(rename = "guitarString10")]
    GuitarString10,
    /// String number 11
    #[serde(rename = "guitarString11")]
    GuitarString11,
    /// String number 12
    #[serde(rename = "guitarString12")]
    GuitarString12,
    /// String number 13
    #[serde(rename = "guitarString13")]
    GuitarString13,
    /// String number 2
    #[serde(rename = "guitarString2")]
    GuitarString2,
    /// String number 3
    #[serde(rename = "guitarString3")]
    GuitarString3,
    /// String number 4
    #[serde(rename = "guitarString4")]
    GuitarString4,
    /// String number 5
    #[serde(rename = "guitarString5")]
    GuitarString5,
    /// String number 6
    #[serde(rename = "guitarString6")]
    GuitarString6,
    /// String number 7
    #[serde(rename = "guitarString7")]
    GuitarString7,
    /// String number 8
    #[serde(rename = "guitarString8")]
    GuitarString8,
    /// String number 9
    #[serde(rename = "guitarString9")]
    GuitarString9,
    /// Strum direction down
    #[serde(rename = "guitarStrumDown")]
    GuitarStrumDown,
    /// Strum direction up
    #[serde(rename = "guitarStrumUp")]
    GuitarStrumUp,
    /// Guitar vibrato bar dip
    #[serde(rename = "guitarVibratoBarDip")]
    GuitarVibratoBarDip,
    /// Guitar vibrato bar scoop
    #[serde(rename = "guitarVibratoBarScoop")]
    GuitarVibratoBarScoop,
    /// Vibrato wiggle segment
    #[serde(rename = "guitarVibratoStroke")]
    GuitarVibratoStroke,
    /// Volume swell
    #[serde(rename = "guitarVolumeSwell")]
    GuitarVolumeSwell,
    /// Wide vibrato wiggle segment
    #[serde(rename = "guitarWideVibratoStroke")]
    GuitarWideVibratoStroke,
    /// Belltree
    #[serde(rename = "handbellsBelltree")]
    HandbellsBelltree,
    /// Damp 3
    #[serde(rename = "handbellsDamp3")]
    HandbellsDamp3,
    /// Echo
    #[serde(rename = "handbellsEcho1")]
    HandbellsEcho1,
    /// Echo 2
    #[serde(rename = "handbellsEcho2")]
    HandbellsEcho2,
    /// Gyro
    #[serde(rename = "handbellsGyro")]
    HandbellsGyro,
    /// Hand martellato
    #[serde(rename = "handbellsHandMartellato")]
    HandbellsHandMartellato,
    /// Mallet, bell on table
    #[serde(rename = "handbellsMalletBellOnTable")]
    HandbellsMalletBellOnTable,
    /// Mallet, bell suspended
    #[serde(rename = "handbellsMalletBellSuspended")]
    HandbellsMalletBellSuspended,
    /// Mallet lift
    #[serde(rename = "handbellsMalletLft")]
    HandbellsMalletLft,
    /// Martellato
    #[serde(rename = "handbellsMartellato")]
    HandbellsMartellato,
    /// Martellato lift
    #[serde(rename = "handbellsMartellatoLift")]
    HandbellsMartellatoLift,
    /// Muted martellato
    #[serde(rename = "handbellsMutedMartellato")]
    HandbellsMutedMartellato,
    /// Pluck lift
    #[serde(rename = "handbellsPluckLift")]
    HandbellsPluckLift,
    /// Swing
    #[serde(rename = "handbellsSwing")]
    HandbellsSwing,
    /// Swing down
    #[serde(rename = "handbellsSwingDown")]
    HandbellsSwingDown,
    /// Swing up
    #[serde(rename = "handbellsSwingUp")]
    HandbellsSwingUp,
    /// Table pair of handbells
    #[serde(rename = "handbellsTablePairBells")]
    HandbellsTablePairBells,
    /// Table single handbell
    #[serde(rename = "handbellsTableSingleBell")]
    HandbellsTableSingleBell,
    /// Metal rod pictogram
    #[serde(rename = "harpMetalRod")]
    HarpMetalRod,
    /// Harp pedal centered (natural)
    #[serde(rename = "harpPedalCentered")]
    HarpPedalCentered,
    /// Harp pedal divider
    #[serde(rename = "harpPedalDivider")]
    HarpPedalDivider,
    /// Harp pedal lowered (sharp)
    #[serde(rename = "harpPedalLowered")]
    HarpPedalLowered,
    /// Harp pedal raised (flat)
    #[serde(rename = "harpPedalRaised")]
    HarpPedalRaised,
    /// Ascending aeolian chords (Salzedo)
    #[serde(rename = "harpSalzedoAeolianAscending")]
    HarpSalzedoAeolianAscending,
    /// Descending aeolian chords (Salzedo)
    #[serde(rename = "harpSalzedoAeolianDescending")]
    HarpSalzedoAeolianDescending,
    /// Damp above (Salzedo)
    #[serde(rename = "harpSalzedoDampAbove")]
    HarpSalzedoDampAbove,
    /// Damp below (Salzedo)
    #[serde(rename = "harpSalzedoDampBelow")]
    HarpSalzedoDampBelow,
    /// Damp with both hands (Salzedo)
    #[serde(rename = "harpSalzedoDampBothHands")]
    HarpSalzedoDampBothHands,
    /// Damp only low strings (Salzedo)
    #[serde(rename = "harpSalzedoDampLowStrings")]
    HarpSalzedoDampLowStrings,
    /// Fluidic sounds, left hand (Salzedo)
    #[serde(rename = "harpSalzedoFluidicSoundsLeft")]
    HarpSalzedoFluidicSoundsLeft,
    /// Fluidic sounds, right hand (Salzedo)
    #[serde(rename = "harpSalzedoFluidicSoundsRight")]
    HarpSalzedoFluidicSoundsRight,
    /// Isolated sounds (Salzedo)
    #[serde(rename = "harpSalzedoIsolatedSounds")]
    HarpSalzedoIsolatedSounds,
    /// Metallic sounds (Salzedo)
    #[serde(rename = "harpSalzedoMetallicSounds")]
    HarpSalzedoMetallicSounds,
    /// Metallic sounds, one string (Salzedo)
    #[serde(rename = "harpSalzedoMetallicSoundsOneString")]
    HarpSalzedoMetallicSoundsOneString,
    /// Muffle totally (Salzedo)
    #[serde(rename = "harpSalzedoMuffleTotally")]
    HarpSalzedoMuffleTotally,
    /// Oboic flux (Salzedo)
    #[serde(rename = "harpSalzedoOboicFlux")]
    HarpSalzedoOboicFlux,
    /// Play at upper end of strings (Salzedo)
    #[serde(rename = "harpSalzedoPlayUpperEnd")]
    HarpSalzedoPlayUpperEnd,
    /// Slide with suppleness (Salzedo)
    #[serde(rename = "harpSalzedoSlideWithSuppleness")]
    HarpSalzedoSlideWithSuppleness,
    /// Snare drum effect (Salzedo)
    #[serde(rename = "harpSalzedoSnareDrum")]
    HarpSalzedoSnareDrum,
    /// Tam-tam sounds (Salzedo)
    #[serde(rename = "harpSalzedoTamTamSounds")]
    HarpSalzedoTamTamSounds,
    /// Thunder effect (Salzedo)
    #[serde(rename = "harpSalzedoThunderEffect")]
    HarpSalzedoThunderEffect,
    /// Timpanic sounds (Salzedo)
    #[serde(rename = "harpSalzedoTimpanicSounds")]
    HarpSalzedoTimpanicSounds,
    /// Whistling sounds (Salzedo)
    #[serde(rename = "harpSalzedoWhistlingSounds")]
    HarpSalzedoWhistlingSounds,
    /// Combining string noise for stem
    #[serde(rename = "harpStringNoiseStem")]
    HarpStringNoiseStem,
    /// Tuning key pictogram
    #[serde(rename = "harpTuningKey")]
    HarpTuningKey,
    /// Retune strings for glissando
    #[serde(rename = "harpTuningKeyGlissando")]
    HarpTuningKeyGlissando,
    /// Use handle of tuning key pictogram
    #[serde(rename = "harpTuningKeyHandle")]
    HarpTuningKeyHandle,
    /// Use shank of tuning key pictogram
    #[serde(rename = "harpTuningKeyShank")]
    HarpTuningKeyShank,
    /// Indian drum clef
    #[serde(rename = "indianDrumClef")]
    IndianDrumClef,
    /// Back-chug
    #[serde(rename = "kahnBackChug")]
    KahnBackChug,
    /// Back-flap
    #[serde(rename = "kahnBackFlap")]
    KahnBackFlap,
    /// Back-riff
    #[serde(rename = "kahnBackRiff")]
    KahnBackRiff,
    /// Back-rip
    #[serde(rename = "kahnBackRip")]
    KahnBackRip,
    /// Ball-change
    #[serde(rename = "kahnBallChange")]
    KahnBallChange,
    /// Ball-dig
    #[serde(rename = "kahnBallDig")]
    KahnBallDig,
    /// Brush-backward
    #[serde(rename = "kahnBrushBackward")]
    KahnBrushBackward,
    /// Brush-forward
    #[serde(rename = "kahnBrushForward")]
    KahnBrushForward,
    /// Chug
    #[serde(rename = "kahnChug")]
    KahnChug,
    /// Clap
    #[serde(rename = "kahnClap")]
    KahnClap,
    /// Double-snap
    #[serde(rename = "kahnDoubleSnap")]
    KahnDoubleSnap,
    /// Double-wing
    #[serde(rename = "kahnDoubleWing")]
    KahnDoubleWing,
    /// Draw-step
    #[serde(rename = "kahnDrawStep")]
    KahnDrawStep,
    /// Draw-tap
    #[serde(rename = "kahnDrawTap")]
    KahnDrawTap,
    /// Flam
    #[serde(rename = "kahnFlam")]
    KahnFlam,
    /// Flap
    #[serde(rename = "kahnFlap")]
    KahnFlap,
    /// Flap-step
    #[serde(rename = "kahnFlapStep")]
    KahnFlapStep,
    /// Flat
    #[serde(rename = "kahnFlat")]
    KahnFlat,
    /// Flea-hop
    #[serde(rename = "kahnFleaHop")]
    KahnFleaHop,
    /// Flea-tap
    #[serde(rename = "kahnFleaTap")]
    KahnFleaTap,
    /// Grace-tap
    #[serde(rename = "kahnGraceTap")]
    KahnGraceTap,
    /// Grace-tap-change
    #[serde(rename = "kahnGraceTapChange")]
    KahnGraceTapChange,
    /// Grace-tap-hop
    #[serde(rename = "kahnGraceTapHop")]
    KahnGraceTapHop,
    /// Grace-tap-stamp
    #[serde(rename = "kahnGraceTapStamp")]
    KahnGraceTapStamp,
    /// Heel
    #[serde(rename = "kahnHeel")]
    KahnHeel,
    /// Heel-change
    #[serde(rename = "kahnHeelChange")]
    KahnHeelChange,
    /// Heel-click
    #[serde(rename = "kahnHeelClick")]
    KahnHeelClick,
    /// Heel-drop
    #[serde(rename = "kahnHeelDrop")]
    KahnHeelDrop,
    /// Heel-step
    #[serde(rename = "kahnHeelStep")]
    KahnHeelStep,
    /// Heel-tap
    #[serde(rename = "kahnHeelTap")]
    KahnHeelTap,
    /// Hop
    #[serde(rename = "kahnHop")]
    KahnHop,
    /// Jump-apart
    #[serde(rename = "kahnJumpApart")]
    KahnJumpApart,
    /// Jump-together
    #[serde(rename = "kahnJumpTogether")]
    KahnJumpTogether,
    /// Knee-inward
    #[serde(rename = "kahnKneeInward")]
    KahnKneeInward,
    /// Knee-outward
    #[serde(rename = "kahnKneeOutward")]
    KahnKneeOutward,
    /// Leap
    #[serde(rename = "kahnLeap")]
    KahnLeap,
    /// Leap-flat-foot
    #[serde(rename = "kahnLeapFlatFoot")]
    KahnLeapFlatFoot,
    /// Leap-heel-click
    #[serde(rename = "kahnLeapHeelClick")]
    KahnLeapHeelClick,
    /// Left-catch
    #[serde(rename = "kahnLeftCatch")]
    KahnLeftCatch,
    /// Left-cross
    #[serde(rename = "kahnLeftCross")]
    KahnLeftCross,
    /// Left-foot
    #[serde(rename = "kahnLeftFoot")]
    KahnLeftFoot,
    /// Left-toe-strike
    #[serde(rename = "kahnLeftToeStrike")]
    KahnLeftToeStrike,
    /// Left-turn
    #[serde(rename = "kahnLeftTurn")]
    KahnLeftTurn,
    /// Over-the-top
    #[serde(rename = "kahnOverTheTop")]
    KahnOverTheTop,
    /// Over-the-top-tap
    #[serde(rename = "kahnOverTheTopTap")]
    KahnOverTheTopTap,
    /// Pull
    #[serde(rename = "kahnPull")]
    KahnPull,
    /// Push
    #[serde(rename = "kahnPush")]
    KahnPush,
    /// Riff
    #[serde(rename = "kahnRiff")]
    KahnRiff,
    /// Riffle
    #[serde(rename = "kahnRiffle")]
    KahnRiffle,
    /// Right-catch
    #[serde(rename = "kahnRightCatch")]
    KahnRightCatch,
    /// Right-cross
    #[serde(rename = "kahnRightCross")]
    KahnRightCross,
    /// Right-foot
    #[serde(rename = "kahnRightFoot")]
    KahnRightFoot,
    /// Right-toe-strike
    #[serde(rename = "kahnRightToeStrike")]
    KahnRightToeStrike,
    /// Right-turn
    #[serde(rename = "kahnRightTurn")]
    KahnRightTurn,
    /// Rip
    #[serde(rename = "kahnRip")]
    KahnRip,
    /// Ripple
    #[serde(rename = "kahnRipple")]
    KahnRipple,
    /// Scrape
    #[serde(rename = "kahnScrape")]
    KahnScrape,
    /// Scuff
    #[serde(rename = "kahnScuff")]
    KahnScuff,
    /// Scuffle
    #[serde(rename = "kahnScuffle")]
    KahnScuffle,
    /// Shuffle
    #[serde(rename = "kahnShuffle")]
    KahnShuffle,
    /// Slam
    #[serde(rename = "kahnSlam")]
    KahnSlam,
    /// Slap
    #[serde(rename = "kahnSlap")]
    KahnSlap,
    /// Slide-step
    #[serde(rename = "kahnSlideStep")]
    KahnSlideStep,
    /// Slide-tap
    #[serde(rename = "kahnSlideTap")]
    KahnSlideTap,
    /// Snap
    #[serde(rename = "kahnSnap")]
    KahnSnap,
    /// Stamp
    #[serde(rename = "kahnStamp")]
    KahnStamp,
    /// Stamp-stamp
    #[serde(rename = "kahnStampStamp")]
    KahnStampStamp,
    /// Step
    #[serde(rename = "kahnStep")]
    KahnStep,
    /// Step-stamp
    #[serde(rename = "kahnStepStamp")]
    KahnStepStamp,
    /// Stomp
    #[serde(rename = "kahnStomp")]
    KahnStomp,
    /// Stomp-brush
    #[serde(rename = "kahnStompBrush")]
    KahnStompBrush,
    /// Tap
    #[serde(rename = "kahnTap")]
    KahnTap,
    /// Toe
    #[serde(rename = "kahnToe")]
    KahnToe,
    /// Toe-click
    #[serde(rename = "kahnToeClick")]
    KahnToeClick,
    /// Toe-drop
    #[serde(rename = "kahnToeDrop")]
    KahnToeDrop,
    /// Toe-step
    #[serde(rename = "kahnToeStep")]
    KahnToeStep,
    /// Toe-tap
    #[serde(rename = "kahnToeTap")]
    KahnToeTap,
    /// Trench
    #[serde(rename = "kahnTrench")]
    KahnTrench,
    /// Wing
    #[serde(rename = "kahnWing")]
    KahnWing,
    /// Wing-change
    #[serde(rename = "kahnWingChange")]
    KahnWingChange,
    /// Zank
    #[serde(rename = "kahnZank")]
    KahnZank,
    /// Zink
    #[serde(rename = "kahnZink")]
    KahnZink,
    /// Clavichord bebung, 2 finger movements (above)
    #[serde(rename = "keyboardBebung2DotsAbove")]
    KeyboardBebung2DotsAbove,
    /// Clavichord bebung, 2 finger movements (below)
    #[serde(rename = "keyboardBebung2DotsBelow")]
    KeyboardBebung2DotsBelow,
    /// Clavichord bebung, 3 finger movements (above)
    #[serde(rename = "keyboardBebung3DotsAbove")]
    KeyboardBebung3DotsAbove,
    /// Clavichord bebung, 3 finger movements (below)
    #[serde(rename = "keyboardBebung3DotsBelow")]
    KeyboardBebung3DotsBelow,
    /// Clavichord bebung, 4 finger movements (above)
    #[serde(rename = "keyboardBebung4DotsAbove")]
    KeyboardBebung4DotsAbove,
    /// Clavichord bebung, 4 finger movements (below)
    #[serde(rename = "keyboardBebung4DotsBelow")]
    KeyboardBebung4DotsBelow,
    /// Left pedal pictogram
    #[serde(rename = "keyboardLeftPedalPictogram")]
    KeyboardLeftPedalPictogram,
    /// Middle pedal pictogram
    #[serde(rename = "keyboardMiddlePedalPictogram")]
    KeyboardMiddlePedalPictogram,
    /// Pedal d
    #[serde(rename = "keyboardPedalD")]
    KeyboardPedalD,
    /// Pedal dot
    #[serde(rename = "keyboardPedalDot")]
    KeyboardPedalDot,
    /// Pedal e
    #[serde(rename = "keyboardPedalE")]
    KeyboardPedalE,
    /// Half-pedal mark
    #[serde(rename = "keyboardPedalHalf")]
    KeyboardPedalHalf,
    /// Half pedal mark 1
    #[serde(rename = "keyboardPedalHalf2")]
    KeyboardPedalHalf2,
    /// Half pedal mark 2
    #[serde(rename = "keyboardPedalHalf3")]
    KeyboardPedalHalf3,
    /// Pedal heel 1
    #[serde(rename = "keyboardPedalHeel1")]
    KeyboardPedalHeel1,
    /// Pedal heel 2
    #[serde(rename = "keyboardPedalHeel2")]
    KeyboardPedalHeel2,
    /// Pedal heel 3 (Davis)
    #[serde(rename = "keyboardPedalHeel3")]
    KeyboardPedalHeel3,
    /// Pedal heel to toe
    #[serde(rename = "keyboardPedalHeelToToe")]
    KeyboardPedalHeelToToe,
    /// Pedal heel or toe
    #[serde(rename = "keyboardPedalHeelToe")]
    KeyboardPedalHeelToe,
    /// Pedal hook end
    #[serde(rename = "keyboardPedalHookEnd")]
    KeyboardPedalHookEnd,
    /// Pedal hook start
    #[serde(rename = "keyboardPedalHookStart")]
    KeyboardPedalHookStart,
    /// Pedal hyphen
    #[serde(rename = "keyboardPedalHyphen")]
    KeyboardPedalHyphen,
    /// Pedal P
    #[serde(rename = "keyboardPedalP")]
    KeyboardPedalP,
    /// Left parenthesis for pedal marking
    #[serde(rename = "keyboardPedalParensLeft")]
    KeyboardPedalParensLeft,
    /// Right parenthesis for pedal marking
    #[serde(rename = "keyboardPedalParensRight")]
    KeyboardPedalParensRight,
    /// Pedal mark
    #[serde(rename = "keyboardPedalPed")]
    KeyboardPedalPed,
    /// Pedal S
    #[serde(rename = "keyboardPedalS")]
    KeyboardPedalS,
    /// Sostenuto pedal mark
    #[serde(rename = "keyboardPedalSost")]
    KeyboardPedalSost,
    /// Pedal toe 1
    #[serde(rename = "keyboardPedalToe1")]
    KeyboardPedalToe1,
    /// Pedal toe 2
    #[serde(rename = "keyboardPedalToe2")]
    KeyboardPedalToe2,
    /// Pedal toe to heel
    #[serde(rename = "keyboardPedalToeToHeel")]
    KeyboardPedalToeToHeel,
    /// Pedal up mark
    #[serde(rename = "keyboardPedalUp")]
    KeyboardPedalUp,
    /// Pedal up notch
    #[serde(rename = "keyboardPedalUpNotch")]
    KeyboardPedalUpNotch,
    /// Pedal up special
    #[serde(rename = "keyboardPedalUpSpecial")]
    KeyboardPedalUpSpecial,
    /// Play with left hand
    #[serde(rename = "keyboardPlayWithLH")]
    KeyboardPlayWithLh,
    /// Play with left hand (end)
    #[serde(rename = "keyboardPlayWithLHEnd")]
    KeyboardPlayWithLhEnd,
    /// Play with right hand
    #[serde(rename = "keyboardPlayWithRH")]
    KeyboardPlayWithRh,
    /// Play with right hand (end)
    #[serde(rename = "keyboardPlayWithRHEnd")]
    KeyboardPlayWithRhEnd,
    /// Pluck strings inside piano (Maderna)
    #[serde(rename = "keyboardPluckInside")]
    KeyboardPluckInside,
    /// Right pedal pictogram
    #[serde(rename = "keyboardRightPedalPictogram")]
    KeyboardRightPedalPictogram,
    /// Kievan flat
    #[serde(rename = "kievanAccidentalFlat")]
    KievanAccidentalFlat,
    /// Kievan sharp
    #[serde(rename = "kievanAccidentalSharp")]
    KievanAccidentalSharp,
    /// Kievan augmentation dot
    #[serde(rename = "kievanAugmentationDot")]
    KievanAugmentationDot,
    /// Kievan C clef (tse-fa-ut)
    #[serde(rename = "kievanCClef")]
    KievanCClef,
    /// Kievan ending symbol
    #[serde(rename = "kievanEndingSymbol")]
    KievanEndingSymbol,
    /// Kievan eighth note, stem down
    #[serde(rename = "kievanNote8thStemDown")]
    KievanNote8ThStemDown,
    /// Kievan eighth note, stem up
    #[serde(rename = "kievanNote8thStemUp")]
    KievanNote8ThStemUp,
    /// Kievan beam
    #[serde(rename = "kievanNoteBeam")]
    KievanNoteBeam,
    /// Kievan half note (on staff line)
    #[serde(rename = "kievanNoteHalfStaffLine")]
    KievanNoteHalfStaffLine,
    /// Kievan half note (in staff space)
    #[serde(rename = "kievanNoteHalfStaffSpace")]
    KievanNoteHalfStaffSpace,
    /// Kievan quarter note, stem down
    #[serde(rename = "kievanNoteQuarterStemDown")]
    KievanNoteQuarterStemDown,
    /// Kievan quarter note, stem up
    #[serde(rename = "kievanNoteQuarterStemUp")]
    KievanNoteQuarterStemUp,
    /// Kievan reciting note
    #[serde(rename = "kievanNoteReciting")]
    KievanNoteReciting,
    /// Kievan whole note
    #[serde(rename = "kievanNoteWhole")]
    KievanNoteWhole,
    /// Kievan final whole note
    #[serde(rename = "kievanNoteWholeFinal")]
    KievanNoteWholeFinal,
    /// Do hand sign
    #[serde(rename = "kodalyHandDo")]
    KodalyHandDo,
    /// Fa hand sign
    #[serde(rename = "kodalyHandFa")]
    KodalyHandFa,
    /// La hand sign
    #[serde(rename = "kodalyHandLa")]
    KodalyHandLa,
    /// Mi hand sign
    #[serde(rename = "kodalyHandMi")]
    KodalyHandMi,
    /// Re hand sign
    #[serde(rename = "kodalyHandRe")]
    KodalyHandRe,
    /// So hand sign
    #[serde(rename = "kodalyHandSo")]
    KodalyHandSo,
    /// Ti hand sign
    #[serde(rename = "kodalyHandTi")]
    KodalyHandTi,
    /// Left repeat sign within bar
    #[serde(rename = "leftRepeatSmall")]
    LeftRepeatSmall,
    /// Leger line
    #[serde(rename = "legerLine")]
    LegerLine,
    /// Leger line (narrow)
    #[serde(rename = "legerLineNarrow")]
    LegerLineNarrow,
    /// Leger line (wide)
    #[serde(rename = "legerLineWide")]
    LegerLineWide,
    /// Lute tablature end repeat barline
    #[serde(rename = "luteBarlineEndRepeat")]
    LuteBarlineEndRepeat,
    /// Lute tablature final barline
    #[serde(rename = "luteBarlineFinal")]
    LuteBarlineFinal,
    /// Lute tablature start repeat barline
    #[serde(rename = "luteBarlineStartRepeat")]
    LuteBarlineStartRepeat,
    /// 16th note (semiquaver) duration sign
    #[serde(rename = "luteDuration16th")]
    LuteDuration16Th,
    /// 32nd note (demisemiquaver) duration sign
    #[serde(rename = "luteDuration32nd")]
    LuteDuration32Nd,
    /// Eighth note (quaver) duration sign
    #[serde(rename = "luteDuration8th")]
    LuteDuration8Th,
    /// Double whole note (breve) duration sign
    #[serde(rename = "luteDurationDoubleWhole")]
    LuteDurationDoubleWhole,
    /// Half note (minim) duration sign
    #[serde(rename = "luteDurationHalf")]
    LuteDurationHalf,
    /// Quarter note (crotchet) duration sign
    #[serde(rename = "luteDurationQuarter")]
    LuteDurationQuarter,
    /// Whole note (semibreve) duration sign
    #[serde(rename = "luteDurationWhole")]
    LuteDurationWhole,
    /// Right-hand fingering, first finger
    #[serde(rename = "luteFingeringRHFirst")]
    LuteFingeringRhFirst,
    /// Right-hand fingering, second finger
    #[serde(rename = "luteFingeringRHSecond")]
    LuteFingeringRhSecond,
    /// Right-hand fingering, third finger
    #[serde(rename = "luteFingeringRHThird")]
    LuteFingeringRhThird,
    /// Right-hand fingering, thumb
    #[serde(rename = "luteFingeringRHThumb")]
    LuteFingeringRhThumb,
    /// 10th course (diapason)
    #[serde(rename = "luteFrench10thCourse")]
    LuteFrench10ThCourse,
    /// Seventh course (diapason)
    #[serde(rename = "luteFrench7thCourse")]
    LuteFrench7ThCourse,
    /// Eighth course (diapason)
    #[serde(rename = "luteFrench8thCourse")]
    LuteFrench8ThCourse,
    /// Ninth course (diapason)
    #[serde(rename = "luteFrench9thCourse")]
    LuteFrench9ThCourse,
    /// Appoggiatura from above
    #[serde(rename = "luteFrenchAppoggiaturaAbove")]
    LuteFrenchAppoggiaturaAbove,
    /// Appoggiatura from below
    #[serde(rename = "luteFrenchAppoggiaturaBelow")]
    LuteFrenchAppoggiaturaBelow,
    /// Open string (a)
    #[serde(rename = "luteFrenchFretA")]
    LuteFrenchFretA,
    /// First fret (b)
    #[serde(rename = "luteFrenchFretB")]
    LuteFrenchFretB,
    /// Second fret (c)
    #[serde(rename = "luteFrenchFretC")]
    LuteFrenchFretC,
    /// Third fret (d)
    #[serde(rename = "luteFrenchFretD")]
    LuteFrenchFretD,
    /// Fourth fret (e)
    #[serde(rename = "luteFrenchFretE")]
    LuteFrenchFretE,
    /// Fifth fret (f)
    #[serde(rename = "luteFrenchFretF")]
    LuteFrenchFretF,
    /// Sixth fret (g)
    #[serde(rename = "luteFrenchFretG")]
    LuteFrenchFretG,
    /// Seventh fret (h)
    #[serde(rename = "luteFrenchFretH")]
    LuteFrenchFretH,
    /// Eighth fret (i)
    #[serde(rename = "luteFrenchFretI")]
    LuteFrenchFretI,
    /// Ninth fret (k)
    #[serde(rename = "luteFrenchFretK")]
    LuteFrenchFretK,
    /// 10th fret (l)
    #[serde(rename = "luteFrenchFretL")]
    LuteFrenchFretL,
    /// 11th fret (m)
    #[serde(rename = "luteFrenchFretM")]
    LuteFrenchFretM,
    /// 12th fret (n)
    #[serde(rename = "luteFrenchFretN")]
    LuteFrenchFretN,
    /// Inverted mordent
    #[serde(rename = "luteFrenchMordentInverted")]
    LuteFrenchMordentInverted,
    /// Mordent with lower auxiliary
    #[serde(rename = "luteFrenchMordentLower")]
    LuteFrenchMordentLower,
    /// Mordent with upper auxiliary
    #[serde(rename = "luteFrenchMordentUpper")]
    LuteFrenchMordentUpper,
    /// 5th course, 1st fret (a)
    #[serde(rename = "luteGermanALower")]
    LuteGermanALower,
    /// 6th course, 1st fret (A)
    #[serde(rename = "luteGermanAUpper")]
    LuteGermanAUpper,
    /// 4th course, 1st fret (b)
    #[serde(rename = "luteGermanBLower")]
    LuteGermanBLower,
    /// 6th course, 2nd fret (B)
    #[serde(rename = "luteGermanBUpper")]
    LuteGermanBUpper,
    /// 3rd course, 1st fret (c)
    #[serde(rename = "luteGermanCLower")]
    LuteGermanCLower,
    /// 6th course, 3rd fret (C)
    #[serde(rename = "luteGermanCUpper")]
    LuteGermanCUpper,
    /// 2nd course, 1st fret (d)
    #[serde(rename = "luteGermanDLower")]
    LuteGermanDLower,
    /// 6th course, 4th fret (D)
    #[serde(rename = "luteGermanDUpper")]
    LuteGermanDUpper,
    /// 1st course, 1st fret (e)
    #[serde(rename = "luteGermanELower")]
    LuteGermanELower,
    /// 6th course, 5th fret (E)
    #[serde(rename = "luteGermanEUpper")]
    LuteGermanEUpper,
    /// 5th course, 2nd fret (f)
    #[serde(rename = "luteGermanFLower")]
    LuteGermanFLower,
    /// 6th course, 6th fret (F)
    #[serde(rename = "luteGermanFUpper")]
    LuteGermanFUpper,
    /// 4th course, 2nd fret (g)
    #[serde(rename = "luteGermanGLower")]
    LuteGermanGLower,
    /// 6th course, 7th fret (G)
    #[serde(rename = "luteGermanGUpper")]
    LuteGermanGUpper,
    /// 3rd course, 2nd fret (h)
    #[serde(rename = "luteGermanHLower")]
    LuteGermanHLower,
    /// 6th course, 8th fret (H)
    #[serde(rename = "luteGermanHUpper")]
    LuteGermanHUpper,
    /// 2nd course, 2nd fret (i)
    #[serde(rename = "luteGermanILower")]
    LuteGermanILower,
    /// 6th course, 9th fret (I)
    #[serde(rename = "luteGermanIUpper")]
    LuteGermanIUpper,
    /// 1st course, 2nd fret (k)
    #[serde(rename = "luteGermanKLower")]
    LuteGermanKLower,
    /// 6th course, 10th fret (K)
    #[serde(rename = "luteGermanKUpper")]
    LuteGermanKUpper,
    /// 5th course, 3rd fret (l)
    #[serde(rename = "luteGermanLLower")]
    LuteGermanLLower,
    /// 6th course, 11th fret (L)
    #[serde(rename = "luteGermanLUpper")]
    LuteGermanLUpper,
    /// 4th course, 3rd fret (m)
    #[serde(rename = "luteGermanMLower")]
    LuteGermanMLower,
    /// 6th course, 12th fret (M)
    #[serde(rename = "luteGermanMUpper")]
    LuteGermanMUpper,
    /// 3rd course, 3rd fret (n)
    #[serde(rename = "luteGermanNLower")]
    LuteGermanNLower,
    /// 6th course, 13th fret (N)
    #[serde(rename = "luteGermanNUpper")]
    LuteGermanNUpper,
    /// 2nd course, 3rd fret (o)
    #[serde(rename = "luteGermanOLower")]
    LuteGermanOLower,
    /// 1st course, 3rd fret (p)
    #[serde(rename = "luteGermanPLower")]
    LuteGermanPLower,
    /// 5th course, 4th fret (q)
    #[serde(rename = "luteGermanQLower")]
    LuteGermanQLower,
    /// 4th course, 4th fret (r)
    #[serde(rename = "luteGermanRLower")]
    LuteGermanRLower,
    /// 3rd course, 4th fret (s)
    #[serde(rename = "luteGermanSLower")]
    LuteGermanSLower,
    /// 2nd course, 4th fret (t)
    #[serde(rename = "luteGermanTLower")]
    LuteGermanTLower,
    /// 1st course, 4th fret (v)
    #[serde(rename = "luteGermanVLower")]
    LuteGermanVLower,
    /// 5th course, 5th fret (x)
    #[serde(rename = "luteGermanXLower")]
    LuteGermanXLower,
    /// 4th course, 5th fret (y)
    #[serde(rename = "luteGermanYLower")]
    LuteGermanYLower,
    /// 3rd course, 5th fret (z)
    #[serde(rename = "luteGermanZLower")]
    LuteGermanZLower,
    /// C sol fa ut clef
    #[serde(rename = "luteItalianClefCSolFaUt")]
    LuteItalianClefCSolFaUt,
    /// F fa ut clef
    #[serde(rename = "luteItalianClefFFaUt")]
    LuteItalianClefFFaUt,
    /// Open string (0)
    #[serde(rename = "luteItalianFret0")]
    LuteItalianFret0,
    /// First fret (1)
    #[serde(rename = "luteItalianFret1")]
    LuteItalianFret1,
    /// Second fret (2)
    #[serde(rename = "luteItalianFret2")]
    LuteItalianFret2,
    /// Third fret (3)
    #[serde(rename = "luteItalianFret3")]
    LuteItalianFret3,
    /// Fourth fret (4)
    #[serde(rename = "luteItalianFret4")]
    LuteItalianFret4,
    /// Fifth fret (5)
    #[serde(rename = "luteItalianFret5")]
    LuteItalianFret5,
    /// Sixth fret (6)
    #[serde(rename = "luteItalianFret6")]
    LuteItalianFret6,
    /// Seventh fret (7)
    #[serde(rename = "luteItalianFret7")]
    LuteItalianFret7,
    /// Eighth fret (8)
    #[serde(rename = "luteItalianFret8")]
    LuteItalianFret8,
    /// Ninth fret (9)
    #[serde(rename = "luteItalianFret9")]
    LuteItalianFret9,
    /// Hold finger in place
    #[serde(rename = "luteItalianHoldFinger")]
    LuteItalianHoldFinger,
    /// Hold note
    #[serde(rename = "luteItalianHoldNote")]
    LuteItalianHoldNote,
    /// Release finger
    #[serde(rename = "luteItalianReleaseFinger")]
    LuteItalianReleaseFinger,
    /// Fast tempo indication (de Mudarra)
    #[serde(rename = "luteItalianTempoFast")]
    LuteItalianTempoFast,
    /// Neither fast nor slow tempo indication (de Mudarra)
    #[serde(rename = "luteItalianTempoNeitherFastNorSlow")]
    LuteItalianTempoNeitherFastNorSlow,
    /// Slow tempo indication (de Mudarra)
    #[serde(rename = "luteItalianTempoSlow")]
    LuteItalianTempoSlow,
    /// Somewhat fast tempo indication (de Narvaez)
    #[serde(rename = "luteItalianTempoSomewhatFast")]
    LuteItalianTempoSomewhatFast,
    /// Very slow indication (de Narvaez)
    #[serde(rename = "luteItalianTempoVerySlow")]
    LuteItalianTempoVerySlow,
    /// Triple time indication
    #[serde(rename = "luteItalianTimeTriple")]
    LuteItalianTimeTriple,
    /// Single-finger tremolo or mordent
    #[serde(rename = "luteItalianTremolo")]
    LuteItalianTremolo,
    /// Vibrato (verre cassé)
    #[serde(rename = "luteItalianVibrato")]
    LuteItalianVibrato,
    /// Lute tablature staff, 6 courses
    #[serde(rename = "luteStaff6Lines")]
    LuteStaff6Lines,
    /// Lute tablature staff, 6 courses (narrow)
    #[serde(rename = "luteStaff6LinesNarrow")]
    LuteStaff6LinesNarrow,
    /// Lute tablature staff, 6 courses (wide)
    #[serde(rename = "luteStaff6LinesWide")]
    LuteStaff6LinesWide,
    /// Elision
    #[serde(rename = "lyricsElision")]
    LyricsElision,
    /// Narrow elision
    #[serde(rename = "lyricsElisionNarrow")]
    LyricsElisionNarrow,
    /// Wide elision
    #[serde(rename = "lyricsElisionWide")]
    LyricsElisionWide,
    /// Baseline hyphen
    #[serde(rename = "lyricsHyphenBaseline")]
    LyricsHyphenBaseline,
    /// Non-breaking baseline hyphen
    #[serde(rename = "lyricsHyphenBaselineNonBreaking")]
    LyricsHyphenBaselineNonBreaking,
    /// Text repeats
    #[serde(rename = "lyricsTextRepeat")]
    LyricsTextRepeat,
    /// Flat, hard b (mi)
    #[serde(rename = "medRenFlatHardB")]
    MedRenFlatHardB,
    /// Flat, soft b (fa)
    #[serde(rename = "medRenFlatSoftB")]
    MedRenFlatSoftB,
    /// Flat with dot
    #[serde(rename = "medRenFlatWithDot")]
    MedRenFlatWithDot,
    /// G clef (Corpus Monodicum)
    #[serde(rename = "medRenGClefCMN")]
    MedRenGClefCmn,
    /// Liquescence
    #[serde(rename = "medRenLiquescenceCMN")]
    MedRenLiquescenceCmn,
    /// Liquescent ascending (Corpus Monodicum)
    #[serde(rename = "medRenLiquescentAscCMN")]
    MedRenLiquescentAscCmn,
    /// Liquescent descending (Corpus Monodicum)
    #[serde(rename = "medRenLiquescentDescCMN")]
    MedRenLiquescentDescCmn,
    /// Natural
    #[serde(rename = "medRenNatural")]
    MedRenNatural,
    /// Natural with interrupted cross
    #[serde(rename = "medRenNaturalWithCross")]
    MedRenNaturalWithCross,
    /// Oriscus (Corpus Monodicum)
    #[serde(rename = "medRenOriscusCMN")]
    MedRenOriscusCmn,
    /// Plica
    #[serde(rename = "medRenPlicaCMN")]
    MedRenPlicaCmn,
    /// Punctum (Corpus Monodicum)
    #[serde(rename = "medRenPunctumCMN")]
    MedRenPunctumCmn,
    /// Quilisma (Corpus Monodicum)
    #[serde(rename = "medRenQuilismaCMN")]
    MedRenQuilismaCmn,
    /// Croix
    #[serde(rename = "medRenSharpCroix")]
    MedRenSharpCroix,
    /// Strophicus (Corpus Monodicum)
    #[serde(rename = "medRenStrophicusCMN")]
    MedRenStrophicusCmn,
    /// Alteration sign
    #[serde(rename = "mensuralAlterationSign")]
    MensuralAlterationSign,
    /// Black mensural brevis
    #[serde(rename = "mensuralBlackBrevis")]
    MensuralBlackBrevis,
    /// Black mensural void brevis
    #[serde(rename = "mensuralBlackBrevisVoid")]
    MensuralBlackBrevisVoid,
    /// Black mensural dragma
    #[serde(rename = "mensuralBlackDragma")]
    MensuralBlackDragma,
    /// Black mensural longa
    #[serde(rename = "mensuralBlackLonga")]
    MensuralBlackLonga,
    /// Black mensural maxima
    #[serde(rename = "mensuralBlackMaxima")]
    MensuralBlackMaxima,
    /// Black mensural minima
    #[serde(rename = "mensuralBlackMinima")]
    MensuralBlackMinima,
    /// Black mensural void minima
    #[serde(rename = "mensuralBlackMinimaVoid")]
    MensuralBlackMinimaVoid,
    /// Black mensural semibrevis
    #[serde(rename = "mensuralBlackSemibrevis")]
    MensuralBlackSemibrevis,
    /// Black mensural semibrevis caudata
    #[serde(rename = "mensuralBlackSemibrevisCaudata")]
    MensuralBlackSemibrevisCaudata,
    /// Black mensural oblique semibrevis
    #[serde(rename = "mensuralBlackSemibrevisOblique")]
    MensuralBlackSemibrevisOblique,
    /// Black mensural void semibrevis
    #[serde(rename = "mensuralBlackSemibrevisVoid")]
    MensuralBlackSemibrevisVoid,
    /// Black mensural semiminima
    #[serde(rename = "mensuralBlackSemiminima")]
    MensuralBlackSemiminima,
    /// Mensural C clef
    #[serde(rename = "mensuralCclef")]
    MensuralCclef,
    /// Petrucci C clef, high position
    #[serde(rename = "mensuralCclefPetrucciPosHigh")]
    MensuralCclefPetrucciPosHigh,
    /// Petrucci C clef, highest position
    #[serde(rename = "mensuralCclefPetrucciPosHighest")]
    MensuralCclefPetrucciPosHighest,
    /// Petrucci C clef, low position
    #[serde(rename = "mensuralCclefPetrucciPosLow")]
    MensuralCclefPetrucciPosLow,
    /// Petrucci C clef, lowest position
    #[serde(rename = "mensuralCclefPetrucciPosLowest")]
    MensuralCclefPetrucciPosLowest,
    /// Petrucci C clef, middle position
    #[serde(rename = "mensuralCclefPetrucciPosMiddle")]
    MensuralCclefPetrucciPosMiddle,
    /// Coloration end, round
    #[serde(rename = "mensuralColorationEndRound")]
    MensuralColorationEndRound,
    /// Coloration end, square
    #[serde(rename = "mensuralColorationEndSquare")]
    MensuralColorationEndSquare,
    /// Coloration start, round
    #[serde(rename = "mensuralColorationStartRound")]
    MensuralColorationStartRound,
    /// Coloration start, square
    #[serde(rename = "mensuralColorationStartSquare")]
    MensuralColorationStartSquare,
    /// Combining stem diagonal
    #[serde(rename = "mensuralCombStemDiagonal")]
    MensuralCombStemDiagonal,
    /// Combining stem down
    #[serde(rename = "mensuralCombStemDown")]
    MensuralCombStemDown,
    /// Combining stem with extended flag down
    #[serde(rename = "mensuralCombStemDownFlagExtended")]
    MensuralCombStemDownFlagExtended,
    /// Combining stem with flared flag down
    #[serde(rename = "mensuralCombStemDownFlagFlared")]
    MensuralCombStemDownFlagFlared,
    /// Combining stem with fusa flag down
    #[serde(rename = "mensuralCombStemDownFlagFusa")]
    MensuralCombStemDownFlagFusa,
    /// Combining stem with flag left down
    #[serde(rename = "mensuralCombStemDownFlagLeft")]
    MensuralCombStemDownFlagLeft,
    /// Combining stem with flag right down
    #[serde(rename = "mensuralCombStemDownFlagRight")]
    MensuralCombStemDownFlagRight,
    /// Combining stem with semiminima flag down
    #[serde(rename = "mensuralCombStemDownFlagSemiminima")]
    MensuralCombStemDownFlagSemiminima,
    /// Combining stem up
    #[serde(rename = "mensuralCombStemUp")]
    MensuralCombStemUp,
    /// Combining stem with extended flag up
    #[serde(rename = "mensuralCombStemUpFlagExtended")]
    MensuralCombStemUpFlagExtended,
    /// Combining stem with flared flag up
    #[serde(rename = "mensuralCombStemUpFlagFlared")]
    MensuralCombStemUpFlagFlared,
    /// Combining stem with fusa flag up
    #[serde(rename = "mensuralCombStemUpFlagFusa")]
    MensuralCombStemUpFlagFusa,
    /// Combining stem with flag left up
    #[serde(rename = "mensuralCombStemUpFlagLeft")]
    MensuralCombStemUpFlagLeft,
    /// Combining stem with flag right up
    #[serde(rename = "mensuralCombStemUpFlagRight")]
    MensuralCombStemUpFlagRight,
    /// Combining stem with semiminima flag up
    #[serde(rename = "mensuralCombStemUpFlagSemiminima")]
    MensuralCombStemUpFlagSemiminima,
    /// Checkmark custos
    #[serde(rename = "mensuralCustosCheckmark")]
    MensuralCustosCheckmark,
    /// Mensural custos down
    #[serde(rename = "mensuralCustosDown")]
    MensuralCustosDown,
    /// Turn-like custos
    #[serde(rename = "mensuralCustosTurn")]
    MensuralCustosTurn,
    /// Mensural custos up
    #[serde(rename = "mensuralCustosUp")]
    MensuralCustosUp,
    /// Mensural F clef
    #[serde(rename = "mensuralFclef")]
    MensuralFclef,
    /// Petrucci F clef
    #[serde(rename = "mensuralFclefPetrucci")]
    MensuralFclefPetrucci,
    /// Mensural G clef
    #[serde(rename = "mensuralGclef")]
    MensuralGclef,
    /// Petrucci G clef
    #[serde(rename = "mensuralGclefPetrucci")]
    MensuralGclefPetrucci,
    /// Modus imperfectum, vertical
    #[serde(rename = "mensuralModusImperfectumVert")]
    MensuralModusImperfectumVert,
    /// Modus perfectum, vertical
    #[serde(rename = "mensuralModusPerfectumVert")]
    MensuralModusPerfectumVert,
    /// Longa/brevis notehead, black
    #[serde(rename = "mensuralNoteheadLongaBlack")]
    MensuralNoteheadLongaBlack,
    /// Longa/brevis notehead, black and void
    #[serde(rename = "mensuralNoteheadLongaBlackVoid")]
    MensuralNoteheadLongaBlackVoid,
    /// Longa/brevis notehead, void
    #[serde(rename = "mensuralNoteheadLongaVoid")]
    MensuralNoteheadLongaVoid,
    /// Longa/brevis notehead, white
    #[serde(rename = "mensuralNoteheadLongaWhite")]
    MensuralNoteheadLongaWhite,
    /// Maxima notehead, black
    #[serde(rename = "mensuralNoteheadMaximaBlack")]
    MensuralNoteheadMaximaBlack,
    /// Maxima notehead, black and void
    #[serde(rename = "mensuralNoteheadMaximaBlackVoid")]
    MensuralNoteheadMaximaBlackVoid,
    /// Maxima notehead, void
    #[serde(rename = "mensuralNoteheadMaximaVoid")]
    MensuralNoteheadMaximaVoid,
    /// Maxima notehead, white
    #[serde(rename = "mensuralNoteheadMaximaWhite")]
    MensuralNoteheadMaximaWhite,
    /// Minima notehead, white
    #[serde(rename = "mensuralNoteheadMinimaWhite")]
    MensuralNoteheadMinimaWhite,
    /// Semibrevis notehead, black
    #[serde(rename = "mensuralNoteheadSemibrevisBlack")]
    MensuralNoteheadSemibrevisBlack,
    /// Semibrevis notehead, black and void
    #[serde(rename = "mensuralNoteheadSemibrevisBlackVoid")]
    MensuralNoteheadSemibrevisBlackVoid,
    /// Semibrevis notehead, black and void (turned)
    #[serde(rename = "mensuralNoteheadSemibrevisBlackVoidTurned")]
    MensuralNoteheadSemibrevisBlackVoidTurned,
    /// Semibrevis notehead, void
    #[serde(rename = "mensuralNoteheadSemibrevisVoid")]
    MensuralNoteheadSemibrevisVoid,
    /// Semiminima/fusa notehead, white
    #[serde(rename = "mensuralNoteheadSemiminimaWhite")]
    MensuralNoteheadSemiminimaWhite,
    /// Oblique form, ascending 2nd, black
    #[serde(rename = "mensuralObliqueAsc2ndBlack")]
    MensuralObliqueAsc2NdBlack,
    /// Oblique form, ascending 2nd, black and void
    #[serde(rename = "mensuralObliqueAsc2ndBlackVoid")]
    MensuralObliqueAsc2NdBlackVoid,
    /// Oblique form, ascending 2nd, void
    #[serde(rename = "mensuralObliqueAsc2ndVoid")]
    MensuralObliqueAsc2NdVoid,
    /// Oblique form, ascending 2nd, white
    #[serde(rename = "mensuralObliqueAsc2ndWhite")]
    MensuralObliqueAsc2NdWhite,
    /// Oblique form, ascending 3rd, black
    #[serde(rename = "mensuralObliqueAsc3rdBlack")]
    MensuralObliqueAsc3RdBlack,
    /// Oblique form, ascending 3rd, black and void
    #[serde(rename = "mensuralObliqueAsc3rdBlackVoid")]
    MensuralObliqueAsc3RdBlackVoid,
    /// Oblique form, ascending 3rd, void
    #[serde(rename = "mensuralObliqueAsc3rdVoid")]
    MensuralObliqueAsc3RdVoid,
    /// Oblique form, ascending 3rd, white
    #[serde(rename = "mensuralObliqueAsc3rdWhite")]
    MensuralObliqueAsc3RdWhite,
    /// Oblique form, ascending 4th, black
    #[serde(rename = "mensuralObliqueAsc4thBlack")]
    MensuralObliqueAsc4ThBlack,
    /// Oblique form, ascending 4th, black and void
    #[serde(rename = "mensuralObliqueAsc4thBlackVoid")]
    MensuralObliqueAsc4ThBlackVoid,
    /// Oblique form, ascending 4th, void
    #[serde(rename = "mensuralObliqueAsc4thVoid")]
    MensuralObliqueAsc4ThVoid,
    /// Oblique form, ascending 4th, white
    #[serde(rename = "mensuralObliqueAsc4thWhite")]
    MensuralObliqueAsc4ThWhite,
    /// Oblique form, ascending 5th, black
    #[serde(rename = "mensuralObliqueAsc5thBlack")]
    MensuralObliqueAsc5ThBlack,
    /// Oblique form, ascending 5th, black and void
    #[serde(rename = "mensuralObliqueAsc5thBlackVoid")]
    MensuralObliqueAsc5ThBlackVoid,
    /// Oblique form, ascending 5th, void
    #[serde(rename = "mensuralObliqueAsc5thVoid")]
    MensuralObliqueAsc5ThVoid,
    /// Oblique form, ascending 5th, white
    #[serde(rename = "mensuralObliqueAsc5thWhite")]
    MensuralObliqueAsc5ThWhite,
    /// Oblique form, descending 2nd, black
    #[serde(rename = "mensuralObliqueDesc2ndBlack")]
    MensuralObliqueDesc2NdBlack,
    /// Oblique form, descending 2nd, black and void
    #[serde(rename = "mensuralObliqueDesc2ndBlackVoid")]
    MensuralObliqueDesc2NdBlackVoid,
    /// Oblique form, descending 2nd, void
    #[serde(rename = "mensuralObliqueDesc2ndVoid")]
    MensuralObliqueDesc2NdVoid,
    /// Oblique form, descending 2nd, white
    #[serde(rename = "mensuralObliqueDesc2ndWhite")]
    MensuralObliqueDesc2NdWhite,
    /// Oblique form, descending 3rd, black
    #[serde(rename = "mensuralObliqueDesc3rdBlack")]
    MensuralObliqueDesc3RdBlack,
    /// Oblique form, descending 3rd, black and void
    #[serde(rename = "mensuralObliqueDesc3rdBlackVoid")]
    MensuralObliqueDesc3RdBlackVoid,
    /// Oblique form, descending 3rd, void
    #[serde(rename = "mensuralObliqueDesc3rdVoid")]
    MensuralObliqueDesc3RdVoid,
    /// Oblique form, descending 3rd, white
    #[serde(rename = "mensuralObliqueDesc3rdWhite")]
    MensuralObliqueDesc3RdWhite,
    /// Oblique form, descending 4th, black
    #[serde(rename = "mensuralObliqueDesc4thBlack")]
    MensuralObliqueDesc4ThBlack,
    /// Oblique form, descending 4th, black and void
    #[serde(rename = "mensuralObliqueDesc4thBlackVoid")]
    MensuralObliqueDesc4ThBlackVoid,
    /// Oblique form, descending 4th, void
    #[serde(rename = "mensuralObliqueDesc4thVoid")]
    MensuralObliqueDesc4ThVoid,
    /// Oblique form, descending 4th, white
    #[serde(rename = "mensuralObliqueDesc4thWhite")]
    MensuralObliqueDesc4ThWhite,
    /// Oblique form, descending 5th, black
    #[serde(rename = "mensuralObliqueDesc5thBlack")]
    MensuralObliqueDesc5ThBlack,
    /// Oblique form, descending 5th, black and void
    #[serde(rename = "mensuralObliqueDesc5thBlackVoid")]
    MensuralObliqueDesc5ThBlackVoid,
    /// Oblique form, descending 5th, void
    #[serde(rename = "mensuralObliqueDesc5thVoid")]
    MensuralObliqueDesc5ThVoid,
    /// Oblique form, descending 5th, white
    #[serde(rename = "mensuralObliqueDesc5thWhite")]
    MensuralObliqueDesc5ThWhite,
    /// Tempus perfectum cum prolatione perfecta (9/8)
    #[serde(rename = "mensuralProlation1")]
    MensuralProlation1,
    /// Tempus imperfectum cum prolatione imperfecta diminution 4
    #[serde(rename = "mensuralProlation10")]
    MensuralProlation10,
    /// Tempus imperfectum cum prolatione imperfecta diminution 5
    #[serde(rename = "mensuralProlation11")]
    MensuralProlation11,
    /// Tempus perfectum cum prolatione imperfecta (3/4)
    #[serde(rename = "mensuralProlation2")]
    MensuralProlation2,
    /// Tempus perfectum cum prolatione imperfecta diminution 1 (3/8)
    #[serde(rename = "mensuralProlation3")]
    MensuralProlation3,
    /// Tempus perfectum cum prolatione perfecta diminution 2 (9/16)
    #[serde(rename = "mensuralProlation4")]
    MensuralProlation4,
    /// Tempus imperfectum cum prolatione perfecta (6/8)
    #[serde(rename = "mensuralProlation5")]
    MensuralProlation5,
    /// Tempus imperfectum cum prolatione imperfecta (2/4)
    #[serde(rename = "mensuralProlation6")]
    MensuralProlation6,
    /// Tempus imperfectum cum prolatione imperfecta diminution 1 (2/2)
    #[serde(rename = "mensuralProlation7")]
    MensuralProlation7,
    /// Tempus imperfectum cum prolatione imperfecta diminution 2 (6/16)
    #[serde(rename = "mensuralProlation8")]
    MensuralProlation8,
    /// Tempus imperfectum cum prolatione imperfecta diminution 3 (2/2)
    #[serde(rename = "mensuralProlation9")]
    MensuralProlation9,
    /// Combining dot
    #[serde(rename = "mensuralProlationCombiningDot")]
    MensuralProlationCombiningDot,
    /// Combining void dot
    #[serde(rename = "mensuralProlationCombiningDotVoid")]
    MensuralProlationCombiningDotVoid,
    /// Combining vertical stroke
    #[serde(rename = "mensuralProlationCombiningStroke")]
    MensuralProlationCombiningStroke,
    /// Combining three dots horizontal
    #[serde(rename = "mensuralProlationCombiningThreeDots")]
    MensuralProlationCombiningThreeDots,
    /// Combining three dots triangular
    #[serde(rename = "mensuralProlationCombiningThreeDotsTri")]
    MensuralProlationCombiningThreeDotsTri,
    /// Combining two dots
    #[serde(rename = "mensuralProlationCombiningTwoDots")]
    MensuralProlationCombiningTwoDots,
    /// Mensural proportion 1
    #[serde(rename = "mensuralProportion1")]
    MensuralProportion1,
    /// Mensural proportion 2
    #[serde(rename = "mensuralProportion2")]
    MensuralProportion2,
    /// Mensural proportion 3
    #[serde(rename = "mensuralProportion3")]
    MensuralProportion3,
    /// Mensural proportion 4
    #[serde(rename = "mensuralProportion4")]
    MensuralProportion4,
    /// Mensural proportion 5
    #[serde(rename = "mensuralProportion5")]
    MensuralProportion5,
    /// Mensural proportion 6
    #[serde(rename = "mensuralProportion6")]
    MensuralProportion6,
    /// Mensural proportion 7
    #[serde(rename = "mensuralProportion7")]
    MensuralProportion7,
    /// Mensural proportion 8
    #[serde(rename = "mensuralProportion8")]
    MensuralProportion8,
    /// Mensural proportion 9
    #[serde(rename = "mensuralProportion9")]
    MensuralProportion9,
    /// Mensural proportion major
    #[serde(rename = "mensuralProportionMajor")]
    MensuralProportionMajor,
    /// Mensural proportion minor
    #[serde(rename = "mensuralProportionMinor")]
    MensuralProportionMinor,
    /// Proportio dupla 1
    #[serde(rename = "mensuralProportionProportioDupla1")]
    MensuralProportionProportioDupla1,
    /// Proportio dupla 2
    #[serde(rename = "mensuralProportionProportioDupla2")]
    MensuralProportionProportioDupla2,
    /// Proportio quadrupla
    #[serde(rename = "mensuralProportionProportioQuadrupla")]
    MensuralProportionProportioQuadrupla,
    /// Proportio tripla
    #[serde(rename = "mensuralProportionProportioTripla")]
    MensuralProportionProportioTripla,
    /// Tempus perfectum
    #[serde(rename = "mensuralProportionTempusPerfectum")]
    MensuralProportionTempusPerfectum,
    /// Brevis rest
    #[serde(rename = "mensuralRestBrevis")]
    MensuralRestBrevis,
    /// Fusa rest
    #[serde(rename = "mensuralRestFusa")]
    MensuralRestFusa,
    /// Longa imperfecta rest
    #[serde(rename = "mensuralRestLongaImperfecta")]
    MensuralRestLongaImperfecta,
    /// Longa perfecta rest
    #[serde(rename = "mensuralRestLongaPerfecta")]
    MensuralRestLongaPerfecta,
    /// Maxima rest
    #[serde(rename = "mensuralRestMaxima")]
    MensuralRestMaxima,
    /// Minima rest
    #[serde(rename = "mensuralRestMinima")]
    MensuralRestMinima,
    /// Semibrevis rest
    #[serde(rename = "mensuralRestSemibrevis")]
    MensuralRestSemibrevis,
    /// Semifusa rest
    #[serde(rename = "mensuralRestSemifusa")]
    MensuralRestSemifusa,
    /// Semiminima rest
    #[serde(rename = "mensuralRestSemiminima")]
    MensuralRestSemiminima,
    /// Signum congruentiae down
    #[serde(rename = "mensuralSignumDown")]
    MensuralSignumDown,
    /// Signum congruentiae up
    #[serde(rename = "mensuralSignumUp")]
    MensuralSignumUp,
    /// Tempus imperfectum, horizontal
    #[serde(rename = "mensuralTempusImperfectumHoriz")]
    MensuralTempusImperfectumHoriz,
    /// Tempus perfectum, horizontal
    #[serde(rename = "mensuralTempusPerfectumHoriz")]
    MensuralTempusPerfectumHoriz,
    /// White mensural brevis
    #[serde(rename = "mensuralWhiteBrevis")]
    MensuralWhiteBrevis,
    /// White mensural fusa
    #[serde(rename = "mensuralWhiteFusa")]
    MensuralWhiteFusa,
    /// White mensural longa
    #[serde(rename = "mensuralWhiteLonga")]
    MensuralWhiteLonga,
    /// White mensural maxima
    #[serde(rename = "mensuralWhiteMaxima")]
    MensuralWhiteMaxima,
    /// White mensural minima
    #[serde(rename = "mensuralWhiteMinima")]
    MensuralWhiteMinima,
    /// White mensural semibrevis
    #[serde(rename = "mensuralWhiteSemibrevis")]
    MensuralWhiteSemibrevis,
    /// White mensural semiminima
    #[serde(rename = "mensuralWhiteSemiminima")]
    MensuralWhiteSemiminima,
    /// Augmentation dot
    #[serde(rename = "metAugmentationDot")]
    MetAugmentationDot,
    /// 1024th note (semihemidemisemihemidemisemiquaver) stem down
    #[serde(rename = "metNote1024thDown")]
    MetNote1024ThDown,
    /// 1024th note (semihemidemisemihemidemisemiquaver) stem up
    #[serde(rename = "metNote1024thUp")]
    MetNote1024ThUp,
    /// 128th note (semihemidemisemiquaver) stem down
    #[serde(rename = "metNote128thDown")]
    MetNote128ThDown,
    /// 128th note (semihemidemisemiquaver) stem up
    #[serde(rename = "metNote128thUp")]
    MetNote128ThUp,
    /// 16th note (semiquaver) stem down
    #[serde(rename = "metNote16thDown")]
    MetNote16ThDown,
    /// 16th note (semiquaver) stem up
    #[serde(rename = "metNote16thUp")]
    MetNote16ThUp,
    /// 256th note (demisemihemidemisemiquaver) stem down
    #[serde(rename = "metNote256thDown")]
    MetNote256ThDown,
    /// 256th note (demisemihemidemisemiquaver) stem up
    #[serde(rename = "metNote256thUp")]
    MetNote256ThUp,
    /// 32nd note (demisemiquaver) stem down
    #[serde(rename = "metNote32ndDown")]
    MetNote32NdDown,
    /// 32nd note (demisemiquaver) stem up
    #[serde(rename = "metNote32ndUp")]
    MetNote32NdUp,
    /// 512th note (hemidemisemihemidemisemiquaver) stem down
    #[serde(rename = "metNote512thDown")]
    MetNote512ThDown,
    /// 512th note (hemidemisemihemidemisemiquaver) stem up
    #[serde(rename = "metNote512thUp")]
    MetNote512ThUp,
    /// 64th note (hemidemisemiquaver) stem down
    #[serde(rename = "metNote64thDown")]
    MetNote64ThDown,
    /// 64th note (hemidemisemiquaver) stem up
    #[serde(rename = "metNote64thUp")]
    MetNote64ThUp,
    /// Eighth note (quaver) stem down
    #[serde(rename = "metNote8thDown")]
    MetNote8ThDown,
    /// Eighth note (quaver) stem up
    #[serde(rename = "metNote8thUp")]
    MetNote8ThUp,
    /// Double whole note (breve)
    #[serde(rename = "metNoteDoubleWhole")]
    MetNoteDoubleWhole,
    /// Double whole note (square)
    #[serde(rename = "metNoteDoubleWholeSquare")]
    MetNoteDoubleWholeSquare,
    /// Half note (minim) stem down
    #[serde(rename = "metNoteHalfDown")]
    MetNoteHalfDown,
    /// Half note (minim) stem up
    #[serde(rename = "metNoteHalfUp")]
    MetNoteHalfUp,
    /// Quarter note (crotchet) stem down
    #[serde(rename = "metNoteQuarterDown")]
    MetNoteQuarterDown,
    /// Quarter note (crotchet) stem up
    #[serde(rename = "metNoteQuarterUp")]
    MetNoteQuarterUp,
    /// Whole note (semibreve)
    #[serde(rename = "metNoteWhole")]
    MetNoteWhole,
    /// Left-pointing arrow for metric modulation
    #[serde(rename = "metricModulationArrowLeft")]
    MetricModulationArrowLeft,
    /// Right-pointing arrow for metric modulation
    #[serde(rename = "metricModulationArrowRight")]
    MetricModulationArrowRight,
    /// Do not copy
    #[serde(rename = "miscDoNotCopy")]
    MiscDoNotCopy,
    /// Do not photocopy
    #[serde(rename = "miscDoNotPhotocopy")]
    MiscDoNotPhotocopy,
    /// Eyeglasses
    #[serde(rename = "miscEyeglasses")]
    MiscEyeglasses,
    /// 1024th note (semihemidemisemihemidemisemiquaver) stem down
    #[serde(rename = "note1024thDown")]
    Note1024ThDown,
    /// 1024th note (semihemidemisemihemidemisemiquaver) stem up
    #[serde(rename = "note1024thUp")]
    Note1024ThUp,
    /// 128th note (semihemidemisemiquaver) stem down
    #[serde(rename = "note128thDown")]
    Note128ThDown,
    /// 128th note (semihemidemisemiquaver) stem up
    #[serde(rename = "note128thUp")]
    Note128ThUp,
    /// 16th note (semiquaver) stem down
    #[serde(rename = "note16thDown")]
    Note16ThDown,
    /// 16th note (semiquaver) stem up
    #[serde(rename = "note16thUp")]
    Note16ThUp,
    /// 256th note (demisemihemidemisemiquaver) stem down
    #[serde(rename = "note256thDown")]
    Note256ThDown,
    /// 256th note (demisemihemidemisemiquaver) stem up
    #[serde(rename = "note256thUp")]
    Note256ThUp,
    /// 32nd note (demisemiquaver) stem down
    #[serde(rename = "note32ndDown")]
    Note32NdDown,
    /// 32nd note (demisemiquaver) stem up
    #[serde(rename = "note32ndUp")]
    Note32NdUp,
    /// 512th note (hemidemisemihemidemisemiquaver) stem down
    #[serde(rename = "note512thDown")]
    Note512ThDown,
    /// 512th note (hemidemisemihemidemisemiquaver) stem up
    #[serde(rename = "note512thUp")]
    Note512ThUp,
    /// 64th note (hemidemisemiquaver) stem down
    #[serde(rename = "note64thDown")]
    Note64ThDown,
    /// 64th note (hemidemisemiquaver) stem up
    #[serde(rename = "note64thUp")]
    Note64ThUp,
    /// Eighth note (quaver) stem down
    #[serde(rename = "note8thDown")]
    Note8ThDown,
    /// Eighth note (quaver) stem up
    #[serde(rename = "note8thUp")]
    Note8ThUp,
    /// A (black note)
    #[serde(rename = "noteABlack")]
    NoteABlack,
    /// A flat (black note)
    #[serde(rename = "noteAFlatBlack")]
    NoteAFlatBlack,
    /// A flat (half note)
    #[serde(rename = "noteAFlatHalf")]
    NoteAFlatHalf,
    /// A flat (whole note)
    #[serde(rename = "noteAFlatWhole")]
    NoteAFlatWhole,
    /// A (half note)
    #[serde(rename = "noteAHalf")]
    NoteAHalf,
    /// A sharp (black note)
    #[serde(rename = "noteASharpBlack")]
    NoteASharpBlack,
    /// A sharp (half note)
    #[serde(rename = "noteASharpHalf")]
    NoteASharpHalf,
    /// A sharp (whole note)
    #[serde(rename = "noteASharpWhole")]
    NoteASharpWhole,
    /// A (whole note)
    #[serde(rename = "noteAWhole")]
    NoteAWhole,
    /// B (black note)
    #[serde(rename = "noteBBlack")]
    NoteBBlack,
    /// B flat (black note)
    #[serde(rename = "noteBFlatBlack")]
    NoteBFlatBlack,
    /// B flat (half note)
    #[serde(rename = "noteBFlatHalf")]
    NoteBFlatHalf,
    /// B flat (whole note)
    #[serde(rename = "noteBFlatWhole")]
    NoteBFlatWhole,
    /// B (half note)
    #[serde(rename = "noteBHalf")]
    NoteBHalf,
    /// B sharp (black note)
    #[serde(rename = "noteBSharpBlack")]
    NoteBSharpBlack,
    /// B sharp (half note)
    #[serde(rename = "noteBSharpHalf")]
    NoteBSharpHalf,
    /// B sharp (whole note)
    #[serde(rename = "noteBSharpWhole")]
    NoteBSharpWhole,
    /// B (whole note)
    #[serde(rename = "noteBWhole")]
    NoteBWhole,
    /// C (black note)
    #[serde(rename = "noteCBlack")]
    NoteCBlack,
    /// C flat (black note)
    #[serde(rename = "noteCFlatBlack")]
    NoteCFlatBlack,
    /// C flat (half note)
    #[serde(rename = "noteCFlatHalf")]
    NoteCFlatHalf,
    /// C flat (whole note)
    #[serde(rename = "noteCFlatWhole")]
    NoteCFlatWhole,
    /// C (half note)
    #[serde(rename = "noteCHalf")]
    NoteCHalf,
    /// C sharp (black note)
    #[serde(rename = "noteCSharpBlack")]
    NoteCSharpBlack,
    /// C sharp (half note)
    #[serde(rename = "noteCSharpHalf")]
    NoteCSharpHalf,
    /// C sharp (whole note)
    #[serde(rename = "noteCSharpWhole")]
    NoteCSharpWhole,
    /// C (whole note)
    #[serde(rename = "noteCWhole")]
    NoteCWhole,
    /// D (black note)
    #[serde(rename = "noteDBlack")]
    NoteDBlack,
    /// D flat (black note)
    #[serde(rename = "noteDFlatBlack")]
    NoteDFlatBlack,
    /// D flat (half note)
    #[serde(rename = "noteDFlatHalf")]
    NoteDFlatHalf,
    /// D flat (whole note)
    #[serde(rename = "noteDFlatWhole")]
    NoteDFlatWhole,
    /// D (half note)
    #[serde(rename = "noteDHalf")]
    NoteDHalf,
    /// D sharp (black note)
    #[serde(rename = "noteDSharpBlack")]
    NoteDSharpBlack,
    /// D sharp (half note)
    #[serde(rename = "noteDSharpHalf")]
    NoteDSharpHalf,
    /// D sharp (whole note)
    #[serde(rename = "noteDSharpWhole")]
    NoteDSharpWhole,
    /// D (whole note)
    #[serde(rename = "noteDWhole")]
    NoteDWhole,
    /// Di (black note)
    #[serde(rename = "noteDiBlack")]
    NoteDiBlack,
    /// Di (half note)
    #[serde(rename = "noteDiHalf")]
    NoteDiHalf,
    /// Di (whole note)
    #[serde(rename = "noteDiWhole")]
    NoteDiWhole,
    /// Do (black note)
    #[serde(rename = "noteDoBlack")]
    NoteDoBlack,
    /// Do (half note)
    #[serde(rename = "noteDoHalf")]
    NoteDoHalf,
    /// Do (whole note)
    #[serde(rename = "noteDoWhole")]
    NoteDoWhole,
    /// Double whole note (breve)
    #[serde(rename = "noteDoubleWhole")]
    NoteDoubleWhole,
    /// Double whole note (square)
    #[serde(rename = "noteDoubleWholeSquare")]
    NoteDoubleWholeSquare,
    /// E (black note)
    #[serde(rename = "noteEBlack")]
    NoteEBlack,
    /// E flat (black note)
    #[serde(rename = "noteEFlatBlack")]
    NoteEFlatBlack,
    /// E flat (half note)
    #[serde(rename = "noteEFlatHalf")]
    NoteEFlatHalf,
    /// E flat (whole note)
    #[serde(rename = "noteEFlatWhole")]
    NoteEFlatWhole,
    /// E (half note)
    #[serde(rename = "noteEHalf")]
    NoteEHalf,
    /// E sharp (black note)
    #[serde(rename = "noteESharpBlack")]
    NoteESharpBlack,
    /// E sharp (half note)
    #[serde(rename = "noteESharpHalf")]
    NoteESharpHalf,
    /// E sharp (whole note)
    #[serde(rename = "noteESharpWhole")]
    NoteESharpWhole,
    /// E (whole note)
    #[serde(rename = "noteEWhole")]
    NoteEWhole,
    /// Empty black note
    #[serde(rename = "noteEmptyBlack")]
    NoteEmptyBlack,
    /// Empty half note
    #[serde(rename = "noteEmptyHalf")]
    NoteEmptyHalf,
    /// Empty whole note
    #[serde(rename = "noteEmptyWhole")]
    NoteEmptyWhole,
    /// F (black note)
    #[serde(rename = "noteFBlack")]
    NoteFBlack,
    /// F flat (black note)
    #[serde(rename = "noteFFlatBlack")]
    NoteFFlatBlack,
    /// F flat (half note)
    #[serde(rename = "noteFFlatHalf")]
    NoteFFlatHalf,
    /// F flat (whole note)
    #[serde(rename = "noteFFlatWhole")]
    NoteFFlatWhole,
    /// F (half note)
    #[serde(rename = "noteFHalf")]
    NoteFHalf,
    /// F sharp (black note)
    #[serde(rename = "noteFSharpBlack")]
    NoteFSharpBlack,
    /// F sharp (half note)
    #[serde(rename = "noteFSharpHalf")]
    NoteFSharpHalf,
    /// F sharp (whole note)
    #[serde(rename = "noteFSharpWhole")]
    NoteFSharpWhole,
    /// F (whole note)
    #[serde(rename = "noteFWhole")]
    NoteFWhole,
    /// Fa (black note)
    #[serde(rename = "noteFaBlack")]
    NoteFaBlack,
    /// Fa (half note)
    #[serde(rename = "noteFaHalf")]
    NoteFaHalf,
    /// Fa (whole note)
    #[serde(rename = "noteFaWhole")]
    NoteFaWhole,
    /// Fi (black note)
    #[serde(rename = "noteFiBlack")]
    NoteFiBlack,
    /// Fi (half note)
    #[serde(rename = "noteFiHalf")]
    NoteFiHalf,
    /// Fi (whole note)
    #[serde(rename = "noteFiWhole")]
    NoteFiWhole,
    /// G (black note)
    #[serde(rename = "noteGBlack")]
    NoteGBlack,
    /// G flat (black note)
    #[serde(rename = "noteGFlatBlack")]
    NoteGFlatBlack,
    /// G flat (half note)
    #[serde(rename = "noteGFlatHalf")]
    NoteGFlatHalf,
    /// G flat (whole note)
    #[serde(rename = "noteGFlatWhole")]
    NoteGFlatWhole,
    /// G (half note)
    #[serde(rename = "noteGHalf")]
    NoteGHalf,
    /// G sharp (black note)
    #[serde(rename = "noteGSharpBlack")]
    NoteGSharpBlack,
    /// G sharp (half note)
    #[serde(rename = "noteGSharpHalf")]
    NoteGSharpHalf,
    /// G sharp (whole note)
    #[serde(rename = "noteGSharpWhole")]
    NoteGSharpWhole,
    /// G (whole note)
    #[serde(rename = "noteGWhole")]
    NoteGWhole,
    /// H (black note)
    #[serde(rename = "noteHBlack")]
    NoteHBlack,
    /// H (half note)
    #[serde(rename = "noteHHalf")]
    NoteHHalf,
    /// H sharp (black note)
    #[serde(rename = "noteHSharpBlack")]
    NoteHSharpBlack,
    /// H sharp (half note)
    #[serde(rename = "noteHSharpHalf")]
    NoteHSharpHalf,
    /// H sharp (whole note)
    #[serde(rename = "noteHSharpWhole")]
    NoteHSharpWhole,
    /// H (whole note)
    #[serde(rename = "noteHWhole")]
    NoteHWhole,
    /// Half note (minim) stem down
    #[serde(rename = "noteHalfDown")]
    NoteHalfDown,
    /// Half note (minim) stem up
    #[serde(rename = "noteHalfUp")]
    NoteHalfUp,
    /// La (black note)
    #[serde(rename = "noteLaBlack")]
    NoteLaBlack,
    /// La (half note)
    #[serde(rename = "noteLaHalf")]
    NoteLaHalf,
    /// La (whole note)
    #[serde(rename = "noteLaWhole")]
    NoteLaWhole,
    /// Le (black note)
    #[serde(rename = "noteLeBlack")]
    NoteLeBlack,
    /// Le (half note)
    #[serde(rename = "noteLeHalf")]
    NoteLeHalf,
    /// Le (whole note)
    #[serde(rename = "noteLeWhole")]
    NoteLeWhole,
    /// Li (black note)
    #[serde(rename = "noteLiBlack")]
    NoteLiBlack,
    /// Li (half note)
    #[serde(rename = "noteLiHalf")]
    NoteLiHalf,
    /// Li (whole note)
    #[serde(rename = "noteLiWhole")]
    NoteLiWhole,
    /// Me (black note)
    #[serde(rename = "noteMeBlack")]
    NoteMeBlack,
    /// Me (half note)
    #[serde(rename = "noteMeHalf")]
    NoteMeHalf,
    /// Me (whole note)
    #[serde(rename = "noteMeWhole")]
    NoteMeWhole,
    /// Mi (black note)
    #[serde(rename = "noteMiBlack")]
    NoteMiBlack,
    /// Mi (half note)
    #[serde(rename = "noteMiHalf")]
    NoteMiHalf,
    /// Mi (whole note)
    #[serde(rename = "noteMiWhole")]
    NoteMiWhole,
    /// Quarter note (crotchet) stem down
    #[serde(rename = "noteQuarterDown")]
    NoteQuarterDown,
    /// Quarter note (crotchet) stem up
    #[serde(rename = "noteQuarterUp")]
    NoteQuarterUp,
    /// Ra (black note)
    #[serde(rename = "noteRaBlack")]
    NoteRaBlack,
    /// Ra (half note)
    #[serde(rename = "noteRaHalf")]
    NoteRaHalf,
    /// Ra (whole note)
    #[serde(rename = "noteRaWhole")]
    NoteRaWhole,
    /// Re (black note)
    #[serde(rename = "noteReBlack")]
    NoteReBlack,
    /// Re (half note)
    #[serde(rename = "noteReHalf")]
    NoteReHalf,
    /// Re (whole note)
    #[serde(rename = "noteReWhole")]
    NoteReWhole,
    /// Ri (black note)
    #[serde(rename = "noteRiBlack")]
    NoteRiBlack,
    /// Ri (half note)
    #[serde(rename = "noteRiHalf")]
    NoteRiHalf,
    /// Ri (whole note)
    #[serde(rename = "noteRiWhole")]
    NoteRiWhole,
    /// Se (black note)
    #[serde(rename = "noteSeBlack")]
    NoteSeBlack,
    /// Se (half note)
    #[serde(rename = "noteSeHalf")]
    NoteSeHalf,
    /// Se (whole note)
    #[serde(rename = "noteSeWhole")]
    NoteSeWhole,
    /// Arrowhead left black (Funk 7-shape re)
    #[serde(rename = "noteShapeArrowheadLeftBlack")]
    NoteShapeArrowheadLeftBlack,
    /// Arrowhead left double whole (Funk 7-shape re)
    #[serde(rename = "noteShapeArrowheadLeftDoubleWhole")]
    NoteShapeArrowheadLeftDoubleWhole,
    /// Arrowhead left white (Funk 7-shape re)
    #[serde(rename = "noteShapeArrowheadLeftWhite")]
    NoteShapeArrowheadLeftWhite,
    /// Diamond black (4-shape mi; 7-shape mi)
    #[serde(rename = "noteShapeDiamondBlack")]
    NoteShapeDiamondBlack,
    /// Diamond double whole (4-shape mi; 7-shape mi)
    #[serde(rename = "noteShapeDiamondDoubleWhole")]
    NoteShapeDiamondDoubleWhole,
    /// Diamond white (4-shape mi; 7-shape mi)
    #[serde(rename = "noteShapeDiamondWhite")]
    NoteShapeDiamondWhite,
    /// Isosceles triangle black (Walker 7-shape ti)
    #[serde(rename = "noteShapeIsoscelesTriangleBlack")]
    NoteShapeIsoscelesTriangleBlack,
    /// Isosceles triangle double whole (Walker 7-shape ti)
    #[serde(rename = "noteShapeIsoscelesTriangleDoubleWhole")]
    NoteShapeIsoscelesTriangleDoubleWhole,
    /// Isosceles triangle white (Walker 7-shape ti)
    #[serde(rename = "noteShapeIsoscelesTriangleWhite")]
    NoteShapeIsoscelesTriangleWhite,
    /// Inverted keystone black (Walker 7-shape do)
    #[serde(rename = "noteShapeKeystoneBlack")]
    NoteShapeKeystoneBlack,
    /// Inverted keystone double whole (Walker 7-shape do)
    #[serde(rename = "noteShapeKeystoneDoubleWhole")]
    NoteShapeKeystoneDoubleWhole,
    /// Inverted keystone white (Walker 7-shape do)
    #[serde(rename = "noteShapeKeystoneWhite")]
    NoteShapeKeystoneWhite,
    /// Moon black (Aikin 7-shape re)
    #[serde(rename = "noteShapeMoonBlack")]
    NoteShapeMoonBlack,
    /// Moon double whole (Aikin 7-shape re)
    #[serde(rename = "noteShapeMoonDoubleWhole")]
    NoteShapeMoonDoubleWhole,
    /// Moon left black (Funk 7-shape do)
    #[serde(rename = "noteShapeMoonLeftBlack")]
    NoteShapeMoonLeftBlack,
    /// Moon left double whole (Funk 7-shape do)
    #[serde(rename = "noteShapeMoonLeftDoubleWhole")]
    NoteShapeMoonLeftDoubleWhole,
    /// Moon left white (Funk 7-shape do)
    #[serde(rename = "noteShapeMoonLeftWhite")]
    NoteShapeMoonLeftWhite,
    /// Moon white (Aikin 7-shape re)
    #[serde(rename = "noteShapeMoonWhite")]
    NoteShapeMoonWhite,
    /// Quarter moon black (Walker 7-shape re)
    #[serde(rename = "noteShapeQuarterMoonBlack")]
    NoteShapeQuarterMoonBlack,
    /// Quarter moon double whole (Walker 7-shape re)
    #[serde(rename = "noteShapeQuarterMoonDoubleWhole")]
    NoteShapeQuarterMoonDoubleWhole,
    /// Quarter moon white (Walker 7-shape re)
    #[serde(rename = "noteShapeQuarterMoonWhite")]
    NoteShapeQuarterMoonWhite,
    /// Round black (4-shape sol; 7-shape so)
    #[serde(rename = "noteShapeRoundBlack")]
    NoteShapeRoundBlack,
    /// Round double whole (4-shape sol; 7-shape so)
    #[serde(rename = "noteShapeRoundDoubleWhole")]
    NoteShapeRoundDoubleWhole,
    /// Round white (4-shape sol; 7-shape so)
    #[serde(rename = "noteShapeRoundWhite")]
    NoteShapeRoundWhite,
    /// Square black (4-shape la; Aikin 7-shape la)
    #[serde(rename = "noteShapeSquareBlack")]
    NoteShapeSquareBlack,
    /// Square double whole (4-shape la; Aikin 7-shape la)
    #[serde(rename = "noteShapeSquareDoubleWhole")]
    NoteShapeSquareDoubleWhole,
    /// Square white (4-shape la; Aikin 7-shape la)
    #[serde(rename = "noteShapeSquareWhite")]
    NoteShapeSquareWhite,
    /// Triangle left black (stem up; 4-shape fa; 7-shape fa)
    #[serde(rename = "noteShapeTriangleLeftBlack")]
    NoteShapeTriangleLeftBlack,
    /// Triangle left double whole (stem up; 4-shape fa; 7-shape fa)
    #[serde(rename = "noteShapeTriangleLeftDoubleWhole")]
    NoteShapeTriangleLeftDoubleWhole,
    /// Triangle left white (stem up; 4-shape fa; 7-shape fa)
    #[serde(rename = "noteShapeTriangleLeftWhite")]
    NoteShapeTriangleLeftWhite,
    /// Triangle right black (stem down; 4-shape fa; 7-shape fa)
    #[serde(rename = "noteShapeTriangleRightBlack")]
    NoteShapeTriangleRightBlack,
    /// Triangle right double whole (stem down; 4-shape fa; 7-shape fa)
    #[serde(rename = "noteShapeTriangleRightDoubleWhole")]
    NoteShapeTriangleRightDoubleWhole,
    /// Triangle right white (stem down; 4-shape fa; 7-shape fa)
    #[serde(rename = "noteShapeTriangleRightWhite")]
    NoteShapeTriangleRightWhite,
    /// Triangle-round black (Aikin 7-shape ti)
    #[serde(rename = "noteShapeTriangleRoundBlack")]
    NoteShapeTriangleRoundBlack,
    /// Triangle-round white (Aikin 7-shape ti)
    #[serde(rename = "noteShapeTriangleRoundDoubleWhole")]
    NoteShapeTriangleRoundDoubleWhole,
    /// Triangle-round left black (Funk 7-shape ti)
    #[serde(rename = "noteShapeTriangleRoundLeftBlack")]
    NoteShapeTriangleRoundLeftBlack,
    /// Triangle-round left double whole (Funk 7-shape ti)
    #[serde(rename = "noteShapeTriangleRoundLeftDoubleWhole")]
    NoteShapeTriangleRoundLeftDoubleWhole,
    /// Triangle-round left white (Funk 7-shape ti)
    #[serde(rename = "noteShapeTriangleRoundLeftWhite")]
    NoteShapeTriangleRoundLeftWhite,
    /// Triangle-round white (Aikin 7-shape ti)
    #[serde(rename = "noteShapeTriangleRoundWhite")]
    NoteShapeTriangleRoundWhite,
    /// Triangle up black (Aikin 7-shape do)
    #[serde(rename = "noteShapeTriangleUpBlack")]
    NoteShapeTriangleUpBlack,
    /// Triangle up double whole (Aikin 7-shape do)
    #[serde(rename = "noteShapeTriangleUpDoubleWhole")]
    NoteShapeTriangleUpDoubleWhole,
    /// Triangle up white (Aikin 7-shape do)
    #[serde(rename = "noteShapeTriangleUpWhite")]
    NoteShapeTriangleUpWhite,
    /// Si (black note)
    #[serde(rename = "noteSiBlack")]
    NoteSiBlack,
    /// Si (half note)
    #[serde(rename = "noteSiHalf")]
    NoteSiHalf,
    /// Si (whole note)
    #[serde(rename = "noteSiWhole")]
    NoteSiWhole,
    /// So (black note)
    #[serde(rename = "noteSoBlack")]
    NoteSoBlack,
    /// So (half note)
    #[serde(rename = "noteSoHalf")]
    NoteSoHalf,
    /// So (whole note)
    #[serde(rename = "noteSoWhole")]
    NoteSoWhole,
    /// Te (black note)
    #[serde(rename = "noteTeBlack")]
    NoteTeBlack,
    /// Te (half note)
    #[serde(rename = "noteTeHalf")]
    NoteTeHalf,
    /// Te (whole note)
    #[serde(rename = "noteTeWhole")]
    NoteTeWhole,
    /// Ti (black note)
    #[serde(rename = "noteTiBlack")]
    NoteTiBlack,
    /// Ti (half note)
    #[serde(rename = "noteTiHalf")]
    NoteTiHalf,
    /// Ti (whole note)
    #[serde(rename = "noteTiWhole")]
    NoteTiWhole,
    /// Whole note (semibreve)
    #[serde(rename = "noteWhole")]
    NoteWhole,
    /// Black notehead
    #[serde(rename = "noteheadBlack")]
    NoteheadBlack,
    /// Circle slash notehead
    #[serde(rename = "noteheadCircleSlash")]
    NoteheadCircleSlash,
    /// Circle X notehead
    #[serde(rename = "noteheadCircleX")]
    NoteheadCircleX,
    /// Circle X double whole
    #[serde(rename = "noteheadCircleXDoubleWhole")]
    NoteheadCircleXDoubleWhole,
    /// Circle X half
    #[serde(rename = "noteheadCircleXHalf")]
    NoteheadCircleXHalf,
    /// Circle X whole
    #[serde(rename = "noteheadCircleXWhole")]
    NoteheadCircleXWhole,
    /// Circled black notehead
    #[serde(rename = "noteheadCircledBlack")]
    NoteheadCircledBlack,
    /// Black notehead in large circle
    #[serde(rename = "noteheadCircledBlackLarge")]
    NoteheadCircledBlackLarge,
    /// Circled double whole notehead
    #[serde(rename = "noteheadCircledDoubleWhole")]
    NoteheadCircledDoubleWhole,
    /// Double whole notehead in large circle
    #[serde(rename = "noteheadCircledDoubleWholeLarge")]
    NoteheadCircledDoubleWholeLarge,
    /// Circled half notehead
    #[serde(rename = "noteheadCircledHalf")]
    NoteheadCircledHalf,
    /// Half notehead in large circle
    #[serde(rename = "noteheadCircledHalfLarge")]
    NoteheadCircledHalfLarge,
    /// Circled whole notehead
    #[serde(rename = "noteheadCircledWhole")]
    NoteheadCircledWhole,
    /// Whole notehead in large circle
    #[serde(rename = "noteheadCircledWholeLarge")]
    NoteheadCircledWholeLarge,
    /// Cross notehead in large circle
    #[serde(rename = "noteheadCircledXLarge")]
    NoteheadCircledXLarge,
    /// Double whole note cluster, 2nd
    #[serde(rename = "noteheadClusterDoubleWhole2nd")]
    NoteheadClusterDoubleWhole2Nd,
    /// Double whole note cluster, 3rd
    #[serde(rename = "noteheadClusterDoubleWhole3rd")]
    NoteheadClusterDoubleWhole3Rd,
    /// Combining double whole note cluster, bottom
    #[serde(rename = "noteheadClusterDoubleWholeBottom")]
    NoteheadClusterDoubleWholeBottom,
    /// Combining double whole note cluster, middle
    #[serde(rename = "noteheadClusterDoubleWholeMiddle")]
    NoteheadClusterDoubleWholeMiddle,
    /// Combining double whole note cluster, top
    #[serde(rename = "noteheadClusterDoubleWholeTop")]
    NoteheadClusterDoubleWholeTop,
    /// Half note cluster, 2nd
    #[serde(rename = "noteheadClusterHalf2nd")]
    NoteheadClusterHalf2Nd,
    /// Half note cluster, 3rd
    #[serde(rename = "noteheadClusterHalf3rd")]
    NoteheadClusterHalf3Rd,
    /// Combining half note cluster, bottom
    #[serde(rename = "noteheadClusterHalfBottom")]
    NoteheadClusterHalfBottom,
    /// Combining half note cluster, middle
    #[serde(rename = "noteheadClusterHalfMiddle")]
    NoteheadClusterHalfMiddle,
    /// Combining half note cluster, top
    #[serde(rename = "noteheadClusterHalfTop")]
    NoteheadClusterHalfTop,
    /// Quarter note cluster, 2nd
    #[serde(rename = "noteheadClusterQuarter2nd")]
    NoteheadClusterQuarter2Nd,
    /// Quarter note cluster, 3rd
    #[serde(rename = "noteheadClusterQuarter3rd")]
    NoteheadClusterQuarter3Rd,
    /// Combining quarter note cluster, bottom
    #[serde(rename = "noteheadClusterQuarterBottom")]
    NoteheadClusterQuarterBottom,
    /// Combining quarter note cluster, middle
    #[serde(rename = "noteheadClusterQuarterMiddle")]
    NoteheadClusterQuarterMiddle,
    /// Combining quarter note cluster, top
    #[serde(rename = "noteheadClusterQuarterTop")]
    NoteheadClusterQuarterTop,
    /// Cluster notehead black (round)
    #[serde(rename = "noteheadClusterRoundBlack")]
    NoteheadClusterRoundBlack,
    /// Cluster notehead white (round)
    #[serde(rename = "noteheadClusterRoundWhite")]
    NoteheadClusterRoundWhite,
    /// Cluster notehead black (square)
    #[serde(rename = "noteheadClusterSquareBlack")]
    NoteheadClusterSquareBlack,
    /// Cluster notehead white (square)
    #[serde(rename = "noteheadClusterSquareWhite")]
    NoteheadClusterSquareWhite,
    /// Whole note cluster, 2nd
    #[serde(rename = "noteheadClusterWhole2nd")]
    NoteheadClusterWhole2Nd,
    /// Whole note cluster, 3rd
    #[serde(rename = "noteheadClusterWhole3rd")]
    NoteheadClusterWhole3Rd,
    /// Combining whole note cluster, bottom
    #[serde(rename = "noteheadClusterWholeBottom")]
    NoteheadClusterWholeBottom,
    /// Combining whole note cluster, middle
    #[serde(rename = "noteheadClusterWholeMiddle")]
    NoteheadClusterWholeMiddle,
    /// Combining whole note cluster, top
    #[serde(rename = "noteheadClusterWholeTop")]
    NoteheadClusterWholeTop,
    /// 4/11 note (eleventh note series, Cowell)
    #[serde(rename = "noteheadCowellEleventhNoteSeriesHalf")]
    NoteheadCowellEleventhNoteSeriesHalf,
    /// 8/11 note (eleventh note series, Cowell)
    #[serde(rename = "noteheadCowellEleventhNoteSeriesWhole")]
    NoteheadCowellEleventhNoteSeriesWhole,
    /// 2/11 note (eleventh note series, Cowell)
    #[serde(rename = "noteheadCowellEleventhSeriesBlack")]
    NoteheadCowellEleventhSeriesBlack,
    /// 2/15 note (fifteenth note series, Cowell)
    #[serde(rename = "noteheadCowellFifteenthNoteSeriesBlack")]
    NoteheadCowellFifteenthNoteSeriesBlack,
    /// 4/15 note (fifteenth note series, Cowell)
    #[serde(rename = "noteheadCowellFifteenthNoteSeriesHalf")]
    NoteheadCowellFifteenthNoteSeriesHalf,
    /// 8/15 note (fifteenth note series, Cowell)
    #[serde(rename = "noteheadCowellFifteenthNoteSeriesWhole")]
    NoteheadCowellFifteenthNoteSeriesWhole,
    /// 1/5 note (fifth note series, Cowell)
    #[serde(rename = "noteheadCowellFifthNoteSeriesBlack")]
    NoteheadCowellFifthNoteSeriesBlack,
    /// 2/5 note (fifth note series, Cowell)
    #[serde(rename = "noteheadCowellFifthNoteSeriesHalf")]
    NoteheadCowellFifthNoteSeriesHalf,
    /// 4/5 note (fifth note series, Cowell)
    #[serde(rename = "noteheadCowellFifthNoteSeriesWhole")]
    NoteheadCowellFifthNoteSeriesWhole,
    /// 2/9 note (ninth note series, Cowell)
    #[serde(rename = "noteheadCowellNinthNoteSeriesBlack")]
    NoteheadCowellNinthNoteSeriesBlack,
    /// 4/9 note (ninth note series, Cowell)
    #[serde(rename = "noteheadCowellNinthNoteSeriesHalf")]
    NoteheadCowellNinthNoteSeriesHalf,
    /// 8/9 note (ninth note series, Cowell)
    #[serde(rename = "noteheadCowellNinthNoteSeriesWhole")]
    NoteheadCowellNinthNoteSeriesWhole,
    /// 1/7 note (seventh note series, Cowell)
    #[serde(rename = "noteheadCowellSeventhNoteSeriesBlack")]
    NoteheadCowellSeventhNoteSeriesBlack,
    /// 2/7 note (seventh note series, Cowell)
    #[serde(rename = "noteheadCowellSeventhNoteSeriesHalf")]
    NoteheadCowellSeventhNoteSeriesHalf,
    /// 4/7 note (seventh note series, Cowell)
    #[serde(rename = "noteheadCowellSeventhNoteSeriesWhole")]
    NoteheadCowellSeventhNoteSeriesWhole,
    /// 1/6 note (third note series, Cowell)
    #[serde(rename = "noteheadCowellThirdNoteSeriesBlack")]
    NoteheadCowellThirdNoteSeriesBlack,
    /// 1/3 note (third note series, Cowell)
    #[serde(rename = "noteheadCowellThirdNoteSeriesHalf")]
    NoteheadCowellThirdNoteSeriesHalf,
    /// 2/3 note (third note series, Cowell)
    #[serde(rename = "noteheadCowellThirdNoteSeriesWhole")]
    NoteheadCowellThirdNoteSeriesWhole,
    /// 2/13 note (thirteenth note series, Cowell)
    #[serde(rename = "noteheadCowellThirteenthNoteSeriesBlack")]
    NoteheadCowellThirteenthNoteSeriesBlack,
    /// 4/13 note (thirteenth note series, Cowell)
    #[serde(rename = "noteheadCowellThirteenthNoteSeriesHalf")]
    NoteheadCowellThirteenthNoteSeriesHalf,
    /// 8/13 note (thirteenth note series, Cowell)
    #[serde(rename = "noteheadCowellThirteenthNoteSeriesWhole")]
    NoteheadCowellThirteenthNoteSeriesWhole,
    /// Diamond black notehead
    #[serde(rename = "noteheadDiamondBlack")]
    NoteheadDiamondBlack,
    /// Diamond black notehead (old)
    #[serde(rename = "noteheadDiamondBlackOld")]
    NoteheadDiamondBlackOld,
    /// Diamond black notehead (wide)
    #[serde(rename = "noteheadDiamondBlackWide")]
    NoteheadDiamondBlackWide,
    /// Black diamond cluster, 2nd
    #[serde(rename = "noteheadDiamondClusterBlack2nd")]
    NoteheadDiamondClusterBlack2Nd,
    /// Black diamond cluster, 3rd
    #[serde(rename = "noteheadDiamondClusterBlack3rd")]
    NoteheadDiamondClusterBlack3Rd,
    /// Combining black diamond cluster, bottom
    #[serde(rename = "noteheadDiamondClusterBlackBottom")]
    NoteheadDiamondClusterBlackBottom,
    /// Combining black diamond cluster, middle
    #[serde(rename = "noteheadDiamondClusterBlackMiddle")]
    NoteheadDiamondClusterBlackMiddle,
    /// Combining black diamond cluster, top
    #[serde(rename = "noteheadDiamondClusterBlackTop")]
    NoteheadDiamondClusterBlackTop,
    /// White diamond cluster, 2nd
    #[serde(rename = "noteheadDiamondClusterWhite2nd")]
    NoteheadDiamondClusterWhite2Nd,
    /// White diamond cluster, 3rd
    #[serde(rename = "noteheadDiamondClusterWhite3rd")]
    NoteheadDiamondClusterWhite3Rd,
    /// Combining white diamond cluster, bottom
    #[serde(rename = "noteheadDiamondClusterWhiteBottom")]
    NoteheadDiamondClusterWhiteBottom,
    /// Combining white diamond cluster, middle
    #[serde(rename = "noteheadDiamondClusterWhiteMiddle")]
    NoteheadDiamondClusterWhiteMiddle,
    /// Combining white diamond cluster, top
    #[serde(rename = "noteheadDiamondClusterWhiteTop")]
    NoteheadDiamondClusterWhiteTop,
    /// Diamond double whole notehead
    #[serde(rename = "noteheadDiamondDoubleWhole")]
    NoteheadDiamondDoubleWhole,
    /// Diamond double whole notehead (old)
    #[serde(rename = "noteheadDiamondDoubleWholeOld")]
    NoteheadDiamondDoubleWholeOld,
    /// Diamond half notehead
    #[serde(rename = "noteheadDiamondHalf")]
    NoteheadDiamondHalf,
    /// Half-filled diamond notehead
    #[serde(rename = "noteheadDiamondHalfFilled")]
    NoteheadDiamondHalfFilled,
    /// Diamond half notehead (old)
    #[serde(rename = "noteheadDiamondHalfOld")]
    NoteheadDiamondHalfOld,
    /// Diamond half notehead (wide)
    #[serde(rename = "noteheadDiamondHalfWide")]
    NoteheadDiamondHalfWide,
    /// Open diamond notehead
    #[serde(rename = "noteheadDiamondOpen")]
    NoteheadDiamondOpen,
    /// Diamond white notehead
    #[serde(rename = "noteheadDiamondWhite")]
    NoteheadDiamondWhite,
    /// Diamond white notehead (wide)
    #[serde(rename = "noteheadDiamondWhiteWide")]
    NoteheadDiamondWhiteWide,
    /// Diamond whole notehead
    #[serde(rename = "noteheadDiamondWhole")]
    NoteheadDiamondWhole,
    /// Diamond whole notehead (old)
    #[serde(rename = "noteheadDiamondWholeOld")]
    NoteheadDiamondWholeOld,
    /// Double whole (breve) notehead
    #[serde(rename = "noteheadDoubleWhole")]
    NoteheadDoubleWhole,
    /// Double whole (breve) notehead (square)
    #[serde(rename = "noteheadDoubleWholeSquare")]
    NoteheadDoubleWholeSquare,
    /// Double whole notehead with X
    #[serde(rename = "noteheadDoubleWholeWithX")]
    NoteheadDoubleWholeWithX,
    /// Half (minim) notehead
    #[serde(rename = "noteheadHalf")]
    NoteheadHalf,
    /// Filled half (minim) notehead
    #[serde(rename = "noteheadHalfFilled")]
    NoteheadHalfFilled,
    /// Half notehead with X
    #[serde(rename = "noteheadHalfWithX")]
    NoteheadHalfWithX,
    /// Heavy X notehead
    #[serde(rename = "noteheadHeavyX")]
    NoteheadHeavyX,
    /// Heavy X with hat notehead
    #[serde(rename = "noteheadHeavyXHat")]
    NoteheadHeavyXHat,
    /// Large arrow down (lowest pitch) black notehead
    #[serde(rename = "noteheadLargeArrowDownBlack")]
    NoteheadLargeArrowDownBlack,
    /// Large arrow down (lowest pitch) double whole notehead
    #[serde(rename = "noteheadLargeArrowDownDoubleWhole")]
    NoteheadLargeArrowDownDoubleWhole,
    /// Large arrow down (lowest pitch) half notehead
    #[serde(rename = "noteheadLargeArrowDownHalf")]
    NoteheadLargeArrowDownHalf,
    /// Large arrow down (lowest pitch) whole notehead
    #[serde(rename = "noteheadLargeArrowDownWhole")]
    NoteheadLargeArrowDownWhole,
    /// Large arrow up (highest pitch) black notehead
    #[serde(rename = "noteheadLargeArrowUpBlack")]
    NoteheadLargeArrowUpBlack,
    /// Large arrow up (highest pitch) double whole notehead
    #[serde(rename = "noteheadLargeArrowUpDoubleWhole")]
    NoteheadLargeArrowUpDoubleWhole,
    /// Large arrow up (highest pitch) half notehead
    #[serde(rename = "noteheadLargeArrowUpHalf")]
    NoteheadLargeArrowUpHalf,
    /// Large arrow up (highest pitch) whole notehead
    #[serde(rename = "noteheadLargeArrowUpWhole")]
    NoteheadLargeArrowUpWhole,
    /// Moon notehead black
    #[serde(rename = "noteheadMoonBlack")]
    NoteheadMoonBlack,
    /// Moon notehead white
    #[serde(rename = "noteheadMoonWhite")]
    NoteheadMoonWhite,
    /// Sine notehead (Nancarrow)
    #[serde(rename = "noteheadNancarrowSine")]
    NoteheadNancarrowSine,
    /// Null notehead
    #[serde(rename = "noteheadNull")]
    NoteheadNull,
    /// Parenthesis notehead
    #[serde(rename = "noteheadParenthesis")]
    NoteheadParenthesis,
    /// Opening parenthesis
    #[serde(rename = "noteheadParenthesisLeft")]
    NoteheadParenthesisLeft,
    /// Closing parenthesis
    #[serde(rename = "noteheadParenthesisRight")]
    NoteheadParenthesisRight,
    /// Plus notehead black
    #[serde(rename = "noteheadPlusBlack")]
    NoteheadPlusBlack,
    /// Plus notehead double whole
    #[serde(rename = "noteheadPlusDoubleWhole")]
    NoteheadPlusDoubleWhole,
    /// Plus notehead half
    #[serde(rename = "noteheadPlusHalf")]
    NoteheadPlusHalf,
    /// Plus notehead whole
    #[serde(rename = "noteheadPlusWhole")]
    NoteheadPlusWhole,
    /// Combining black rectangular cluster, bottom
    #[serde(rename = "noteheadRectangularClusterBlackBottom")]
    NoteheadRectangularClusterBlackBottom,
    /// Combining black rectangular cluster, middle
    #[serde(rename = "noteheadRectangularClusterBlackMiddle")]
    NoteheadRectangularClusterBlackMiddle,
    /// Combining black rectangular cluster, top
    #[serde(rename = "noteheadRectangularClusterBlackTop")]
    NoteheadRectangularClusterBlackTop,
    /// Combining white rectangular cluster, bottom
    #[serde(rename = "noteheadRectangularClusterWhiteBottom")]
    NoteheadRectangularClusterWhiteBottom,
    /// Combining white rectangular cluster, middle
    #[serde(rename = "noteheadRectangularClusterWhiteMiddle")]
    NoteheadRectangularClusterWhiteMiddle,
    /// Combining white rectangular cluster, top
    #[serde(rename = "noteheadRectangularClusterWhiteTop")]
    NoteheadRectangularClusterWhiteTop,
    /// Round black notehead
    #[serde(rename = "noteheadRoundBlack")]
    NoteheadRoundBlack,
    /// Round black notehead, double slashed
    #[serde(rename = "noteheadRoundBlackDoubleSlashed")]
    NoteheadRoundBlackDoubleSlashed,
    /// Large round black notehead
    #[serde(rename = "noteheadRoundBlackLarge")]
    NoteheadRoundBlackLarge,
    /// Round black notehead, slashed
    #[serde(rename = "noteheadRoundBlackSlashed")]
    NoteheadRoundBlackSlashed,
    /// Large round black notehead, slashed
    #[serde(rename = "noteheadRoundBlackSlashedLarge")]
    NoteheadRoundBlackSlashedLarge,
    /// Round white notehead
    #[serde(rename = "noteheadRoundWhite")]
    NoteheadRoundWhite,
    /// Round white notehead, double slashed
    #[serde(rename = "noteheadRoundWhiteDoubleSlashed")]
    NoteheadRoundWhiteDoubleSlashed,
    /// Large round white notehead
    #[serde(rename = "noteheadRoundWhiteLarge")]
    NoteheadRoundWhiteLarge,
    /// Round white notehead, slashed
    #[serde(rename = "noteheadRoundWhiteSlashed")]
    NoteheadRoundWhiteSlashed,
    /// Large round white notehead, slashed
    #[serde(rename = "noteheadRoundWhiteSlashedLarge")]
    NoteheadRoundWhiteSlashedLarge,
    /// Round white notehead with dot
    #[serde(rename = "noteheadRoundWhiteWithDot")]
    NoteheadRoundWhiteWithDot,
    /// Large round white notehead with dot
    #[serde(rename = "noteheadRoundWhiteWithDotLarge")]
    NoteheadRoundWhiteWithDotLarge,
    /// Large white diamond
    #[serde(rename = "noteheadSlashDiamondWhite")]
    NoteheadSlashDiamondWhite,
    /// Slash with horizontal ends
    #[serde(rename = "noteheadSlashHorizontalEnds")]
    NoteheadSlashHorizontalEnds,
    /// Muted slash with horizontal ends
    #[serde(rename = "noteheadSlashHorizontalEndsMuted")]
    NoteheadSlashHorizontalEndsMuted,
    /// Slash with vertical ends
    #[serde(rename = "noteheadSlashVerticalEnds")]
    NoteheadSlashVerticalEnds,
    /// Muted slash with vertical ends
    #[serde(rename = "noteheadSlashVerticalEndsMuted")]
    NoteheadSlashVerticalEndsMuted,
    /// Small slash with vertical ends
    #[serde(rename = "noteheadSlashVerticalEndsSmall")]
    NoteheadSlashVerticalEndsSmall,
    /// White slash double whole
    #[serde(rename = "noteheadSlashWhiteDoubleWhole")]
    NoteheadSlashWhiteDoubleWhole,
    /// White slash half
    #[serde(rename = "noteheadSlashWhiteHalf")]
    NoteheadSlashWhiteHalf,
    /// Muted white slash
    #[serde(rename = "noteheadSlashWhiteMuted")]
    NoteheadSlashWhiteMuted,
    /// White slash whole
    #[serde(rename = "noteheadSlashWhiteWhole")]
    NoteheadSlashWhiteWhole,
    /// Large X notehead
    #[serde(rename = "noteheadSlashX")]
    NoteheadSlashX,
    /// Slashed black notehead (bottom left to top right)
    #[serde(rename = "noteheadSlashedBlack1")]
    NoteheadSlashedBlack1,
    /// Slashed black notehead (top left to bottom right)
    #[serde(rename = "noteheadSlashedBlack2")]
    NoteheadSlashedBlack2,
    /// Slashed double whole notehead (bottom left to top right)
    #[serde(rename = "noteheadSlashedDoubleWhole1")]
    NoteheadSlashedDoubleWhole1,
    /// Slashed double whole notehead (top left to bottom right)
    #[serde(rename = "noteheadSlashedDoubleWhole2")]
    NoteheadSlashedDoubleWhole2,
    /// Slashed half notehead (bottom left to top right)
    #[serde(rename = "noteheadSlashedHalf1")]
    NoteheadSlashedHalf1,
    /// Slashed half notehead (top left to bottom right)
    #[serde(rename = "noteheadSlashedHalf2")]
    NoteheadSlashedHalf2,
    /// Slashed whole notehead (bottom left to top right)
    #[serde(rename = "noteheadSlashedWhole1")]
    NoteheadSlashedWhole1,
    /// Slashed whole notehead (top left to bottom right)
    #[serde(rename = "noteheadSlashedWhole2")]
    NoteheadSlashedWhole2,
    /// Square notehead black
    #[serde(rename = "noteheadSquareBlack")]
    NoteheadSquareBlack,
    /// Large square black notehead
    #[serde(rename = "noteheadSquareBlackLarge")]
    NoteheadSquareBlackLarge,
    /// Large square white notehead
    #[serde(rename = "noteheadSquareBlackWhite")]
    NoteheadSquareBlackWhite,
    /// Square notehead white
    #[serde(rename = "noteheadSquareWhite")]
    NoteheadSquareWhite,
    /// Triangle notehead down black
    #[serde(rename = "noteheadTriangleDownBlack")]
    NoteheadTriangleDownBlack,
    /// Triangle notehead down double whole
    #[serde(rename = "noteheadTriangleDownDoubleWhole")]
    NoteheadTriangleDownDoubleWhole,
    /// Triangle notehead down half
    #[serde(rename = "noteheadTriangleDownHalf")]
    NoteheadTriangleDownHalf,
    /// Triangle notehead down white
    #[serde(rename = "noteheadTriangleDownWhite")]
    NoteheadTriangleDownWhite,
    /// Triangle notehead down whole
    #[serde(rename = "noteheadTriangleDownWhole")]
    NoteheadTriangleDownWhole,
    /// Triangle notehead left black
    #[serde(rename = "noteheadTriangleLeftBlack")]
    NoteheadTriangleLeftBlack,
    /// Triangle notehead left white
    #[serde(rename = "noteheadTriangleLeftWhite")]
    NoteheadTriangleLeftWhite,
    /// Triangle notehead right black
    #[serde(rename = "noteheadTriangleRightBlack")]
    NoteheadTriangleRightBlack,
    /// Triangle notehead right white
    #[serde(rename = "noteheadTriangleRightWhite")]
    NoteheadTriangleRightWhite,
    /// Triangle-round notehead down black
    #[serde(rename = "noteheadTriangleRoundDownBlack")]
    NoteheadTriangleRoundDownBlack,
    /// Triangle-round notehead down white
    #[serde(rename = "noteheadTriangleRoundDownWhite")]
    NoteheadTriangleRoundDownWhite,
    /// Triangle notehead up black
    #[serde(rename = "noteheadTriangleUpBlack")]
    NoteheadTriangleUpBlack,
    /// Triangle notehead up double whole
    #[serde(rename = "noteheadTriangleUpDoubleWhole")]
    NoteheadTriangleUpDoubleWhole,
    /// Triangle notehead up half
    #[serde(rename = "noteheadTriangleUpHalf")]
    NoteheadTriangleUpHalf,
    /// Triangle notehead up right black
    #[serde(rename = "noteheadTriangleUpRightBlack")]
    NoteheadTriangleUpRightBlack,
    /// Triangle notehead up right white
    #[serde(rename = "noteheadTriangleUpRightWhite")]
    NoteheadTriangleUpRightWhite,
    /// Triangle notehead up white
    #[serde(rename = "noteheadTriangleUpWhite")]
    NoteheadTriangleUpWhite,
    /// Triangle notehead up whole
    #[serde(rename = "noteheadTriangleUpWhole")]
    NoteheadTriangleUpWhole,
    /// Void notehead with X
    #[serde(rename = "noteheadVoidWithX")]
    NoteheadVoidWithX,
    /// Whole (semibreve) notehead
    #[serde(rename = "noteheadWhole")]
    NoteheadWhole,
    /// Filled whole (semibreve) notehead
    #[serde(rename = "noteheadWholeFilled")]
    NoteheadWholeFilled,
    /// Whole notehead with X
    #[serde(rename = "noteheadWholeWithX")]
    NoteheadWholeWithX,
    /// X notehead black
    #[serde(rename = "noteheadXBlack")]
    NoteheadXBlack,
    /// X notehead double whole
    #[serde(rename = "noteheadXDoubleWhole")]
    NoteheadXDoubleWhole,
    /// X notehead half
    #[serde(rename = "noteheadXHalf")]
    NoteheadXHalf,
    /// Ornate X notehead
    #[serde(rename = "noteheadXOrnate")]
    NoteheadXOrnate,
    /// Ornate X notehead in ellipse
    #[serde(rename = "noteheadXOrnateEllipse")]
    NoteheadXOrnateEllipse,
    /// X notehead whole
    #[serde(rename = "noteheadXWhole")]
    NoteheadXWhole,
    /// a (baseline)
    #[serde(rename = "octaveBaselineA")]
    OctaveBaselineA,
    /// b (baseline)
    #[serde(rename = "octaveBaselineB")]
    OctaveBaselineB,
    /// m (baseline)
    #[serde(rename = "octaveBaselineM")]
    OctaveBaselineM,
    /// v (baseline)
    #[serde(rename = "octaveBaselineV")]
    OctaveBaselineV,
    /// Bassa
    #[serde(rename = "octaveBassa")]
    OctaveBassa,
    /// Loco
    #[serde(rename = "octaveLoco")]
    OctaveLoco,
    /// Left parenthesis for octave signs
    #[serde(rename = "octaveParensLeft")]
    OctaveParensLeft,
    /// Right parenthesis for octave signs
    #[serde(rename = "octaveParensRight")]
    OctaveParensRight,
    /// a (superscript)
    #[serde(rename = "octaveSuperscriptA")]
    OctaveSuperscriptA,
    /// b (superscript)
    #[serde(rename = "octaveSuperscriptB")]
    OctaveSuperscriptB,
    /// m (superscript)
    #[serde(rename = "octaveSuperscriptM")]
    OctaveSuperscriptM,
    /// v (superscript)
    #[serde(rename = "octaveSuperscriptV")]
    OctaveSuperscriptV,
    /// One-handed roll (Stevens)
    #[serde(rename = "oneHandedRollStevens")]
    OneHandedRollStevens,
    /// Two Fusae
    #[serde(rename = "organGerman2Fusae")]
    OrganGerman2Fusae,
    /// Two Minimae
    #[serde(rename = "organGerman2Minimae")]
    OrganGerman2Minimae,
    /// Combining double octave line above
    #[serde(rename = "organGerman2OctaveUp")]
    OrganGerman2OctaveUp,
    /// Two Semifusae
    #[serde(rename = "organGerman2Semifusae")]
    OrganGerman2Semifusae,
    /// Two Semiminimae
    #[serde(rename = "organGerman2Semiminimae")]
    OrganGerman2Semiminimae,
    /// Three Fusae
    #[serde(rename = "organGerman3Fusae")]
    OrganGerman3Fusae,
    /// Three Minimae
    #[serde(rename = "organGerman3Minimae")]
    OrganGerman3Minimae,
    /// Three Semifusae
    #[serde(rename = "organGerman3Semifusae")]
    OrganGerman3Semifusae,
    /// Three Semiminimae
    #[serde(rename = "organGerman3Semiminimae")]
    OrganGerman3Semiminimae,
    /// Four Fusae
    #[serde(rename = "organGerman4Fusae")]
    OrganGerman4Fusae,
    /// Four Minimae
    #[serde(rename = "organGerman4Minimae")]
    OrganGerman4Minimae,
    /// Four Semifusae
    #[serde(rename = "organGerman4Semifusae")]
    OrganGerman4Semifusae,
    /// Four Semiminimae
    #[serde(rename = "organGerman4Semiminimae")]
    OrganGerman4Semiminimae,
    /// Five Fusae
    #[serde(rename = "organGerman5Fusae")]
    OrganGerman5Fusae,
    /// Five Minimae
    #[serde(rename = "organGerman5Minimae")]
    OrganGerman5Minimae,
    /// Five Semifusae
    #[serde(rename = "organGerman5Semifusae")]
    OrganGerman5Semifusae,
    /// Five Semiminimae
    #[serde(rename = "organGerman5Semiminimae")]
    OrganGerman5Semiminimae,
    /// Six Fusae
    #[serde(rename = "organGerman6Fusae")]
    OrganGerman6Fusae,
    /// Six Minimae
    #[serde(rename = "organGerman6Minimae")]
    OrganGerman6Minimae,
    /// Six Semifusae
    #[serde(rename = "organGerman6Semifusae")]
    OrganGerman6Semifusae,
    /// Six Semiminimae
    #[serde(rename = "organGerman6Semiminimae")]
    OrganGerman6Semiminimae,
    /// German organ tablature small A
    #[serde(rename = "organGermanALower")]
    OrganGermanALower,
    /// German organ tablature great A
    #[serde(rename = "organGermanAUpper")]
    OrganGermanAUpper,
    /// Rhythm Dot
    #[serde(rename = "organGermanAugmentationDot")]
    OrganGermanAugmentationDot,
    /// German organ tablature small B
    #[serde(rename = "organGermanBLower")]
    OrganGermanBLower,
    /// German organ tablature great B
    #[serde(rename = "organGermanBUpper")]
    OrganGermanBUpper,
    /// Brevis (Binary) Buxheimer Orgelbuch
    #[serde(rename = "organGermanBuxheimerBrevis2")]
    OrganGermanBuxheimerBrevis2,
    /// Brevis (Ternary) Buxheimer Orgelbuch
    #[serde(rename = "organGermanBuxheimerBrevis3")]
    OrganGermanBuxheimerBrevis3,
    /// Minima Rest Buxheimer Orgelbuch
    #[serde(rename = "organGermanBuxheimerMinimaRest")]
    OrganGermanBuxheimerMinimaRest,
    /// Semibrevis Buxheimer Orgelbuch
    #[serde(rename = "organGermanBuxheimerSemibrevis")]
    OrganGermanBuxheimerSemibrevis,
    /// Semibrevis Rest Buxheimer Orgelbuch
    #[serde(rename = "organGermanBuxheimerSemibrevisRest")]
    OrganGermanBuxheimerSemibrevisRest,
    /// German organ tablature small C
    #[serde(rename = "organGermanCLower")]
    OrganGermanCLower,
    /// German organ tablature great C
    #[serde(rename = "organGermanCUpper")]
    OrganGermanCUpper,
    /// German organ tablature small Cis
    #[serde(rename = "organGermanCisLower")]
    OrganGermanCisLower,
    /// German organ tablature great Cis
    #[serde(rename = "organGermanCisUpper")]
    OrganGermanCisUpper,
    /// German organ tablature small D
    #[serde(rename = "organGermanDLower")]
    OrganGermanDLower,
    /// German organ tablature great D
    #[serde(rename = "organGermanDUpper")]
    OrganGermanDUpper,
    /// German organ tablature small Dis
    #[serde(rename = "organGermanDisLower")]
    OrganGermanDisLower,
    /// German organ tablature great Dis
    #[serde(rename = "organGermanDisUpper")]
    OrganGermanDisUpper,
    /// German organ tablature small E
    #[serde(rename = "organGermanELower")]
    OrganGermanELower,
    /// German organ tablature great E
    #[serde(rename = "organGermanEUpper")]
    OrganGermanEUpper,
    /// German organ tablature small F
    #[serde(rename = "organGermanFLower")]
    OrganGermanFLower,
    /// German organ tablature great F
    #[serde(rename = "organGermanFUpper")]
    OrganGermanFUpper,
    /// German organ tablature small Fis
    #[serde(rename = "organGermanFisLower")]
    OrganGermanFisLower,
    /// German organ tablature great Fis
    #[serde(rename = "organGermanFisUpper")]
    OrganGermanFisUpper,
    /// Fusa
    #[serde(rename = "organGermanFusa")]
    OrganGermanFusa,
    /// Fusa Rest
    #[serde(rename = "organGermanFusaRest")]
    OrganGermanFusaRest,
    /// German organ tablature small G
    #[serde(rename = "organGermanGLower")]
    OrganGermanGLower,
    /// German organ tablature great G
    #[serde(rename = "organGermanGUpper")]
    OrganGermanGUpper,
    /// German organ tablature small Gis
    #[serde(rename = "organGermanGisLower")]
    OrganGermanGisLower,
    /// German organ tablature great Gis
    #[serde(rename = "organGermanGisUpper")]
    OrganGermanGisUpper,
    /// German organ tablature small H
    #[serde(rename = "organGermanHLower")]
    OrganGermanHLower,
    /// German organ tablature great H
    #[serde(rename = "organGermanHUpper")]
    OrganGermanHUpper,
    /// Minima
    #[serde(rename = "organGermanMinima")]
    OrganGermanMinima,
    /// Minima Rest
    #[serde(rename = "organGermanMinimaRest")]
    OrganGermanMinimaRest,
    /// Combining single octave line below
    #[serde(rename = "organGermanOctaveDown")]
    OrganGermanOctaveDown,
    /// Combining single octave line above
    #[serde(rename = "organGermanOctaveUp")]
    OrganGermanOctaveUp,
    /// Semibrevis
    #[serde(rename = "organGermanSemibrevis")]
    OrganGermanSemibrevis,
    /// Semibrevis Rest
    #[serde(rename = "organGermanSemibrevisRest")]
    OrganGermanSemibrevisRest,
    /// Semifusa
    #[serde(rename = "organGermanSemifusa")]
    OrganGermanSemifusa,
    /// Semifusa Rest
    #[serde(rename = "organGermanSemifusaRest")]
    OrganGermanSemifusaRest,
    /// Semiminima
    #[serde(rename = "organGermanSemiminima")]
    OrganGermanSemiminima,
    /// Semiminima Rest
    #[serde(rename = "organGermanSemiminimaRest")]
    OrganGermanSemiminimaRest,
    /// Tie
    #[serde(rename = "organGermanTie")]
    OrganGermanTie,
    /// Ornament bottom left concave stroke
    #[serde(rename = "ornamentBottomLeftConcaveStroke")]
    OrnamentBottomLeftConcaveStroke,
    /// Ornament bottom left concave stroke, large
    #[serde(rename = "ornamentBottomLeftConcaveStrokeLarge")]
    OrnamentBottomLeftConcaveStrokeLarge,
    /// Ornament bottom left convex stroke
    #[serde(rename = "ornamentBottomLeftConvexStroke")]
    OrnamentBottomLeftConvexStroke,
    /// Ornament bottom right concave stroke
    #[serde(rename = "ornamentBottomRightConcaveStroke")]
    OrnamentBottomRightConcaveStroke,
    /// Ornament bottom right convex stroke
    #[serde(rename = "ornamentBottomRightConvexStroke")]
    OrnamentBottomRightConvexStroke,
    /// Comma
    #[serde(rename = "ornamentComma")]
    OrnamentComma,
    /// Double oblique straight lines NW-SE
    #[serde(rename = "ornamentDoubleObliqueLinesAfterNote")]
    OrnamentDoubleObliqueLinesAfterNote,
    /// Double oblique straight lines SW-NE
    #[serde(rename = "ornamentDoubleObliqueLinesBeforeNote")]
    OrnamentDoubleObliqueLinesBeforeNote,
    /// Curve below
    #[serde(rename = "ornamentDownCurve")]
    OrnamentDownCurve,
    /// Haydn ornament
    #[serde(rename = "ornamentHaydn")]
    OrnamentHaydn,
    /// Ornament high left concave stroke
    #[serde(rename = "ornamentHighLeftConcaveStroke")]
    OrnamentHighLeftConcaveStroke,
    /// Ornament high left convex stroke
    #[serde(rename = "ornamentHighLeftConvexStroke")]
    OrnamentHighLeftConvexStroke,
    /// Ornament high right concave stroke
    #[serde(rename = "ornamentHighRightConcaveStroke")]
    OrnamentHighRightConcaveStroke,
    /// Ornament high right convex stroke
    #[serde(rename = "ornamentHighRightConvexStroke")]
    OrnamentHighRightConvexStroke,
    /// Hook after note
    #[serde(rename = "ornamentHookAfterNote")]
    OrnamentHookAfterNote,
    /// Hook before note
    #[serde(rename = "ornamentHookBeforeNote")]
    OrnamentHookBeforeNote,
    /// Left-facing half circle
    #[serde(rename = "ornamentLeftFacingHalfCircle")]
    OrnamentLeftFacingHalfCircle,
    /// Left-facing hook
    #[serde(rename = "ornamentLeftFacingHook")]
    OrnamentLeftFacingHook,
    /// Ornament left +
    #[serde(rename = "ornamentLeftPlus")]
    OrnamentLeftPlus,
    /// Ornament left shake t
    #[serde(rename = "ornamentLeftShakeT")]
    OrnamentLeftShakeT,
    /// Ornament left vertical stroke
    #[serde(rename = "ornamentLeftVerticalStroke")]
    OrnamentLeftVerticalStroke,
    /// Ornament left vertical stroke with cross (+)
    #[serde(rename = "ornamentLeftVerticalStrokeWithCross")]
    OrnamentLeftVerticalStrokeWithCross,
    /// Ornament low left concave stroke
    #[serde(rename = "ornamentLowLeftConcaveStroke")]
    OrnamentLowLeftConcaveStroke,
    /// Ornament low left convex stroke
    #[serde(rename = "ornamentLowLeftConvexStroke")]
    OrnamentLowLeftConvexStroke,
    /// Ornament low right concave stroke
    #[serde(rename = "ornamentLowRightConcaveStroke")]
    OrnamentLowRightConcaveStroke,
    /// Ornament low right convex stroke
    #[serde(rename = "ornamentLowRightConvexStroke")]
    OrnamentLowRightConvexStroke,
    /// Ornament middle vertical stroke
    #[serde(rename = "ornamentMiddleVerticalStroke")]
    OrnamentMiddleVerticalStroke,
    /// Mordent
    #[serde(rename = "ornamentMordent")]
    OrnamentMordent,
    /// Oblique straight line NW-SE
    #[serde(rename = "ornamentObliqueLineAfterNote")]
    OrnamentObliqueLineAfterNote,
    /// Oblique straight line SW-NE
    #[serde(rename = "ornamentObliqueLineBeforeNote")]
    OrnamentObliqueLineBeforeNote,
    /// Oblique straight line tilted NW-SE
    #[serde(rename = "ornamentObliqueLineHorizAfterNote")]
    OrnamentObliqueLineHorizAfterNote,
    /// Oblique straight line tilted SW-NE
    #[serde(rename = "ornamentObliqueLineHorizBeforeNote")]
    OrnamentObliqueLineHorizBeforeNote,
    /// Oriscus
    #[serde(rename = "ornamentOriscus")]
    OrnamentOriscus,
    /// Pincé (Couperin)
    #[serde(rename = "ornamentPinceCouperin")]
    OrnamentPinceCouperin,
    /// Port de voix
    #[serde(rename = "ornamentPortDeVoixV")]
    OrnamentPortDeVoixV,
    /// Supported appoggiatura trill
    #[serde(rename = "ornamentPrecompAppoggTrill")]
    OrnamentPrecompAppoggTrill,
    /// Supported appoggiatura trill with two-note suffix
    #[serde(rename = "ornamentPrecompAppoggTrillSuffix")]
    OrnamentPrecompAppoggTrillSuffix,
    /// Cadence
    #[serde(rename = "ornamentPrecompCadence")]
    OrnamentPrecompCadence,
    /// Cadence with upper prefix
    #[serde(rename = "ornamentPrecompCadenceUpperPrefix")]
    OrnamentPrecompCadenceUpperPrefix,
    /// Cadence with upper prefix and turn
    #[serde(rename = "ornamentPrecompCadenceUpperPrefixTurn")]
    OrnamentPrecompCadenceUpperPrefixTurn,
    /// Cadence with turn
    #[serde(rename = "ornamentPrecompCadenceWithTurn")]
    OrnamentPrecompCadenceWithTurn,
    /// Descending slide
    #[serde(rename = "ornamentPrecompDescendingSlide")]
    OrnamentPrecompDescendingSlide,
    /// Double cadence with lower prefix
    #[serde(rename = "ornamentPrecompDoubleCadenceLowerPrefix")]
    OrnamentPrecompDoubleCadenceLowerPrefix,
    /// Double cadence with upper prefix
    #[serde(rename = "ornamentPrecompDoubleCadenceUpperPrefix")]
    OrnamentPrecompDoubleCadenceUpperPrefix,
    /// Double cadence with upper prefix and turn
    #[serde(rename = "ornamentPrecompDoubleCadenceUpperPrefixTurn")]
    OrnamentPrecompDoubleCadenceUpperPrefixTurn,
    /// Inverted mordent with upper prefix
    #[serde(rename = "ornamentPrecompInvertedMordentUpperPrefix")]
    OrnamentPrecompInvertedMordentUpperPrefix,
    /// Mordent with release
    #[serde(rename = "ornamentPrecompMordentRelease")]
    OrnamentPrecompMordentRelease,
    /// Mordent with upper prefix
    #[serde(rename = "ornamentPrecompMordentUpperPrefix")]
    OrnamentPrecompMordentUpperPrefix,
    /// Pre-beat port de voix followed by multiple mordent (Dandrieu)
    #[serde(rename = "ornamentPrecompPortDeVoixMordent")]
    OrnamentPrecompPortDeVoixMordent,
    /// Slide
    #[serde(rename = "ornamentPrecompSlide")]
    OrnamentPrecompSlide,
    /// Slide-trill with two-note suffix (J.S. Bach)
    #[serde(rename = "ornamentPrecompSlideTrillBach")]
    OrnamentPrecompSlideTrillBach,
    /// Slide-trill (D'Anglebert)
    #[serde(rename = "ornamentPrecompSlideTrillDAnglebert")]
    OrnamentPrecompSlideTrillDAnglebert,
    /// Slide-trill with one-note suffix (Marpurg)
    #[serde(rename = "ornamentPrecompSlideTrillMarpurg")]
    OrnamentPrecompSlideTrillMarpurg,
    /// Slide-trill (Muffat)
    #[serde(rename = "ornamentPrecompSlideTrillMuffat")]
    OrnamentPrecompSlideTrillMuffat,
    /// Slide-trill with two-note suffix (Muffat)
    #[serde(rename = "ornamentPrecompSlideTrillSuffixMuffat")]
    OrnamentPrecompSlideTrillSuffixMuffat,
    /// Trill with lower suffix
    #[serde(rename = "ornamentPrecompTrillLowerSuffix")]
    OrnamentPrecompTrillLowerSuffix,
    /// Trill with two-note suffix (Dandrieu)
    #[serde(rename = "ornamentPrecompTrillSuffixDandrieu")]
    OrnamentPrecompTrillSuffixDandrieu,
    /// Trill with mordent
    #[serde(rename = "ornamentPrecompTrillWithMordent")]
    OrnamentPrecompTrillWithMordent,
    /// Turn-trill with two-note suffix (J.S. Bach)
    #[serde(rename = "ornamentPrecompTurnTrillBach")]
    OrnamentPrecompTurnTrillBach,
    /// Turn-trill (D'Anglebert)
    #[serde(rename = "ornamentPrecompTurnTrillDAnglebert")]
    OrnamentPrecompTurnTrillDAnglebert,
    /// Quilisma
    #[serde(rename = "ornamentQuilisma")]
    OrnamentQuilisma,
    /// Right-facing half circle
    #[serde(rename = "ornamentRightFacingHalfCircle")]
    OrnamentRightFacingHalfCircle,
    /// Right-facing hook
    #[serde(rename = "ornamentRightFacingHook")]
    OrnamentRightFacingHook,
    /// Ornament right vertical stroke
    #[serde(rename = "ornamentRightVerticalStroke")]
    OrnamentRightVerticalStroke,
    /// Schleifer (long mordent)
    #[serde(rename = "ornamentSchleifer")]
    OrnamentSchleifer,
    /// Shake
    #[serde(rename = "ornamentShake3")]
    OrnamentShake3,
    /// Shake (Muffat)
    #[serde(rename = "ornamentShakeMuffat1")]
    OrnamentShakeMuffat1,
    /// Short oblique straight line NW-SE
    #[serde(rename = "ornamentShortObliqueLineAfterNote")]
    OrnamentShortObliqueLineAfterNote,
    /// Short oblique straight line SW-NE
    #[serde(rename = "ornamentShortObliqueLineBeforeNote")]
    OrnamentShortObliqueLineBeforeNote,
    /// Short trill
    #[serde(rename = "ornamentShortTrill")]
    OrnamentShortTrill,
    /// Ornament top left concave stroke
    #[serde(rename = "ornamentTopLeftConcaveStroke")]
    OrnamentTopLeftConcaveStroke,
    /// Ornament top left convex stroke
    #[serde(rename = "ornamentTopLeftConvexStroke")]
    OrnamentTopLeftConvexStroke,
    /// Ornament top right concave stroke
    #[serde(rename = "ornamentTopRightConcaveStroke")]
    OrnamentTopRightConcaveStroke,
    /// Ornament top right convex stroke
    #[serde(rename = "ornamentTopRightConvexStroke")]
    OrnamentTopRightConvexStroke,
    /// Tremblement
    #[serde(rename = "ornamentTremblement")]
    OrnamentTremblement,
    /// Tremblement appuyé (Couperin)
    #[serde(rename = "ornamentTremblementCouperin")]
    OrnamentTremblementCouperin,
    /// Trill
    #[serde(rename = "ornamentTrill")]
    OrnamentTrill,
    /// Turn
    #[serde(rename = "ornamentTurn")]
    OrnamentTurn,
    /// Inverted turn
    #[serde(rename = "ornamentTurnInverted")]
    OrnamentTurnInverted,
    /// Turn with slash
    #[serde(rename = "ornamentTurnSlash")]
    OrnamentTurnSlash,
    /// Turn up
    #[serde(rename = "ornamentTurnUp")]
    OrnamentTurnUp,
    /// Inverted turn up
    #[serde(rename = "ornamentTurnUpS")]
    OrnamentTurnUpS,
    /// Curve above
    #[serde(rename = "ornamentUpCurve")]
    OrnamentUpCurve,
    /// Vertical line
    #[serde(rename = "ornamentVerticalLine")]
    OrnamentVerticalLine,
    /// Ornament zig-zag line without right-hand end
    #[serde(rename = "ornamentZigZagLineNoRightEnd")]
    OrnamentZigZagLineNoRightEnd,
    /// Ornament zig-zag line with right-hand end
    #[serde(rename = "ornamentZigZagLineWithRightEnd")]
    OrnamentZigZagLineWithRightEnd,
    /// Ottava
    #[serde(rename = "ottava")]
    Ottava,
    /// Ottava alta
    #[serde(rename = "ottavaAlta")]
    OttavaAlta,
    /// Ottava bassa
    #[serde(rename = "ottavaBassa")]
    OttavaBassa,
    /// Ottava bassa (ba)
    #[serde(rename = "ottavaBassaBa")]
    OttavaBassaBa,
    /// Ottava bassa (8vb)
    #[serde(rename = "ottavaBassaVb")]
    OttavaBassaVb,
    /// Penderecki unmeasured tremolo
    #[serde(rename = "pendereckiTremolo")]
    PendereckiTremolo,
    /// Agogo
    #[serde(rename = "pictAgogo")]
    PictAgogo,
    /// Almglocken
    #[serde(rename = "pictAlmglocken")]
    PictAlmglocken,
    /// Anvil
    #[serde(rename = "pictAnvil")]
    PictAnvil,
    /// Bamboo tube chimes
    #[serde(rename = "pictBambooChimes")]
    PictBambooChimes,
    /// Bamboo scraper
    #[serde(rename = "pictBambooScraper")]
    PictBambooScraper,
    /// Bass drum
    #[serde(rename = "pictBassDrum")]
    PictBassDrum,
    /// Bass drum on side
    #[serde(rename = "pictBassDrumOnSide")]
    PictBassDrumOnSide,
    /// Bow
    #[serde(rename = "pictBeaterBow")]
    PictBeaterBow,
    /// Box for percussion beater
    #[serde(rename = "pictBeaterBox")]
    PictBeaterBox,
    /// Brass mallets down
    #[serde(rename = "pictBeaterBrassMalletsDown")]
    PictBeaterBrassMalletsDown,
    /// Brass mallets left
    #[serde(rename = "pictBeaterBrassMalletsLeft")]
    PictBeaterBrassMalletsLeft,
    /// Brass mallets right
    #[serde(rename = "pictBeaterBrassMalletsRight")]
    PictBeaterBrassMalletsRight,
    /// Brass mallets up
    #[serde(rename = "pictBeaterBrassMalletsUp")]
    PictBeaterBrassMalletsUp,
    /// Combining dashed circle for round beaters (plated)
    #[serde(rename = "pictBeaterCombiningDashedCircle")]
    PictBeaterCombiningDashedCircle,
    /// Combining parentheses for round beaters (padded)
    #[serde(rename = "pictBeaterCombiningParentheses")]
    PictBeaterCombiningParentheses,
    /// Double bass drum stick down
    #[serde(rename = "pictBeaterDoubleBassDrumDown")]
    PictBeaterDoubleBassDrumDown,
    /// Double bass drum stick up
    #[serde(rename = "pictBeaterDoubleBassDrumUp")]
    PictBeaterDoubleBassDrumUp,
    /// Finger
    #[serde(rename = "pictBeaterFinger")]
    PictBeaterFinger,
    /// Fingernails
    #[serde(rename = "pictBeaterFingernails")]
    PictBeaterFingernails,
    /// Fist
    #[serde(rename = "pictBeaterFist")]
    PictBeaterFist,
    /// Guiro scraper
    #[serde(rename = "pictBeaterGuiroScraper")]
    PictBeaterGuiroScraper,
    /// Hammer
    #[serde(rename = "pictBeaterHammer")]
    PictBeaterHammer,
    /// Metal hammer, down
    #[serde(rename = "pictBeaterHammerMetalDown")]
    PictBeaterHammerMetalDown,
    /// Metal hammer, up
    #[serde(rename = "pictBeaterHammerMetalUp")]
    PictBeaterHammerMetalUp,
    /// Plastic hammer, down
    #[serde(rename = "pictBeaterHammerPlasticDown")]
    PictBeaterHammerPlasticDown,
    /// Plastic hammer, up
    #[serde(rename = "pictBeaterHammerPlasticUp")]
    PictBeaterHammerPlasticUp,
    /// Wooden hammer, down
    #[serde(rename = "pictBeaterHammerWoodDown")]
    PictBeaterHammerWoodDown,
    /// Wooden hammer, up
    #[serde(rename = "pictBeaterHammerWoodUp")]
    PictBeaterHammerWoodUp,
    /// Hand
    #[serde(rename = "pictBeaterHand")]
    PictBeaterHand,
    /// Hard bass drum stick down
    #[serde(rename = "pictBeaterHardBassDrumDown")]
    PictBeaterHardBassDrumDown,
    /// Hard bass drum stick up
    #[serde(rename = "pictBeaterHardBassDrumUp")]
    PictBeaterHardBassDrumUp,
    /// Hard glockenspiel stick down
    #[serde(rename = "pictBeaterHardGlockenspielDown")]
    PictBeaterHardGlockenspielDown,
    /// Hard glockenspiel stick left
    #[serde(rename = "pictBeaterHardGlockenspielLeft")]
    PictBeaterHardGlockenspielLeft,
    /// Hard glockenspiel stick right
    #[serde(rename = "pictBeaterHardGlockenspielRight")]
    PictBeaterHardGlockenspielRight,
    /// Hard glockenspiel stick up
    #[serde(rename = "pictBeaterHardGlockenspielUp")]
    PictBeaterHardGlockenspielUp,
    /// Hard timpani stick down
    #[serde(rename = "pictBeaterHardTimpaniDown")]
    PictBeaterHardTimpaniDown,
    /// Hard timpani stick left
    #[serde(rename = "pictBeaterHardTimpaniLeft")]
    PictBeaterHardTimpaniLeft,
    /// Hard timpani stick right
    #[serde(rename = "pictBeaterHardTimpaniRight")]
    PictBeaterHardTimpaniRight,
    /// Hard timpani stick up
    #[serde(rename = "pictBeaterHardTimpaniUp")]
    PictBeaterHardTimpaniUp,
    /// Hard xylophone stick down
    #[serde(rename = "pictBeaterHardXylophoneDown")]
    PictBeaterHardXylophoneDown,
    /// Hard xylophone stick left
    #[serde(rename = "pictBeaterHardXylophoneLeft")]
    PictBeaterHardXylophoneLeft,
    /// Hard xylophone stick right
    #[serde(rename = "pictBeaterHardXylophoneRight")]
    PictBeaterHardXylophoneRight,
    /// Hard xylophone stick up
    #[serde(rename = "pictBeaterHardXylophoneUp")]
    PictBeaterHardXylophoneUp,
    /// Hard yarn beater down
    #[serde(rename = "pictBeaterHardYarnDown")]
    PictBeaterHardYarnDown,
    /// Hard yarn beater left
    #[serde(rename = "pictBeaterHardYarnLeft")]
    PictBeaterHardYarnLeft,
    /// Hard yarn beater right
    #[serde(rename = "pictBeaterHardYarnRight")]
    PictBeaterHardYarnRight,
    /// Hard yarn beater up
    #[serde(rename = "pictBeaterHardYarnUp")]
    PictBeaterHardYarnUp,
    /// Jazz sticks down
    #[serde(rename = "pictBeaterJazzSticksDown")]
    PictBeaterJazzSticksDown,
    /// Jazz sticks up
    #[serde(rename = "pictBeaterJazzSticksUp")]
    PictBeaterJazzSticksUp,
    /// Knitting needle
    #[serde(rename = "pictBeaterKnittingNeedle")]
    PictBeaterKnittingNeedle,
    /// Chime hammer up
    #[serde(rename = "pictBeaterMallet")]
    PictBeaterMallet,
    /// Chime hammer down
    #[serde(rename = "pictBeaterMalletDown")]
    PictBeaterMalletDown,
    /// Medium bass drum stick down
    #[serde(rename = "pictBeaterMediumBassDrumDown")]
    PictBeaterMediumBassDrumDown,
    /// Medium bass drum stick up
    #[serde(rename = "pictBeaterMediumBassDrumUp")]
    PictBeaterMediumBassDrumUp,
    /// Medium timpani stick down
    #[serde(rename = "pictBeaterMediumTimpaniDown")]
    PictBeaterMediumTimpaniDown,
    /// Medium timpani stick left
    #[serde(rename = "pictBeaterMediumTimpaniLeft")]
    PictBeaterMediumTimpaniLeft,
    /// Medium timpani stick right
    #[serde(rename = "pictBeaterMediumTimpaniRight")]
    PictBeaterMediumTimpaniRight,
    /// Medium timpani stick up
    #[serde(rename = "pictBeaterMediumTimpaniUp")]
    PictBeaterMediumTimpaniUp,
    /// Medium xylophone stick down
    #[serde(rename = "pictBeaterMediumXylophoneDown")]
    PictBeaterMediumXylophoneDown,
    /// Medium xylophone stick left
    #[serde(rename = "pictBeaterMediumXylophoneLeft")]
    PictBeaterMediumXylophoneLeft,
    /// Medium xylophone stick right
    #[serde(rename = "pictBeaterMediumXylophoneRight")]
    PictBeaterMediumXylophoneRight,
    /// Medium xylophone stick up
    #[serde(rename = "pictBeaterMediumXylophoneUp")]
    PictBeaterMediumXylophoneUp,
    /// Medium yarn beater down
    #[serde(rename = "pictBeaterMediumYarnDown")]
    PictBeaterMediumYarnDown,
    /// Medium yarn beater left
    #[serde(rename = "pictBeaterMediumYarnLeft")]
    PictBeaterMediumYarnLeft,
    /// Medium yarn beater right
    #[serde(rename = "pictBeaterMediumYarnRight")]
    PictBeaterMediumYarnRight,
    /// Medium yarn beater up
    #[serde(rename = "pictBeaterMediumYarnUp")]
    PictBeaterMediumYarnUp,
    /// Metal bass drum stick down
    #[serde(rename = "pictBeaterMetalBassDrumDown")]
    PictBeaterMetalBassDrumDown,
    /// Metal bass drum stick up
    #[serde(rename = "pictBeaterMetalBassDrumUp")]
    PictBeaterMetalBassDrumUp,
    /// Metal beater down
    #[serde(rename = "pictBeaterMetalDown")]
    PictBeaterMetalDown,
    /// Metal hammer
    #[serde(rename = "pictBeaterMetalHammer")]
    PictBeaterMetalHammer,
    /// Metal beater, left
    #[serde(rename = "pictBeaterMetalLeft")]
    PictBeaterMetalLeft,
    /// Metal beater, right
    #[serde(rename = "pictBeaterMetalRight")]
    PictBeaterMetalRight,
    /// Metal beater, up
    #[serde(rename = "pictBeaterMetalUp")]
    PictBeaterMetalUp,
    /// Snare sticks down
    #[serde(rename = "pictBeaterSnareSticksDown")]
    PictBeaterSnareSticksDown,
    /// Snare sticks up
    #[serde(rename = "pictBeaterSnareSticksUp")]
    PictBeaterSnareSticksUp,
    /// Soft bass drum stick down
    #[serde(rename = "pictBeaterSoftBassDrumDown")]
    PictBeaterSoftBassDrumDown,
    /// Soft bass drum stick up
    #[serde(rename = "pictBeaterSoftBassDrumUp")]
    PictBeaterSoftBassDrumUp,
    /// Soft glockenspiel stick down
    #[serde(rename = "pictBeaterSoftGlockenspielDown")]
    PictBeaterSoftGlockenspielDown,
    /// Soft glockenspiel stick left
    #[serde(rename = "pictBeaterSoftGlockenspielLeft")]
    PictBeaterSoftGlockenspielLeft,
    /// Soft glockenspiel stick right
    #[serde(rename = "pictBeaterSoftGlockenspielRight")]
    PictBeaterSoftGlockenspielRight,
    /// Soft glockenspiel stick up
    #[serde(rename = "pictBeaterSoftGlockenspielUp")]
    PictBeaterSoftGlockenspielUp,
    /// Soft timpani stick down
    #[serde(rename = "pictBeaterSoftTimpaniDown")]
    PictBeaterSoftTimpaniDown,
    /// Soft timpani stick left
    #[serde(rename = "pictBeaterSoftTimpaniLeft")]
    PictBeaterSoftTimpaniLeft,
    /// Soft timpani stick right
    #[serde(rename = "pictBeaterSoftTimpaniRight")]
    PictBeaterSoftTimpaniRight,
    /// Soft timpani stick up
    #[serde(rename = "pictBeaterSoftTimpaniUp")]
    PictBeaterSoftTimpaniUp,
    /// Soft xylophone beaters
    #[serde(rename = "pictBeaterSoftXylophone")]
    PictBeaterSoftXylophone,
    /// Soft xylophone stick down
    #[serde(rename = "pictBeaterSoftXylophoneDown")]
    PictBeaterSoftXylophoneDown,
    /// Soft xylophone stick left
    #[serde(rename = "pictBeaterSoftXylophoneLeft")]
    PictBeaterSoftXylophoneLeft,
    /// Soft xylophone stick right
    #[serde(rename = "pictBeaterSoftXylophoneRight")]
    PictBeaterSoftXylophoneRight,
    /// Soft xylophone stick up
    #[serde(rename = "pictBeaterSoftXylophoneUp")]
    PictBeaterSoftXylophoneUp,
    /// Soft yarn beater down
    #[serde(rename = "pictBeaterSoftYarnDown")]
    PictBeaterSoftYarnDown,
    /// Soft yarn beater left
    #[serde(rename = "pictBeaterSoftYarnLeft")]
    PictBeaterSoftYarnLeft,
    /// Soft yarn beater right
    #[serde(rename = "pictBeaterSoftYarnRight")]
    PictBeaterSoftYarnRight,
    /// Soft yarn beater up
    #[serde(rename = "pictBeaterSoftYarnUp")]
    PictBeaterSoftYarnUp,
    /// Spoon-shaped wooden mallet
    #[serde(rename = "pictBeaterSpoonWoodenMallet")]
    PictBeaterSpoonWoodenMallet,
    /// Superball beater down
    #[serde(rename = "pictBeaterSuperballDown")]
    PictBeaterSuperballDown,
    /// Superball beater left
    #[serde(rename = "pictBeaterSuperballLeft")]
    PictBeaterSuperballLeft,
    /// Superball beater right
    #[serde(rename = "pictBeaterSuperballRight")]
    PictBeaterSuperballRight,
    /// Superball beater up
    #[serde(rename = "pictBeaterSuperballUp")]
    PictBeaterSuperballUp,
    /// Triangle beater down
    #[serde(rename = "pictBeaterTriangleDown")]
    PictBeaterTriangleDown,
    /// Triangle beater plain
    #[serde(rename = "pictBeaterTrianglePlain")]
    PictBeaterTrianglePlain,
    /// Triangle beater up
    #[serde(rename = "pictBeaterTriangleUp")]
    PictBeaterTriangleUp,
    /// Wire brushes down
    #[serde(rename = "pictBeaterWireBrushesDown")]
    PictBeaterWireBrushesDown,
    /// Wire brushes up
    #[serde(rename = "pictBeaterWireBrushesUp")]
    PictBeaterWireBrushesUp,
    /// Wood timpani stick down
    #[serde(rename = "pictBeaterWoodTimpaniDown")]
    PictBeaterWoodTimpaniDown,
    /// Wood timpani stick left
    #[serde(rename = "pictBeaterWoodTimpaniLeft")]
    PictBeaterWoodTimpaniLeft,
    /// Wood timpani stick right
    #[serde(rename = "pictBeaterWoodTimpaniRight")]
    PictBeaterWoodTimpaniRight,
    /// Wood timpani stick up
    #[serde(rename = "pictBeaterWoodTimpaniUp")]
    PictBeaterWoodTimpaniUp,
    /// Wood xylophone stick down
    #[serde(rename = "pictBeaterWoodXylophoneDown")]
    PictBeaterWoodXylophoneDown,
    /// Wood xylophone stick left
    #[serde(rename = "pictBeaterWoodXylophoneLeft")]
    PictBeaterWoodXylophoneLeft,
    /// Wood xylophone stick right
    #[serde(rename = "pictBeaterWoodXylophoneRight")]
    PictBeaterWoodXylophoneRight,
    /// Wood xylophone stick up
    #[serde(rename = "pictBeaterWoodXylophoneUp")]
    PictBeaterWoodXylophoneUp,
    /// Bell
    #[serde(rename = "pictBell")]
    PictBell,
    /// Bell of cymbal
    #[serde(rename = "pictBellOfCymbal")]
    PictBellOfCymbal,
    /// Bell plate
    #[serde(rename = "pictBellPlate")]
    PictBellPlate,
    /// Bell tree
    #[serde(rename = "pictBellTree")]
    PictBellTree,
    /// Bird whistle
    #[serde(rename = "pictBirdWhistle")]
    PictBirdWhistle,
    /// Board clapper
    #[serde(rename = "pictBoardClapper")]
    PictBoardClapper,
    /// Bongos
    #[serde(rename = "pictBongos")]
    PictBongos,
    /// Brake drum
    #[serde(rename = "pictBrakeDrum")]
    PictBrakeDrum,
    /// Cabasa
    #[serde(rename = "pictCabasa")]
    PictCabasa,
    /// Cannon
    #[serde(rename = "pictCannon")]
    PictCannon,
    /// Car horn
    #[serde(rename = "pictCarHorn")]
    PictCarHorn,
    /// Castanets
    #[serde(rename = "pictCastanets")]
    PictCastanets,
    /// Castanets with handle
    #[serde(rename = "pictCastanetsWithHandle")]
    PictCastanetsWithHandle,
    /// Celesta
    #[serde(rename = "pictCelesta")]
    PictCelesta,
    /// Cencerro
    #[serde(rename = "pictCencerro")]
    PictCencerro,
    /// Center (Weinberg)
    #[serde(rename = "pictCenter1")]
    PictCenter1,
    /// Center (Ghent)
    #[serde(rename = "pictCenter2")]
    PictCenter2,
    /// Center (Caltabiano)
    #[serde(rename = "pictCenter3")]
    PictCenter3,
    /// Chain rattle
    #[serde(rename = "pictChainRattle")]
    PictChainRattle,
    /// Chimes
    #[serde(rename = "pictChimes")]
    PictChimes,
    /// Chinese cymbal
    #[serde(rename = "pictChineseCymbal")]
    PictChineseCymbal,
    /// Choke (Weinberg)
    #[serde(rename = "pictChokeCymbal")]
    PictChokeCymbal,
    /// Claves
    #[serde(rename = "pictClaves")]
    PictClaves,
    /// Coins
    #[serde(rename = "pictCoins")]
    PictCoins,
    /// Conga
    #[serde(rename = "pictConga")]
    PictConga,
    /// Cow bell
    #[serde(rename = "pictCowBell")]
    PictCowBell,
    /// Crash cymbals
    #[serde(rename = "pictCrashCymbals")]
    PictCrashCymbals,
    /// Crotales
    #[serde(rename = "pictCrotales")]
    PictCrotales,
    /// Combining crush for stem
    #[serde(rename = "pictCrushStem")]
    PictCrushStem,
    /// Cuica
    #[serde(rename = "pictCuica")]
    PictCuica,
    /// Cymbal tongs
    #[serde(rename = "pictCymbalTongs")]
    PictCymbalTongs,
    /// Damp
    #[serde(rename = "pictDamp1")]
    PictDamp1,
    /// Damp 2
    #[serde(rename = "pictDamp2")]
    PictDamp2,
    /// Damp 3
    #[serde(rename = "pictDamp3")]
    PictDamp3,
    /// Damp 4
    #[serde(rename = "pictDamp4")]
    PictDamp4,
    /// Combining X for stem (dead note)
    #[serde(rename = "pictDeadNoteStem")]
    PictDeadNoteStem,
    /// Drum stick
    #[serde(rename = "pictDrumStick")]
    PictDrumStick,
    /// Duck call
    #[serde(rename = "pictDuckCall")]
    PictDuckCall,
    /// Edge of cymbal
    #[serde(rename = "pictEdgeOfCymbal")]
    PictEdgeOfCymbal,
    /// Empty trapezoid
    #[serde(rename = "pictEmptyTrap")]
    PictEmptyTrap,
    /// Finger cymbals
    #[serde(rename = "pictFingerCymbals")]
    PictFingerCymbals,
    /// Flexatone
    #[serde(rename = "pictFlexatone")]
    PictFlexatone,
    /// Football rattle
    #[serde(rename = "pictFootballRatchet")]
    PictFootballRatchet,
    /// Glass harmonica
    #[serde(rename = "pictGlassHarmonica")]
    PictGlassHarmonica,
    /// Glass harp
    #[serde(rename = "pictGlassHarp")]
    PictGlassHarp,
    /// Glass plate chimes
    #[serde(rename = "pictGlassPlateChimes")]
    PictGlassPlateChimes,
    /// Glass tube chimes
    #[serde(rename = "pictGlassTubeChimes")]
    PictGlassTubeChimes,
    /// Glockenspiel
    #[serde(rename = "pictGlsp")]
    PictGlsp,
    /// Glockenspiel (Smith Brindle)
    #[serde(rename = "pictGlspSmithBrindle")]
    PictGlspSmithBrindle,
    /// Goblet drum (djembe, dumbek)
    #[serde(rename = "pictGobletDrum")]
    PictGobletDrum,
    /// Gong
    #[serde(rename = "pictGong")]
    PictGong,
    /// Gong with button (nipple)
    #[serde(rename = "pictGongWithButton")]
    PictGongWithButton,
    /// Guiro
    #[serde(rename = "pictGuiro")]
    PictGuiro,
    /// Hard gum beater, down
    #[serde(rename = "pictGumHardDown")]
    PictGumHardDown,
    /// Hard gum beater, left
    #[serde(rename = "pictGumHardLeft")]
    PictGumHardLeft,
    /// Hard gum beater, right
    #[serde(rename = "pictGumHardRight")]
    PictGumHardRight,
    /// Hard gum beater, up
    #[serde(rename = "pictGumHardUp")]
    PictGumHardUp,
    /// Medium gum beater, down
    #[serde(rename = "pictGumMediumDown")]
    PictGumMediumDown,
    /// Medium gum beater, left
    #[serde(rename = "pictGumMediumLeft")]
    PictGumMediumLeft,
    /// Medium gum beater, right
    #[serde(rename = "pictGumMediumRight")]
    PictGumMediumRight,
    /// Medium gum beater, up
    #[serde(rename = "pictGumMediumUp")]
    PictGumMediumUp,
    /// Soft gum beater, down
    #[serde(rename = "pictGumSoftDown")]
    PictGumSoftDown,
    /// Soft gum beater, left
    #[serde(rename = "pictGumSoftLeft")]
    PictGumSoftLeft,
    /// Soft gum beater, right
    #[serde(rename = "pictGumSoftRight")]
    PictGumSoftRight,
    /// Soft gum beater, up
    #[serde(rename = "pictGumSoftUp")]
    PictGumSoftUp,
    /// Half-open
    #[serde(rename = "pictHalfOpen1")]
    PictHalfOpen1,
    /// Half-open 2 (Weinberg)
    #[serde(rename = "pictHalfOpen2")]
    PictHalfOpen2,
    /// Handbell
    #[serde(rename = "pictHandbell")]
    PictHandbell,
    /// Hi-hat
    #[serde(rename = "pictHiHat")]
    PictHiHat,
    /// Hi-hat cymbals on stand
    #[serde(rename = "pictHiHatOnStand")]
    PictHiHatOnStand,
    /// Jaw harp
    #[serde(rename = "pictJawHarp")]
    PictJawHarp,
    /// Jingle bells
    #[serde(rename = "pictJingleBells")]
    PictJingleBells,
    /// Klaxon horn
    #[serde(rename = "pictKlaxonHorn")]
    PictKlaxonHorn,
    /// Right hand (Agostini)
    #[serde(rename = "pictLeftHandCircle")]
    PictLeftHandCircle,
    /// Lion's roar
    #[serde(rename = "pictLionsRoar")]
    PictLionsRoar,
    /// Lithophone
    #[serde(rename = "pictLithophone")]
    PictLithophone,
    /// Log drum
    #[serde(rename = "pictLogDrum")]
    PictLogDrum,
    /// Lotus flute
    #[serde(rename = "pictLotusFlute")]
    PictLotusFlute,
    /// Marimba
    #[serde(rename = "pictMar")]
    PictMar,
    /// Marimba (Smith Brindle)
    #[serde(rename = "pictMarSmithBrindle")]
    PictMarSmithBrindle,
    /// Maraca
    #[serde(rename = "pictMaraca")]
    PictMaraca,
    /// Maracas
    #[serde(rename = "pictMaracas")]
    PictMaracas,
    /// Megaphone
    #[serde(rename = "pictMegaphone")]
    PictMegaphone,
    /// Metal plate chimes
    #[serde(rename = "pictMetalPlateChimes")]
    PictMetalPlateChimes,
    /// Metal tube chimes
    #[serde(rename = "pictMetalTubeChimes")]
    PictMetalTubeChimes,
    /// Musical saw
    #[serde(rename = "pictMusicalSaw")]
    PictMusicalSaw,
    /// Normal position (Caltabiano)
    #[serde(rename = "pictNormalPosition")]
    PictNormalPosition,
    /// On rim
    #[serde(rename = "pictOnRim")]
    PictOnRim,
    /// Open
    #[serde(rename = "pictOpen")]
    PictOpen,
    /// Closed / rim shot
    #[serde(rename = "pictOpenRimShot")]
    PictOpenRimShot,
    /// Pistol shot
    #[serde(rename = "pictPistolShot")]
    PictPistolShot,
    /// Police whistle
    #[serde(rename = "pictPoliceWhistle")]
    PictPoliceWhistle,
    /// Quijada (jawbone)
    #[serde(rename = "pictQuijada")]
    PictQuijada,
    /// Rainstick
    #[serde(rename = "pictRainstick")]
    PictRainstick,
    /// Ratchet
    #[serde(rename = "pictRatchet")]
    PictRatchet,
    /// Reco-reco
    #[serde(rename = "pictRecoReco")]
    PictRecoReco,
    /// Left hand (Agostini)
    #[serde(rename = "pictRightHandSquare")]
    PictRightHandSquare,
    /// Rim or edge (Weinberg)
    #[serde(rename = "pictRim1")]
    PictRim1,
    /// Rim (Ghent)
    #[serde(rename = "pictRim2")]
    PictRim2,
    /// Rim (Caltabiano)
    #[serde(rename = "pictRim3")]
    PictRim3,
    /// Rim shot for stem
    #[serde(rename = "pictRimShotOnStem")]
    PictRimShotOnStem,
    /// Sandpaper blocks
    #[serde(rename = "pictSandpaperBlocks")]
    PictSandpaperBlocks,
    /// Scrape around rim (counter-clockwise)
    #[serde(rename = "pictScrapeAroundRim")]
    PictScrapeAroundRim,
    /// Scrape around rim (clockwise)
    #[serde(rename = "pictScrapeAroundRimClockwise")]
    PictScrapeAroundRimClockwise,
    /// Scrape from center to edge
    #[serde(rename = "pictScrapeCenterToEdge")]
    PictScrapeCenterToEdge,
    /// Scrape from edge to center
    #[serde(rename = "pictScrapeEdgeToCenter")]
    PictScrapeEdgeToCenter,
    /// Shell bells
    #[serde(rename = "pictShellBells")]
    PictShellBells,
    /// Shell chimes
    #[serde(rename = "pictShellChimes")]
    PictShellChimes,
    /// Siren
    #[serde(rename = "pictSiren")]
    PictSiren,
    /// Sistrum
    #[serde(rename = "pictSistrum")]
    PictSistrum,
    /// Sizzle cymbal
    #[serde(rename = "pictSizzleCymbal")]
    PictSizzleCymbal,
    /// Sleigh bell
    #[serde(rename = "pictSleighBell")]
    PictSleighBell,
    /// Slide brush on gong
    #[serde(rename = "pictSlideBrushOnGong")]
    PictSlideBrushOnGong,
    /// Slide whistle
    #[serde(rename = "pictSlideWhistle")]
    PictSlideWhistle,
    /// Slit drum
    #[serde(rename = "pictSlitDrum")]
    PictSlitDrum,
    /// Snare drum
    #[serde(rename = "pictSnareDrum")]
    PictSnareDrum,
    /// Military snare drum
    #[serde(rename = "pictSnareDrumMilitary")]
    PictSnareDrumMilitary,
    /// Snare drum, snares off
    #[serde(rename = "pictSnareDrumSnaresOff")]
    PictSnareDrumSnaresOff,
    /// Steel drums
    #[serde(rename = "pictSteelDrums")]
    PictSteelDrums,
    /// Stick shot
    #[serde(rename = "pictStickShot")]
    PictStickShot,
    /// Superball
    #[serde(rename = "pictSuperball")]
    PictSuperball,
    /// Suspended cymbal
    #[serde(rename = "pictSuspendedCymbal")]
    PictSuspendedCymbal,
    /// Combining swish for stem
    #[serde(rename = "pictSwishStem")]
    PictSwishStem,
    /// Indian tabla
    #[serde(rename = "pictTabla")]
    PictTabla,
    /// Tam-tam
    #[serde(rename = "pictTamTam")]
    PictTamTam,
    /// Tam-tam with beater (Smith Brindle)
    #[serde(rename = "pictTamTamWithBeater")]
    PictTamTamWithBeater,
    /// Tambourine
    #[serde(rename = "pictTambourine")]
    PictTambourine,
    /// Temple blocks
    #[serde(rename = "pictTempleBlocks")]
    PictTempleBlocks,
    /// Tenor drum
    #[serde(rename = "pictTenorDrum")]
    PictTenorDrum,
    /// Thundersheet
    #[serde(rename = "pictThundersheet")]
    PictThundersheet,
    /// Timbales
    #[serde(rename = "pictTimbales")]
    PictTimbales,
    /// Timpani
    #[serde(rename = "pictTimpani")]
    PictTimpani,
    /// Tom-tom
    #[serde(rename = "pictTomTom")]
    PictTomTom,
    /// Chinese tom-tom
    #[serde(rename = "pictTomTomChinese")]
    PictTomTomChinese,
    /// Indo-American tom tom
    #[serde(rename = "pictTomTomIndoAmerican")]
    PictTomTomIndoAmerican,
    /// Japanese tom-tom
    #[serde(rename = "pictTomTomJapanese")]
    PictTomTomJapanese,
    /// Triangle
    #[serde(rename = "pictTriangle")]
    PictTriangle,
    /// Tubaphone
    #[serde(rename = "pictTubaphone")]
    PictTubaphone,
    /// Tubular bells
    #[serde(rename = "pictTubularBells")]
    PictTubularBells,
    /// Combining turn left for stem
    #[serde(rename = "pictTurnLeftStem")]
    PictTurnLeftStem,
    /// Combining turn left or right for stem
    #[serde(rename = "pictTurnRightLeftStem")]
    PictTurnRightLeftStem,
    /// Combining turn right for stem
    #[serde(rename = "pictTurnRightStem")]
    PictTurnRightStem,
    /// Vibraphone
    #[serde(rename = "pictVib")]
    PictVib,
    /// Metallophone (vibraphone motor off)
    #[serde(rename = "pictVibMotorOff")]
    PictVibMotorOff,
    /// Vibraphone (Smith Brindle)
    #[serde(rename = "pictVibSmithBrindle")]
    PictVibSmithBrindle,
    /// Vibraslap
    #[serde(rename = "pictVibraslap")]
    PictVibraslap,
    /// Vietnamese hat cymbal
    #[serde(rename = "pictVietnameseHat")]
    PictVietnameseHat,
    /// Whip
    #[serde(rename = "pictWhip")]
    PictWhip,
    /// Wind chimes (glass)
    #[serde(rename = "pictWindChimesGlass")]
    PictWindChimesGlass,
    /// Wind machine
    #[serde(rename = "pictWindMachine")]
    PictWindMachine,
    /// Wind whistle (or mouth siren)
    #[serde(rename = "pictWindWhistle")]
    PictWindWhistle,
    /// Wood block
    #[serde(rename = "pictWoodBlock")]
    PictWoodBlock,
    /// Wound beater, hard core down
    #[serde(rename = "pictWoundHardDown")]
    PictWoundHardDown,
    /// Wound beater, hard core left
    #[serde(rename = "pictWoundHardLeft")]
    PictWoundHardLeft,
    /// Wound beater, hard core right
    #[serde(rename = "pictWoundHardRight")]
    PictWoundHardRight,
    /// Wound beater, hard core up
    #[serde(rename = "pictWoundHardUp")]
    PictWoundHardUp,
    /// Wound beater, soft core down
    #[serde(rename = "pictWoundSoftDown")]
    PictWoundSoftDown,
    /// Wound beater, soft core left
    #[serde(rename = "pictWoundSoftLeft")]
    PictWoundSoftLeft,
    /// Wound beater, soft core right
    #[serde(rename = "pictWoundSoftRight")]
    PictWoundSoftRight,
    /// Wound beater, soft core up
    #[serde(rename = "pictWoundSoftUp")]
    PictWoundSoftUp,
    /// Xylophone
    #[serde(rename = "pictXyl")]
    PictXyl,
    /// Bass xylophone
    #[serde(rename = "pictXylBass")]
    PictXylBass,
    /// Xylophone (Smith Brindle)
    #[serde(rename = "pictXylSmithBrindle")]
    PictXylSmithBrindle,
    /// Tenor xylophone
    #[serde(rename = "pictXylTenor")]
    PictXylTenor,
    /// Trough tenor xylophone
    #[serde(rename = "pictXylTenorTrough")]
    PictXylTenorTrough,
    /// Trough xylophone
    #[serde(rename = "pictXylTrough")]
    PictXylTrough,
    /// Buzz pizzicato
    #[serde(rename = "pluckedBuzzPizzicato")]
    PluckedBuzzPizzicato,
    /// Damp
    #[serde(rename = "pluckedDamp")]
    PluckedDamp,
    /// Damp all
    #[serde(rename = "pluckedDampAll")]
    PluckedDampAll,
    /// Damp for stem
    #[serde(rename = "pluckedDampOnStem")]
    PluckedDampOnStem,
    /// Fingernail flick
    #[serde(rename = "pluckedFingernailFlick")]
    PluckedFingernailFlick,
    /// Left-hand pizzicato
    #[serde(rename = "pluckedLeftHandPizzicato")]
    PluckedLeftHandPizzicato,
    /// Plectrum
    #[serde(rename = "pluckedPlectrum")]
    PluckedPlectrum,
    /// Snap pizzicato above
    #[serde(rename = "pluckedSnapPizzicatoAbove")]
    PluckedSnapPizzicatoAbove,
    /// Snap pizzicato below
    #[serde(rename = "pluckedSnapPizzicatoBelow")]
    PluckedSnapPizzicatoBelow,
    /// With fingernails
    #[serde(rename = "pluckedWithFingernails")]
    PluckedWithFingernails,
    /// Quindicesima
    #[serde(rename = "quindicesima")]
    Quindicesima,
    /// Quindicesima alta
    #[serde(rename = "quindicesimaAlta")]
    QuindicesimaAlta,
    /// Quindicesima bassa
    #[serde(rename = "quindicesimaBassa")]
    QuindicesimaBassa,
    /// Quindicesima bassa (mb)
    #[serde(rename = "quindicesimaBassaMb")]
    QuindicesimaBassaMb,
    /// Repeat last bar
    #[serde(rename = "repeat1Bar")]
    Repeat1Bar,
    /// Repeat last two bars
    #[serde(rename = "repeat2Bars")]
    Repeat2Bars,
    /// Repeat last four bars
    #[serde(rename = "repeat4Bars")]
    Repeat4Bars,
    /// Repeat bar lower dot
    #[serde(rename = "repeatBarLowerDot")]
    RepeatBarLowerDot,
    /// Repeat bar slash
    #[serde(rename = "repeatBarSlash")]
    RepeatBarSlash,
    /// Repeat bar upper dot
    #[serde(rename = "repeatBarUpperDot")]
    RepeatBarUpperDot,
    /// Single repeat dot
    #[serde(rename = "repeatDot")]
    RepeatDot,
    /// Repeat dots
    #[serde(rename = "repeatDots")]
    RepeatDots,
    /// Left (start) repeat sign
    #[serde(rename = "repeatLeft")]
    RepeatLeft,
    /// Right (end) repeat sign
    #[serde(rename = "repeatRight")]
    RepeatRight,
    /// Right and left repeat sign
    #[serde(rename = "repeatRightLeft")]
    RepeatRightLeft,
    /// 1024th rest
    #[serde(rename = "rest1024th")]
    Rest1024Th,
    /// 128th (semihemidemisemiquaver) rest
    #[serde(rename = "rest128th")]
    Rest128Th,
    /// 16th (semiquaver) rest
    #[serde(rename = "rest16th")]
    Rest16Th,
    /// 256th rest
    #[serde(rename = "rest256th")]
    Rest256Th,
    /// 32nd (demisemiquaver) rest
    #[serde(rename = "rest32nd")]
    Rest32Nd,
    /// 512th rest
    #[serde(rename = "rest512th")]
    Rest512Th,
    /// 64th (hemidemisemiquaver) rest
    #[serde(rename = "rest64th")]
    Rest64Th,
    /// Eighth (quaver) rest
    #[serde(rename = "rest8th")]
    Rest8Th,
    /// Double whole (breve) rest
    #[serde(rename = "restDoubleWhole")]
    RestDoubleWhole,
    /// Double whole rest on leger lines
    #[serde(rename = "restDoubleWholeLegerLine")]
    RestDoubleWholeLegerLine,
    /// Multiple measure rest
    #[serde(rename = "restHBar")]
    RestHBar,
    /// H-bar, left half
    #[serde(rename = "restHBarLeft")]
    RestHBarLeft,
    /// H-bar, middle
    #[serde(rename = "restHBarMiddle")]
    RestHBarMiddle,
    /// H-bar, right half
    #[serde(rename = "restHBarRight")]
    RestHBarRight,
    /// Half (minim) rest
    #[serde(rename = "restHalf")]
    RestHalf,
    /// Half rest on leger line
    #[serde(rename = "restHalfLegerLine")]
    RestHalfLegerLine,
    /// Longa rest
    #[serde(rename = "restLonga")]
    RestLonga,
    /// Maxima rest
    #[serde(rename = "restMaxima")]
    RestMaxima,
    /// Quarter (crotchet) rest
    #[serde(rename = "restQuarter")]
    RestQuarter,
    /// Old-style quarter (crotchet) rest
    #[serde(rename = "restQuarterOld")]
    RestQuarterOld,
    /// Z-style quarter (crotchet) rest
    #[serde(rename = "restQuarterZ")]
    RestQuarterZ,
    /// Whole (semibreve) rest
    #[serde(rename = "restWhole")]
    RestWhole,
    /// Whole rest on leger line
    #[serde(rename = "restWholeLegerLine")]
    RestWholeLegerLine,
    /// Reversed brace
    #[serde(rename = "reversedBrace")]
    ReversedBrace,
    /// Reversed bracket bottom
    #[serde(rename = "reversedBracketBottom")]
    ReversedBracketBottom,
    /// Reversed bracket top
    #[serde(rename = "reversedBracketTop")]
    ReversedBracketTop,
    /// Right repeat sign within bar
    #[serde(rename = "rightRepeatSmall")]
    RightRepeatSmall,
    /// Scale degree 1
    #[serde(rename = "scaleDegree1")]
    ScaleDegree1,
    /// Scale degree 2
    #[serde(rename = "scaleDegree2")]
    ScaleDegree2,
    /// Scale degree 3
    #[serde(rename = "scaleDegree3")]
    ScaleDegree3,
    /// Scale degree 4
    #[serde(rename = "scaleDegree4")]
    ScaleDegree4,
    /// Scale degree 5
    #[serde(rename = "scaleDegree5")]
    ScaleDegree5,
    /// Scale degree 6
    #[serde(rename = "scaleDegree6")]
    ScaleDegree6,
    /// Scale degree 7
    #[serde(rename = "scaleDegree7")]
    ScaleDegree7,
    /// Scale degree 8
    #[serde(rename = "scaleDegree8")]
    ScaleDegree8,
    /// Scale degree 9
    #[serde(rename = "scaleDegree9")]
    ScaleDegree9,
    /// Schäffer clef
    #[serde(rename = "schaefferClef")]
    SchaefferClef,
    /// Schäffer F clef to G clef change
    #[serde(rename = "schaefferFClefToGClef")]
    SchaefferFClefToGClef,
    /// Schäffer G clef to F clef change
    #[serde(rename = "schaefferGClefToFClef")]
    SchaefferGClefToFClef,
    /// Schäffer previous clef
    #[serde(rename = "schaefferPreviousClef")]
    SchaefferPreviousClef,
    /// Segno
    #[serde(rename = "segno")]
    Segno,
    /// Segno (serpent)
    #[serde(rename = "segnoSerpent1")]
    SegnoSerpent1,
    /// Segno (serpent with vertical lines)
    #[serde(rename = "segnoSerpent2")]
    SegnoSerpent2,
    /// Semi-pitched percussion clef 1
    #[serde(rename = "semipitchedPercussionClef1")]
    SemipitchedPercussionClef1,
    /// Semi-pitched percussion clef 2
    #[serde(rename = "semipitchedPercussionClef2")]
    SemipitchedPercussionClef2,
    /// Flat
    #[serde(rename = "smnFlat")]
    SmnFlat,
    /// Flat (white)
    #[serde(rename = "smnFlatWhite")]
    SmnFlatWhite,
    /// Double flat history sign
    #[serde(rename = "smnHistoryDoubleFlat")]
    SmnHistoryDoubleFlat,
    /// Double sharp history sign
    #[serde(rename = "smnHistoryDoubleSharp")]
    SmnHistoryDoubleSharp,
    /// Flat history sign
    #[serde(rename = "smnHistoryFlat")]
    SmnHistoryFlat,
    /// Sharp history sign
    #[serde(rename = "smnHistorySharp")]
    SmnHistorySharp,
    /// Natural (N)
    #[serde(rename = "smnNatural")]
    SmnNatural,
    /// Sharp stem up
    #[serde(rename = "smnSharp")]
    SmnSharp,
    /// Sharp stem down
    #[serde(rename = "smnSharpDown")]
    SmnSharpDown,
    /// Sharp (white) stem up
    #[serde(rename = "smnSharpWhite")]
    SmnSharpWhite,
    /// Sharp (white) stem down
    #[serde(rename = "smnSharpWhiteDown")]
    SmnSharpWhiteDown,
    /// Split bar divider (bar spans a system break)
    #[serde(rename = "splitBarDivider")]
    SplitBarDivider,
    /// 1-line staff
    #[serde(rename = "staff1Line")]
    Staff1Line,
    /// 1-line staff (narrow)
    #[serde(rename = "staff1LineNarrow")]
    Staff1LineNarrow,
    /// 1-line staff (wide)
    #[serde(rename = "staff1LineWide")]
    Staff1LineWide,
    /// 2-line staff
    #[serde(rename = "staff2Lines")]
    Staff2Lines,
    /// 2-line staff (narrow)
    #[serde(rename = "staff2LinesNarrow")]
    Staff2LinesNarrow,
    /// 2-line staff (wide)
    #[serde(rename = "staff2LinesWide")]
    Staff2LinesWide,
    /// 3-line staff
    #[serde(rename = "staff3Lines")]
    Staff3Lines,
    /// 3-line staff (narrow)
    #[serde(rename = "staff3LinesNarrow")]
    Staff3LinesNarrow,
    /// 3-line staff (wide)
    #[serde(rename = "staff3LinesWide")]
    Staff3LinesWide,
    /// 4-line staff
    #[serde(rename = "staff4Lines")]
    Staff4Lines,
    /// 4-line staff (narrow)
    #[serde(rename = "staff4LinesNarrow")]
    Staff4LinesNarrow,
    /// 4-line staff (wide)
    #[serde(rename = "staff4LinesWide")]
    Staff4LinesWide,
    /// 5-line staff
    #[serde(rename = "staff5Lines")]
    Staff5Lines,
    /// 5-line staff (narrow)
    #[serde(rename = "staff5LinesNarrow")]
    Staff5LinesNarrow,
    /// 5-line staff (wide)
    #[serde(rename = "staff5LinesWide")]
    Staff5LinesWide,
    /// 6-line staff
    #[serde(rename = "staff6Lines")]
    Staff6Lines,
    /// 6-line staff (narrow)
    #[serde(rename = "staff6LinesNarrow")]
    Staff6LinesNarrow,
    /// 6-line staff (wide)
    #[serde(rename = "staff6LinesWide")]
    Staff6LinesWide,
    /// Staff divide arrow down
    #[serde(rename = "staffDivideArrowDown")]
    StaffDivideArrowDown,
    /// Staff divide arrow up
    #[serde(rename = "staffDivideArrowUp")]
    StaffDivideArrowUp,
    /// Staff divide arrows
    #[serde(rename = "staffDivideArrowUpDown")]
    StaffDivideArrowUpDown,
    /// Lower 1 staff position
    #[serde(rename = "staffPosLower1")]
    StaffPosLower1,
    /// Lower 2 staff positions
    #[serde(rename = "staffPosLower2")]
    StaffPosLower2,
    /// Lower 3 staff positions
    #[serde(rename = "staffPosLower3")]
    StaffPosLower3,
    /// Lower 4 staff positions
    #[serde(rename = "staffPosLower4")]
    StaffPosLower4,
    /// Lower 5 staff positions
    #[serde(rename = "staffPosLower5")]
    StaffPosLower5,
    /// Lower 6 staff positions
    #[serde(rename = "staffPosLower6")]
    StaffPosLower6,
    /// Lower 7 staff positions
    #[serde(rename = "staffPosLower7")]
    StaffPosLower7,
    /// Lower 8 staff positions
    #[serde(rename = "staffPosLower8")]
    StaffPosLower8,
    /// Raise 1 staff position
    #[serde(rename = "staffPosRaise1")]
    StaffPosRaise1,
    /// Raise 2 staff positions
    #[serde(rename = "staffPosRaise2")]
    StaffPosRaise2,
    /// Raise 3 staff positions
    #[serde(rename = "staffPosRaise3")]
    StaffPosRaise3,
    /// Raise 4 staff positions
    #[serde(rename = "staffPosRaise4")]
    StaffPosRaise4,
    /// Raise 5 staff positions
    #[serde(rename = "staffPosRaise5")]
    StaffPosRaise5,
    /// Raise 6 staff positions
    #[serde(rename = "staffPosRaise6")]
    StaffPosRaise6,
    /// Raise 7 staff positions
    #[serde(rename = "staffPosRaise7")]
    StaffPosRaise7,
    /// Raise 8 staff positions
    #[serde(rename = "staffPosRaise8")]
    StaffPosRaise8,
    /// Combining stem
    #[serde(rename = "stem")]
    Stem,
    /// Combining bow on bridge stem
    #[serde(rename = "stemBowOnBridge")]
    StemBowOnBridge,
    /// Combining bow on tailpiece stem
    #[serde(rename = "stemBowOnTailpiece")]
    StemBowOnTailpiece,
    /// Combining buzz roll stem
    #[serde(rename = "stemBuzzRoll")]
    StemBuzzRoll,
    /// Combining damp stem
    #[serde(rename = "stemDamp")]
    StemDamp,
    /// Combining harp string noise stem
    #[serde(rename = "stemHarpStringNoise")]
    StemHarpStringNoise,
    /// Combining multiphonics (black) stem
    #[serde(rename = "stemMultiphonicsBlack")]
    StemMultiphonicsBlack,
    /// Combining multiphonics (black and white) stem
    #[serde(rename = "stemMultiphonicsBlackWhite")]
    StemMultiphonicsBlackWhite,
    /// Combining multiphonics (white) stem
    #[serde(rename = "stemMultiphonicsWhite")]
    StemMultiphonicsWhite,
    /// Combining Penderecki unmeasured tremolo stem
    #[serde(rename = "stemPendereckiTremolo")]
    StemPendereckiTremolo,
    /// Combining rim shot stem
    #[serde(rename = "stemRimShot")]
    StemRimShot,
    /// Combining sprechgesang stem
    #[serde(rename = "stemSprechgesang")]
    StemSprechgesang,
    /// Combining sul ponticello (bow behind bridge) stem
    #[serde(rename = "stemSulPonticello")]
    StemSulPonticello,
    /// Combining sussurando stem
    #[serde(rename = "stemSussurando")]
    StemSussurando,
    /// Combining swished stem
    #[serde(rename = "stemSwished")]
    StemSwished,
    /// Combining vibrato pulse accent (Saunders) stem
    #[serde(rename = "stemVibratoPulse")]
    StemVibratoPulse,
    /// Stockhausen irregular tremolo ("Morsen", like Morse code)
    #[serde(rename = "stockhausenTremolo")]
    StockhausenTremolo,
    /// Bow behind bridge (sul ponticello)
    #[serde(rename = "stringsBowBehindBridge")]
    StringsBowBehindBridge,
    /// Bow behind bridge on four strings
    #[serde(rename = "stringsBowBehindBridgeFourStrings")]
    StringsBowBehindBridgeFourStrings,
    /// Bow behind bridge on one string
    #[serde(rename = "stringsBowBehindBridgeOneString")]
    StringsBowBehindBridgeOneString,
    /// Bow behind bridge on three strings
    #[serde(rename = "stringsBowBehindBridgeThreeStrings")]
    StringsBowBehindBridgeThreeStrings,
    /// Bow behind bridge on two strings
    #[serde(rename = "stringsBowBehindBridgeTwoStrings")]
    StringsBowBehindBridgeTwoStrings,
    /// Bow on top of bridge
    #[serde(rename = "stringsBowOnBridge")]
    StringsBowOnBridge,
    /// Bow on tailpiece
    #[serde(rename = "stringsBowOnTailpiece")]
    StringsBowOnTailpiece,
    /// Change bow direction, indeterminate
    #[serde(rename = "stringsChangeBowDirection")]
    StringsChangeBowDirection,
    /// Down bow
    #[serde(rename = "stringsDownBow")]
    StringsDownBow,
    /// Down bow, away from body
    #[serde(rename = "stringsDownBowAwayFromBody")]
    StringsDownBowAwayFromBody,
    /// Down bow, beyond bridge
    #[serde(rename = "stringsDownBowBeyondBridge")]
    StringsDownBowBeyondBridge,
    /// Down bow, towards body
    #[serde(rename = "stringsDownBowTowardsBody")]
    StringsDownBowTowardsBody,
    /// Turned down bow
    #[serde(rename = "stringsDownBowTurned")]
    StringsDownBowTurned,
    /// Fouetté
    #[serde(rename = "stringsFouette")]
    StringsFouette,
    /// Half-harmonic
    #[serde(rename = "stringsHalfHarmonic")]
    StringsHalfHarmonic,
    /// Harmonic
    #[serde(rename = "stringsHarmonic")]
    StringsHarmonic,
    /// Jeté (gettato) above
    #[serde(rename = "stringsJeteAbove")]
    StringsJeteAbove,
    /// Jeté (gettato) below
    #[serde(rename = "stringsJeteBelow")]
    StringsJeteBelow,
    /// Mute off
    #[serde(rename = "stringsMuteOff")]
    StringsMuteOff,
    /// Mute on
    #[serde(rename = "stringsMuteOn")]
    StringsMuteOn,
    /// Overpressure, down bow
    #[serde(rename = "stringsOverpressureDownBow")]
    StringsOverpressureDownBow,
    /// Overpressure, no bow direction
    #[serde(rename = "stringsOverpressureNoDirection")]
    StringsOverpressureNoDirection,
    /// Overpressure possibile, down bow
    #[serde(rename = "stringsOverpressurePossibileDownBow")]
    StringsOverpressurePossibileDownBow,
    /// Overpressure possibile, up bow
    #[serde(rename = "stringsOverpressurePossibileUpBow")]
    StringsOverpressurePossibileUpBow,
    /// Overpressure, up bow
    #[serde(rename = "stringsOverpressureUpBow")]
    StringsOverpressureUpBow,
    /// Scrape, circular clockwise
    #[serde(rename = "stringsScrapeCircularClockwise")]
    StringsScrapeCircularClockwise,
    /// Scrape, circular counter-clockwise
    #[serde(rename = "stringsScrapeCircularCounterclockwise")]
    StringsScrapeCircularCounterclockwise,
    /// Scrape, parallel inward
    #[serde(rename = "stringsScrapeParallelInward")]
    StringsScrapeParallelInward,
    /// Scrape, parallel outward
    #[serde(rename = "stringsScrapeParallelOutward")]
    StringsScrapeParallelOutward,
    /// Thumb position
    #[serde(rename = "stringsThumbPosition")]
    StringsThumbPosition,
    /// Turned thumb position
    #[serde(rename = "stringsThumbPositionTurned")]
    StringsThumbPositionTurned,
    /// Triple chop, inward
    #[serde(rename = "stringsTripleChopInward")]
    StringsTripleChopInward,
    /// Triple chop, outward
    #[serde(rename = "stringsTripleChopOutward")]
    StringsTripleChopOutward,
    /// Up bow
    #[serde(rename = "stringsUpBow")]
    StringsUpBow,
    /// Up bow, away from body
    #[serde(rename = "stringsUpBowAwayFromBody")]
    StringsUpBowAwayFromBody,
    /// Up bow, beyond bridge
    #[serde(rename = "stringsUpBowBeyondBridge")]
    StringsUpBowBeyondBridge,
    /// Up bow, towards body
    #[serde(rename = "stringsUpBowTowardsBody")]
    StringsUpBowTowardsBody,
    /// Turned up bow
    #[serde(rename = "stringsUpBowTurned")]
    StringsUpBowTurned,
    /// Vibrato pulse accent (Saunders) for stem
    #[serde(rename = "stringsVibratoPulse")]
    StringsVibratoPulse,
    /// Swiss rudiments doublé black notehead
    #[serde(rename = "swissRudimentsNoteheadBlackDouble")]
    SwissRudimentsNoteheadBlackDouble,
    /// Swiss rudiments flam black notehead
    #[serde(rename = "swissRudimentsNoteheadBlackFlam")]
    SwissRudimentsNoteheadBlackFlam,
    /// Swiss rudiments doublé half (minim) notehead
    #[serde(rename = "swissRudimentsNoteheadHalfDouble")]
    SwissRudimentsNoteheadHalfDouble,
    /// Swiss rudiments flam half (minim) notehead
    #[serde(rename = "swissRudimentsNoteheadHalfFlam")]
    SwissRudimentsNoteheadHalfFlam,
    /// System divider
    #[serde(rename = "systemDivider")]
    SystemDivider,
    /// Extra long system divider
    #[serde(rename = "systemDividerExtraLong")]
    SystemDividerExtraLong,
    /// Long system divider
    #[serde(rename = "systemDividerLong")]
    SystemDividerLong,
    /// Augmentation dot
    #[serde(rename = "textAugmentationDot")]
    TextAugmentationDot,
    /// Black note, fractional 16th beam, long stem
    #[serde(rename = "textBlackNoteFrac16thLongStem")]
    TextBlackNoteFrac16ThLongStem,
    /// Black note, fractional 16th beam, short stem
    #[serde(rename = "textBlackNoteFrac16thShortStem")]
    TextBlackNoteFrac16ThShortStem,
    /// Black note, fractional 32nd beam, long stem
    #[serde(rename = "textBlackNoteFrac32ndLongStem")]
    TextBlackNoteFrac32NdLongStem,
    /// Black note, fractional 8th beam, long stem
    #[serde(rename = "textBlackNoteFrac8thLongStem")]
    TextBlackNoteFrac8ThLongStem,
    /// Black note, fractional 8th beam, short stem
    #[serde(rename = "textBlackNoteFrac8thShortStem")]
    TextBlackNoteFrac8ThShortStem,
    /// Black note, long stem
    #[serde(rename = "textBlackNoteLongStem")]
    TextBlackNoteLongStem,
    /// Black note, short stem
    #[serde(rename = "textBlackNoteShortStem")]
    TextBlackNoteShortStem,
    /// Continuing 16th beam for long stem
    #[serde(rename = "textCont16thBeamLongStem")]
    TextCont16ThBeamLongStem,
    /// Continuing 16th beam for short stem
    #[serde(rename = "textCont16thBeamShortStem")]
    TextCont16ThBeamShortStem,
    /// Continuing 32nd beam for long stem
    #[serde(rename = "textCont32ndBeamLongStem")]
    TextCont32NdBeamLongStem,
    /// Continuing 8th beam for long stem
    #[serde(rename = "textCont8thBeamLongStem")]
    TextCont8ThBeamLongStem,
    /// Continuing 8th beam for short stem
    #[serde(rename = "textCont8thBeamShortStem")]
    TextCont8ThBeamShortStem,
    /// Headless black note, fractional 16th beam, long stem
    #[serde(rename = "textHeadlessBlackNoteFrac16thLongStem")]
    TextHeadlessBlackNoteFrac16ThLongStem,
    /// Headless black note, fractional 16th beam, short stem
    #[serde(rename = "textHeadlessBlackNoteFrac16thShortStem")]
    TextHeadlessBlackNoteFrac16ThShortStem,
    /// Headless black note, fractional 32nd beam, long stem
    #[serde(rename = "textHeadlessBlackNoteFrac32ndLongStem")]
    TextHeadlessBlackNoteFrac32NdLongStem,
    /// Headless black note, fractional 8th beam, long stem
    #[serde(rename = "textHeadlessBlackNoteFrac8thLongStem")]
    TextHeadlessBlackNoteFrac8ThLongStem,
    /// Headless black note, fractional 8th beam, short stem
    #[serde(rename = "textHeadlessBlackNoteFrac8thShortStem")]
    TextHeadlessBlackNoteFrac8ThShortStem,
    /// Headless black note, long stem
    #[serde(rename = "textHeadlessBlackNoteLongStem")]
    TextHeadlessBlackNoteLongStem,
    /// Headless black note, short stem
    #[serde(rename = "textHeadlessBlackNoteShortStem")]
    TextHeadlessBlackNoteShortStem,
    /// Tie
    #[serde(rename = "textTie")]
    TextTie,
    /// Tuplet number 3 for long stem
    #[serde(rename = "textTuplet3LongStem")]
    TextTuplet3LongStem,
    /// Tuplet number 3 for short stem
    #[serde(rename = "textTuplet3ShortStem")]
    TextTuplet3ShortStem,
    /// Tuplet bracket end for long stem
    #[serde(rename = "textTupletBracketEndLongStem")]
    TextTupletBracketEndLongStem,
    /// Tuplet bracket end for short stem
    #[serde(rename = "textTupletBracketEndShortStem")]
    TextTupletBracketEndShortStem,
    /// Tuplet bracket start for long stem
    #[serde(rename = "textTupletBracketStartLongStem")]
    TextTupletBracketStartLongStem,
    /// Tuplet bracket start for short stem
    #[serde(rename = "textTupletBracketStartShortStem")]
    TextTupletBracketStartShortStem,
    /// Time signature 0
    #[serde(rename = "timeSig0")]
    TimeSig0,
    /// Reversed time signature 0
    #[serde(rename = "timeSig0Reversed")]
    TimeSig0Reversed,
    /// Turned time signature 0
    #[serde(rename = "timeSig0Turned")]
    TimeSig0Turned,
    /// Time signature 1
    #[serde(rename = "timeSig1")]
    TimeSig1,
    /// Reversed time signature 1
    #[serde(rename = "timeSig1Reversed")]
    TimeSig1Reversed,
    /// Turned time signature 1
    #[serde(rename = "timeSig1Turned")]
    TimeSig1Turned,
    /// Time signature 2
    #[serde(rename = "timeSig2")]
    TimeSig2,
    /// Reversed time signature 2
    #[serde(rename = "timeSig2Reversed")]
    TimeSig2Reversed,
    /// Turned time signature 2
    #[serde(rename = "timeSig2Turned")]
    TimeSig2Turned,
    /// Time signature 3
    #[serde(rename = "timeSig3")]
    TimeSig3,
    /// Reversed time signature 3
    #[serde(rename = "timeSig3Reversed")]
    TimeSig3Reversed,
    /// Turned time signature 3
    #[serde(rename = "timeSig3Turned")]
    TimeSig3Turned,
    /// Time signature 4
    #[serde(rename = "timeSig4")]
    TimeSig4,
    /// Reversed time signature 4
    #[serde(rename = "timeSig4Reversed")]
    TimeSig4Reversed,
    /// Turned time signature 4
    #[serde(rename = "timeSig4Turned")]
    TimeSig4Turned,
    /// Time signature 5
    #[serde(rename = "timeSig5")]
    TimeSig5,
    /// Reversed time signature 5
    #[serde(rename = "timeSig5Reversed")]
    TimeSig5Reversed,
    /// Turned time signature 5
    #[serde(rename = "timeSig5Turned")]
    TimeSig5Turned,
    /// Time signature 6
    #[serde(rename = "timeSig6")]
    TimeSig6,
    /// Reversed time signature 6
    #[serde(rename = "timeSig6Reversed")]
    TimeSig6Reversed,
    /// Turned time signature 6
    #[serde(rename = "timeSig6Turned")]
    TimeSig6Turned,
    /// Time signature 7
    #[serde(rename = "timeSig7")]
    TimeSig7,
    /// Reversed time signature 7
    #[serde(rename = "timeSig7Reversed")]
    TimeSig7Reversed,
    /// Turned time signature 7
    #[serde(rename = "timeSig7Turned")]
    TimeSig7Turned,
    /// Time signature 8
    #[serde(rename = "timeSig8")]
    TimeSig8,
    /// Reversed time signature 8
    #[serde(rename = "timeSig8Reversed")]
    TimeSig8Reversed,
    /// Turned time signature 8
    #[serde(rename = "timeSig8Turned")]
    TimeSig8Turned,
    /// Time signature 9
    #[serde(rename = "timeSig9")]
    TimeSig9,
    /// Reversed time signature 9
    #[serde(rename = "timeSig9Reversed")]
    TimeSig9Reversed,
    /// Turned time signature 9
    #[serde(rename = "timeSig9Turned")]
    TimeSig9Turned,
    /// Left bracket for whole time signature
    #[serde(rename = "timeSigBracketLeft")]
    TimeSigBracketLeft,
    /// Left bracket for numerator only
    #[serde(rename = "timeSigBracketLeftSmall")]
    TimeSigBracketLeftSmall,
    /// Right bracket for whole time signature
    #[serde(rename = "timeSigBracketRight")]
    TimeSigBracketRight,
    /// Right bracket for numerator only
    #[serde(rename = "timeSigBracketRightSmall")]
    TimeSigBracketRightSmall,
    /// Control character for denominator digit
    #[serde(rename = "timeSigCombDenominator")]
    TimeSigCombDenominator,
    /// Control character for numerator digit
    #[serde(rename = "timeSigCombNumerator")]
    TimeSigCombNumerator,
    /// Time signature comma
    #[serde(rename = "timeSigComma")]
    TimeSigComma,
    /// Common time
    #[serde(rename = "timeSigCommon")]
    TimeSigCommon,
    /// Reversed common time
    #[serde(rename = "timeSigCommonReversed")]
    TimeSigCommonReversed,
    /// Turned common time
    #[serde(rename = "timeSigCommonTurned")]
    TimeSigCommonTurned,
    /// Cut time (Bach)
    #[serde(rename = "timeSigCut2")]
    TimeSigCut2,
    /// Cut triple time (9/8)
    #[serde(rename = "timeSigCut3")]
    TimeSigCut3,
    /// Cut time
    #[serde(rename = "timeSigCutCommon")]
    TimeSigCutCommon,
    /// Reversed cut time
    #[serde(rename = "timeSigCutCommonReversed")]
    TimeSigCutCommonReversed,
    /// Turned cut time
    #[serde(rename = "timeSigCutCommonTurned")]
    TimeSigCutCommonTurned,
    /// Time signature equals
    #[serde(rename = "timeSigEquals")]
    TimeSigEquals,
    /// Time signature fraction ½
    #[serde(rename = "timeSigFractionHalf")]
    TimeSigFractionHalf,
    /// Time signature fraction ⅓
    #[serde(rename = "timeSigFractionOneThird")]
    TimeSigFractionOneThird,
    /// Time signature fraction ¼
    #[serde(rename = "timeSigFractionQuarter")]
    TimeSigFractionQuarter,
    /// Time signature fraction ¾
    #[serde(rename = "timeSigFractionThreeQuarters")]
    TimeSigFractionThreeQuarters,
    /// Time signature fraction ⅔
    #[serde(rename = "timeSigFractionTwoThirds")]
    TimeSigFractionTwoThirds,
    /// Time signature fraction slash
    #[serde(rename = "timeSigFractionalSlash")]
    TimeSigFractionalSlash,
    /// Time signature minus
    #[serde(rename = "timeSigMinus")]
    TimeSigMinus,
    /// Time signature multiply
    #[serde(rename = "timeSigMultiply")]
    TimeSigMultiply,
    /// Open time signature (Penderecki)
    #[serde(rename = "timeSigOpenPenderecki")]
    TimeSigOpenPenderecki,
    /// Left parenthesis for whole time signature
    #[serde(rename = "timeSigParensLeft")]
    TimeSigParensLeft,
    /// Left parenthesis for numerator only
    #[serde(rename = "timeSigParensLeftSmall")]
    TimeSigParensLeftSmall,
    /// Right parenthesis for whole time signature
    #[serde(rename = "timeSigParensRight")]
    TimeSigParensRight,
    /// Right parenthesis for numerator only
    #[serde(rename = "timeSigParensRightSmall")]
    TimeSigParensRightSmall,
    /// Time signature +
    #[serde(rename = "timeSigPlus")]
    TimeSigPlus,
    /// Time signature + (for numerators)
    #[serde(rename = "timeSigPlusSmall")]
    TimeSigPlusSmall,
    /// Time signature slash separator
    #[serde(rename = "timeSigSlash")]
    TimeSigSlash,
    /// Open time signature
    #[serde(rename = "timeSigX")]
    TimeSigX,
    /// Combining tremolo 1
    #[serde(rename = "tremolo1")]
    Tremolo1,
    /// Combining tremolo 2
    #[serde(rename = "tremolo2")]
    Tremolo2,
    /// Combining tremolo 3
    #[serde(rename = "tremolo3")]
    Tremolo3,
    /// Combining tremolo 4
    #[serde(rename = "tremolo4")]
    Tremolo4,
    /// Combining tremolo 5
    #[serde(rename = "tremolo5")]
    Tremolo5,
    /// Divide measured tremolo by 2
    #[serde(rename = "tremoloDivisiDots2")]
    TremoloDivisiDots2,
    /// Divide measured tremolo by 3
    #[serde(rename = "tremoloDivisiDots3")]
    TremoloDivisiDots3,
    /// Divide measured tremolo by 4
    #[serde(rename = "tremoloDivisiDots4")]
    TremoloDivisiDots4,
    /// Divide measured tremolo by 6
    #[serde(rename = "tremoloDivisiDots6")]
    TremoloDivisiDots6,
    /// Fingered tremolo 1
    #[serde(rename = "tremoloFingered1")]
    TremoloFingered1,
    /// Fingered tremolo 2
    #[serde(rename = "tremoloFingered2")]
    TremoloFingered2,
    /// Fingered tremolo 3
    #[serde(rename = "tremoloFingered3")]
    TremoloFingered3,
    /// Fingered tremolo 4
    #[serde(rename = "tremoloFingered4")]
    TremoloFingered4,
    /// Fingered tremolo 5
    #[serde(rename = "tremoloFingered5")]
    TremoloFingered5,
    /// Triple-tongue above
    #[serde(rename = "tripleTongueAbove")]
    TripleTongueAbove,
    /// Triple-tongue below
    #[serde(rename = "tripleTongueBelow")]
    TripleTongueBelow,
    /// Tuplet 0
    #[serde(rename = "tuplet0")]
    Tuplet0,
    /// Tuplet 1
    #[serde(rename = "tuplet1")]
    Tuplet1,
    /// Tuplet 2
    #[serde(rename = "tuplet2")]
    Tuplet2,
    /// Tuplet 3
    #[serde(rename = "tuplet3")]
    Tuplet3,
    /// Tuplet 4
    #[serde(rename = "tuplet4")]
    Tuplet4,
    /// Tuplet 5
    #[serde(rename = "tuplet5")]
    Tuplet5,
    /// Tuplet 6
    #[serde(rename = "tuplet6")]
    Tuplet6,
    /// Tuplet 7
    #[serde(rename = "tuplet7")]
    Tuplet7,
    /// Tuplet 8
    #[serde(rename = "tuplet8")]
    Tuplet8,
    /// Tuplet 9
    #[serde(rename = "tuplet9")]
    Tuplet9,
    /// Tuplet colon
    #[serde(rename = "tupletColon")]
    TupletColon,
    /// Wieniawski unmeasured tremolo
    #[serde(rename = "unmeasuredTremolo")]
    UnmeasuredTremolo,
    /// Wieniawski unmeasured tremolo (simpler)
    #[serde(rename = "unmeasuredTremoloSimple")]
    UnmeasuredTremoloSimple,
    /// Unpitched percussion clef 1
    #[serde(rename = "unpitchedPercussionClef1")]
    UnpitchedPercussionClef1,
    /// Unpitched percussion clef 2
    #[serde(rename = "unpitchedPercussionClef2")]
    UnpitchedPercussionClef2,
    /// Ventiduesima
    #[serde(rename = "ventiduesima")]
    Ventiduesima,
    /// Ventiduesima alta
    #[serde(rename = "ventiduesimaAlta")]
    VentiduesimaAlta,
    /// Ventiduesima bassa
    #[serde(rename = "ventiduesimaBassa")]
    VentiduesimaBassa,
    /// Ventiduesima bassa (mb)
    #[serde(rename = "ventiduesimaBassaMb")]
    VentiduesimaBassaMb,
    /// Finger click (Stockhausen)
    #[serde(rename = "vocalFingerClickStockhausen")]
    VocalFingerClickStockhausen,
    /// Halb gesungen (semi-sprechgesang)
    #[serde(rename = "vocalHalbGesungen")]
    VocalHalbGesungen,
    /// Mouth closed
    #[serde(rename = "vocalMouthClosed")]
    VocalMouthClosed,
    /// Mouth open
    #[serde(rename = "vocalMouthOpen")]
    VocalMouthOpen,
    /// Mouth pursed
    #[serde(rename = "vocalMouthPursed")]
    VocalMouthPursed,
    /// Mouth slightly open
    #[serde(rename = "vocalMouthSlightlyOpen")]
    VocalMouthSlightlyOpen,
    /// Mouth wide open
    #[serde(rename = "vocalMouthWideOpen")]
    VocalMouthWideOpen,
    /// Nasal voice
    #[serde(rename = "vocalNasalVoice")]
    VocalNasalVoice,
    /// Sprechgesang
    #[serde(rename = "vocalSprechgesang")]
    VocalSprechgesang,
    /// Tongue click (Stockhausen)
    #[serde(rename = "vocalTongueClickStockhausen")]
    VocalTongueClickStockhausen,
    /// Tongue and finger click (Stockhausen)
    #[serde(rename = "vocalTongueFingerClickStockhausen")]
    VocalTongueFingerClickStockhausen,
    /// Combining sussurando for stem
    #[serde(rename = "vocalsSussurando")]
    VocalsSussurando,
    /// Arpeggiato wiggle segment, downwards
    #[serde(rename = "wiggleArpeggiatoDown")]
    WiggleArpeggiatoDown,
    /// Arpeggiato arrowhead down
    #[serde(rename = "wiggleArpeggiatoDownArrow")]
    WiggleArpeggiatoDownArrow,
    /// Arpeggiato downward swash
    #[serde(rename = "wiggleArpeggiatoDownSwash")]
    WiggleArpeggiatoDownSwash,
    /// Arpeggiato wiggle segment, upwards
    #[serde(rename = "wiggleArpeggiatoUp")]
    WiggleArpeggiatoUp,
    /// Arpeggiato arrowhead up
    #[serde(rename = "wiggleArpeggiatoUpArrow")]
    WiggleArpeggiatoUpArrow,
    /// Arpeggiato upward swash
    #[serde(rename = "wiggleArpeggiatoUpSwash")]
    WiggleArpeggiatoUpSwash,
    /// Circular motion segment
    #[serde(rename = "wiggleCircular")]
    WiggleCircular,
    /// Constant circular motion segment
    #[serde(rename = "wiggleCircularConstant")]
    WiggleCircularConstant,
    /// Constant circular motion segment (flipped)
    #[serde(rename = "wiggleCircularConstantFlipped")]
    WiggleCircularConstantFlipped,
    /// Constant circular motion segment (flipped, large)
    #[serde(rename = "wiggleCircularConstantFlippedLarge")]
    WiggleCircularConstantFlippedLarge,
    /// Constant circular motion segment (large)
    #[serde(rename = "wiggleCircularConstantLarge")]
    WiggleCircularConstantLarge,
    /// Circular motion end
    #[serde(rename = "wiggleCircularEnd")]
    WiggleCircularEnd,
    /// Circular motion segment, large
    #[serde(rename = "wiggleCircularLarge")]
    WiggleCircularLarge,
    /// Circular motion segment, larger
    #[serde(rename = "wiggleCircularLarger")]
    WiggleCircularLarger,
    /// Circular motion segment, larger still
    #[serde(rename = "wiggleCircularLargerStill")]
    WiggleCircularLargerStill,
    /// Circular motion segment, largest
    #[serde(rename = "wiggleCircularLargest")]
    WiggleCircularLargest,
    /// Circular motion segment, small
    #[serde(rename = "wiggleCircularSmall")]
    WiggleCircularSmall,
    /// Circular motion start
    #[serde(rename = "wiggleCircularStart")]
    WiggleCircularStart,
    /// Glissando wiggle segment
    #[serde(rename = "wiggleGlissando")]
    WiggleGlissando,
    /// Group glissando 1
    #[serde(rename = "wiggleGlissandoGroup1")]
    WiggleGlissandoGroup1,
    /// Group glissando 2
    #[serde(rename = "wiggleGlissandoGroup2")]
    WiggleGlissandoGroup2,
    /// Group glissando 3
    #[serde(rename = "wiggleGlissandoGroup3")]
    WiggleGlissandoGroup3,
    /// Quasi-random squiggle 1
    #[serde(rename = "wiggleRandom1")]
    WiggleRandom1,
    /// Quasi-random squiggle 2
    #[serde(rename = "wiggleRandom2")]
    WiggleRandom2,
    /// Quasi-random squiggle 3
    #[serde(rename = "wiggleRandom3")]
    WiggleRandom3,
    /// Quasi-random squiggle 4
    #[serde(rename = "wiggleRandom4")]
    WiggleRandom4,
    /// Sawtooth line segment
    #[serde(rename = "wiggleSawtooth")]
    WiggleSawtooth,
    /// Narrow sawtooth line segment
    #[serde(rename = "wiggleSawtoothNarrow")]
    WiggleSawtoothNarrow,
    /// Wide sawtooth line segment
    #[serde(rename = "wiggleSawtoothWide")]
    WiggleSawtoothWide,
    /// Square wave line segment
    #[serde(rename = "wiggleSquareWave")]
    WiggleSquareWave,
    /// Narrow square wave line segment
    #[serde(rename = "wiggleSquareWaveNarrow")]
    WiggleSquareWaveNarrow,
    /// Wide square wave line segment
    #[serde(rename = "wiggleSquareWaveWide")]
    WiggleSquareWaveWide,
    /// Trill wiggle segment
    #[serde(rename = "wiggleTrill")]
    WiggleTrill,
    /// Trill wiggle segment, fast
    #[serde(rename = "wiggleTrillFast")]
    WiggleTrillFast,
    /// Trill wiggle segment, faster
    #[serde(rename = "wiggleTrillFaster")]
    WiggleTrillFaster,
    /// Trill wiggle segment, faster still
    #[serde(rename = "wiggleTrillFasterStill")]
    WiggleTrillFasterStill,
    /// Trill wiggle segment, fastest
    #[serde(rename = "wiggleTrillFastest")]
    WiggleTrillFastest,
    /// Trill wiggle segment, slow
    #[serde(rename = "wiggleTrillSlow")]
    WiggleTrillSlow,
    /// Trill wiggle segment, slower
    #[serde(rename = "wiggleTrillSlower")]
    WiggleTrillSlower,
    /// Trill wiggle segment, slower still
    #[serde(rename = "wiggleTrillSlowerStill")]
    WiggleTrillSlowerStill,
    /// Trill wiggle segment, slowest
    #[serde(rename = "wiggleTrillSlowest")]
    WiggleTrillSlowest,
    /// Vibrato largest, slower
    #[serde(rename = "wiggleVIbratoLargestSlower")]
    WiggleVIbratoLargestSlower,
    /// Vibrato medium, slower
    #[serde(rename = "wiggleVIbratoMediumSlower")]
    WiggleVIbratoMediumSlower,
    /// Vibrato / shake wiggle segment
    #[serde(rename = "wiggleVibrato")]
    WiggleVibrato,
    /// Vibrato large, fast
    #[serde(rename = "wiggleVibratoLargeFast")]
    WiggleVibratoLargeFast,
    /// Vibrato large, faster
    #[serde(rename = "wiggleVibratoLargeFaster")]
    WiggleVibratoLargeFaster,
    /// Vibrato large, faster still
    #[serde(rename = "wiggleVibratoLargeFasterStill")]
    WiggleVibratoLargeFasterStill,
    /// Vibrato large, fastest
    #[serde(rename = "wiggleVibratoLargeFastest")]
    WiggleVibratoLargeFastest,
    /// Vibrato large, slow
    #[serde(rename = "wiggleVibratoLargeSlow")]
    WiggleVibratoLargeSlow,
    /// Vibrato large, slower
    #[serde(rename = "wiggleVibratoLargeSlower")]
    WiggleVibratoLargeSlower,
    /// Vibrato large, slowest
    #[serde(rename = "wiggleVibratoLargeSlowest")]
    WiggleVibratoLargeSlowest,
    /// Vibrato largest, fast
    #[serde(rename = "wiggleVibratoLargestFast")]
    WiggleVibratoLargestFast,
    /// Vibrato largest, faster
    #[serde(rename = "wiggleVibratoLargestFaster")]
    WiggleVibratoLargestFaster,
    /// Vibrato largest, faster still
    #[serde(rename = "wiggleVibratoLargestFasterStill")]
    WiggleVibratoLargestFasterStill,
    /// Vibrato largest, fastest
    #[serde(rename = "wiggleVibratoLargestFastest")]
    WiggleVibratoLargestFastest,
    /// Vibrato largest, slow
    #[serde(rename = "wiggleVibratoLargestSlow")]
    WiggleVibratoLargestSlow,
    /// Vibrato largest, slowest
    #[serde(rename = "wiggleVibratoLargestSlowest")]
    WiggleVibratoLargestSlowest,
    /// Vibrato medium, fast
    #[serde(rename = "wiggleVibratoMediumFast")]
    WiggleVibratoMediumFast,
    /// Vibrato medium, faster
    #[serde(rename = "wiggleVibratoMediumFaster")]
    WiggleVibratoMediumFaster,
    /// Vibrato medium, faster still
    #[serde(rename = "wiggleVibratoMediumFasterStill")]
    WiggleVibratoMediumFasterStill,
    /// Vibrato medium, fastest
    #[serde(rename = "wiggleVibratoMediumFastest")]
    WiggleVibratoMediumFastest,
    /// Vibrato medium, slow
    #[serde(rename = "wiggleVibratoMediumSlow")]
    WiggleVibratoMediumSlow,
    /// Vibrato medium, slowest
    #[serde(rename = "wiggleVibratoMediumSlowest")]
    WiggleVibratoMediumSlowest,
    /// Vibrato small, fast
    #[serde(rename = "wiggleVibratoSmallFast")]
    WiggleVibratoSmallFast,
    /// Vibrato small, faster
    #[serde(rename = "wiggleVibratoSmallFaster")]
    WiggleVibratoSmallFaster,
    /// Vibrato small, faster still
    #[serde(rename = "wiggleVibratoSmallFasterStill")]
    WiggleVibratoSmallFasterStill,
    /// Vibrato small, fastest
    #[serde(rename = "wiggleVibratoSmallFastest")]
    WiggleVibratoSmallFastest,
    /// Vibrato small, slow
    #[serde(rename = "wiggleVibratoSmallSlow")]
    WiggleVibratoSmallSlow,
    /// Vibrato small, slower
    #[serde(rename = "wiggleVibratoSmallSlower")]
    WiggleVibratoSmallSlower,
    /// Vibrato small, slowest
    #[serde(rename = "wiggleVibratoSmallSlowest")]
    WiggleVibratoSmallSlowest,
    /// Vibrato smallest, fast
    #[serde(rename = "wiggleVibratoSmallestFast")]
    WiggleVibratoSmallestFast,
    /// Vibrato smallest, faster
    #[serde(rename = "wiggleVibratoSmallestFaster")]
    WiggleVibratoSmallestFaster,
    /// Vibrato smallest, faster still
    #[serde(rename = "wiggleVibratoSmallestFasterStill")]
    WiggleVibratoSmallestFasterStill,
    /// Vibrato smallest, fastest
    #[serde(rename = "wiggleVibratoSmallestFastest")]
    WiggleVibratoSmallestFastest,
    /// Vibrato smallest, slow
    #[serde(rename = "wiggleVibratoSmallestSlow")]
    WiggleVibratoSmallestSlow,
    /// Vibrato smallest, slower
    #[serde(rename = "wiggleVibratoSmallestSlower")]
    WiggleVibratoSmallestSlower,
    /// Vibrato smallest, slowest
    #[serde(rename = "wiggleVibratoSmallestSlowest")]
    WiggleVibratoSmallestSlowest,
    /// Vibrato start
    #[serde(rename = "wiggleVibratoStart")]
    WiggleVibratoStart,
    /// Wide vibrato / shake wiggle segment
    #[serde(rename = "wiggleVibratoWide")]
    WiggleVibratoWide,
    /// Wavy line segment
    #[serde(rename = "wiggleWavy")]
    WiggleWavy,
    /// Narrow wavy line segment
    #[serde(rename = "wiggleWavyNarrow")]
    WiggleWavyNarrow,
    /// Wide wavy line segment
    #[serde(rename = "wiggleWavyWide")]
    WiggleWavyWide,
    /// Closed hole
    #[serde(rename = "windClosedHole")]
    WindClosedHole,
    /// Flatter embouchure
    #[serde(rename = "windFlatEmbouchure")]
    WindFlatEmbouchure,
    /// Half-closed hole
    #[serde(rename = "windHalfClosedHole1")]
    WindHalfClosedHole1,
    /// Half-closed hole 2
    #[serde(rename = "windHalfClosedHole2")]
    WindHalfClosedHole2,
    /// Half-open hole
    #[serde(rename = "windHalfClosedHole3")]
    WindHalfClosedHole3,
    /// Somewhat relaxed embouchure
    #[serde(rename = "windLessRelaxedEmbouchure")]
    WindLessRelaxedEmbouchure,
    /// Somewhat tight embouchure
    #[serde(rename = "windLessTightEmbouchure")]
    WindLessTightEmbouchure,
    /// Mouthpiece or hand pop
    #[serde(rename = "windMouthpiecePop")]
    WindMouthpiecePop,
    /// Combining multiphonics (black) for stem
    #[serde(rename = "windMultiphonicsBlackStem")]
    WindMultiphonicsBlackStem,
    /// Combining multiphonics (black and white) for stem
    #[serde(rename = "windMultiphonicsBlackWhiteStem")]
    WindMultiphonicsBlackWhiteStem,
    /// Combining multiphonics (white) for stem
    #[serde(rename = "windMultiphonicsWhiteStem")]
    WindMultiphonicsWhiteStem,
    /// Open hole
    #[serde(rename = "windOpenHole")]
    WindOpenHole,
    /// Much more reed (push inwards)
    #[serde(rename = "windReedPositionIn")]
    WindReedPositionIn,
    /// Normal reed position
    #[serde(rename = "windReedPositionNormal")]
    WindReedPositionNormal,
    /// Very little reed (pull outwards)
    #[serde(rename = "windReedPositionOut")]
    WindReedPositionOut,
    /// Relaxed embouchure
    #[serde(rename = "windRelaxedEmbouchure")]
    WindRelaxedEmbouchure,
    /// Rim only
    #[serde(rename = "windRimOnly")]
    WindRimOnly,
    /// Sharper embouchure
    #[serde(rename = "windSharpEmbouchure")]
    WindSharpEmbouchure,
    /// Very tight embouchure / strong air pressure
    #[serde(rename = "windStrongAirPressure")]
    WindStrongAirPressure,
    /// Three-quarters closed hole
    #[serde(rename = "windThreeQuartersClosedHole")]
    WindThreeQuartersClosedHole,
    /// Tight embouchure
    #[serde(rename = "windTightEmbouchure")]
    WindTightEmbouchure,
    /// Trill key
    #[serde(rename = "windTrillKey")]
    WindTrillKey,
    /// Very tight embouchure
    #[serde(rename = "windVeryTightEmbouchure")]
    WindVeryTightEmbouchure,
    /// Very relaxed embouchure / weak air-pressure
    #[serde(rename = "windWeakAirPressure")]
    WindWeakAirPressure,
    /// 4-string tab clef
    #[serde(rename = "4stringTabClef")]
    _4StringTabClef,
    /// 6-string tab clef
    #[serde(rename = "6stringTabClef")]
    _6StringTabClef,
}

impl Glyph {
    /// SMuFL code point
    pub fn codepoint(&self) -> char {
        match self {
            Self::AccSagittal11LargeDiesisDown => '\u{e30d}',
            Self::AccSagittal11LargeDiesisUp => '\u{e30c}',
            Self::AccSagittal11MediumDiesisDown => '\u{e30b}',
            Self::AccSagittal11MediumDiesisUp => '\u{e30a}',
            Self::AccSagittal11V19LargeDiesisDown => '\u{e3ab}',
            Self::AccSagittal11V19LargeDiesisUp => '\u{e3aa}',
            Self::AccSagittal11V19MediumDiesisDown => '\u{e3a3}',
            Self::AccSagittal11V19MediumDiesisUp => '\u{e3a2}',
            Self::AccSagittal11V49CommaDown => '\u{e397}',
            Self::AccSagittal11V49CommaUp => '\u{e396}',
            Self::AccSagittal143CommaDown => '\u{e395}',
            Self::AccSagittal143CommaUp => '\u{e394}',
            Self::AccSagittal17CommaDown => '\u{e343}',
            Self::AccSagittal17CommaUp => '\u{e342}',
            Self::AccSagittal17KleismaDown => '\u{e393}',
            Self::AccSagittal17KleismaUp => '\u{e392}',
            Self::AccSagittal19CommaDown => '\u{e399}',
            Self::AccSagittal19CommaUp => '\u{e398}',
            Self::AccSagittal19SchismaDown => '\u{e391}',
            Self::AccSagittal19SchismaUp => '\u{e390}',
            Self::AccSagittal1MinaDown => '\u{e3f5}',
            Self::AccSagittal1MinaUp => '\u{e3f4}',
            Self::AccSagittal1TinaDown => '\u{e3f9}',
            Self::AccSagittal1TinaUp => '\u{e3f8}',
            Self::AccSagittal23CommaDown => '\u{e371}',
            Self::AccSagittal23CommaUp => '\u{e370}',
            Self::AccSagittal23SmallDiesisDown => '\u{e39f}',
            Self::AccSagittal23SmallDiesisUp => '\u{e39e}',
            Self::AccSagittal25SmallDiesisDown => '\u{e307}',
            Self::AccSagittal25SmallDiesisUp => '\u{e306}',
            Self::AccSagittal2MinasDown => '\u{e3f7}',
            Self::AccSagittal2MinasUp => '\u{e3f6}',
            Self::AccSagittal2TinasDown => '\u{e3fb}',
            Self::AccSagittal2TinasUp => '\u{e3fa}',
            Self::AccSagittal35LargeDiesisDown => '\u{e30f}',
            Self::AccSagittal35LargeDiesisUp => '\u{e30e}',
            Self::AccSagittal35MediumDiesisDown => '\u{e309}',
            Self::AccSagittal35MediumDiesisUp => '\u{e308}',
            Self::AccSagittal3TinasDown => '\u{e3fd}',
            Self::AccSagittal3TinasUp => '\u{e3fc}',
            Self::AccSagittal49LargeDiesisDown => '\u{e3a9}',
            Self::AccSagittal49LargeDiesisUp => '\u{e3a8}',
            Self::AccSagittal49MediumDiesisDown => '\u{e3a5}',
            Self::AccSagittal49MediumDiesisUp => '\u{e3a4}',
            Self::AccSagittal49SmallDiesisDown => '\u{e39d}',
            Self::AccSagittal49SmallDiesisUp => '\u{e39c}',
            Self::AccSagittal4TinasDown => '\u{e3ff}',
            Self::AccSagittal4TinasUp => '\u{e3fe}',
            Self::AccSagittal55CommaDown => '\u{e345}',
            Self::AccSagittal55CommaUp => '\u{e344}',
            Self::AccSagittal5CommaDown => '\u{e303}',
            Self::AccSagittal5CommaUp => '\u{e302}',
            Self::AccSagittal5TinasDown => '\u{e401}',
            Self::AccSagittal5TinasUp => '\u{e400}',
            Self::AccSagittal5V11SmallDiesisDown => '\u{e349}',
            Self::AccSagittal5V11SmallDiesisUp => '\u{e348}',
            Self::AccSagittal5V13LargeDiesisDown => '\u{e3ad}',
            Self::AccSagittal5V13LargeDiesisUp => '\u{e3ac}',
            Self::AccSagittal5V13MediumDiesisDown => '\u{e3a1}',
            Self::AccSagittal5V13MediumDiesisUp => '\u{e3a0}',
            Self::AccSagittal5V19CommaDown => '\u{e373}',
            Self::AccSagittal5V19CommaUp => '\u{e372}',
            Self::AccSagittal5V23SmallDiesisDown => '\u{e375}',
            Self::AccSagittal5V23SmallDiesisUp => '\u{e374}',
            Self::AccSagittal5V49MediumDiesisDown => '\u{e3a7}',
            Self::AccSagittal5V49MediumDiesisUp => '\u{e3a6}',
            Self::AccSagittal5V7KleismaDown => '\u{e301}',
            Self::AccSagittal5V7KleismaUp => '\u{e300}',
            Self::AccSagittal6TinasDown => '\u{e403}',
            Self::AccSagittal6TinasUp => '\u{e402}',
            Self::AccSagittal7CommaDown => '\u{e305}',
            Self::AccSagittal7CommaUp => '\u{e304}',
            Self::AccSagittal7TinasDown => '\u{e405}',
            Self::AccSagittal7TinasUp => '\u{e404}',
            Self::AccSagittal7V11CommaDown => '\u{e347}',
            Self::AccSagittal7V11CommaUp => '\u{e346}',
            Self::AccSagittal7V11KleismaDown => '\u{e341}',
            Self::AccSagittal7V11KleismaUp => '\u{e340}',
            Self::AccSagittal7V19CommaDown => '\u{e39b}',
            Self::AccSagittal7V19CommaUp => '\u{e39a}',
            Self::AccSagittal8TinasDown => '\u{e407}',
            Self::AccSagittal8TinasUp => '\u{e406}',
            Self::AccSagittal9TinasDown => '\u{e409}',
            Self::AccSagittal9TinasUp => '\u{e408}',
            Self::AccSagittalAcute => '\u{e3f2}',
            Self::AccSagittalDoubleFlat => '\u{e335}',
            Self::AccSagittalDoubleFlat11V49CUp => '\u{e3e9}',
            Self::AccSagittalDoubleFlat143CUp => '\u{e3eb}',
            Self::AccSagittalDoubleFlat17CUp => '\u{e365}',
            Self::AccSagittalDoubleFlat17KUp => '\u{e3ed}',
            Self::AccSagittalDoubleFlat19CUp => '\u{e3e7}',
            Self::AccSagittalDoubleFlat19SUp => '\u{e3ef}',
            Self::AccSagittalDoubleFlat23CUp => '\u{e387}',
            Self::AccSagittalDoubleFlat23SUp => '\u{e3e1}',
            Self::AccSagittalDoubleFlat25SUp => '\u{e32d}',
            Self::AccSagittalDoubleFlat49SUp => '\u{e3e3}',
            Self::AccSagittalDoubleFlat55CUp => '\u{e363}',
            Self::AccSagittalDoubleFlat5CUp => '\u{e331}',
            Self::AccSagittalDoubleFlat5V11SUp => '\u{e35f}',
            Self::AccSagittalDoubleFlat5V19CUp => '\u{e385}',
            Self::AccSagittalDoubleFlat5V23SUp => '\u{e383}',
            Self::AccSagittalDoubleFlat5V7KUp => '\u{e333}',
            Self::AccSagittalDoubleFlat7CUp => '\u{e32f}',
            Self::AccSagittalDoubleFlat7V11CUp => '\u{e361}',
            Self::AccSagittalDoubleFlat7V11KUp => '\u{e367}',
            Self::AccSagittalDoubleFlat7V19CUp => '\u{e3e5}',
            Self::AccSagittalDoubleSharp => '\u{e334}',
            Self::AccSagittalDoubleSharp11V49CDown => '\u{e3e8}',
            Self::AccSagittalDoubleSharp143CDown => '\u{e3ea}',
            Self::AccSagittalDoubleSharp17CDown => '\u{e364}',
            Self::AccSagittalDoubleSharp17KDown => '\u{e3ec}',
            Self::AccSagittalDoubleSharp19CDown => '\u{e3e6}',
            Self::AccSagittalDoubleSharp19SDown => '\u{e3ee}',
            Self::AccSagittalDoubleSharp23CDown => '\u{e386}',
            Self::AccSagittalDoubleSharp23SDown => '\u{e3e0}',
            Self::AccSagittalDoubleSharp25SDown => '\u{e32c}',
            Self::AccSagittalDoubleSharp49SDown => '\u{e3e2}',
            Self::AccSagittalDoubleSharp55CDown => '\u{e362}',
            Self::AccSagittalDoubleSharp5CDown => '\u{e330}',
            Self::AccSagittalDoubleSharp5V11SDown => '\u{e35e}',
            Self::AccSagittalDoubleSharp5V19CDown => '\u{e384}',
            Self::AccSagittalDoubleSharp5V23SDown => '\u{e382}',
            Self::AccSagittalDoubleSharp5V7KDown => '\u{e332}',
            Self::AccSagittalDoubleSharp7CDown => '\u{e32e}',
            Self::AccSagittalDoubleSharp7V11CDown => '\u{e360}',
            Self::AccSagittalDoubleSharp7V11KDown => '\u{e366}',
            Self::AccSagittalDoubleSharp7V19CDown => '\u{e3e4}',
            Self::AccSagittalFlat => '\u{e319}',
            Self::AccSagittalFlat11LDown => '\u{e329}',
            Self::AccSagittalFlat11MDown => '\u{e327}',
            Self::AccSagittalFlat11V19LDown => '\u{e3db}',
            Self::AccSagittalFlat11V19MDown => '\u{e3d3}',
            Self::AccSagittalFlat11V49CDown => '\u{e3c7}',
            Self::AccSagittalFlat11V49CUp => '\u{e3b9}',
            Self::AccSagittalFlat143CDown => '\u{e3c5}',
            Self::AccSagittalFlat143CUp => '\u{e3bb}',
            Self::AccSagittalFlat17CDown => '\u{e357}',
            Self::AccSagittalFlat17CUp => '\u{e351}',
            Self::AccSagittalFlat17KDown => '\u{e3c3}',
            Self::AccSagittalFlat17KUp => '\u{e3bd}',
            Self::AccSagittalFlat19CDown => '\u{e3c9}',
            Self::AccSagittalFlat19CUp => '\u{e3b7}',
            Self::AccSagittalFlat19SDown => '\u{e3c1}',
            Self::AccSagittalFlat19SUp => '\u{e3bf}',
            Self::AccSagittalFlat23CDown => '\u{e37d}',
            Self::AccSagittalFlat23CUp => '\u{e37b}',
            Self::AccSagittalFlat23SDown => '\u{e3cf}',
            Self::AccSagittalFlat23SUp => '\u{e3b1}',
            Self::AccSagittalFlat25SDown => '\u{e323}',
            Self::AccSagittalFlat25SUp => '\u{e311}',
            Self::AccSagittalFlat35LDown => '\u{e32b}',
            Self::AccSagittalFlat35MDown => '\u{e325}',
            Self::AccSagittalFlat49LDown => '\u{e3d9}',
            Self::AccSagittalFlat49MDown => '\u{e3d5}',
            Self::AccSagittalFlat49SDown => '\u{e3cd}',
            Self::AccSagittalFlat49SUp => '\u{e3b3}',
            Self::AccSagittalFlat55CDown => '\u{e359}',
            Self::AccSagittalFlat55CUp => '\u{e34f}',
            Self::AccSagittalFlat5CDown => '\u{e31f}',
            Self::AccSagittalFlat5CUp => '\u{e315}',
            Self::AccSagittalFlat5V11SDown => '\u{e35d}',
            Self::AccSagittalFlat5V11SUp => '\u{e34b}',
            Self::AccSagittalFlat5V13LDown => '\u{e3dd}',
            Self::AccSagittalFlat5V13MDown => '\u{e3d1}',
            Self::AccSagittalFlat5V19CDown => '\u{e37f}',
            Self::AccSagittalFlat5V19CUp => '\u{e379}',
            Self::AccSagittalFlat5V23SDown => '\u{e381}',
            Self::AccSagittalFlat5V23SUp => '\u{e377}',
            Self::AccSagittalFlat5V49MDown => '\u{e3d7}',
            Self::AccSagittalFlat5V7KDown => '\u{e31d}',
            Self::AccSagittalFlat5V7KUp => '\u{e317}',
            Self::AccSagittalFlat7CDown => '\u{e321}',
            Self::AccSagittalFlat7CUp => '\u{e313}',
            Self::AccSagittalFlat7V11CDown => '\u{e35b}',
            Self::AccSagittalFlat7V11CUp => '\u{e34d}',
            Self::AccSagittalFlat7V11KDown => '\u{e355}',
            Self::AccSagittalFlat7V11KUp => '\u{e353}',
            Self::AccSagittalFlat7V19CDown => '\u{e3cb}',
            Self::AccSagittalFlat7V19CUp => '\u{e3b5}',
            Self::AccSagittalFractionalTinaDown => '\u{e40b}',
            Self::AccSagittalFractionalTinaUp => '\u{e40a}',
            Self::AccSagittalGrave => '\u{e3f3}',
            Self::AccSagittalShaftDown => '\u{e3f1}',
            Self::AccSagittalShaftUp => '\u{e3f0}',
            Self::AccSagittalSharp => '\u{e318}',
            Self::AccSagittalSharp11LUp => '\u{e328}',
            Self::AccSagittalSharp11MUp => '\u{e326}',
            Self::AccSagittalSharp11V19LUp => '\u{e3da}',
            Self::AccSagittalSharp11V19MUp => '\u{e3d2}',
            Self::AccSagittalSharp11V49CDown => '\u{e3b8}',
            Self::AccSagittalSharp11V49CUp => '\u{e3c6}',
            Self::AccSagittalSharp143CDown => '\u{e3ba}',
            Self::AccSagittalSharp143CUp => '\u{e3c4}',
            Self::AccSagittalSharp17CDown => '\u{e350}',
            Self::AccSagittalSharp17CUp => '\u{e356}',
            Self::AccSagittalSharp17KDown => '\u{e3bc}',
            Self::AccSagittalSharp17KUp => '\u{e3c2}',
            Self::AccSagittalSharp19CDown => '\u{e3b6}',
            Self::AccSagittalSharp19CUp => '\u{e3c8}',
            Self::AccSagittalSharp19SDown => '\u{e3be}',
            Self::AccSagittalSharp19SUp => '\u{e3c0}',
            Self::AccSagittalSharp23CDown => '\u{e37a}',
            Self::AccSagittalSharp23CUp => '\u{e37c}',
            Self::AccSagittalSharp23SDown => '\u{e3b0}',
            Self::AccSagittalSharp23SUp => '\u{e3ce}',
            Self::AccSagittalSharp25SDown => '\u{e310}',
            Self::AccSagittalSharp25SUp => '\u{e322}',
            Self::AccSagittalSharp35LUp => '\u{e32a}',
            Self::AccSagittalSharp35MUp => '\u{e324}',
            Self::AccSagittalSharp49LUp => '\u{e3d8}',
            Self::AccSagittalSharp49MUp => '\u{e3d4}',
            Self::AccSagittalSharp49SDown => '\u{e3b2}',
            Self::AccSagittalSharp49SUp => '\u{e3cc}',
            Self::AccSagittalSharp55CDown => '\u{e34e}',
            Self::AccSagittalSharp55CUp => '\u{e358}',
            Self::AccSagittalSharp5CDown => '\u{e314}',
            Self::AccSagittalSharp5CUp => '\u{e31e}',
            Self::AccSagittalSharp5V11SDown => '\u{e34a}',
            Self::AccSagittalSharp5V11SUp => '\u{e35c}',
            Self::AccSagittalSharp5V13LUp => '\u{e3dc}',
            Self::AccSagittalSharp5V13MUp => '\u{e3d0}',
            Self::AccSagittalSharp5V19CDown => '\u{e378}',
            Self::AccSagittalSharp5V19CUp => '\u{e37e}',
            Self::AccSagittalSharp5V23SDown => '\u{e376}',
            Self::AccSagittalSharp5V23SUp => '\u{e380}',
            Self::AccSagittalSharp5V49MUp => '\u{e3d6}',
            Self::AccSagittalSharp5V7KDown => '\u{e316}',
            Self::AccSagittalSharp5V7KUp => '\u{e31c}',
            Self::AccSagittalSharp7CDown => '\u{e312}',
            Self::AccSagittalSharp7CUp => '\u{e320}',
            Self::AccSagittalSharp7V11CDown => '\u{e34c}',
            Self::AccSagittalSharp7V11CUp => '\u{e35a}',
            Self::AccSagittalSharp7V11KDown => '\u{e352}',
            Self::AccSagittalSharp7V11KUp => '\u{e354}',
            Self::AccSagittalSharp7V19CDown => '\u{e3b4}',
            Self::AccSagittalSharp7V19CUp => '\u{e3ca}',
            Self::AccSagittalUnused1 => '\u{e31a}',
            Self::AccSagittalUnused2 => '\u{e31b}',
            Self::AccSagittalUnused3 => '\u{e3de}',
            Self::AccSagittalUnused4 => '\u{e3df}',
            Self::AccdnCombDot => '\u{e8ca}',
            Self::AccdnCombLh2RanksEmpty => '\u{e8c8}',
            Self::AccdnCombLh3RanksEmptySquare => '\u{e8c9}',
            Self::AccdnCombRh3RanksEmpty => '\u{e8c6}',
            Self::AccdnCombRh4RanksEmpty => '\u{e8c7}',
            Self::AccdnDiatonicClef => '\u{e079}',
            Self::AccdnLh2Ranks16Round => '\u{e8bc}',
            Self::AccdnLh2Ranks8Plus16Round => '\u{e8bd}',
            Self::AccdnLh2Ranks8Round => '\u{e8bb}',
            Self::AccdnLh2RanksFullMasterRound => '\u{e8c0}',
            Self::AccdnLh2RanksMasterPlus16Round => '\u{e8bf}',
            Self::AccdnLh2RanksMasterRound => '\u{e8be}',
            Self::AccdnLh3Ranks2Plus8Square => '\u{e8c4}',
            Self::AccdnLh3Ranks2Square => '\u{e8c2}',
            Self::AccdnLh3Ranks8Square => '\u{e8c1}',
            Self::AccdnLh3RanksDouble8Square => '\u{e8c3}',
            Self::AccdnLh3RanksTuttiSquare => '\u{e8c5}',
            Self::AccdnPull => '\u{e8cc}',
            Self::AccdnPush => '\u{e8cb}',
            Self::AccdnRh3RanksAccordion => '\u{e8ac}',
            Self::AccdnRh3RanksAuthenticMusette => '\u{e8a8}',
            Self::AccdnRh3RanksBandoneon => '\u{e8ab}',
            Self::AccdnRh3RanksBassoon => '\u{e8a4}',
            Self::AccdnRh3RanksClarinet => '\u{e8a1}',
            Self::AccdnRh3RanksDoubleTremoloLower8Ve => '\u{e8b1}',
            Self::AccdnRh3RanksDoubleTremoloUpper8Ve => '\u{e8b2}',
            Self::AccdnRh3RanksFullFactory => '\u{e8b3}',
            Self::AccdnRh3RanksHarmonium => '\u{e8aa}',
            Self::AccdnRh3RanksImitationMusette => '\u{e8a7}',
            Self::AccdnRh3RanksLowerTremolo8 => '\u{e8a3}',
            Self::AccdnRh3RanksMaster => '\u{e8ad}',
            Self::AccdnRh3RanksOboe => '\u{e8a5}',
            Self::AccdnRh3RanksOrgan => '\u{e8a9}',
            Self::AccdnRh3RanksPiccolo => '\u{e8a0}',
            Self::AccdnRh3RanksTremoloLower8Ve => '\u{e8af}',
            Self::AccdnRh3RanksTremoloUpper8Ve => '\u{e8b0}',
            Self::AccdnRh3RanksTwoChoirs => '\u{e8ae}',
            Self::AccdnRh3RanksUpperTremolo8 => '\u{e8a2}',
            Self::AccdnRh3RanksViolin => '\u{e8a6}',
            Self::AccdnRh4RanksAlto => '\u{e8b5}',
            Self::AccdnRh4RanksBassAlto => '\u{e8ba}',
            Self::AccdnRh4RanksMaster => '\u{e8b7}',
            Self::AccdnRh4RanksSoftBass => '\u{e8b8}',
            Self::AccdnRh4RanksSoftTenor => '\u{e8b9}',
            Self::AccdnRh4RanksSoprano => '\u{e8b4}',
            Self::AccdnRh4RanksTenor => '\u{e8b6}',
            Self::AccdnRicochet2 => '\u{e8cd}',
            Self::AccdnRicochet3 => '\u{e8ce}',
            Self::AccdnRicochet4 => '\u{e8cf}',
            Self::AccdnRicochet5 => '\u{e8d0}',
            Self::AccdnRicochet6 => '\u{e8d1}',
            Self::AccdnRicochetStem2 => '\u{e8d2}',
            Self::AccdnRicochetStem3 => '\u{e8d3}',
            Self::AccdnRicochetStem4 => '\u{e8d4}',
            Self::AccdnRicochetStem5 => '\u{e8d5}',
            Self::AccdnRicochetStem6 => '\u{e8d6}',
            Self::Accidental1CommaFlat => '\u{e454}',
            Self::Accidental1CommaSharp => '\u{e450}',
            Self::Accidental2CommaFlat => '\u{e455}',
            Self::Accidental2CommaSharp => '\u{e451}',
            Self::Accidental3CommaFlat => '\u{e456}',
            Self::Accidental3CommaSharp => '\u{e452}',
            Self::Accidental4CommaFlat => '\u{e457}',
            Self::Accidental5CommaSharp => '\u{e453}',
            Self::AccidentalArrowDown => '\u{e27b}',
            Self::AccidentalArrowUp => '\u{e27a}',
            Self::AccidentalBakiyeFlat => '\u{e442}',
            Self::AccidentalBakiyeSharp => '\u{e445}',
            Self::AccidentalBracketLeft => '\u{e26c}',
            Self::AccidentalBracketRight => '\u{e26d}',
            Self::AccidentalBuyukMucennebFlat => '\u{e440}',
            Self::AccidentalBuyukMucennebSharp => '\u{e447}',
            Self::AccidentalCombiningCloseCurlyBrace => '\u{e2ef}',
            Self::AccidentalCombiningLower17Schisma => '\u{e2e6}',
            Self::AccidentalCombiningLower19Schisma => '\u{e2e8}',
            Self::AccidentalCombiningLower23Limit29LimitComma => '\u{e2ea}',
            Self::AccidentalCombiningLower29LimitComma => '\u{ee50}',
            Self::AccidentalCombiningLower31Schisma => '\u{e2ec}',
            Self::AccidentalCombiningLower37Quartertone => '\u{ee52}',
            Self::AccidentalCombiningLower41Comma => '\u{ee54}',
            Self::AccidentalCombiningLower43Comma => '\u{ee56}',
            Self::AccidentalCombiningLower47Quartertone => '\u{ee58}',
            Self::AccidentalCombiningLower53LimitComma => '\u{e2f7}',
            Self::AccidentalCombiningOpenCurlyBrace => '\u{e2ee}',
            Self::AccidentalCombiningRaise17Schisma => '\u{e2e7}',
            Self::AccidentalCombiningRaise19Schisma => '\u{e2e9}',
            Self::AccidentalCombiningRaise23Limit29LimitComma => '\u{e2eb}',
            Self::AccidentalCombiningRaise29LimitComma => '\u{ee51}',
            Self::AccidentalCombiningRaise31Schisma => '\u{e2ed}',
            Self::AccidentalCombiningRaise37Quartertone => '\u{ee53}',
            Self::AccidentalCombiningRaise41Comma => '\u{ee55}',
            Self::AccidentalCombiningRaise43Comma => '\u{ee57}',
            Self::AccidentalCombiningRaise47Quartertone => '\u{ee59}',
            Self::AccidentalCombiningRaise53LimitComma => '\u{e2f8}',
            Self::AccidentalCommaSlashDown => '\u{e47a}',
            Self::AccidentalCommaSlashUp => '\u{e479}',
            Self::AccidentalDoubleFlat => '\u{e264}',
            Self::AccidentalDoubleFlatArabic => '\u{ed30}',
            Self::AccidentalDoubleFlatEqualTempered => '\u{e2f0}',
            Self::AccidentalDoubleFlatOneArrowDown => '\u{e2c0}',
            Self::AccidentalDoubleFlatOneArrowUp => '\u{e2c5}',
            Self::AccidentalDoubleFlatReversed => '\u{e483}',
            Self::AccidentalDoubleFlatThreeArrowsDown => '\u{e2d4}',
            Self::AccidentalDoubleFlatThreeArrowsUp => '\u{e2d9}',
            Self::AccidentalDoubleFlatTurned => '\u{e485}',
            Self::AccidentalDoubleFlatTwoArrowsDown => '\u{e2ca}',
            Self::AccidentalDoubleFlatTwoArrowsUp => '\u{e2cf}',
            Self::AccidentalDoubleSharp => '\u{e263}',
            Self::AccidentalDoubleSharpArabic => '\u{ed38}',
            Self::AccidentalDoubleSharpEqualTempered => '\u{e2f4}',
            Self::AccidentalDoubleSharpOneArrowDown => '\u{e2c4}',
            Self::AccidentalDoubleSharpOneArrowUp => '\u{e2c9}',
            Self::AccidentalDoubleSharpThreeArrowsDown => '\u{e2d8}',
            Self::AccidentalDoubleSharpThreeArrowsUp => '\u{e2dd}',
            Self::AccidentalDoubleSharpTwoArrowsDown => '\u{e2ce}',
            Self::AccidentalDoubleSharpTwoArrowsUp => '\u{e2d3}',
            Self::AccidentalEnharmonicAlmostEqualTo => '\u{e2fa}',
            Self::AccidentalEnharmonicEquals => '\u{e2fb}',
            Self::AccidentalEnharmonicTilde => '\u{e2f9}',
            Self::AccidentalFilledReversedFlatAndFlat => '\u{e296}',
            Self::AccidentalFilledReversedFlatAndFlatArrowDown => '\u{e298}',
            Self::AccidentalFilledReversedFlatAndFlatArrowUp => '\u{e297}',
            Self::AccidentalFilledReversedFlatArrowDown => '\u{e293}',
            Self::AccidentalFilledReversedFlatArrowUp => '\u{e292}',
            Self::AccidentalFiveQuarterTonesFlatArrowDown => '\u{e279}',
            Self::AccidentalFiveQuarterTonesSharpArrowUp => '\u{e276}',
            Self::AccidentalFlat => '\u{e260}',
            Self::AccidentalFlatArabic => '\u{ed32}',
            Self::AccidentalFlatEqualTempered => '\u{e2f1}',
            Self::AccidentalFlatLoweredStockhausen => '\u{ed53}',
            Self::AccidentalFlatOneArrowDown => '\u{e2c1}',
            Self::AccidentalFlatOneArrowUp => '\u{e2c6}',
            Self::AccidentalFlatRaisedStockhausen => '\u{ed52}',
            Self::AccidentalFlatRepeatedLineStockhausen => '\u{ed5c}',
            Self::AccidentalFlatRepeatedSpaceStockhausen => '\u{ed5b}',
            Self::AccidentalFlatThreeArrowsDown => '\u{e2d5}',
            Self::AccidentalFlatThreeArrowsUp => '\u{e2da}',
            Self::AccidentalFlatTurned => '\u{e484}',
            Self::AccidentalFlatTwoArrowsDown => '\u{e2cb}',
            Self::AccidentalFlatTwoArrowsUp => '\u{e2d0}',
            Self::AccidentalHabaFlatQuarterToneHigher => '\u{ee65}',
            Self::AccidentalHabaFlatThreeQuarterTonesLower => '\u{ee69}',
            Self::AccidentalHabaQuarterToneHigher => '\u{ee64}',
            Self::AccidentalHabaQuarterToneLower => '\u{ee67}',
            Self::AccidentalHabaSharpQuarterToneLower => '\u{ee68}',
            Self::AccidentalHabaSharpThreeQuarterTonesHigher => '\u{ee66}',
            Self::AccidentalHalfSharpArrowDown => '\u{e29a}',
            Self::AccidentalHalfSharpArrowUp => '\u{e299}',
            Self::AccidentalJohnston13 => '\u{e2b6}',
            Self::AccidentalJohnston31 => '\u{e2b7}',
            Self::AccidentalJohnstonDown => '\u{e2b5}',
            Self::AccidentalJohnstonEl => '\u{e2b2}',
            Self::AccidentalJohnstonMinus => '\u{e2b1}',
            Self::AccidentalJohnstonPlus => '\u{e2b0}',
            Self::AccidentalJohnstonSeven => '\u{e2b3}',
            Self::AccidentalJohnstonUp => '\u{e2b4}',
            Self::AccidentalKomaFlat => '\u{e443}',
            Self::AccidentalKomaSharp => '\u{e444}',
            Self::AccidentalKoron => '\u{e460}',
            Self::AccidentalKucukMucennebFlat => '\u{e441}',
            Self::AccidentalKucukMucennebSharp => '\u{e446}',
            Self::AccidentalLargeDoubleSharp => '\u{e47d}',
            Self::AccidentalLowerOneSeptimalComma => '\u{e2de}',
            Self::AccidentalLowerOneTridecimalQuartertone => '\u{e2e4}',
            Self::AccidentalLowerOneUndecimalQuartertone => '\u{e2e2}',
            Self::AccidentalLowerTwoSeptimalCommas => '\u{e2e0}',
            Self::AccidentalLoweredStockhausen => '\u{ed51}',
            Self::AccidentalNarrowReversedFlat => '\u{e284}',
            Self::AccidentalNarrowReversedFlatAndFlat => '\u{e285}',
            Self::AccidentalNatural => '\u{e261}',
            Self::AccidentalNaturalArabic => '\u{ed34}',
            Self::AccidentalNaturalEqualTempered => '\u{e2f2}',
            Self::AccidentalNaturalFlat => '\u{e267}',
            Self::AccidentalNaturalLoweredStockhausen => '\u{ed55}',
            Self::AccidentalNaturalOneArrowDown => '\u{e2c2}',
            Self::AccidentalNaturalOneArrowUp => '\u{e2c7}',
            Self::AccidentalNaturalRaisedStockhausen => '\u{ed54}',
            Self::AccidentalNaturalReversed => '\u{e482}',
            Self::AccidentalNaturalSharp => '\u{e268}',
            Self::AccidentalNaturalThreeArrowsDown => '\u{e2d6}',
            Self::AccidentalNaturalThreeArrowsUp => '\u{e2db}',
            Self::AccidentalNaturalTwoArrowsDown => '\u{e2cc}',
            Self::AccidentalNaturalTwoArrowsUp => '\u{e2d1}',
            Self::AccidentalOneAndAHalfSharpsArrowDown => '\u{e29c}',
            Self::AccidentalOneAndAHalfSharpsArrowUp => '\u{e29b}',
            Self::AccidentalOneQuarterToneFlatFerneyhough => '\u{e48f}',
            Self::AccidentalOneQuarterToneFlatStockhausen => '\u{ed59}',
            Self::AccidentalOneQuarterToneSharpFerneyhough => '\u{e48e}',
            Self::AccidentalOneQuarterToneSharpStockhausen => '\u{ed58}',
            Self::AccidentalOneThirdToneFlatFerneyhough => '\u{e48b}',
            Self::AccidentalOneThirdToneSharpFerneyhough => '\u{e48a}',
            Self::AccidentalParensLeft => '\u{e26a}',
            Self::AccidentalParensRight => '\u{e26b}',
            Self::AccidentalQuarterFlatEqualTempered => '\u{e2f5}',
            Self::AccidentalQuarterSharpEqualTempered => '\u{e2f6}',
            Self::AccidentalQuarterToneFlat4 => '\u{e47f}',
            Self::AccidentalQuarterToneFlatArabic => '\u{ed33}',
            Self::AccidentalQuarterToneFlatArrowUp => '\u{e270}',
            Self::AccidentalQuarterToneFlatFilledReversed => '\u{e480}',
            Self::AccidentalQuarterToneFlatNaturalArrowDown => '\u{e273}',
            Self::AccidentalQuarterToneFlatPenderecki => '\u{e478}',
            Self::AccidentalQuarterToneFlatStein => '\u{e280}',
            Self::AccidentalQuarterToneFlatVanBlankenburg => '\u{e488}',
            Self::AccidentalQuarterToneSharp4 => '\u{e47e}',
            Self::AccidentalQuarterToneSharpArabic => '\u{ed35}',
            Self::AccidentalQuarterToneSharpArrowDown => '\u{e275}',
            Self::AccidentalQuarterToneSharpBusotti => '\u{e472}',
            Self::AccidentalQuarterToneSharpNaturalArrowUp => '\u{e272}',
            Self::AccidentalQuarterToneSharpStein => '\u{e282}',
            Self::AccidentalQuarterToneSharpWiggle => '\u{e475}',
            Self::AccidentalRaiseOneSeptimalComma => '\u{e2df}',
            Self::AccidentalRaiseOneTridecimalQuartertone => '\u{e2e5}',
            Self::AccidentalRaiseOneUndecimalQuartertone => '\u{e2e3}',
            Self::AccidentalRaiseTwoSeptimalCommas => '\u{e2e1}',
            Self::AccidentalRaisedStockhausen => '\u{ed50}',
            Self::AccidentalReversedFlatAndFlatArrowDown => '\u{e295}',
            Self::AccidentalReversedFlatAndFlatArrowUp => '\u{e294}',
            Self::AccidentalReversedFlatArrowDown => '\u{e291}',
            Self::AccidentalReversedFlatArrowUp => '\u{e290}',
            Self::AccidentalSharp => '\u{e262}',
            Self::AccidentalSharpArabic => '\u{ed36}',
            Self::AccidentalSharpEqualTempered => '\u{e2f3}',
            Self::AccidentalSharpLoweredStockhausen => '\u{ed57}',
            Self::AccidentalSharpOneArrowDown => '\u{e2c3}',
            Self::AccidentalSharpOneArrowUp => '\u{e2c8}',
            Self::AccidentalSharpOneHorizontalStroke => '\u{e473}',
            Self::AccidentalSharpRaisedStockhausen => '\u{ed56}',
            Self::AccidentalSharpRepeatedLineStockhausen => '\u{ed5e}',
            Self::AccidentalSharpRepeatedSpaceStockhausen => '\u{ed5d}',
            Self::AccidentalSharpReversed => '\u{e481}',
            Self::AccidentalSharpSharp => '\u{e269}',
            Self::AccidentalSharpThreeArrowsDown => '\u{e2d7}',
            Self::AccidentalSharpThreeArrowsUp => '\u{e2dc}',
            Self::AccidentalSharpTwoArrowsDown => '\u{e2cd}',
            Self::AccidentalSharpTwoArrowsUp => '\u{e2d2}',
            Self::AccidentalSims12Down => '\u{e2a0}',
            Self::AccidentalSims12Up => '\u{e2a3}',
            Self::AccidentalSims4Down => '\u{e2a2}',
            Self::AccidentalSims4Up => '\u{e2a5}',
            Self::AccidentalSims6Down => '\u{e2a1}',
            Self::AccidentalSims6Up => '\u{e2a4}',
            Self::AccidentalSori => '\u{e461}',
            Self::AccidentalTavenerFlat => '\u{e477}',
            Self::AccidentalTavenerSharp => '\u{e476}',
            Self::AccidentalThreeQuarterTonesFlatArabic => '\u{ed31}',
            Self::AccidentalThreeQuarterTonesFlatArrowDown => '\u{e271}',
            Self::AccidentalThreeQuarterTonesFlatArrowUp => '\u{e278}',
            Self::AccidentalThreeQuarterTonesFlatCouper => '\u{e489}',
            Self::AccidentalThreeQuarterTonesFlatGrisey => '\u{e486}',
            Self::AccidentalThreeQuarterTonesFlatTartini => '\u{e487}',
            Self::AccidentalThreeQuarterTonesFlatZimmermann => '\u{e281}',
            Self::AccidentalThreeQuarterTonesSharpArabic => '\u{ed37}',
            Self::AccidentalThreeQuarterTonesSharpArrowDown => '\u{e277}',
            Self::AccidentalThreeQuarterTonesSharpArrowUp => '\u{e274}',
            Self::AccidentalThreeQuarterTonesSharpBusotti => '\u{e474}',
            Self::AccidentalThreeQuarterTonesSharpStein => '\u{e283}',
            Self::AccidentalThreeQuarterTonesSharpStockhausen => '\u{ed5a}',
            Self::AccidentalTripleFlat => '\u{e266}',
            Self::AccidentalTripleSharp => '\u{e265}',
            Self::AccidentalTwoThirdTonesFlatFerneyhough => '\u{e48d}',
            Self::AccidentalTwoThirdTonesSharpFerneyhough => '\u{e48c}',
            Self::AccidentalUpsAndDownsDown => '\u{ee61}',
            Self::AccidentalUpsAndDownsLess => '\u{ee63}',
            Self::AccidentalUpsAndDownsMore => '\u{ee62}',
            Self::AccidentalUpsAndDownsUp => '\u{ee60}',
            Self::AccidentalWilsonMinus => '\u{e47c}',
            Self::AccidentalWilsonPlus => '\u{e47b}',
            Self::AccidentalWyschnegradsky10TwelfthsFlat => '\u{e434}',
            Self::AccidentalWyschnegradsky10TwelfthsSharp => '\u{e429}',
            Self::AccidentalWyschnegradsky11TwelfthsFlat => '\u{e435}',
            Self::AccidentalWyschnegradsky11TwelfthsSharp => '\u{e42a}',
            Self::AccidentalWyschnegradsky1TwelfthsFlat => '\u{e42b}',
            Self::AccidentalWyschnegradsky1TwelfthsSharp => '\u{e420}',
            Self::AccidentalWyschnegradsky2TwelfthsFlat => '\u{e42c}',
            Self::AccidentalWyschnegradsky2TwelfthsSharp => '\u{e421}',
            Self::AccidentalWyschnegradsky3TwelfthsFlat => '\u{e42d}',
            Self::AccidentalWyschnegradsky3TwelfthsSharp => '\u{e422}',
            Self::AccidentalWyschnegradsky4TwelfthsFlat => '\u{e42e}',
            Self::AccidentalWyschnegradsky4TwelfthsSharp => '\u{e423}',
            Self::AccidentalWyschnegradsky5TwelfthsFlat => '\u{e42f}',
            Self::AccidentalWyschnegradsky5TwelfthsSharp => '\u{e424}',
            Self::AccidentalWyschnegradsky6TwelfthsFlat => '\u{e430}',
            Self::AccidentalWyschnegradsky6TwelfthsSharp => '\u{e425}',
            Self::AccidentalWyschnegradsky7TwelfthsFlat => '\u{e431}',
            Self::AccidentalWyschnegradsky7TwelfthsSharp => '\u{e426}',
            Self::AccidentalWyschnegradsky8TwelfthsFlat => '\u{e432}',
            Self::AccidentalWyschnegradsky8TwelfthsSharp => '\u{e427}',
            Self::AccidentalWyschnegradsky9TwelfthsFlat => '\u{e433}',
            Self::AccidentalWyschnegradsky9TwelfthsSharp => '\u{e428}',
            Self::AccidentalXenakisOneThirdToneSharp => '\u{e470}',
            Self::AccidentalXenakisTwoThirdTonesSharp => '\u{e471}',
            Self::AnalyticsChoralmelodie => '\u{e86a}',
            Self::AnalyticsEndStimme => '\u{e863}',
            Self::AnalyticsHauptrhythmus => '\u{e86b}',
            Self::AnalyticsHauptstimme => '\u{e860}',
            Self::AnalyticsInversion1 => '\u{e869}',
            Self::AnalyticsNebenstimme => '\u{e861}',
            Self::AnalyticsStartStimme => '\u{e862}',
            Self::AnalyticsTheme => '\u{e864}',
            Self::AnalyticsTheme1 => '\u{e868}',
            Self::AnalyticsThemeInversion => '\u{e867}',
            Self::AnalyticsThemeRetrograde => '\u{e865}',
            Self::AnalyticsThemeRetrogradeInversion => '\u{e866}',
            Self::Arpeggiato => '\u{e63c}',
            Self::ArpeggiatoDown => '\u{e635}',
            Self::ArpeggiatoUp => '\u{e634}',
            Self::ArrowBlackDown => '\u{eb64}',
            Self::ArrowBlackDownLeft => '\u{eb65}',
            Self::ArrowBlackDownRight => '\u{eb63}',
            Self::ArrowBlackLeft => '\u{eb66}',
            Self::ArrowBlackRight => '\u{eb62}',
            Self::ArrowBlackUp => '\u{eb60}',
            Self::ArrowBlackUpLeft => '\u{eb67}',
            Self::ArrowBlackUpRight => '\u{eb61}',
            Self::ArrowOpenDown => '\u{eb74}',
            Self::ArrowOpenDownLeft => '\u{eb75}',
            Self::ArrowOpenDownRight => '\u{eb73}',
            Self::ArrowOpenLeft => '\u{eb76}',
            Self::ArrowOpenRight => '\u{eb72}',
            Self::ArrowOpenUp => '\u{eb70}',
            Self::ArrowOpenUpLeft => '\u{eb77}',
            Self::ArrowOpenUpRight => '\u{eb71}',
            Self::ArrowWhiteDown => '\u{eb6c}',
            Self::ArrowWhiteDownLeft => '\u{eb6d}',
            Self::ArrowWhiteDownRight => '\u{eb6b}',
            Self::ArrowWhiteLeft => '\u{eb6e}',
            Self::ArrowWhiteRight => '\u{eb6a}',
            Self::ArrowWhiteUp => '\u{eb68}',
            Self::ArrowWhiteUpLeft => '\u{eb6f}',
            Self::ArrowWhiteUpRight => '\u{eb69}',
            Self::ArrowheadBlackDown => '\u{eb7c}',
            Self::ArrowheadBlackDownLeft => '\u{eb7d}',
            Self::ArrowheadBlackDownRight => '\u{eb7b}',
            Self::ArrowheadBlackLeft => '\u{eb7e}',
            Self::ArrowheadBlackRight => '\u{eb7a}',
            Self::ArrowheadBlackUp => '\u{eb78}',
            Self::ArrowheadBlackUpLeft => '\u{eb7f}',
            Self::ArrowheadBlackUpRight => '\u{eb79}',
            Self::ArrowheadOpenDown => '\u{eb8c}',
            Self::ArrowheadOpenDownLeft => '\u{eb8d}',
            Self::ArrowheadOpenDownRight => '\u{eb8b}',
            Self::ArrowheadOpenLeft => '\u{eb8e}',
            Self::ArrowheadOpenRight => '\u{eb8a}',
            Self::ArrowheadOpenUp => '\u{eb88}',
            Self::ArrowheadOpenUpLeft => '\u{eb8f}',
            Self::ArrowheadOpenUpRight => '\u{eb89}',
            Self::ArrowheadWhiteDown => '\u{eb84}',
            Self::ArrowheadWhiteDownLeft => '\u{eb85}',
            Self::ArrowheadWhiteDownRight => '\u{eb83}',
            Self::ArrowheadWhiteLeft => '\u{eb86}',
            Self::ArrowheadWhiteRight => '\u{eb82}',
            Self::ArrowheadWhiteUp => '\u{eb80}',
            Self::ArrowheadWhiteUpLeft => '\u{eb87}',
            Self::ArrowheadWhiteUpRight => '\u{eb81}',
            Self::ArticAccentAbove => '\u{e4a0}',
            Self::ArticAccentBelow => '\u{e4a1}',
            Self::ArticAccentStaccatoAbove => '\u{e4b0}',
            Self::ArticAccentStaccatoBelow => '\u{e4b1}',
            Self::ArticLaissezVibrerAbove => '\u{e4ba}',
            Self::ArticLaissezVibrerBelow => '\u{e4bb}',
            Self::ArticMarcatoAbove => '\u{e4ac}',
            Self::ArticMarcatoBelow => '\u{e4ad}',
            Self::ArticMarcatoStaccatoAbove => '\u{e4ae}',
            Self::ArticMarcatoStaccatoBelow => '\u{e4af}',
            Self::ArticMarcatoTenutoAbove => '\u{e4bc}',
            Self::ArticMarcatoTenutoBelow => '\u{e4bd}',
            Self::ArticSoftAccentAbove => '\u{ed40}',
            Self::ArticSoftAccentBelow => '\u{ed41}',
            Self::ArticSoftAccentStaccatoAbove => '\u{ed42}',
            Self::ArticSoftAccentStaccatoBelow => '\u{ed43}',
            Self::ArticSoftAccentTenutoAbove => '\u{ed44}',
            Self::ArticSoftAccentTenutoBelow => '\u{ed45}',
            Self::ArticSoftAccentTenutoStaccatoAbove => '\u{ed46}',
            Self::ArticSoftAccentTenutoStaccatoBelow => '\u{ed47}',
            Self::ArticStaccatissimoAbove => '\u{e4a6}',
            Self::ArticStaccatissimoBelow => '\u{e4a7}',
            Self::ArticStaccatissimoStrokeAbove => '\u{e4aa}',
            Self::ArticStaccatissimoStrokeBelow => '\u{e4ab}',
            Self::ArticStaccatissimoWedgeAbove => '\u{e4a8}',
            Self::ArticStaccatissimoWedgeBelow => '\u{e4a9}',
            Self::ArticStaccatoAbove => '\u{e4a2}',
            Self::ArticStaccatoBelow => '\u{e4a3}',
            Self::ArticStressAbove => '\u{e4b6}',
            Self::ArticStressBelow => '\u{e4b7}',
            Self::ArticTenutoAbove => '\u{e4a4}',
            Self::ArticTenutoAccentAbove => '\u{e4b4}',
            Self::ArticTenutoAccentBelow => '\u{e4b5}',
            Self::ArticTenutoBelow => '\u{e4a5}',
            Self::ArticTenutoStaccatoAbove => '\u{e4b2}',
            Self::ArticTenutoStaccatoBelow => '\u{e4b3}',
            Self::ArticUnstressAbove => '\u{e4b8}',
            Self::ArticUnstressBelow => '\u{e4b9}',
            Self::AugmentationDot => '\u{e1e7}',
            Self::BarlineDashed => '\u{e036}',
            Self::BarlineDotted => '\u{e037}',
            Self::BarlineDouble => '\u{e031}',
            Self::BarlineFinal => '\u{e032}',
            Self::BarlineHeavy => '\u{e034}',
            Self::BarlineHeavyHeavy => '\u{e035}',
            Self::BarlineReverseFinal => '\u{e033}',
            Self::BarlineShort => '\u{e038}',
            Self::BarlineSingle => '\u{e030}',
            Self::BarlineTick => '\u{e039}',
            Self::BeamAccelRit1 => '\u{eaf4}',
            Self::BeamAccelRit10 => '\u{eafd}',
            Self::BeamAccelRit11 => '\u{eafe}',
            Self::BeamAccelRit12 => '\u{eaff}',
            Self::BeamAccelRit13 => '\u{eb00}',
            Self::BeamAccelRit14 => '\u{eb01}',
            Self::BeamAccelRit15 => '\u{eb02}',
            Self::BeamAccelRit2 => '\u{eaf5}',
            Self::BeamAccelRit3 => '\u{eaf6}',
            Self::BeamAccelRit4 => '\u{eaf7}',
            Self::BeamAccelRit5 => '\u{eaf8}',
            Self::BeamAccelRit6 => '\u{eaf9}',
            Self::BeamAccelRit7 => '\u{eafa}',
            Self::BeamAccelRit8 => '\u{eafb}',
            Self::BeamAccelRit9 => '\u{eafc}',
            Self::BeamAccelRitFinal => '\u{eb03}',
            Self::Brace => '\u{e000}',
            Self::Bracket => '\u{e002}',
            Self::BracketBottom => '\u{e004}',
            Self::BracketTop => '\u{e003}',
            Self::BrassBend => '\u{e5e3}',
            Self::BrassDoitLong => '\u{e5d6}',
            Self::BrassDoitMedium => '\u{e5d5}',
            Self::BrassDoitShort => '\u{e5d4}',
            Self::BrassFallLipLong => '\u{e5d9}',
            Self::BrassFallLipMedium => '\u{e5d8}',
            Self::BrassFallLipShort => '\u{e5d7}',
            Self::BrassFallRoughLong => '\u{e5df}',
            Self::BrassFallRoughMedium => '\u{e5de}',
            Self::BrassFallRoughShort => '\u{e5dd}',
            Self::BrassFallSmoothLong => '\u{e5dc}',
            Self::BrassFallSmoothMedium => '\u{e5db}',
            Self::BrassFallSmoothShort => '\u{e5da}',
            Self::BrassFlip => '\u{e5e1}',
            Self::BrassHarmonMuteClosed => '\u{e5e8}',
            Self::BrassHarmonMuteStemHalfLeft => '\u{e5e9}',
            Self::BrassHarmonMuteStemHalfRight => '\u{e5ea}',
            Self::BrassHarmonMuteStemOpen => '\u{e5eb}',
            Self::BrassJazzTurn => '\u{e5e4}',
            Self::BrassLiftLong => '\u{e5d3}',
            Self::BrassLiftMedium => '\u{e5d2}',
            Self::BrassLiftShort => '\u{e5d1}',
            Self::BrassLiftSmoothLong => '\u{e5ee}',
            Self::BrassLiftSmoothMedium => '\u{e5ed}',
            Self::BrassLiftSmoothShort => '\u{e5ec}',
            Self::BrassMuteClosed => '\u{e5e5}',
            Self::BrassMuteHalfClosed => '\u{e5e6}',
            Self::BrassMuteOpen => '\u{e5e7}',
            Self::BrassPlop => '\u{e5e0}',
            Self::BrassScoop => '\u{e5d0}',
            Self::BrassSmear => '\u{e5e2}',
            Self::BrassValveTrill => '\u{e5ef}',
            Self::BreathMarkComma => '\u{e4ce}',
            Self::BreathMarkSalzedo => '\u{e4d5}',
            Self::BreathMarkTick => '\u{e4cf}',
            Self::BreathMarkUpbow => '\u{e4d0}',
            Self::BridgeClef => '\u{e078}',
            Self::BuzzRoll => '\u{e22a}',
            Self::CClef => '\u{e05c}',
            Self::CClef8Vb => '\u{e05d}',
            Self::CClefArrowDown => '\u{e05f}',
            Self::CClefArrowUp => '\u{e05e}',
            Self::CClefChange => '\u{e07b}',
            Self::CClefCombining => '\u{e061}',
            Self::CClefReversed => '\u{e075}',
            Self::CClefSquare => '\u{e060}',
            Self::Caesura => '\u{e4d1}',
            Self::CaesuraCurved => '\u{e4d4}',
            Self::CaesuraShort => '\u{e4d3}',
            Self::CaesuraSingleStroke => '\u{e4d7}',
            Self::CaesuraThick => '\u{e4d2}',
            Self::ChantAccentusAbove => '\u{e9d6}',
            Self::ChantAccentusBelow => '\u{e9d7}',
            Self::ChantAuctumAsc => '\u{e994}',
            Self::ChantAuctumDesc => '\u{e995}',
            Self::ChantAugmentum => '\u{e9d9}',
            Self::ChantCaesura => '\u{e8f8}',
            Self::ChantCclef => '\u{e906}',
            Self::ChantCirculusAbove => '\u{e9d2}',
            Self::ChantCirculusBelow => '\u{e9d3}',
            Self::ChantConnectingLineAsc2Nd => '\u{e9bd}',
            Self::ChantConnectingLineAsc3Rd => '\u{e9be}',
            Self::ChantConnectingLineAsc4Th => '\u{e9bf}',
            Self::ChantConnectingLineAsc5Th => '\u{e9c0}',
            Self::ChantConnectingLineAsc6Th => '\u{e9c1}',
            Self::ChantCustosStemDownPosHigh => '\u{ea08}',
            Self::ChantCustosStemDownPosHighest => '\u{ea09}',
            Self::ChantCustosStemDownPosMiddle => '\u{ea07}',
            Self::ChantCustosStemUpPosLow => '\u{ea05}',
            Self::ChantCustosStemUpPosLowest => '\u{ea04}',
            Self::ChantCustosStemUpPosMiddle => '\u{ea06}',
            Self::ChantDeminutumLower => '\u{e9b3}',
            Self::ChantDeminutumUpper => '\u{e9b2}',
            Self::ChantDivisioFinalis => '\u{e8f6}',
            Self::ChantDivisioMaior => '\u{e8f4}',
            Self::ChantDivisioMaxima => '\u{e8f5}',
            Self::ChantDivisioMinima => '\u{e8f3}',
            Self::ChantEntryLineAsc2Nd => '\u{e9b4}',
            Self::ChantEntryLineAsc3Rd => '\u{e9b5}',
            Self::ChantEntryLineAsc4Th => '\u{e9b6}',
            Self::ChantEntryLineAsc5Th => '\u{e9b7}',
            Self::ChantEntryLineAsc6Th => '\u{e9b8}',
            Self::ChantEpisema => '\u{e9d8}',
            Self::ChantFclef => '\u{e902}',
            Self::ChantIctusAbove => '\u{e9d0}',
            Self::ChantIctusBelow => '\u{e9d1}',
            Self::ChantLigaturaDesc2Nd => '\u{e9b9}',
            Self::ChantLigaturaDesc3Rd => '\u{e9ba}',
            Self::ChantLigaturaDesc4Th => '\u{e9bb}',
            Self::ChantLigaturaDesc5Th => '\u{e9bc}',
            Self::ChantOriscusAscending => '\u{e99c}',
            Self::ChantOriscusDescending => '\u{e99d}',
            Self::ChantOriscusLiquescens => '\u{e99e}',
            Self::ChantPodatusLower => '\u{e9b0}',
            Self::ChantPodatusUpper => '\u{e9b1}',
            Self::ChantPunctum => '\u{e990}',
            Self::ChantPunctumCavum => '\u{e998}',
            Self::ChantPunctumDeminutum => '\u{e9a1}',
            Self::ChantPunctumInclinatum => '\u{e991}',
            Self::ChantPunctumInclinatumAuctum => '\u{e992}',
            Self::ChantPunctumInclinatumDeminutum => '\u{e993}',
            Self::ChantPunctumLinea => '\u{e999}',
            Self::ChantPunctumLineaCavum => '\u{e99a}',
            Self::ChantPunctumVirga => '\u{e996}',
            Self::ChantPunctumVirgaReversed => '\u{e997}',
            Self::ChantQuilisma => '\u{e99b}',
            Self::ChantSemicirculusAbove => '\u{e9d4}',
            Self::ChantSemicirculusBelow => '\u{e9d5}',
            Self::ChantStaff => '\u{e8f0}',
            Self::ChantStaffNarrow => '\u{e8f2}',
            Self::ChantStaffWide => '\u{e8f1}',
            Self::ChantStrophicus => '\u{e99f}',
            Self::ChantStrophicusAuctus => '\u{e9a0}',
            Self::ChantStrophicusLiquescens2Nd => '\u{e9c2}',
            Self::ChantStrophicusLiquescens3Rd => '\u{e9c3}',
            Self::ChantStrophicusLiquescens4Th => '\u{e9c4}',
            Self::ChantStrophicusLiquescens5Th => '\u{e9c5}',
            Self::ChantVirgula => '\u{e8f7}',
            Self::Clef15 => '\u{e07e}',
            Self::Clef8 => '\u{e07d}',
            Self::ClefChangeCombining => '\u{e07f}',
            Self::Coda => '\u{e048}',
            Self::CodaSquare => '\u{e049}',
            Self::ConductorBeat2Compound => '\u{e897}',
            Self::ConductorBeat2Simple => '\u{e894}',
            Self::ConductorBeat3Compound => '\u{e898}',
            Self::ConductorBeat3Simple => '\u{e895}',
            Self::ConductorBeat4Compound => '\u{e899}',
            Self::ConductorBeat4Simple => '\u{e896}',
            Self::ConductorLeftBeat => '\u{e891}',
            Self::ConductorRightBeat => '\u{e892}',
            Self::ConductorStrongBeat => '\u{e890}',
            Self::ConductorUnconducted => '\u{e89a}',
            Self::ConductorWeakBeat => '\u{e893}',
            Self::ControlBeginBeam => '\u{e8e0}',
            Self::ControlBeginPhrase => '\u{e8e6}',
            Self::ControlBeginSlur => '\u{e8e4}',
            Self::ControlBeginTie => '\u{e8e2}',
            Self::ControlEndBeam => '\u{e8e1}',
            Self::ControlEndPhrase => '\u{e8e7}',
            Self::ControlEndSlur => '\u{e8e5}',
            Self::ControlEndTie => '\u{e8e3}',
            Self::CsymAccidentalDoubleFlat => '\u{ed64}',
            Self::CsymAccidentalDoubleSharp => '\u{ed63}',
            Self::CsymAccidentalFlat => '\u{ed60}',
            Self::CsymAccidentalNatural => '\u{ed61}',
            Self::CsymAccidentalSharp => '\u{ed62}',
            Self::CsymAccidentalTripleFlat => '\u{ed66}',
            Self::CsymAccidentalTripleSharp => '\u{ed65}',
            Self::CsymAlteredBassSlash => '\u{e87b}',
            Self::CsymAugmented => '\u{e872}',
            Self::CsymBracketLeftTall => '\u{e877}',
            Self::CsymBracketRightTall => '\u{e878}',
            Self::CsymDiagonalArrangementSlash => '\u{e87c}',
            Self::CsymDiminished => '\u{e870}',
            Self::CsymHalfDiminished => '\u{e871}',
            Self::CsymMajorSeventh => '\u{e873}',
            Self::CsymMinor => '\u{e874}',
            Self::CsymParensLeftTall => '\u{e875}',
            Self::CsymParensLeftVeryTall => '\u{e879}',
            Self::CsymParensRightTall => '\u{e876}',
            Self::CsymParensRightVeryTall => '\u{e87a}',
            Self::CurlewSign => '\u{e4d6}',
            Self::DaCapo => '\u{e046}',
            Self::DalSegno => '\u{e045}',
            Self::DaseianExcellentes1 => '\u{ea3c}',
            Self::DaseianExcellentes2 => '\u{ea3d}',
            Self::DaseianExcellentes3 => '\u{ea3e}',
            Self::DaseianExcellentes4 => '\u{ea3f}',
            Self::DaseianFinales1 => '\u{ea34}',
            Self::DaseianFinales2 => '\u{ea35}',
            Self::DaseianFinales3 => '\u{ea36}',
            Self::DaseianFinales4 => '\u{ea37}',
            Self::DaseianGraves1 => '\u{ea30}',
            Self::DaseianGraves2 => '\u{ea31}',
            Self::DaseianGraves3 => '\u{ea32}',
            Self::DaseianGraves4 => '\u{ea33}',
            Self::DaseianResidua1 => '\u{ea40}',
            Self::DaseianResidua2 => '\u{ea41}',
            Self::DaseianSuperiores1 => '\u{ea38}',
            Self::DaseianSuperiores2 => '\u{ea39}',
            Self::DaseianSuperiores3 => '\u{ea3a}',
            Self::DaseianSuperiores4 => '\u{ea3b}',
            Self::DoubleLateralRollStevens => '\u{e234}',
            Self::DoubleTongueAbove => '\u{e5f0}',
            Self::DoubleTongueBelow => '\u{e5f1}',
            Self::DynamicCombinedSeparatorColon => '\u{e546}',
            Self::DynamicCombinedSeparatorHyphen => '\u{e547}',
            Self::DynamicCombinedSeparatorSlash => '\u{e549}',
            Self::DynamicCombinedSeparatorSpace => '\u{e548}',
            Self::DynamicCrescendoHairpin => '\u{e53e}',
            Self::DynamicDiminuendoHairpin => '\u{e53f}',
            Self::DynamicFf => '\u{e52f}',
            Self::DynamicFff => '\u{e530}',
            Self::DynamicFfff => '\u{e531}',
            Self::DynamicFffff => '\u{e532}',
            Self::DynamicFfffff => '\u{e533}',
            Self::DynamicForte => '\u{e522}',
            Self::DynamicFortePiano => '\u{e534}',
            Self::DynamicForzando => '\u{e535}',
            Self::DynamicHairpinBracketLeft => '\u{e544}',
            Self::DynamicHairpinBracketRight => '\u{e545}',
            Self::DynamicHairpinParenthesisLeft => '\u{e542}',
            Self::DynamicHairpinParenthesisRight => '\u{e543}',
            Self::DynamicMessaDiVoce => '\u{e540}',
            Self::DynamicMezzo => '\u{e521}',
            Self::DynamicMf => '\u{e52d}',
            Self::DynamicMp => '\u{e52c}',
            Self::DynamicNiente => '\u{e526}',
            Self::DynamicNienteForHairpin => '\u{e541}',
            Self::DynamicPf => '\u{e52e}',
            Self::DynamicPiano => '\u{e520}',
            Self::DynamicPp => '\u{e52b}',
            Self::DynamicPpp => '\u{e52a}',
            Self::DynamicPppp => '\u{e529}',
            Self::DynamicPpppp => '\u{e528}',
            Self::DynamicPppppp => '\u{e527}',
            Self::DynamicRinforzando => '\u{e523}',
            Self::DynamicRinforzando1 => '\u{e53c}',
            Self::DynamicRinforzando2 => '\u{e53d}',
            Self::DynamicSforzando => '\u{e524}',
            Self::DynamicSforzando1 => '\u{e536}',
            Self::DynamicSforzandoPianissimo => '\u{e538}',
            Self::DynamicSforzandoPiano => '\u{e537}',
            Self::DynamicSforzato => '\u{e539}',
            Self::DynamicSforzatoFf => '\u{e53b}',
            Self::DynamicSforzatoPiano => '\u{e53a}',
            Self::DynamicZ => '\u{e525}',
            Self::ElecAudioChannelsEight => '\u{eb46}',
            Self::ElecAudioChannelsFive => '\u{eb43}',
            Self::ElecAudioChannelsFour => '\u{eb42}',
            Self::ElecAudioChannelsOne => '\u{eb3e}',
            Self::ElecAudioChannelsSeven => '\u{eb45}',
            Self::ElecAudioChannelsSix => '\u{eb44}',
            Self::ElecAudioChannelsThreeFrontal => '\u{eb40}',
            Self::ElecAudioChannelsThreeSurround => '\u{eb41}',
            Self::ElecAudioChannelsTwo => '\u{eb3f}',
            Self::ElecAudioIn => '\u{eb49}',
            Self::ElecAudioMono => '\u{eb3c}',
            Self::ElecAudioOut => '\u{eb4a}',
            Self::ElecAudioStereo => '\u{eb3d}',
            Self::ElecCamera => '\u{eb1b}',
            Self::ElecDataIn => '\u{eb4d}',
            Self::ElecDataOut => '\u{eb4e}',
            Self::ElecDisc => '\u{eb13}',
            Self::ElecDownload => '\u{eb4f}',
            Self::ElecEject => '\u{eb2b}',
            Self::ElecFastForward => '\u{eb1f}',
            Self::ElecHeadphones => '\u{eb11}',
            Self::ElecHeadset => '\u{eb12}',
            Self::ElecLineIn => '\u{eb47}',
            Self::ElecLineOut => '\u{eb48}',
            Self::ElecLoop => '\u{eb23}',
            Self::ElecLoudspeaker => '\u{eb1a}',
            Self::ElecMicrophone => '\u{eb10}',
            Self::ElecMicrophoneMute => '\u{eb28}',
            Self::ElecMicrophoneUnmute => '\u{eb29}',
            Self::ElecMidiController0 => '\u{eb36}',
            Self::ElecMidiController100 => '\u{eb3b}',
            Self::ElecMidiController20 => '\u{eb37}',
            Self::ElecMidiController40 => '\u{eb38}',
            Self::ElecMidiController60 => '\u{eb39}',
            Self::ElecMidiController80 => '\u{eb3a}',
            Self::ElecMidiIn => '\u{eb34}',
            Self::ElecMidiOut => '\u{eb35}',
            Self::ElecMixingConsole => '\u{eb15}',
            Self::ElecMonitor => '\u{eb18}',
            Self::ElecMute => '\u{eb26}',
            Self::ElecPause => '\u{eb1e}',
            Self::ElecPlay => '\u{eb1c}',
            Self::ElecPowerOnOff => '\u{eb2a}',
            Self::ElecProjector => '\u{eb19}',
            Self::ElecReplay => '\u{eb24}',
            Self::ElecRewind => '\u{eb20}',
            Self::ElecShuffle => '\u{eb25}',
            Self::ElecSkipBackwards => '\u{eb22}',
            Self::ElecSkipForwards => '\u{eb21}',
            Self::ElecStop => '\u{eb1d}',
            Self::ElecTape => '\u{eb14}',
            Self::ElecUnmute => '\u{eb27}',
            Self::ElecUpload => '\u{eb50}',
            Self::ElecUsb => '\u{eb16}',
            Self::ElecVideoCamera => '\u{eb17}',
            Self::ElecVideoIn => '\u{eb4b}',
            Self::ElecVideoOut => '\u{eb4c}',
            Self::ElecVolumeFader => '\u{eb2c}',
            Self::ElecVolumeFaderThumb => '\u{eb2d}',
            Self::ElecVolumeLevel0 => '\u{eb2e}',
            Self::ElecVolumeLevel100 => '\u{eb33}',
            Self::ElecVolumeLevel20 => '\u{eb2f}',
            Self::ElecVolumeLevel40 => '\u{eb30}',
            Self::ElecVolumeLevel60 => '\u{eb31}',
            Self::ElecVolumeLevel80 => '\u{eb32}',
            Self::FClef => '\u{e062}',
            Self::FClef15Ma => '\u{e066}',
            Self::FClef15Mb => '\u{e063}',
            Self::FClef8Va => '\u{e065}',
            Self::FClef8Vb => '\u{e064}',
            Self::FClefArrowDown => '\u{e068}',
            Self::FClefArrowUp => '\u{e067}',
            Self::FClefChange => '\u{e07c}',
            Self::FClefReversed => '\u{e076}',
            Self::FClefTurned => '\u{e077}',
            Self::FermataAbove => '\u{e4c0}',
            Self::FermataBelow => '\u{e4c1}',
            Self::FermataLongAbove => '\u{e4c6}',
            Self::FermataLongBelow => '\u{e4c7}',
            Self::FermataLongHenzeAbove => '\u{e4ca}',
            Self::FermataLongHenzeBelow => '\u{e4cb}',
            Self::FermataShortAbove => '\u{e4c4}',
            Self::FermataShortBelow => '\u{e4c5}',
            Self::FermataShortHenzeAbove => '\u{e4cc}',
            Self::FermataShortHenzeBelow => '\u{e4cd}',
            Self::FermataVeryLongAbove => '\u{e4c8}',
            Self::FermataVeryLongBelow => '\u{e4c9}',
            Self::FermataVeryShortAbove => '\u{e4c2}',
            Self::FermataVeryShortBelow => '\u{e4c3}',
            Self::Figbass0 => '\u{ea50}',
            Self::Figbass1 => '\u{ea51}',
            Self::Figbass2 => '\u{ea52}',
            Self::Figbass2Raised => '\u{ea53}',
            Self::Figbass3 => '\u{ea54}',
            Self::Figbass4 => '\u{ea55}',
            Self::Figbass4Raised => '\u{ea56}',
            Self::Figbass5 => '\u{ea57}',
            Self::Figbass5Raised1 => '\u{ea58}',
            Self::Figbass5Raised2 => '\u{ea59}',
            Self::Figbass5Raised3 => '\u{ea5a}',
            Self::Figbass6 => '\u{ea5b}',
            Self::Figbass6Raised => '\u{ea5c}',
            Self::Figbass6Raised2 => '\u{ea6f}',
            Self::Figbass7 => '\u{ea5d}',
            Self::Figbass7Diminished => '\u{ecc0}',
            Self::Figbass7Raised1 => '\u{ea5e}',
            Self::Figbass7Raised2 => '\u{ea5f}',
            Self::Figbass8 => '\u{ea60}',
            Self::Figbass9 => '\u{ea61}',
            Self::Figbass9Raised => '\u{ea62}',
            Self::FigbassBracketLeft => '\u{ea68}',
            Self::FigbassBracketRight => '\u{ea69}',
            Self::FigbassCombiningLowering => '\u{ea6e}',
            Self::FigbassCombiningRaising => '\u{ea6d}',
            Self::FigbassDoubleFlat => '\u{ea63}',
            Self::FigbassDoubleSharp => '\u{ea67}',
            Self::FigbassFlat => '\u{ea64}',
            Self::FigbassNatural => '\u{ea65}',
            Self::FigbassParensLeft => '\u{ea6a}',
            Self::FigbassParensRight => '\u{ea6b}',
            Self::FigbassPlus => '\u{ea6c}',
            Self::FigbassSharp => '\u{ea66}',
            Self::FigbassTripleFlat => '\u{ecc1}',
            Self::FigbassTripleSharp => '\u{ecc2}',
            Self::Fingering0 => '\u{ed10}',
            Self::Fingering0Italic => '\u{ed80}',
            Self::Fingering1 => '\u{ed11}',
            Self::Fingering1Italic => '\u{ed81}',
            Self::Fingering2 => '\u{ed12}',
            Self::Fingering2Italic => '\u{ed82}',
            Self::Fingering3 => '\u{ed13}',
            Self::Fingering3Italic => '\u{ed83}',
            Self::Fingering4 => '\u{ed14}',
            Self::Fingering4Italic => '\u{ed84}',
            Self::Fingering5 => '\u{ed15}',
            Self::Fingering5Italic => '\u{ed85}',
            Self::Fingering6 => '\u{ed24}',
            Self::Fingering6Italic => '\u{ed86}',
            Self::Fingering7 => '\u{ed25}',
            Self::Fingering7Italic => '\u{ed87}',
            Self::Fingering8 => '\u{ed26}',
            Self::Fingering8Italic => '\u{ed88}',
            Self::Fingering9 => '\u{ed27}',
            Self::Fingering9Italic => '\u{ed89}',
            Self::FingeringALower => '\u{ed1b}',
            Self::FingeringCLower => '\u{ed1c}',
            Self::FingeringELower => '\u{ed1e}',
            Self::FingeringILower => '\u{ed19}',
            Self::FingeringLeftBracket => '\u{ed2a}',
            Self::FingeringLeftBracketItalic => '\u{ed8c}',
            Self::FingeringLeftParenthesis => '\u{ed28}',
            Self::FingeringLeftParenthesisItalic => '\u{ed8a}',
            Self::FingeringMLower => '\u{ed1a}',
            Self::FingeringMultipleNotes => '\u{ed23}',
            Self::FingeringOLower => '\u{ed1f}',
            Self::FingeringPLower => '\u{ed17}',
            Self::FingeringQLower => '\u{ed8e}',
            Self::FingeringRightBracket => '\u{ed2b}',
            Self::FingeringRightBracketItalic => '\u{ed8d}',
            Self::FingeringRightParenthesis => '\u{ed29}',
            Self::FingeringRightParenthesisItalic => '\u{ed8b}',
            Self::FingeringSLower => '\u{ed8f}',
            Self::FingeringSeparatorMiddleDot => '\u{ed2c}',
            Self::FingeringSeparatorMiddleDotWhite => '\u{ed2d}',
            Self::FingeringSeparatorSlash => '\u{ed2e}',
            Self::FingeringSubstitutionAbove => '\u{ed20}',
            Self::FingeringSubstitutionBelow => '\u{ed21}',
            Self::FingeringSubstitutionDash => '\u{ed22}',
            Self::FingeringTLower => '\u{ed18}',
            Self::FingeringTUpper => '\u{ed16}',
            Self::FingeringXLower => '\u{ed1d}',
            Self::Flag1024ThDown => '\u{e24f}',
            Self::Flag1024ThUp => '\u{e24e}',
            Self::Flag128ThDown => '\u{e249}',
            Self::Flag128ThUp => '\u{e248}',
            Self::Flag16ThDown => '\u{e243}',
            Self::Flag16ThUp => '\u{e242}',
            Self::Flag256ThDown => '\u{e24b}',
            Self::Flag256ThUp => '\u{e24a}',
            Self::Flag32NdDown => '\u{e245}',
            Self::Flag32NdUp => '\u{e244}',
            Self::Flag512ThDown => '\u{e24d}',
            Self::Flag512ThUp => '\u{e24c}',
            Self::Flag64ThDown => '\u{e247}',
            Self::Flag64ThUp => '\u{e246}',
            Self::Flag8ThDown => '\u{e241}',
            Self::Flag8ThUp => '\u{e240}',
            Self::FlagInternalDown => '\u{e251}',
            Self::FlagInternalUp => '\u{e250}',
            Self::Fretboard3String => '\u{e850}',
            Self::Fretboard3StringNut => '\u{e851}',
            Self::Fretboard4String => '\u{e852}',
            Self::Fretboard4StringNut => '\u{e853}',
            Self::Fretboard5String => '\u{e854}',
            Self::Fretboard5StringNut => '\u{e855}',
            Self::Fretboard6String => '\u{e856}',
            Self::Fretboard6StringNut => '\u{e857}',
            Self::FretboardFilledCircle => '\u{e858}',
            Self::FretboardO => '\u{e85a}',
            Self::FretboardX => '\u{e859}',
            Self::FunctionAngleLeft => '\u{ea93}',
            Self::FunctionAngleRight => '\u{ea94}',
            Self::FunctionBracketLeft => '\u{ea8f}',
            Self::FunctionBracketRight => '\u{ea90}',
            Self::FunctionDLower => '\u{ea80}',
            Self::FunctionDUpper => '\u{ea7f}',
            Self::FunctionDd => '\u{ea81}',
            Self::FunctionEight => '\u{ea78}',
            Self::FunctionFUpper => '\u{ea99}',
            Self::FunctionFive => '\u{ea75}',
            Self::FunctionFour => '\u{ea74}',
            Self::FunctionGLower => '\u{ea84}',
            Self::FunctionGUpper => '\u{ea83}',
            Self::FunctionGreaterThan => '\u{ea7c}',
            Self::FunctionILower => '\u{ea9b}',
            Self::FunctionIUpper => '\u{ea9a}',
            Self::FunctionKLower => '\u{ea9d}',
            Self::FunctionKUpper => '\u{ea9c}',
            Self::FunctionLLower => '\u{ea9f}',
            Self::FunctionLUpper => '\u{ea9e}',
            Self::FunctionLessThan => '\u{ea7a}',
            Self::FunctionMLower => '\u{ed01}',
            Self::FunctionMUpper => '\u{ed00}',
            Self::FunctionMinus => '\u{ea7b}',
            Self::FunctionNLower => '\u{ea86}',
            Self::FunctionNUpper => '\u{ea85}',
            Self::FunctionNUpperSuperscript => '\u{ed02}',
            Self::FunctionNine => '\u{ea79}',
            Self::FunctionOne => '\u{ea71}',
            Self::FunctionPLower => '\u{ea88}',
            Self::FunctionPUpper => '\u{ea87}',
            Self::FunctionParensLeft => '\u{ea91}',
            Self::FunctionParensRight => '\u{ea92}',
            Self::FunctionPlus => '\u{ea98}',
            Self::FunctionRLower => '\u{ed03}',
            Self::FunctionRepetition1 => '\u{ea95}',
            Self::FunctionRepetition2 => '\u{ea96}',
            Self::FunctionRing => '\u{ea97}',
            Self::FunctionSLower => '\u{ea8a}',
            Self::FunctionSUpper => '\u{ea89}',
            Self::FunctionSeven => '\u{ea77}',
            Self::FunctionSix => '\u{ea76}',
            Self::FunctionSlashedDd => '\u{ea82}',
            Self::FunctionSsLower => '\u{ea7e}',
            Self::FunctionSsUpper => '\u{ea7d}',
            Self::FunctionTLower => '\u{ea8c}',
            Self::FunctionTUpper => '\u{ea8b}',
            Self::FunctionThree => '\u{ea73}',
            Self::FunctionTwo => '\u{ea72}',
            Self::FunctionVLower => '\u{ea8e}',
            Self::FunctionVUpper => '\u{ea8d}',
            Self::FunctionZero => '\u{ea70}',
            Self::GClef => '\u{e050}',
            Self::GClef15Ma => '\u{e054}',
            Self::GClef15Mb => '\u{e051}',
            Self::GClef8Va => '\u{e053}',
            Self::GClef8Vb => '\u{e052}',
            Self::GClef8VbCClef => '\u{e056}',
            Self::GClef8VbOld => '\u{e055}',
            Self::GClef8VbParens => '\u{e057}',
            Self::GClefArrowDown => '\u{e05b}',
            Self::GClefArrowUp => '\u{e05a}',
            Self::GClefChange => '\u{e07a}',
            Self::GClefLigatedNumberAbove => '\u{e059}',
            Self::GClefLigatedNumberBelow => '\u{e058}',
            Self::GClefReversed => '\u{e073}',
            Self::GClefTurned => '\u{e074}',
            Self::GlissandoDown => '\u{e586}',
            Self::GlissandoUp => '\u{e585}',
            Self::GraceNoteAcciaccaturaStemDown => '\u{e561}',
            Self::GraceNoteAcciaccaturaStemUp => '\u{e560}',
            Self::GraceNoteAppoggiaturaStemDown => '\u{e563}',
            Self::GraceNoteAppoggiaturaStemUp => '\u{e562}',
            Self::GraceNoteSlashStemDown => '\u{e565}',
            Self::GraceNoteSlashStemUp => '\u{e564}',
            Self::GuitarBarreFull => '\u{e848}',
            Self::GuitarBarreHalf => '\u{e849}',
            Self::GuitarClosePedal => '\u{e83f}',
            Self::GuitarFadeIn => '\u{e843}',
            Self::GuitarFadeOut => '\u{e844}',
            Self::GuitarGolpe => '\u{e842}',
            Self::GuitarHalfOpenPedal => '\u{e83e}',
            Self::GuitarLeftHandTapping => '\u{e840}',
            Self::GuitarOpenPedal => '\u{e83d}',
            Self::GuitarRightHandTapping => '\u{e841}',
            Self::GuitarShake => '\u{e832}',
            Self::GuitarString0 => '\u{e833}',
            Self::GuitarString1 => '\u{e834}',
            Self::GuitarString10 => '\u{e84a}',
            Self::GuitarString11 => '\u{e84b}',
            Self::GuitarString12 => '\u{e84c}',
            Self::GuitarString13 => '\u{e84d}',
            Self::GuitarString2 => '\u{e835}',
            Self::GuitarString3 => '\u{e836}',
            Self::GuitarString4 => '\u{e837}',
            Self::GuitarString5 => '\u{e838}',
            Self::GuitarString6 => '\u{e839}',
            Self::GuitarString7 => '\u{e83a}',
            Self::GuitarString8 => '\u{e83b}',
            Self::GuitarString9 => '\u{e83c}',
            Self::GuitarStrumDown => '\u{e847}',
            Self::GuitarStrumUp => '\u{e846}',
            Self::GuitarVibratoBarDip => '\u{e831}',
            Self::GuitarVibratoBarScoop => '\u{e830}',
            Self::GuitarVibratoStroke => '\u{eab2}',
            Self::GuitarVolumeSwell => '\u{e845}',
            Self::GuitarWideVibratoStroke => '\u{eab3}',
            Self::HandbellsBelltree => '\u{e81f}',
            Self::HandbellsDamp3 => '\u{e81e}',
            Self::HandbellsEcho1 => '\u{e81b}',
            Self::HandbellsEcho2 => '\u{e81c}',
            Self::HandbellsGyro => '\u{e81d}',
            Self::HandbellsHandMartellato => '\u{e812}',
            Self::HandbellsMalletBellOnTable => '\u{e815}',
            Self::HandbellsMalletBellSuspended => '\u{e814}',
            Self::HandbellsMalletLft => '\u{e816}',
            Self::HandbellsMartellato => '\u{e810}',
            Self::HandbellsMartellatoLift => '\u{e811}',
            Self::HandbellsMutedMartellato => '\u{e813}',
            Self::HandbellsPluckLift => '\u{e817}',
            Self::HandbellsSwing => '\u{e81a}',
            Self::HandbellsSwingDown => '\u{e819}',
            Self::HandbellsSwingUp => '\u{e818}',
            Self::HandbellsTablePairBells => '\u{e821}',
            Self::HandbellsTableSingleBell => '\u{e820}',
            Self::HarpMetalRod => '\u{e68f}',
            Self::HarpPedalCentered => '\u{e681}',
            Self::HarpPedalDivider => '\u{e683}',
            Self::HarpPedalLowered => '\u{e682}',
            Self::HarpPedalRaised => '\u{e680}',
            Self::HarpSalzedoAeolianAscending => '\u{e695}',
            Self::HarpSalzedoAeolianDescending => '\u{e696}',
            Self::HarpSalzedoDampAbove => '\u{e69a}',
            Self::HarpSalzedoDampBelow => '\u{e699}',
            Self::HarpSalzedoDampBothHands => '\u{e698}',
            Self::HarpSalzedoDampLowStrings => '\u{e697}',
            Self::HarpSalzedoFluidicSoundsLeft => '\u{e68d}',
            Self::HarpSalzedoFluidicSoundsRight => '\u{e68e}',
            Self::HarpSalzedoIsolatedSounds => '\u{e69c}',
            Self::HarpSalzedoMetallicSounds => '\u{e688}',
            Self::HarpSalzedoMetallicSoundsOneString => '\u{e69b}',
            Self::HarpSalzedoMuffleTotally => '\u{e68c}',
            Self::HarpSalzedoOboicFlux => '\u{e685}',
            Self::HarpSalzedoPlayUpperEnd => '\u{e68a}',
            Self::HarpSalzedoSlideWithSuppleness => '\u{e684}',
            Self::HarpSalzedoSnareDrum => '\u{e69d}',
            Self::HarpSalzedoTamTamSounds => '\u{e689}',
            Self::HarpSalzedoThunderEffect => '\u{e686}',
            Self::HarpSalzedoTimpanicSounds => '\u{e68b}',
            Self::HarpSalzedoWhistlingSounds => '\u{e687}',
            Self::HarpStringNoiseStem => '\u{e694}',
            Self::HarpTuningKey => '\u{e690}',
            Self::HarpTuningKeyGlissando => '\u{e693}',
            Self::HarpTuningKeyHandle => '\u{e691}',
            Self::HarpTuningKeyShank => '\u{e692}',
            Self::IndianDrumClef => '\u{ed70}',
            Self::KahnBackChug => '\u{ede2}',
            Self::KahnBackFlap => '\u{edd8}',
            Self::KahnBackRiff => '\u{ede1}',
            Self::KahnBackRip => '\u{edda}',
            Self::KahnBallChange => '\u{edc6}',
            Self::KahnBallDig => '\u{edcd}',
            Self::KahnBrushBackward => '\u{eda7}',
            Self::KahnBrushForward => '\u{eda6}',
            Self::KahnChug => '\u{eddd}',
            Self::KahnClap => '\u{edb8}',
            Self::KahnDoubleSnap => '\u{edba}',
            Self::KahnDoubleWing => '\u{edeb}',
            Self::KahnDrawStep => '\u{edb2}',
            Self::KahnDrawTap => '\u{edb3}',
            Self::KahnFlam => '\u{edcf}',
            Self::KahnFlap => '\u{edd5}',
            Self::KahnFlapStep => '\u{edd7}',
            Self::KahnFlat => '\u{eda9}',
            Self::KahnFleaHop => '\u{edb0}',
            Self::KahnFleaTap => '\u{edb1}',
            Self::KahnGraceTap => '\u{eda8}',
            Self::KahnGraceTapChange => '\u{edd1}',
            Self::KahnGraceTapHop => '\u{edd0}',
            Self::KahnGraceTapStamp => '\u{edd3}',
            Self::KahnHeel => '\u{edaa}',
            Self::KahnHeelChange => '\u{edc9}',
            Self::KahnHeelClick => '\u{edbb}',
            Self::KahnHeelDrop => '\u{edb6}',
            Self::KahnHeelStep => '\u{edc4}',
            Self::KahnHeelTap => '\u{edcb}',
            Self::KahnHop => '\u{eda2}',
            Self::KahnJumpApart => '\u{eda5}',
            Self::KahnJumpTogether => '\u{eda4}',
            Self::KahnKneeInward => '\u{edad}',
            Self::KahnKneeOutward => '\u{edac}',
            Self::KahnLeap => '\u{eda3}',
            Self::KahnLeapFlatFoot => '\u{edd2}',
            Self::KahnLeapHeelClick => '\u{edd4}',
            Self::KahnLeftCatch => '\u{edbf}',
            Self::KahnLeftCross => '\u{edbd}',
            Self::KahnLeftFoot => '\u{edee}',
            Self::KahnLeftToeStrike => '\u{edc1}',
            Self::KahnLeftTurn => '\u{edf0}',
            Self::KahnOverTheTop => '\u{edec}',
            Self::KahnOverTheTopTap => '\u{eded}',
            Self::KahnPull => '\u{ede3}',
            Self::KahnPush => '\u{edde}',
            Self::KahnRiff => '\u{ede0}',
            Self::KahnRiffle => '\u{ede7}',
            Self::KahnRightCatch => '\u{edc0}',
            Self::KahnRightCross => '\u{edbe}',
            Self::KahnRightFoot => '\u{edef}',
            Self::KahnRightToeStrike => '\u{edc2}',
            Self::KahnRightTurn => '\u{edf1}',
            Self::KahnRip => '\u{edd6}',
            Self::KahnRipple => '\u{ede8}',
            Self::KahnScrape => '\u{edae}',
            Self::KahnScuff => '\u{eddc}',
            Self::KahnScuffle => '\u{ede6}',
            Self::KahnShuffle => '\u{ede5}',
            Self::KahnSlam => '\u{edce}',
            Self::KahnSlap => '\u{edd9}',
            Self::KahnSlideStep => '\u{edb4}',
            Self::KahnSlideTap => '\u{edb5}',
            Self::KahnSnap => '\u{edb9}',
            Self::KahnStamp => '\u{edc3}',
            Self::KahnStampStamp => '\u{edc8}',
            Self::KahnStep => '\u{eda0}',
            Self::KahnStepStamp => '\u{edc7}',
            Self::KahnStomp => '\u{edca}',
            Self::KahnStompBrush => '\u{eddb}',
            Self::KahnTap => '\u{eda1}',
            Self::KahnToe => '\u{edab}',
            Self::KahnToeClick => '\u{edbc}',
            Self::KahnToeDrop => '\u{edb7}',
            Self::KahnToeStep => '\u{edc5}',
            Self::KahnToeTap => '\u{edcc}',
            Self::KahnTrench => '\u{edaf}',
            Self::KahnWing => '\u{ede9}',
            Self::KahnWingChange => '\u{edea}',
            Self::KahnZank => '\u{ede4}',
            Self::KahnZink => '\u{eddf}',
            Self::KeyboardBebung2DotsAbove => '\u{e668}',
            Self::KeyboardBebung2DotsBelow => '\u{e669}',
            Self::KeyboardBebung3DotsAbove => '\u{e66a}',
            Self::KeyboardBebung3DotsBelow => '\u{e66b}',
            Self::KeyboardBebung4DotsAbove => '\u{e66c}',
            Self::KeyboardBebung4DotsBelow => '\u{e66d}',
            Self::KeyboardLeftPedalPictogram => '\u{e65e}',
            Self::KeyboardMiddlePedalPictogram => '\u{e65f}',
            Self::KeyboardPedalD => '\u{e653}',
            Self::KeyboardPedalDot => '\u{e654}',
            Self::KeyboardPedalE => '\u{e652}',
            Self::KeyboardPedalHalf => '\u{e656}',
            Self::KeyboardPedalHalf2 => '\u{e65b}',
            Self::KeyboardPedalHalf3 => '\u{e65c}',
            Self::KeyboardPedalHeel1 => '\u{e661}',
            Self::KeyboardPedalHeel2 => '\u{e662}',
            Self::KeyboardPedalHeel3 => '\u{e663}',
            Self::KeyboardPedalHeelToToe => '\u{e674}',
            Self::KeyboardPedalHeelToe => '\u{e666}',
            Self::KeyboardPedalHookEnd => '\u{e673}',
            Self::KeyboardPedalHookStart => '\u{e672}',
            Self::KeyboardPedalHyphen => '\u{e658}',
            Self::KeyboardPedalP => '\u{e651}',
            Self::KeyboardPedalParensLeft => '\u{e676}',
            Self::KeyboardPedalParensRight => '\u{e677}',
            Self::KeyboardPedalPed => '\u{e650}',
            Self::KeyboardPedalS => '\u{e65a}',
            Self::KeyboardPedalSost => '\u{e659}',
            Self::KeyboardPedalToe1 => '\u{e664}',
            Self::KeyboardPedalToe2 => '\u{e665}',
            Self::KeyboardPedalToeToHeel => '\u{e675}',
            Self::KeyboardPedalUp => '\u{e655}',
            Self::KeyboardPedalUpNotch => '\u{e657}',
            Self::KeyboardPedalUpSpecial => '\u{e65d}',
            Self::KeyboardPlayWithLh => '\u{e670}',
            Self::KeyboardPlayWithLhEnd => '\u{e671}',
            Self::KeyboardPlayWithRh => '\u{e66e}',
            Self::KeyboardPlayWithRhEnd => '\u{e66f}',
            Self::KeyboardPluckInside => '\u{e667}',
            Self::KeyboardRightPedalPictogram => '\u{e660}',
            Self::KievanAccidentalFlat => '\u{ec3e}',
            Self::KievanAccidentalSharp => '\u{ec3d}',
            Self::KievanAugmentationDot => '\u{ec3c}',
            Self::KievanCClef => '\u{ec30}',
            Self::KievanEndingSymbol => '\u{ec31}',
            Self::KievanNote8ThStemDown => '\u{ec3a}',
            Self::KievanNote8ThStemUp => '\u{ec39}',
            Self::KievanNoteBeam => '\u{ec3b}',
            Self::KievanNoteHalfStaffLine => '\u{ec35}',
            Self::KievanNoteHalfStaffSpace => '\u{ec36}',
            Self::KievanNoteQuarterStemDown => '\u{ec38}',
            Self::KievanNoteQuarterStemUp => '\u{ec37}',
            Self::KievanNoteReciting => '\u{ec32}',
            Self::KievanNoteWhole => '\u{ec33}',
            Self::KievanNoteWholeFinal => '\u{ec34}',
            Self::KodalyHandDo => '\u{ec40}',
            Self::KodalyHandFa => '\u{ec43}',
            Self::KodalyHandLa => '\u{ec45}',
            Self::KodalyHandMi => '\u{ec42}',
            Self::KodalyHandRe => '\u{ec41}',
            Self::KodalyHandSo => '\u{ec44}',
            Self::KodalyHandTi => '\u{ec46}',
            Self::LeftRepeatSmall => '\u{e04c}',
            Self::LegerLine => '\u{e022}',
            Self::LegerLineNarrow => '\u{e024}',
            Self::LegerLineWide => '\u{e023}',
            Self::LuteBarlineEndRepeat => '\u{eba4}',
            Self::LuteBarlineFinal => '\u{eba5}',
            Self::LuteBarlineStartRepeat => '\u{eba3}',
            Self::LuteDuration16Th => '\u{ebab}',
            Self::LuteDuration32Nd => '\u{ebac}',
            Self::LuteDuration8Th => '\u{ebaa}',
            Self::LuteDurationDoubleWhole => '\u{eba6}',
            Self::LuteDurationHalf => '\u{eba8}',
            Self::LuteDurationQuarter => '\u{eba9}',
            Self::LuteDurationWhole => '\u{eba7}',
            Self::LuteFingeringRhFirst => '\u{ebae}',
            Self::LuteFingeringRhSecond => '\u{ebaf}',
            Self::LuteFingeringRhThird => '\u{ebb0}',
            Self::LuteFingeringRhThumb => '\u{ebad}',
            Self::LuteFrench10ThCourse => '\u{ebd0}',
            Self::LuteFrench7ThCourse => '\u{ebcd}',
            Self::LuteFrench8ThCourse => '\u{ebce}',
            Self::LuteFrench9ThCourse => '\u{ebcf}',
            Self::LuteFrenchAppoggiaturaAbove => '\u{ebd5}',
            Self::LuteFrenchAppoggiaturaBelow => '\u{ebd4}',
            Self::LuteFrenchFretA => '\u{ebc0}',
            Self::LuteFrenchFretB => '\u{ebc1}',
            Self::LuteFrenchFretC => '\u{ebc2}',
            Self::LuteFrenchFretD => '\u{ebc3}',
            Self::LuteFrenchFretE => '\u{ebc4}',
            Self::LuteFrenchFretF => '\u{ebc5}',
            Self::LuteFrenchFretG => '\u{ebc6}',
            Self::LuteFrenchFretH => '\u{ebc7}',
            Self::LuteFrenchFretI => '\u{ebc8}',
            Self::LuteFrenchFretK => '\u{ebc9}',
            Self::LuteFrenchFretL => '\u{ebca}',
            Self::LuteFrenchFretM => '\u{ebcb}',
            Self::LuteFrenchFretN => '\u{ebcc}',
            Self::LuteFrenchMordentInverted => '\u{ebd3}',
            Self::LuteFrenchMordentLower => '\u{ebd2}',
            Self::LuteFrenchMordentUpper => '\u{ebd1}',
            Self::LuteGermanALower => '\u{ec00}',
            Self::LuteGermanAUpper => '\u{ec17}',
            Self::LuteGermanBLower => '\u{ec01}',
            Self::LuteGermanBUpper => '\u{ec18}',
            Self::LuteGermanCLower => '\u{ec02}',
            Self::LuteGermanCUpper => '\u{ec19}',
            Self::LuteGermanDLower => '\u{ec03}',
            Self::LuteGermanDUpper => '\u{ec1a}',
            Self::LuteGermanELower => '\u{ec04}',
            Self::LuteGermanEUpper => '\u{ec1b}',
            Self::LuteGermanFLower => '\u{ec05}',
            Self::LuteGermanFUpper => '\u{ec1c}',
            Self::LuteGermanGLower => '\u{ec06}',
            Self::LuteGermanGUpper => '\u{ec1d}',
            Self::LuteGermanHLower => '\u{ec07}',
            Self::LuteGermanHUpper => '\u{ec1e}',
            Self::LuteGermanILower => '\u{ec08}',
            Self::LuteGermanIUpper => '\u{ec1f}',
            Self::LuteGermanKLower => '\u{ec09}',
            Self::LuteGermanKUpper => '\u{ec20}',
            Self::LuteGermanLLower => '\u{ec0a}',
            Self::LuteGermanLUpper => '\u{ec21}',
            Self::LuteGermanMLower => '\u{ec0b}',
            Self::LuteGermanMUpper => '\u{ec22}',
            Self::LuteGermanNLower => '\u{ec0c}',
            Self::LuteGermanNUpper => '\u{ec23}',
            Self::LuteGermanOLower => '\u{ec0d}',
            Self::LuteGermanPLower => '\u{ec0e}',
            Self::LuteGermanQLower => '\u{ec0f}',
            Self::LuteGermanRLower => '\u{ec10}',
            Self::LuteGermanSLower => '\u{ec11}',
            Self::LuteGermanTLower => '\u{ec12}',
            Self::LuteGermanVLower => '\u{ec13}',
            Self::LuteGermanXLower => '\u{ec14}',
            Self::LuteGermanYLower => '\u{ec15}',
            Self::LuteGermanZLower => '\u{ec16}',
            Self::LuteItalianClefCSolFaUt => '\u{ebf1}',
            Self::LuteItalianClefFFaUt => '\u{ebf0}',
            Self::LuteItalianFret0 => '\u{ebe0}',
            Self::LuteItalianFret1 => '\u{ebe1}',
            Self::LuteItalianFret2 => '\u{ebe2}',
            Self::LuteItalianFret3 => '\u{ebe3}',
            Self::LuteItalianFret4 => '\u{ebe4}',
            Self::LuteItalianFret5 => '\u{ebe5}',
            Self::LuteItalianFret6 => '\u{ebe6}',
            Self::LuteItalianFret7 => '\u{ebe7}',
            Self::LuteItalianFret8 => '\u{ebe8}',
            Self::LuteItalianFret9 => '\u{ebe9}',
            Self::LuteItalianHoldFinger => '\u{ebf4}',
            Self::LuteItalianHoldNote => '\u{ebf3}',
            Self::LuteItalianReleaseFinger => '\u{ebf5}',
            Self::LuteItalianTempoFast => '\u{ebea}',
            Self::LuteItalianTempoNeitherFastNorSlow => '\u{ebec}',
            Self::LuteItalianTempoSlow => '\u{ebed}',
            Self::LuteItalianTempoSomewhatFast => '\u{ebeb}',
            Self::LuteItalianTempoVerySlow => '\u{ebee}',
            Self::LuteItalianTimeTriple => '\u{ebef}',
            Self::LuteItalianTremolo => '\u{ebf2}',
            Self::LuteItalianVibrato => '\u{ebf6}',
            Self::LuteStaff6Lines => '\u{eba0}',
            Self::LuteStaff6LinesNarrow => '\u{eba2}',
            Self::LuteStaff6LinesWide => '\u{eba1}',
            Self::LyricsElision => '\u{e551}',
            Self::LyricsElisionNarrow => '\u{e550}',
            Self::LyricsElisionWide => '\u{e552}',
            Self::LyricsHyphenBaseline => '\u{e553}',
            Self::LyricsHyphenBaselineNonBreaking => '\u{e554}',
            Self::LyricsTextRepeat => '\u{e555}',
            Self::MedRenFlatHardB => '\u{e9e1}',
            Self::MedRenFlatSoftB => '\u{e9e0}',
            Self::MedRenFlatWithDot => '\u{e9e4}',
            Self::MedRenGClefCmn => '\u{ea24}',
            Self::MedRenLiquescenceCmn => '\u{ea22}',
            Self::MedRenLiquescentAscCmn => '\u{ea26}',
            Self::MedRenLiquescentDescCmn => '\u{ea27}',
            Self::MedRenNatural => '\u{e9e2}',
            Self::MedRenNaturalWithCross => '\u{e9e5}',
            Self::MedRenOriscusCmn => '\u{ea2a}',
            Self::MedRenPlicaCmn => '\u{ea23}',
            Self::MedRenPunctumCmn => '\u{ea25}',
            Self::MedRenQuilismaCmn => '\u{ea28}',
            Self::MedRenSharpCroix => '\u{e9e3}',
            Self::MedRenStrophicusCmn => '\u{ea29}',
            Self::MensuralAlterationSign => '\u{ea10}',
            Self::MensuralBlackBrevis => '\u{e952}',
            Self::MensuralBlackBrevisVoid => '\u{e956}',
            Self::MensuralBlackDragma => '\u{e95a}',
            Self::MensuralBlackLonga => '\u{e951}',
            Self::MensuralBlackMaxima => '\u{e950}',
            Self::MensuralBlackMinima => '\u{e954}',
            Self::MensuralBlackMinimaVoid => '\u{e958}',
            Self::MensuralBlackSemibrevis => '\u{e953}',
            Self::MensuralBlackSemibrevisCaudata => '\u{e959}',
            Self::MensuralBlackSemibrevisOblique => '\u{e95b}',
            Self::MensuralBlackSemibrevisVoid => '\u{e957}',
            Self::MensuralBlackSemiminima => '\u{e955}',
            Self::MensuralCclef => '\u{e905}',
            Self::MensuralCclefPetrucciPosHigh => '\u{e90a}',
            Self::MensuralCclefPetrucciPosHighest => '\u{e90b}',
            Self::MensuralCclefPetrucciPosLow => '\u{e908}',
            Self::MensuralCclefPetrucciPosLowest => '\u{e907}',
            Self::MensuralCclefPetrucciPosMiddle => '\u{e909}',
            Self::MensuralColorationEndRound => '\u{ea0f}',
            Self::MensuralColorationEndSquare => '\u{ea0d}',
            Self::MensuralColorationStartRound => '\u{ea0e}',
            Self::MensuralColorationStartSquare => '\u{ea0c}',
            Self::MensuralCombStemDiagonal => '\u{e940}',
            Self::MensuralCombStemDown => '\u{e93f}',
            Self::MensuralCombStemDownFlagExtended => '\u{e948}',
            Self::MensuralCombStemDownFlagFlared => '\u{e946}',
            Self::MensuralCombStemDownFlagFusa => '\u{e94c}',
            Self::MensuralCombStemDownFlagLeft => '\u{e944}',
            Self::MensuralCombStemDownFlagRight => '\u{e942}',
            Self::MensuralCombStemDownFlagSemiminima => '\u{e94a}',
            Self::MensuralCombStemUp => '\u{e93e}',
            Self::MensuralCombStemUpFlagExtended => '\u{e947}',
            Self::MensuralCombStemUpFlagFlared => '\u{e945}',
            Self::MensuralCombStemUpFlagFusa => '\u{e94b}',
            Self::MensuralCombStemUpFlagLeft => '\u{e943}',
            Self::MensuralCombStemUpFlagRight => '\u{e941}',
            Self::MensuralCombStemUpFlagSemiminima => '\u{e949}',
            Self::MensuralCustosCheckmark => '\u{ea0a}',
            Self::MensuralCustosDown => '\u{ea03}',
            Self::MensuralCustosTurn => '\u{ea0b}',
            Self::MensuralCustosUp => '\u{ea02}',
            Self::MensuralFclef => '\u{e903}',
            Self::MensuralFclefPetrucci => '\u{e904}',
            Self::MensuralGclef => '\u{e900}',
            Self::MensuralGclefPetrucci => '\u{e901}',
            Self::MensuralModusImperfectumVert => '\u{e92d}',
            Self::MensuralModusPerfectumVert => '\u{e92c}',
            Self::MensuralNoteheadLongaBlack => '\u{e934}',
            Self::MensuralNoteheadLongaBlackVoid => '\u{e936}',
            Self::MensuralNoteheadLongaVoid => '\u{e935}',
            Self::MensuralNoteheadLongaWhite => '\u{e937}',
            Self::MensuralNoteheadMaximaBlack => '\u{e930}',
            Self::MensuralNoteheadMaximaBlackVoid => '\u{e932}',
            Self::MensuralNoteheadMaximaVoid => '\u{e931}',
            Self::MensuralNoteheadMaximaWhite => '\u{e933}',
            Self::MensuralNoteheadMinimaWhite => '\u{e93c}',
            Self::MensuralNoteheadSemibrevisBlack => '\u{e938}',
            Self::MensuralNoteheadSemibrevisBlackVoid => '\u{e93a}',
            Self::MensuralNoteheadSemibrevisBlackVoidTurned => '\u{e93b}',
            Self::MensuralNoteheadSemibrevisVoid => '\u{e939}',
            Self::MensuralNoteheadSemiminimaWhite => '\u{e93d}',
            Self::MensuralObliqueAsc2NdBlack => '\u{e970}',
            Self::MensuralObliqueAsc2NdBlackVoid => '\u{e972}',
            Self::MensuralObliqueAsc2NdVoid => '\u{e971}',
            Self::MensuralObliqueAsc2NdWhite => '\u{e973}',
            Self::MensuralObliqueAsc3RdBlack => '\u{e974}',
            Self::MensuralObliqueAsc3RdBlackVoid => '\u{e976}',
            Self::MensuralObliqueAsc3RdVoid => '\u{e975}',
            Self::MensuralObliqueAsc3RdWhite => '\u{e977}',
            Self::MensuralObliqueAsc4ThBlack => '\u{e978}',
            Self::MensuralObliqueAsc4ThBlackVoid => '\u{e97a}',
            Self::MensuralObliqueAsc4ThVoid => '\u{e979}',
            Self::MensuralObliqueAsc4ThWhite => '\u{e97b}',
            Self::MensuralObliqueAsc5ThBlack => '\u{e97c}',
            Self::MensuralObliqueAsc5ThBlackVoid => '\u{e97e}',
            Self::MensuralObliqueAsc5ThVoid => '\u{e97d}',
            Self::MensuralObliqueAsc5ThWhite => '\u{e97f}',
            Self::MensuralObliqueDesc2NdBlack => '\u{e980}',
            Self::MensuralObliqueDesc2NdBlackVoid => '\u{e982}',
            Self::MensuralObliqueDesc2NdVoid => '\u{e981}',
            Self::MensuralObliqueDesc2NdWhite => '\u{e983}',
            Self::MensuralObliqueDesc3RdBlack => '\u{e984}',
            Self::MensuralObliqueDesc3RdBlackVoid => '\u{e986}',
            Self::MensuralObliqueDesc3RdVoid => '\u{e985}',
            Self::MensuralObliqueDesc3RdWhite => '\u{e987}',
            Self::MensuralObliqueDesc4ThBlack => '\u{e988}',
            Self::MensuralObliqueDesc4ThBlackVoid => '\u{e98a}',
            Self::MensuralObliqueDesc4ThVoid => '\u{e989}',
            Self::MensuralObliqueDesc4ThWhite => '\u{e98b}',
            Self::MensuralObliqueDesc5ThBlack => '\u{e98c}',
            Self::MensuralObliqueDesc5ThBlackVoid => '\u{e98e}',
            Self::MensuralObliqueDesc5ThVoid => '\u{e98d}',
            Self::MensuralObliqueDesc5ThWhite => '\u{e98f}',
            Self::MensuralProlation1 => '\u{e910}',
            Self::MensuralProlation10 => '\u{e919}',
            Self::MensuralProlation11 => '\u{e91a}',
            Self::MensuralProlation2 => '\u{e911}',
            Self::MensuralProlation3 => '\u{e912}',
            Self::MensuralProlation4 => '\u{e913}',
            Self::MensuralProlation5 => '\u{e914}',
            Self::MensuralProlation6 => '\u{e915}',
            Self::MensuralProlation7 => '\u{e916}',
            Self::MensuralProlation8 => '\u{e917}',
            Self::MensuralProlation9 => '\u{e918}',
            Self::MensuralProlationCombiningDot => '\u{e920}',
            Self::MensuralProlationCombiningDotVoid => '\u{e924}',
            Self::MensuralProlationCombiningStroke => '\u{e925}',
            Self::MensuralProlationCombiningThreeDots => '\u{e922}',
            Self::MensuralProlationCombiningThreeDotsTri => '\u{e923}',
            Self::MensuralProlationCombiningTwoDots => '\u{e921}',
            Self::MensuralProportion1 => '\u{e926}',
            Self::MensuralProportion2 => '\u{e927}',
            Self::MensuralProportion3 => '\u{e928}',
            Self::MensuralProportion4 => '\u{e929}',
            Self::MensuralProportion5 => '\u{ee90}',
            Self::MensuralProportion6 => '\u{ee91}',
            Self::MensuralProportion7 => '\u{ee92}',
            Self::MensuralProportion8 => '\u{ee93}',
            Self::MensuralProportion9 => '\u{ee94}',
            Self::MensuralProportionMajor => '\u{e92b}',
            Self::MensuralProportionMinor => '\u{e92a}',
            Self::MensuralProportionProportioDupla1 => '\u{e91c}',
            Self::MensuralProportionProportioDupla2 => '\u{e91d}',
            Self::MensuralProportionProportioQuadrupla => '\u{e91f}',
            Self::MensuralProportionProportioTripla => '\u{e91e}',
            Self::MensuralProportionTempusPerfectum => '\u{e91b}',
            Self::MensuralRestBrevis => '\u{e9f3}',
            Self::MensuralRestFusa => '\u{e9f7}',
            Self::MensuralRestLongaImperfecta => '\u{e9f2}',
            Self::MensuralRestLongaPerfecta => '\u{e9f1}',
            Self::MensuralRestMaxima => '\u{e9f0}',
            Self::MensuralRestMinima => '\u{e9f5}',
            Self::MensuralRestSemibrevis => '\u{e9f4}',
            Self::MensuralRestSemifusa => '\u{e9f8}',
            Self::MensuralRestSemiminima => '\u{e9f6}',
            Self::MensuralSignumDown => '\u{ea01}',
            Self::MensuralSignumUp => '\u{ea00}',
            Self::MensuralTempusImperfectumHoriz => '\u{e92f}',
            Self::MensuralTempusPerfectumHoriz => '\u{e92e}',
            Self::MensuralWhiteBrevis => '\u{e95e}',
            Self::MensuralWhiteFusa => '\u{e961}',
            Self::MensuralWhiteLonga => '\u{e95d}',
            Self::MensuralWhiteMaxima => '\u{e95c}',
            Self::MensuralWhiteMinima => '\u{e95f}',
            Self::MensuralWhiteSemibrevis => '\u{e962}',
            Self::MensuralWhiteSemiminima => '\u{e960}',
            Self::MetAugmentationDot => '\u{ecb7}',
            Self::MetNote1024ThDown => '\u{ecb6}',
            Self::MetNote1024ThUp => '\u{ecb5}',
            Self::MetNote128ThDown => '\u{ecb0}',
            Self::MetNote128ThUp => '\u{ecaf}',
            Self::MetNote16ThDown => '\u{ecaa}',
            Self::MetNote16ThUp => '\u{eca9}',
            Self::MetNote256ThDown => '\u{ecb2}',
            Self::MetNote256ThUp => '\u{ecb1}',
            Self::MetNote32NdDown => '\u{ecac}',
            Self::MetNote32NdUp => '\u{ecab}',
            Self::MetNote512ThDown => '\u{ecb4}',
            Self::MetNote512ThUp => '\u{ecb3}',
            Self::MetNote64ThDown => '\u{ecae}',
            Self::MetNote64ThUp => '\u{ecad}',
            Self::MetNote8ThDown => '\u{eca8}',
            Self::MetNote8ThUp => '\u{eca7}',
            Self::MetNoteDoubleWhole => '\u{eca0}',
            Self::MetNoteDoubleWholeSquare => '\u{eca1}',
            Self::MetNoteHalfDown => '\u{eca4}',
            Self::MetNoteHalfUp => '\u{eca3}',
            Self::MetNoteQuarterDown => '\u{eca6}',
            Self::MetNoteQuarterUp => '\u{eca5}',
            Self::MetNoteWhole => '\u{eca2}',
            Self::MetricModulationArrowLeft => '\u{ec63}',
            Self::MetricModulationArrowRight => '\u{ec64}',
            Self::MiscDoNotCopy => '\u{ec61}',
            Self::MiscDoNotPhotocopy => '\u{ec60}',
            Self::MiscEyeglasses => '\u{ec62}',
            Self::Note1024ThDown => '\u{e1e6}',
            Self::Note1024ThUp => '\u{e1e5}',
            Self::Note128ThDown => '\u{e1e0}',
            Self::Note128ThUp => '\u{e1df}',
            Self::Note16ThDown => '\u{e1da}',
            Self::Note16ThUp => '\u{e1d9}',
            Self::Note256ThDown => '\u{e1e2}',
            Self::Note256ThUp => '\u{e1e1}',
            Self::Note32NdDown => '\u{e1dc}',
            Self::Note32NdUp => '\u{e1db}',
            Self::Note512ThDown => '\u{e1e4}',
            Self::Note512ThUp => '\u{e1e3}',
            Self::Note64ThDown => '\u{e1de}',
            Self::Note64ThUp => '\u{e1dd}',
            Self::Note8ThDown => '\u{e1d8}',
            Self::Note8ThUp => '\u{e1d7}',
            Self::NoteABlack => '\u{e197}',
            Self::NoteAFlatBlack => '\u{e196}',
            Self::NoteAFlatHalf => '\u{e17f}',
            Self::NoteAFlatWhole => '\u{e168}',
            Self::NoteAHalf => '\u{e180}',
            Self::NoteASharpBlack => '\u{e198}',
            Self::NoteASharpHalf => '\u{e181}',
            Self::NoteASharpWhole => '\u{e16a}',
            Self::NoteAWhole => '\u{e169}',
            Self::NoteBBlack => '\u{e19a}',
            Self::NoteBFlatBlack => '\u{e199}',
            Self::NoteBFlatHalf => '\u{e182}',
            Self::NoteBFlatWhole => '\u{e16b}',
            Self::NoteBHalf => '\u{e183}',
            Self::NoteBSharpBlack => '\u{e19b}',
            Self::NoteBSharpHalf => '\u{e184}',
            Self::NoteBSharpWhole => '\u{e16d}',
            Self::NoteBWhole => '\u{e16c}',
            Self::NoteCBlack => '\u{e19d}',
            Self::NoteCFlatBlack => '\u{e19c}',
            Self::NoteCFlatHalf => '\u{e185}',
            Self::NoteCFlatWhole => '\u{e16e}',
            Self::NoteCHalf => '\u{e186}',
            Self::NoteCSharpBlack => '\u{e19e}',
            Self::NoteCSharpHalf => '\u{e187}',
            Self::NoteCSharpWhole => '\u{e170}',
            Self::NoteCWhole => '\u{e16f}',
            Self::NoteDBlack => '\u{e1a0}',
            Self::NoteDFlatBlack => '\u{e19f}',
            Self::NoteDFlatHalf => '\u{e188}',
            Self::NoteDFlatWhole => '\u{e171}',
            Self::NoteDHalf => '\u{e189}',
            Self::NoteDSharpBlack => '\u{e1a1}',
            Self::NoteDSharpHalf => '\u{e18a}',
            Self::NoteDSharpWhole => '\u{e173}',
            Self::NoteDWhole => '\u{e172}',
            Self::NoteDiBlack => '\u{eef2}',
            Self::NoteDiHalf => '\u{eee9}',
            Self::NoteDiWhole => '\u{eee0}',
            Self::NoteDoBlack => '\u{e160}',
            Self::NoteDoHalf => '\u{e158}',
            Self::NoteDoWhole => '\u{e150}',
            Self::NoteDoubleWhole => '\u{e1d0}',
            Self::NoteDoubleWholeSquare => '\u{e1d1}',
            Self::NoteEBlack => '\u{e1a3}',
            Self::NoteEFlatBlack => '\u{e1a2}',
            Self::NoteEFlatHalf => '\u{e18b}',
            Self::NoteEFlatWhole => '\u{e174}',
            Self::NoteEHalf => '\u{e18c}',
            Self::NoteESharpBlack => '\u{e1a4}',
            Self::NoteESharpHalf => '\u{e18d}',
            Self::NoteESharpWhole => '\u{e176}',
            Self::NoteEWhole => '\u{e175}',
            Self::NoteEmptyBlack => '\u{e1af}',
            Self::NoteEmptyHalf => '\u{e1ae}',
            Self::NoteEmptyWhole => '\u{e1ad}',
            Self::NoteFBlack => '\u{e1a6}',
            Self::NoteFFlatBlack => '\u{e1a5}',
            Self::NoteFFlatHalf => '\u{e18e}',
            Self::NoteFFlatWhole => '\u{e177}',
            Self::NoteFHalf => '\u{e18f}',
            Self::NoteFSharpBlack => '\u{e1a7}',
            Self::NoteFSharpHalf => '\u{e190}',
            Self::NoteFSharpWhole => '\u{e179}',
            Self::NoteFWhole => '\u{e178}',
            Self::NoteFaBlack => '\u{e163}',
            Self::NoteFaHalf => '\u{e15b}',
            Self::NoteFaWhole => '\u{e153}',
            Self::NoteFiBlack => '\u{eef6}',
            Self::NoteFiHalf => '\u{eeed}',
            Self::NoteFiWhole => '\u{eee4}',
            Self::NoteGBlack => '\u{e1a9}',
            Self::NoteGFlatBlack => '\u{e1a8}',
            Self::NoteGFlatHalf => '\u{e191}',
            Self::NoteGFlatWhole => '\u{e17a}',
            Self::NoteGHalf => '\u{e192}',
            Self::NoteGSharpBlack => '\u{e1aa}',
            Self::NoteGSharpHalf => '\u{e193}',
            Self::NoteGSharpWhole => '\u{e17c}',
            Self::NoteGWhole => '\u{e17b}',
            Self::NoteHBlack => '\u{e1ab}',
            Self::NoteHHalf => '\u{e194}',
            Self::NoteHSharpBlack => '\u{e1ac}',
            Self::NoteHSharpHalf => '\u{e195}',
            Self::NoteHSharpWhole => '\u{e17e}',
            Self::NoteHWhole => '\u{e17d}',
            Self::NoteHalfDown => '\u{e1d4}',
            Self::NoteHalfUp => '\u{e1d3}',
            Self::NoteLaBlack => '\u{e165}',
            Self::NoteLaHalf => '\u{e15d}',
            Self::NoteLaWhole => '\u{e155}',
            Self::NoteLeBlack => '\u{eef9}',
            Self::NoteLeHalf => '\u{eef0}',
            Self::NoteLeWhole => '\u{eee7}',
            Self::NoteLiBlack => '\u{eef8}',
            Self::NoteLiHalf => '\u{eeef}',
            Self::NoteLiWhole => '\u{eee6}',
            Self::NoteMeBlack => '\u{eef5}',
            Self::NoteMeHalf => '\u{eeec}',
            Self::NoteMeWhole => '\u{eee3}',
            Self::NoteMiBlack => '\u{e162}',
            Self::NoteMiHalf => '\u{e15a}',
            Self::NoteMiWhole => '\u{e152}',
            Self::NoteQuarterDown => '\u{e1d6}',
            Self::NoteQuarterUp => '\u{e1d5}',
            Self::NoteRaBlack => '\u{eef4}',
            Self::NoteRaHalf => '\u{eeeb}',
            Self::NoteRaWhole => '\u{eee2}',
            Self::NoteReBlack => '\u{e161}',
            Self::NoteReHalf => '\u{e159}',
            Self::NoteReWhole => '\u{e151}',
            Self::NoteRiBlack => '\u{eef3}',
            Self::NoteRiHalf => '\u{eeea}',
            Self::NoteRiWhole => '\u{eee1}',
            Self::NoteSeBlack => '\u{eef7}',
            Self::NoteSeHalf => '\u{eeee}',
            Self::NoteSeWhole => '\u{eee5}',
            Self::NoteShapeArrowheadLeftBlack => '\u{e1c9}',
            Self::NoteShapeArrowheadLeftDoubleWhole => '\u{ecdc}',
            Self::NoteShapeArrowheadLeftWhite => '\u{e1c8}',
            Self::NoteShapeDiamondBlack => '\u{e1b9}',
            Self::NoteShapeDiamondDoubleWhole => '\u{ecd4}',
            Self::NoteShapeDiamondWhite => '\u{e1b8}',
            Self::NoteShapeIsoscelesTriangleBlack => '\u{e1c5}',
            Self::NoteShapeIsoscelesTriangleDoubleWhole => '\u{ecda}',
            Self::NoteShapeIsoscelesTriangleWhite => '\u{e1c4}',
            Self::NoteShapeKeystoneBlack => '\u{e1c1}',
            Self::NoteShapeKeystoneDoubleWhole => '\u{ecd8}',
            Self::NoteShapeKeystoneWhite => '\u{e1c0}',
            Self::NoteShapeMoonBlack => '\u{e1bd}',
            Self::NoteShapeMoonDoubleWhole => '\u{ecd6}',
            Self::NoteShapeMoonLeftBlack => '\u{e1c7}',
            Self::NoteShapeMoonLeftDoubleWhole => '\u{ecdb}',
            Self::NoteShapeMoonLeftWhite => '\u{e1c6}',
            Self::NoteShapeMoonWhite => '\u{e1bc}',
            Self::NoteShapeQuarterMoonBlack => '\u{e1c3}',
            Self::NoteShapeQuarterMoonDoubleWhole => '\u{ecd9}',
            Self::NoteShapeQuarterMoonWhite => '\u{e1c2}',
            Self::NoteShapeRoundBlack => '\u{e1b1}',
            Self::NoteShapeRoundDoubleWhole => '\u{ecd0}',
            Self::NoteShapeRoundWhite => '\u{e1b0}',
            Self::NoteShapeSquareBlack => '\u{e1b3}',
            Self::NoteShapeSquareDoubleWhole => '\u{ecd1}',
            Self::NoteShapeSquareWhite => '\u{e1b2}',
            Self::NoteShapeTriangleLeftBlack => '\u{e1b7}',
            Self::NoteShapeTriangleLeftDoubleWhole => '\u{ecd3}',
            Self::NoteShapeTriangleLeftWhite => '\u{e1b6}',
            Self::NoteShapeTriangleRightBlack => '\u{e1b5}',
            Self::NoteShapeTriangleRightDoubleWhole => '\u{ecd2}',
            Self::NoteShapeTriangleRightWhite => '\u{e1b4}',
            Self::NoteShapeTriangleRoundBlack => '\u{e1bf}',
            Self::NoteShapeTriangleRoundDoubleWhole => '\u{ecd7}',
            Self::NoteShapeTriangleRoundLeftBlack => '\u{e1cb}',
            Self::NoteShapeTriangleRoundLeftDoubleWhole => '\u{ecdd}',
            Self::NoteShapeTriangleRoundLeftWhite => '\u{e1ca}',
            Self::NoteShapeTriangleRoundWhite => '\u{e1be}',
            Self::NoteShapeTriangleUpBlack => '\u{e1bb}',
            Self::NoteShapeTriangleUpDoubleWhole => '\u{ecd5}',
            Self::NoteShapeTriangleUpWhite => '\u{e1ba}',
            Self::NoteSiBlack => '\u{e167}',
            Self::NoteSiHalf => '\u{e15f}',
            Self::NoteSiWhole => '\u{e157}',
            Self::NoteSoBlack => '\u{e164}',
            Self::NoteSoHalf => '\u{e15c}',
            Self::NoteSoWhole => '\u{e154}',
            Self::NoteTeBlack => '\u{eefa}',
            Self::NoteTeHalf => '\u{eef1}',
            Self::NoteTeWhole => '\u{eee8}',
            Self::NoteTiBlack => '\u{e166}',
            Self::NoteTiHalf => '\u{e15e}',
            Self::NoteTiWhole => '\u{e156}',
            Self::NoteWhole => '\u{e1d2}',
            Self::NoteheadBlack => '\u{e0a4}',
            Self::NoteheadCircleSlash => '\u{e0f7}',
            Self::NoteheadCircleX => '\u{e0b3}',
            Self::NoteheadCircleXDoubleWhole => '\u{e0b0}',
            Self::NoteheadCircleXHalf => '\u{e0b2}',
            Self::NoteheadCircleXWhole => '\u{e0b1}',
            Self::NoteheadCircledBlack => '\u{e0e4}',
            Self::NoteheadCircledBlackLarge => '\u{e0e8}',
            Self::NoteheadCircledDoubleWhole => '\u{e0e7}',
            Self::NoteheadCircledDoubleWholeLarge => '\u{e0eb}',
            Self::NoteheadCircledHalf => '\u{e0e5}',
            Self::NoteheadCircledHalfLarge => '\u{e0e9}',
            Self::NoteheadCircledWhole => '\u{e0e6}',
            Self::NoteheadCircledWholeLarge => '\u{e0ea}',
            Self::NoteheadCircledXLarge => '\u{e0ec}',
            Self::NoteheadClusterDoubleWhole2Nd => '\u{e124}',
            Self::NoteheadClusterDoubleWhole3Rd => '\u{e128}',
            Self::NoteheadClusterDoubleWholeBottom => '\u{e12e}',
            Self::NoteheadClusterDoubleWholeMiddle => '\u{e12d}',
            Self::NoteheadClusterDoubleWholeTop => '\u{e12c}',
            Self::NoteheadClusterHalf2Nd => '\u{e126}',
            Self::NoteheadClusterHalf3Rd => '\u{e12a}',
            Self::NoteheadClusterHalfBottom => '\u{e134}',
            Self::NoteheadClusterHalfMiddle => '\u{e133}',
            Self::NoteheadClusterHalfTop => '\u{e132}',
            Self::NoteheadClusterQuarter2Nd => '\u{e127}',
            Self::NoteheadClusterQuarter3Rd => '\u{e12b}',
            Self::NoteheadClusterQuarterBottom => '\u{e137}',
            Self::NoteheadClusterQuarterMiddle => '\u{e136}',
            Self::NoteheadClusterQuarterTop => '\u{e135}',
            Self::NoteheadClusterRoundBlack => '\u{e123}',
            Self::NoteheadClusterRoundWhite => '\u{e122}',
            Self::NoteheadClusterSquareBlack => '\u{e121}',
            Self::NoteheadClusterSquareWhite => '\u{e120}',
            Self::NoteheadClusterWhole2Nd => '\u{e125}',
            Self::NoteheadClusterWhole3Rd => '\u{e129}',
            Self::NoteheadClusterWholeBottom => '\u{e131}',
            Self::NoteheadClusterWholeMiddle => '\u{e130}',
            Self::NoteheadClusterWholeTop => '\u{e12f}',
            Self::NoteheadCowellEleventhNoteSeriesHalf => '\u{eeae}',
            Self::NoteheadCowellEleventhNoteSeriesWhole => '\u{eead}',
            Self::NoteheadCowellEleventhSeriesBlack => '\u{eeaf}',
            Self::NoteheadCowellFifteenthNoteSeriesBlack => '\u{eeb5}',
            Self::NoteheadCowellFifteenthNoteSeriesHalf => '\u{eeb4}',
            Self::NoteheadCowellFifteenthNoteSeriesWhole => '\u{eeb3}',
            Self::NoteheadCowellFifthNoteSeriesBlack => '\u{eea6}',
            Self::NoteheadCowellFifthNoteSeriesHalf => '\u{eea5}',
            Self::NoteheadCowellFifthNoteSeriesWhole => '\u{eea4}',
            Self::NoteheadCowellNinthNoteSeriesBlack => '\u{eeac}',
            Self::NoteheadCowellNinthNoteSeriesHalf => '\u{eeab}',
            Self::NoteheadCowellNinthNoteSeriesWhole => '\u{eeaa}',
            Self::NoteheadCowellSeventhNoteSeriesBlack => '\u{eea9}',
            Self::NoteheadCowellSeventhNoteSeriesHalf => '\u{eea8}',
            Self::NoteheadCowellSeventhNoteSeriesWhole => '\u{eea7}',
            Self::NoteheadCowellThirdNoteSeriesBlack => '\u{eea3}',
            Self::NoteheadCowellThirdNoteSeriesHalf => '\u{eea2}',
            Self::NoteheadCowellThirdNoteSeriesWhole => '\u{eea1}',
            Self::NoteheadCowellThirteenthNoteSeriesBlack => '\u{eeb2}',
            Self::NoteheadCowellThirteenthNoteSeriesHalf => '\u{eeb1}',
            Self::NoteheadCowellThirteenthNoteSeriesWhole => '\u{eeb0}',
            Self::NoteheadDiamondBlack => '\u{e0db}',
            Self::NoteheadDiamondBlackOld => '\u{e0e2}',
            Self::NoteheadDiamondBlackWide => '\u{e0dc}',
            Self::NoteheadDiamondClusterBlack2Nd => '\u{e139}',
            Self::NoteheadDiamondClusterBlack3Rd => '\u{e13b}',
            Self::NoteheadDiamondClusterBlackBottom => '\u{e141}',
            Self::NoteheadDiamondClusterBlackMiddle => '\u{e140}',
            Self::NoteheadDiamondClusterBlackTop => '\u{e13f}',
            Self::NoteheadDiamondClusterWhite2Nd => '\u{e138}',
            Self::NoteheadDiamondClusterWhite3Rd => '\u{e13a}',
            Self::NoteheadDiamondClusterWhiteBottom => '\u{e13e}',
            Self::NoteheadDiamondClusterWhiteMiddle => '\u{e13d}',
            Self::NoteheadDiamondClusterWhiteTop => '\u{e13c}',
            Self::NoteheadDiamondDoubleWhole => '\u{e0d7}',
            Self::NoteheadDiamondDoubleWholeOld => '\u{e0df}',
            Self::NoteheadDiamondHalf => '\u{e0d9}',
            Self::NoteheadDiamondHalfFilled => '\u{e0e3}',
            Self::NoteheadDiamondHalfOld => '\u{e0e1}',
            Self::NoteheadDiamondHalfWide => '\u{e0da}',
            Self::NoteheadDiamondOpen => '\u{e0fc}',
            Self::NoteheadDiamondWhite => '\u{e0dd}',
            Self::NoteheadDiamondWhiteWide => '\u{e0de}',
            Self::NoteheadDiamondWhole => '\u{e0d8}',
            Self::NoteheadDiamondWholeOld => '\u{e0e0}',
            Self::NoteheadDoubleWhole => '\u{e0a0}',
            Self::NoteheadDoubleWholeSquare => '\u{e0a1}',
            Self::NoteheadDoubleWholeWithX => '\u{e0b4}',
            Self::NoteheadHalf => '\u{e0a3}',
            Self::NoteheadHalfFilled => '\u{e0fb}',
            Self::NoteheadHalfWithX => '\u{e0b6}',
            Self::NoteheadHeavyX => '\u{e0f8}',
            Self::NoteheadHeavyXHat => '\u{e0f9}',
            Self::NoteheadLargeArrowDownBlack => '\u{e0f4}',
            Self::NoteheadLargeArrowDownDoubleWhole => '\u{e0f1}',
            Self::NoteheadLargeArrowDownHalf => '\u{e0f3}',
            Self::NoteheadLargeArrowDownWhole => '\u{e0f2}',
            Self::NoteheadLargeArrowUpBlack => '\u{e0f0}',
            Self::NoteheadLargeArrowUpDoubleWhole => '\u{e0ed}',
            Self::NoteheadLargeArrowUpHalf => '\u{e0ef}',
            Self::NoteheadLargeArrowUpWhole => '\u{e0ee}',
            Self::NoteheadMoonBlack => '\u{e0cb}',
            Self::NoteheadMoonWhite => '\u{e0ca}',
            Self::NoteheadNancarrowSine => '\u{eea0}',
            Self::NoteheadNull => '\u{e0a5}',
            Self::NoteheadParenthesis => '\u{e0ce}',
            Self::NoteheadParenthesisLeft => '\u{e0f5}',
            Self::NoteheadParenthesisRight => '\u{e0f6}',
            Self::NoteheadPlusBlack => '\u{e0af}',
            Self::NoteheadPlusDoubleWhole => '\u{e0ac}',
            Self::NoteheadPlusHalf => '\u{e0ae}',
            Self::NoteheadPlusWhole => '\u{e0ad}',
            Self::NoteheadRectangularClusterBlackBottom => '\u{e144}',
            Self::NoteheadRectangularClusterBlackMiddle => '\u{e143}',
            Self::NoteheadRectangularClusterBlackTop => '\u{e142}',
            Self::NoteheadRectangularClusterWhiteBottom => '\u{e147}',
            Self::NoteheadRectangularClusterWhiteMiddle => '\u{e146}',
            Self::NoteheadRectangularClusterWhiteTop => '\u{e145}',
            Self::NoteheadRoundBlack => '\u{e113}',
            Self::NoteheadRoundBlackDoubleSlashed => '\u{e11c}',
            Self::NoteheadRoundBlackLarge => '\u{e110}',
            Self::NoteheadRoundBlackSlashed => '\u{e118}',
            Self::NoteheadRoundBlackSlashedLarge => '\u{e116}',
            Self::NoteheadRoundWhite => '\u{e114}',
            Self::NoteheadRoundWhiteDoubleSlashed => '\u{e11d}',
            Self::NoteheadRoundWhiteLarge => '\u{e111}',
            Self::NoteheadRoundWhiteSlashed => '\u{e119}',
            Self::NoteheadRoundWhiteSlashedLarge => '\u{e117}',
            Self::NoteheadRoundWhiteWithDot => '\u{e115}',
            Self::NoteheadRoundWhiteWithDotLarge => '\u{e112}',
            Self::NoteheadSlashDiamondWhite => '\u{e104}',
            Self::NoteheadSlashHorizontalEnds => '\u{e101}',
            Self::NoteheadSlashHorizontalEndsMuted => '\u{e108}',
            Self::NoteheadSlashVerticalEnds => '\u{e100}',
            Self::NoteheadSlashVerticalEndsMuted => '\u{e107}',
            Self::NoteheadSlashVerticalEndsSmall => '\u{e105}',
            Self::NoteheadSlashWhiteDoubleWhole => '\u{e10a}',
            Self::NoteheadSlashWhiteHalf => '\u{e103}',
            Self::NoteheadSlashWhiteMuted => '\u{e109}',
            Self::NoteheadSlashWhiteWhole => '\u{e102}',
            Self::NoteheadSlashX => '\u{e106}',
            Self::NoteheadSlashedBlack1 => '\u{e0cf}',
            Self::NoteheadSlashedBlack2 => '\u{e0d0}',
            Self::NoteheadSlashedDoubleWhole1 => '\u{e0d5}',
            Self::NoteheadSlashedDoubleWhole2 => '\u{e0d6}',
            Self::NoteheadSlashedHalf1 => '\u{e0d1}',
            Self::NoteheadSlashedHalf2 => '\u{e0d2}',
            Self::NoteheadSlashedWhole1 => '\u{e0d3}',
            Self::NoteheadSlashedWhole2 => '\u{e0d4}',
            Self::NoteheadSquareBlack => '\u{e0b9}',
            Self::NoteheadSquareBlackLarge => '\u{e11a}',
            Self::NoteheadSquareBlackWhite => '\u{e11b}',
            Self::NoteheadSquareWhite => '\u{e0b8}',
            Self::NoteheadTriangleDownBlack => '\u{e0c7}',
            Self::NoteheadTriangleDownDoubleWhole => '\u{e0c3}',
            Self::NoteheadTriangleDownHalf => '\u{e0c5}',
            Self::NoteheadTriangleDownWhite => '\u{e0c6}',
            Self::NoteheadTriangleDownWhole => '\u{e0c4}',
            Self::NoteheadTriangleLeftBlack => '\u{e0c0}',
            Self::NoteheadTriangleLeftWhite => '\u{e0bf}',
            Self::NoteheadTriangleRightBlack => '\u{e0c2}',
            Self::NoteheadTriangleRightWhite => '\u{e0c1}',
            Self::NoteheadTriangleRoundDownBlack => '\u{e0cd}',
            Self::NoteheadTriangleRoundDownWhite => '\u{e0cc}',
            Self::NoteheadTriangleUpBlack => '\u{e0be}',
            Self::NoteheadTriangleUpDoubleWhole => '\u{e0ba}',
            Self::NoteheadTriangleUpHalf => '\u{e0bc}',
            Self::NoteheadTriangleUpRightBlack => '\u{e0c9}',
            Self::NoteheadTriangleUpRightWhite => '\u{e0c8}',
            Self::NoteheadTriangleUpWhite => '\u{e0bd}',
            Self::NoteheadTriangleUpWhole => '\u{e0bb}',
            Self::NoteheadVoidWithX => '\u{e0b7}',
            Self::NoteheadWhole => '\u{e0a2}',
            Self::NoteheadWholeFilled => '\u{e0fa}',
            Self::NoteheadWholeWithX => '\u{e0b5}',
            Self::NoteheadXBlack => '\u{e0a9}',
            Self::NoteheadXDoubleWhole => '\u{e0a6}',
            Self::NoteheadXHalf => '\u{e0a8}',
            Self::NoteheadXOrnate => '\u{e0aa}',
            Self::NoteheadXOrnateEllipse => '\u{e0ab}',
            Self::NoteheadXWhole => '\u{e0a7}',
            Self::OctaveBaselineA => '\u{ec91}',
            Self::OctaveBaselineB => '\u{ec93}',
            Self::OctaveBaselineM => '\u{ec95}',
            Self::OctaveBaselineV => '\u{ec97}',
            Self::OctaveBassa => '\u{e51f}',
            Self::OctaveLoco => '\u{ec90}',
            Self::OctaveParensLeft => '\u{e51a}',
            Self::OctaveParensRight => '\u{e51b}',
            Self::OctaveSuperscriptA => '\u{ec92}',
            Self::OctaveSuperscriptB => '\u{ec94}',
            Self::OctaveSuperscriptM => '\u{ec96}',
            Self::OctaveSuperscriptV => '\u{ec98}',
            Self::OneHandedRollStevens => '\u{e233}',
            Self::OrganGerman2Fusae => '\u{ee2e}',
            Self::OrganGerman2Minimae => '\u{ee2c}',
            Self::OrganGerman2OctaveUp => '\u{ee19}',
            Self::OrganGerman2Semifusae => '\u{ee2f}',
            Self::OrganGerman2Semiminimae => '\u{ee2d}',
            Self::OrganGerman3Fusae => '\u{ee32}',
            Self::OrganGerman3Minimae => '\u{ee30}',
            Self::OrganGerman3Semifusae => '\u{ee33}',
            Self::OrganGerman3Semiminimae => '\u{ee31}',
            Self::OrganGerman4Fusae => '\u{ee36}',
            Self::OrganGerman4Minimae => '\u{ee34}',
            Self::OrganGerman4Semifusae => '\u{ee37}',
            Self::OrganGerman4Semiminimae => '\u{ee35}',
            Self::OrganGerman5Fusae => '\u{ee3a}',
            Self::OrganGerman5Minimae => '\u{ee38}',
            Self::OrganGerman5Semifusae => '\u{ee3b}',
            Self::OrganGerman5Semiminimae => '\u{ee39}',
            Self::OrganGerman6Fusae => '\u{ee3e}',
            Self::OrganGerman6Minimae => '\u{ee3c}',
            Self::OrganGerman6Semifusae => '\u{ee3f}',
            Self::OrganGerman6Semiminimae => '\u{ee3d}',
            Self::OrganGermanALower => '\u{ee15}',
            Self::OrganGermanAUpper => '\u{ee09}',
            Self::OrganGermanAugmentationDot => '\u{ee1c}',
            Self::OrganGermanBLower => '\u{ee16}',
            Self::OrganGermanBUpper => '\u{ee0a}',
            Self::OrganGermanBuxheimerBrevis2 => '\u{ee25}',
            Self::OrganGermanBuxheimerBrevis3 => '\u{ee24}',
            Self::OrganGermanBuxheimerMinimaRest => '\u{ee1e}',
            Self::OrganGermanBuxheimerSemibrevis => '\u{ee26}',
            Self::OrganGermanBuxheimerSemibrevisRest => '\u{ee1d}',
            Self::OrganGermanCLower => '\u{ee0c}',
            Self::OrganGermanCUpper => '\u{ee00}',
            Self::OrganGermanCisLower => '\u{ee0d}',
            Self::OrganGermanCisUpper => '\u{ee01}',
            Self::OrganGermanDLower => '\u{ee0e}',
            Self::OrganGermanDUpper => '\u{ee02}',
            Self::OrganGermanDisLower => '\u{ee0f}',
            Self::OrganGermanDisUpper => '\u{ee03}',
            Self::OrganGermanELower => '\u{ee10}',
            Self::OrganGermanEUpper => '\u{ee04}',
            Self::OrganGermanFLower => '\u{ee11}',
            Self::OrganGermanFUpper => '\u{ee05}',
            Self::OrganGermanFisLower => '\u{ee12}',
            Self::OrganGermanFisUpper => '\u{ee06}',
            Self::OrganGermanFusa => '\u{ee2a}',
            Self::OrganGermanFusaRest => '\u{ee22}',
            Self::OrganGermanGLower => '\u{ee13}',
            Self::OrganGermanGUpper => '\u{ee07}',
            Self::OrganGermanGisLower => '\u{ee14}',
            Self::OrganGermanGisUpper => '\u{ee08}',
            Self::OrganGermanHLower => '\u{ee17}',
            Self::OrganGermanHUpper => '\u{ee0b}',
            Self::OrganGermanMinima => '\u{ee28}',
            Self::OrganGermanMinimaRest => '\u{ee20}',
            Self::OrganGermanOctaveDown => '\u{ee1a}',
            Self::OrganGermanOctaveUp => '\u{ee18}',
            Self::OrganGermanSemibrevis => '\u{ee27}',
            Self::OrganGermanSemibrevisRest => '\u{ee1f}',
            Self::OrganGermanSemifusa => '\u{ee2b}',
            Self::OrganGermanSemifusaRest => '\u{ee23}',
            Self::OrganGermanSemiminima => '\u{ee29}',
            Self::OrganGermanSemiminimaRest => '\u{ee21}',
            Self::OrganGermanTie => '\u{ee1b}',
            Self::OrnamentBottomLeftConcaveStroke => '\u{e59a}',
            Self::OrnamentBottomLeftConcaveStrokeLarge => '\u{e59b}',
            Self::OrnamentBottomLeftConvexStroke => '\u{e59c}',
            Self::OrnamentBottomRightConcaveStroke => '\u{e5a7}',
            Self::OrnamentBottomRightConvexStroke => '\u{e5a8}',
            Self::OrnamentComma => '\u{e581}',
            Self::OrnamentDoubleObliqueLinesAfterNote => '\u{e57e}',
            Self::OrnamentDoubleObliqueLinesBeforeNote => '\u{e57d}',
            Self::OrnamentDownCurve => '\u{e578}',
            Self::OrnamentHaydn => '\u{e56f}',
            Self::OrnamentHighLeftConcaveStroke => '\u{e592}',
            Self::OrnamentHighLeftConvexStroke => '\u{e593}',
            Self::OrnamentHighRightConcaveStroke => '\u{e5a2}',
            Self::OrnamentHighRightConvexStroke => '\u{e5a3}',
            Self::OrnamentHookAfterNote => '\u{e576}',
            Self::OrnamentHookBeforeNote => '\u{e575}',
            Self::OrnamentLeftFacingHalfCircle => '\u{e572}',
            Self::OrnamentLeftFacingHook => '\u{e574}',
            Self::OrnamentLeftPlus => '\u{e597}',
            Self::OrnamentLeftShakeT => '\u{e596}',
            Self::OrnamentLeftVerticalStroke => '\u{e594}',
            Self::OrnamentLeftVerticalStrokeWithCross => '\u{e595}',
            Self::OrnamentLowLeftConcaveStroke => '\u{e598}',
            Self::OrnamentLowLeftConvexStroke => '\u{e599}',
            Self::OrnamentLowRightConcaveStroke => '\u{e5a5}',
            Self::OrnamentLowRightConvexStroke => '\u{e5a6}',
            Self::OrnamentMiddleVerticalStroke => '\u{e59f}',
            Self::OrnamentMordent => '\u{e56d}',
            Self::OrnamentObliqueLineAfterNote => '\u{e57c}',
            Self::OrnamentObliqueLineBeforeNote => '\u{e57b}',
            Self::OrnamentObliqueLineHorizAfterNote => '\u{e580}',
            Self::OrnamentObliqueLineHorizBeforeNote => '\u{e57f}',
            Self::OrnamentOriscus => '\u{ea21}',
            Self::OrnamentPinceCouperin => '\u{e588}',
            Self::OrnamentPortDeVoixV => '\u{e570}',
            Self::OrnamentPrecompAppoggTrill => '\u{e5b2}',
            Self::OrnamentPrecompAppoggTrillSuffix => '\u{e5b3}',
            Self::OrnamentPrecompCadence => '\u{e5be}',
            Self::OrnamentPrecompCadenceUpperPrefix => '\u{e5c1}',
            Self::OrnamentPrecompCadenceUpperPrefixTurn => '\u{e5c2}',
            Self::OrnamentPrecompCadenceWithTurn => '\u{e5bf}',
            Self::OrnamentPrecompDescendingSlide => '\u{e5b1}',
            Self::OrnamentPrecompDoubleCadenceLowerPrefix => '\u{e5c0}',
            Self::OrnamentPrecompDoubleCadenceUpperPrefix => '\u{e5c3}',
            Self::OrnamentPrecompDoubleCadenceUpperPrefixTurn => '\u{e5c4}',
            Self::OrnamentPrecompInvertedMordentUpperPrefix => '\u{e5c7}',
            Self::OrnamentPrecompMordentRelease => '\u{e5c5}',
            Self::OrnamentPrecompMordentUpperPrefix => '\u{e5c6}',
            Self::OrnamentPrecompPortDeVoixMordent => '\u{e5bc}',
            Self::OrnamentPrecompSlide => '\u{e5b0}',
            Self::OrnamentPrecompSlideTrillBach => '\u{e5b8}',
            Self::OrnamentPrecompSlideTrillDAnglebert => '\u{e5b5}',
            Self::OrnamentPrecompSlideTrillMarpurg => '\u{e5b6}',
            Self::OrnamentPrecompSlideTrillMuffat => '\u{e5b9}',
            Self::OrnamentPrecompSlideTrillSuffixMuffat => '\u{e5ba}',
            Self::OrnamentPrecompTrillLowerSuffix => '\u{e5c8}',
            Self::OrnamentPrecompTrillSuffixDandrieu => '\u{e5bb}',
            Self::OrnamentPrecompTrillWithMordent => '\u{e5bd}',
            Self::OrnamentPrecompTurnTrillBach => '\u{e5b7}',
            Self::OrnamentPrecompTurnTrillDAnglebert => '\u{e5b4}',
            Self::OrnamentQuilisma => '\u{ea20}',
            Self::OrnamentRightFacingHalfCircle => '\u{e571}',
            Self::OrnamentRightFacingHook => '\u{e573}',
            Self::OrnamentRightVerticalStroke => '\u{e5a4}',
            Self::OrnamentSchleifer => '\u{e587}',
            Self::OrnamentShake3 => '\u{e582}',
            Self::OrnamentShakeMuffat1 => '\u{e584}',
            Self::OrnamentShortObliqueLineAfterNote => '\u{e57a}',
            Self::OrnamentShortObliqueLineBeforeNote => '\u{e579}',
            Self::OrnamentShortTrill => '\u{e56c}',
            Self::OrnamentTopLeftConcaveStroke => '\u{e590}',
            Self::OrnamentTopLeftConvexStroke => '\u{e591}',
            Self::OrnamentTopRightConcaveStroke => '\u{e5a0}',
            Self::OrnamentTopRightConvexStroke => '\u{e5a1}',
            Self::OrnamentTremblement => '\u{e56e}',
            Self::OrnamentTremblementCouperin => '\u{e589}',
            Self::OrnamentTrill => '\u{e566}',
            Self::OrnamentTurn => '\u{e567}',
            Self::OrnamentTurnInverted => '\u{e568}',
            Self::OrnamentTurnSlash => '\u{e569}',
            Self::OrnamentTurnUp => '\u{e56a}',
            Self::OrnamentTurnUpS => '\u{e56b}',
            Self::OrnamentUpCurve => '\u{e577}',
            Self::OrnamentVerticalLine => '\u{e583}',
            Self::OrnamentZigZagLineNoRightEnd => '\u{e59d}',
            Self::OrnamentZigZagLineWithRightEnd => '\u{e59e}',
            Self::Ottava => '\u{e510}',
            Self::OttavaAlta => '\u{e511}',
            Self::OttavaBassa => '\u{e512}',
            Self::OttavaBassaBa => '\u{e513}',
            Self::OttavaBassaVb => '\u{e51c}',
            Self::PendereckiTremolo => '\u{e22b}',
            Self::PictAgogo => '\u{e717}',
            Self::PictAlmglocken => '\u{e712}',
            Self::PictAnvil => '\u{e701}',
            Self::PictBambooChimes => '\u{e6c3}',
            Self::PictBambooScraper => '\u{e6fb}',
            Self::PictBassDrum => '\u{e6d4}',
            Self::PictBassDrumOnSide => '\u{e6d5}',
            Self::PictBeaterBow => '\u{e7de}',
            Self::PictBeaterBox => '\u{e7eb}',
            Self::PictBeaterBrassMalletsDown => '\u{e7da}',
            Self::PictBeaterBrassMalletsLeft => '\u{e7ee}',
            Self::PictBeaterBrassMalletsRight => '\u{e7ed}',
            Self::PictBeaterBrassMalletsUp => '\u{e7d9}',
            Self::PictBeaterCombiningDashedCircle => '\u{e7ea}',
            Self::PictBeaterCombiningParentheses => '\u{e7e9}',
            Self::PictBeaterDoubleBassDrumDown => '\u{e7a1}',
            Self::PictBeaterDoubleBassDrumUp => '\u{e7a0}',
            Self::PictBeaterFinger => '\u{e7e4}',
            Self::PictBeaterFingernails => '\u{e7e6}',
            Self::PictBeaterFist => '\u{e7e5}',
            Self::PictBeaterGuiroScraper => '\u{e7dd}',
            Self::PictBeaterHammer => '\u{e7e1}',
            Self::PictBeaterHammerMetalDown => '\u{e7d0}',
            Self::PictBeaterHammerMetalUp => '\u{e7cf}',
            Self::PictBeaterHammerPlasticDown => '\u{e7ce}',
            Self::PictBeaterHammerPlasticUp => '\u{e7cd}',
            Self::PictBeaterHammerWoodDown => '\u{e7cc}',
            Self::PictBeaterHammerWoodUp => '\u{e7cb}',
            Self::PictBeaterHand => '\u{e7e3}',
            Self::PictBeaterHardBassDrumDown => '\u{e79d}',
            Self::PictBeaterHardBassDrumUp => '\u{e79c}',
            Self::PictBeaterHardGlockenspielDown => '\u{e785}',
            Self::PictBeaterHardGlockenspielLeft => '\u{e787}',
            Self::PictBeaterHardGlockenspielRight => '\u{e786}',
            Self::PictBeaterHardGlockenspielUp => '\u{e784}',
            Self::PictBeaterHardTimpaniDown => '\u{e791}',
            Self::PictBeaterHardTimpaniLeft => '\u{e793}',
            Self::PictBeaterHardTimpaniRight => '\u{e792}',
            Self::PictBeaterHardTimpaniUp => '\u{e790}',
            Self::PictBeaterHardXylophoneDown => '\u{e779}',
            Self::PictBeaterHardXylophoneLeft => '\u{e77b}',
            Self::PictBeaterHardXylophoneRight => '\u{e77a}',
            Self::PictBeaterHardXylophoneUp => '\u{e778}',
            Self::PictBeaterHardYarnDown => '\u{e7ab}',
            Self::PictBeaterHardYarnLeft => '\u{e7ad}',
            Self::PictBeaterHardYarnRight => '\u{e7ac}',
            Self::PictBeaterHardYarnUp => '\u{e7aa}',
            Self::PictBeaterJazzSticksDown => '\u{e7d4}',
            Self::PictBeaterJazzSticksUp => '\u{e7d3}',
            Self::PictBeaterKnittingNeedle => '\u{e7e2}',
            Self::PictBeaterMallet => '\u{e7df}',
            Self::PictBeaterMalletDown => '\u{e7ec}',
            Self::PictBeaterMediumBassDrumDown => '\u{e79b}',
            Self::PictBeaterMediumBassDrumUp => '\u{e79a}',
            Self::PictBeaterMediumTimpaniDown => '\u{e78d}',
            Self::PictBeaterMediumTimpaniLeft => '\u{e78f}',
            Self::PictBeaterMediumTimpaniRight => '\u{e78e}',
            Self::PictBeaterMediumTimpaniUp => '\u{e78c}',
            Self::PictBeaterMediumXylophoneDown => '\u{e775}',
            Self::PictBeaterMediumXylophoneLeft => '\u{e777}',
            Self::PictBeaterMediumXylophoneRight => '\u{e776}',
            Self::PictBeaterMediumXylophoneUp => '\u{e774}',
            Self::PictBeaterMediumYarnDown => '\u{e7a7}',
            Self::PictBeaterMediumYarnLeft => '\u{e7a9}',
            Self::PictBeaterMediumYarnRight => '\u{e7a8}',
            Self::PictBeaterMediumYarnUp => '\u{e7a6}',
            Self::PictBeaterMetalBassDrumDown => '\u{e79f}',
            Self::PictBeaterMetalBassDrumUp => '\u{e79e}',
            Self::PictBeaterMetalDown => '\u{e7c8}',
            Self::PictBeaterMetalHammer => '\u{e7e0}',
            Self::PictBeaterMetalLeft => '\u{e7ca}',
            Self::PictBeaterMetalRight => '\u{e7c9}',
            Self::PictBeaterMetalUp => '\u{e7c7}',
            Self::PictBeaterSnareSticksDown => '\u{e7d2}',
            Self::PictBeaterSnareSticksUp => '\u{e7d1}',
            Self::PictBeaterSoftBassDrumDown => '\u{e799}',
            Self::PictBeaterSoftBassDrumUp => '\u{e798}',
            Self::PictBeaterSoftGlockenspielDown => '\u{e781}',
            Self::PictBeaterSoftGlockenspielLeft => '\u{e783}',
            Self::PictBeaterSoftGlockenspielRight => '\u{e782}',
            Self::PictBeaterSoftGlockenspielUp => '\u{e780}',
            Self::PictBeaterSoftTimpaniDown => '\u{e789}',
            Self::PictBeaterSoftTimpaniLeft => '\u{e78b}',
            Self::PictBeaterSoftTimpaniRight => '\u{e78a}',
            Self::PictBeaterSoftTimpaniUp => '\u{e788}',
            Self::PictBeaterSoftXylophone => '\u{e7db}',
            Self::PictBeaterSoftXylophoneDown => '\u{e771}',
            Self::PictBeaterSoftXylophoneLeft => '\u{e773}',
            Self::PictBeaterSoftXylophoneRight => '\u{e772}',
            Self::PictBeaterSoftXylophoneUp => '\u{e770}',
            Self::PictBeaterSoftYarnDown => '\u{e7a3}',
            Self::PictBeaterSoftYarnLeft => '\u{e7a5}',
            Self::PictBeaterSoftYarnRight => '\u{e7a4}',
            Self::PictBeaterSoftYarnUp => '\u{e7a2}',
            Self::PictBeaterSpoonWoodenMallet => '\u{e7dc}',
            Self::PictBeaterSuperballDown => '\u{e7af}',
            Self::PictBeaterSuperballLeft => '\u{e7b1}',
            Self::PictBeaterSuperballRight => '\u{e7b0}',
            Self::PictBeaterSuperballUp => '\u{e7ae}',
            Self::PictBeaterTriangleDown => '\u{e7d6}',
            Self::PictBeaterTrianglePlain => '\u{e7ef}',
            Self::PictBeaterTriangleUp => '\u{e7d5}',
            Self::PictBeaterWireBrushesDown => '\u{e7d8}',
            Self::PictBeaterWireBrushesUp => '\u{e7d7}',
            Self::PictBeaterWoodTimpaniDown => '\u{e795}',
            Self::PictBeaterWoodTimpaniLeft => '\u{e797}',
            Self::PictBeaterWoodTimpaniRight => '\u{e796}',
            Self::PictBeaterWoodTimpaniUp => '\u{e794}',
            Self::PictBeaterWoodXylophoneDown => '\u{e77d}',
            Self::PictBeaterWoodXylophoneLeft => '\u{e77f}',
            Self::PictBeaterWoodXylophoneRight => '\u{e77e}',
            Self::PictBeaterWoodXylophoneUp => '\u{e77c}',
            Self::PictBell => '\u{e714}',
            Self::PictBellOfCymbal => '\u{e72a}',
            Self::PictBellPlate => '\u{e713}',
            Self::PictBellTree => '\u{e71a}',
            Self::PictBirdWhistle => '\u{e751}',
            Self::PictBoardClapper => '\u{e6f7}',
            Self::PictBongos => '\u{e6dd}',
            Self::PictBrakeDrum => '\u{e6e1}',
            Self::PictCabasa => '\u{e743}',
            Self::PictCannon => '\u{e761}',
            Self::PictCarHorn => '\u{e755}',
            Self::PictCastanets => '\u{e6f8}',
            Self::PictCastanetsWithHandle => '\u{e6f9}',
            Self::PictCelesta => '\u{e6b0}',
            Self::PictCencerro => '\u{e716}',
            Self::PictCenter1 => '\u{e7fe}',
            Self::PictCenter2 => '\u{e7ff}',
            Self::PictCenter3 => '\u{e800}',
            Self::PictChainRattle => '\u{e748}',
            Self::PictChimes => '\u{e6c2}',
            Self::PictChineseCymbal => '\u{e726}',
            Self::PictChokeCymbal => '\u{e805}',
            Self::PictClaves => '\u{e6f2}',
            Self::PictCoins => '\u{e7e7}',
            Self::PictConga => '\u{e6de}',
            Self::PictCowBell => '\u{e711}',
            Self::PictCrashCymbals => '\u{e720}',
            Self::PictCrotales => '\u{e6ae}',
            Self::PictCrushStem => '\u{e80c}',
            Self::PictCuica => '\u{e6e4}',
            Self::PictCymbalTongs => '\u{e728}',
            Self::PictDamp1 => '\u{e7f9}',
            Self::PictDamp2 => '\u{e7fa}',
            Self::PictDamp3 => '\u{e7fb}',
            Self::PictDamp4 => '\u{e7fc}',
            Self::PictDeadNoteStem => '\u{e80d}',
            Self::PictDrumStick => '\u{e7e8}',
            Self::PictDuckCall => '\u{e757}',
            Self::PictEdgeOfCymbal => '\u{e729}',
            Self::PictEmptyTrap => '\u{e6a9}',
            Self::PictFingerCymbals => '\u{e727}',
            Self::PictFlexatone => '\u{e740}',
            Self::PictFootballRatchet => '\u{e6f5}',
            Self::PictGlassHarmonica => '\u{e765}',
            Self::PictGlassHarp => '\u{e764}',
            Self::PictGlassPlateChimes => '\u{e6c6}',
            Self::PictGlassTubeChimes => '\u{e6c5}',
            Self::PictGlsp => '\u{e6a0}',
            Self::PictGlspSmithBrindle => '\u{e6aa}',
            Self::PictGobletDrum => '\u{e6e2}',
            Self::PictGong => '\u{e732}',
            Self::PictGongWithButton => '\u{e733}',
            Self::PictGuiro => '\u{e6f3}',
            Self::PictGumHardDown => '\u{e7c4}',
            Self::PictGumHardLeft => '\u{e7c6}',
            Self::PictGumHardRight => '\u{e7c5}',
            Self::PictGumHardUp => '\u{e7c3}',
            Self::PictGumMediumDown => '\u{e7c0}',
            Self::PictGumMediumLeft => '\u{e7c2}',
            Self::PictGumMediumRight => '\u{e7c1}',
            Self::PictGumMediumUp => '\u{e7bf}',
            Self::PictGumSoftDown => '\u{e7bc}',
            Self::PictGumSoftLeft => '\u{e7be}',
            Self::PictGumSoftRight => '\u{e7bd}',
            Self::PictGumSoftUp => '\u{e7bb}',
            Self::PictHalfOpen1 => '\u{e7f6}',
            Self::PictHalfOpen2 => '\u{e7f7}',
            Self::PictHandbell => '\u{e715}',
            Self::PictHiHat => '\u{e722}',
            Self::PictHiHatOnStand => '\u{e723}',
            Self::PictJawHarp => '\u{e767}',
            Self::PictJingleBells => '\u{e719}',
            Self::PictKlaxonHorn => '\u{e756}',
            Self::PictLeftHandCircle => '\u{e807}',
            Self::PictLionsRoar => '\u{e763}',
            Self::PictLithophone => '\u{e6b1}',
            Self::PictLogDrum => '\u{e6df}',
            Self::PictLotusFlute => '\u{e75a}',
            Self::PictMar => '\u{e6a6}',
            Self::PictMarSmithBrindle => '\u{e6ac}',
            Self::PictMaraca => '\u{e741}',
            Self::PictMaracas => '\u{e742}',
            Self::PictMegaphone => '\u{e759}',
            Self::PictMetalPlateChimes => '\u{e6c8}',
            Self::PictMetalTubeChimes => '\u{e6c7}',
            Self::PictMusicalSaw => '\u{e766}',
            Self::PictNormalPosition => '\u{e804}',
            Self::PictOnRim => '\u{e7f4}',
            Self::PictOpen => '\u{e7f8}',
            Self::PictOpenRimShot => '\u{e7f5}',
            Self::PictPistolShot => '\u{e760}',
            Self::PictPoliceWhistle => '\u{e752}',
            Self::PictQuijada => '\u{e6fa}',
            Self::PictRainstick => '\u{e747}',
            Self::PictRatchet => '\u{e6f4}',
            Self::PictRecoReco => '\u{e6fc}',
            Self::PictRightHandSquare => '\u{e806}',
            Self::PictRim1 => '\u{e801}',
            Self::PictRim2 => '\u{e802}',
            Self::PictRim3 => '\u{e803}',
            Self::PictRimShotOnStem => '\u{e7fd}',
            Self::PictSandpaperBlocks => '\u{e762}',
            Self::PictScrapeAroundRim => '\u{e7f3}',
            Self::PictScrapeAroundRimClockwise => '\u{e80e}',
            Self::PictScrapeCenterToEdge => '\u{e7f1}',
            Self::PictScrapeEdgeToCenter => '\u{e7f2}',
            Self::PictShellBells => '\u{e718}',
            Self::PictShellChimes => '\u{e6c4}',
            Self::PictSiren => '\u{e753}',
            Self::PictSistrum => '\u{e746}',
            Self::PictSizzleCymbal => '\u{e724}',
            Self::PictSleighBell => '\u{e710}',
            Self::PictSlideBrushOnGong => '\u{e734}',
            Self::PictSlideWhistle => '\u{e750}',
            Self::PictSlitDrum => '\u{e6e0}',
            Self::PictSnareDrum => '\u{e6d1}',
            Self::PictSnareDrumMilitary => '\u{e6d3}',
            Self::PictSnareDrumSnaresOff => '\u{e6d2}',
            Self::PictSteelDrums => '\u{e6af}',
            Self::PictStickShot => '\u{e7f0}',
            Self::PictSuperball => '\u{e7b2}',
            Self::PictSuspendedCymbal => '\u{e721}',
            Self::PictSwishStem => '\u{e808}',
            Self::PictTabla => '\u{e6e3}',
            Self::PictTamTam => '\u{e730}',
            Self::PictTamTamWithBeater => '\u{e731}',
            Self::PictTambourine => '\u{e6db}',
            Self::PictTempleBlocks => '\u{e6f1}',
            Self::PictTenorDrum => '\u{e6d6}',
            Self::PictThundersheet => '\u{e744}',
            Self::PictTimbales => '\u{e6dc}',
            Self::PictTimpani => '\u{e6d0}',
            Self::PictTomTom => '\u{e6d7}',
            Self::PictTomTomChinese => '\u{e6d8}',
            Self::PictTomTomIndoAmerican => '\u{e6da}',
            Self::PictTomTomJapanese => '\u{e6d9}',
            Self::PictTriangle => '\u{e700}',
            Self::PictTubaphone => '\u{e6b2}',
            Self::PictTubularBells => '\u{e6c0}',
            Self::PictTurnLeftStem => '\u{e80a}',
            Self::PictTurnRightLeftStem => '\u{e80b}',
            Self::PictTurnRightStem => '\u{e809}',
            Self::PictVib => '\u{e6a7}',
            Self::PictVibMotorOff => '\u{e6a8}',
            Self::PictVibSmithBrindle => '\u{e6ad}',
            Self::PictVibraslap => '\u{e745}',
            Self::PictVietnameseHat => '\u{e725}',
            Self::PictWhip => '\u{e6f6}',
            Self::PictWindChimesGlass => '\u{e6c1}',
            Self::PictWindMachine => '\u{e754}',
            Self::PictWindWhistle => '\u{e758}',
            Self::PictWoodBlock => '\u{e6f0}',
            Self::PictWoundHardDown => '\u{e7b4}',
            Self::PictWoundHardLeft => '\u{e7b6}',
            Self::PictWoundHardRight => '\u{e7b5}',
            Self::PictWoundHardUp => '\u{e7b3}',
            Self::PictWoundSoftDown => '\u{e7b8}',
            Self::PictWoundSoftLeft => '\u{e7ba}',
            Self::PictWoundSoftRight => '\u{e7b9}',
            Self::PictWoundSoftUp => '\u{e7b7}',
            Self::PictXyl => '\u{e6a1}',
            Self::PictXylBass => '\u{e6a3}',
            Self::PictXylSmithBrindle => '\u{e6ab}',
            Self::PictXylTenor => '\u{e6a2}',
            Self::PictXylTenorTrough => '\u{e6a5}',
            Self::PictXylTrough => '\u{e6a4}',
            Self::PluckedBuzzPizzicato => '\u{e632}',
            Self::PluckedDamp => '\u{e638}',
            Self::PluckedDampAll => '\u{e639}',
            Self::PluckedDampOnStem => '\u{e63b}',
            Self::PluckedFingernailFlick => '\u{e637}',
            Self::PluckedLeftHandPizzicato => '\u{e633}',
            Self::PluckedPlectrum => '\u{e63a}',
            Self::PluckedSnapPizzicatoAbove => '\u{e631}',
            Self::PluckedSnapPizzicatoBelow => '\u{e630}',
            Self::PluckedWithFingernails => '\u{e636}',
            Self::Quindicesima => '\u{e514}',
            Self::QuindicesimaAlta => '\u{e515}',
            Self::QuindicesimaBassa => '\u{e516}',
            Self::QuindicesimaBassaMb => '\u{e51d}',
            Self::Repeat1Bar => '\u{e500}',
            Self::Repeat2Bars => '\u{e501}',
            Self::Repeat4Bars => '\u{e502}',
            Self::RepeatBarLowerDot => '\u{e505}',
            Self::RepeatBarSlash => '\u{e504}',
            Self::RepeatBarUpperDot => '\u{e503}',
            Self::RepeatDot => '\u{e044}',
            Self::RepeatDots => '\u{e043}',
            Self::RepeatLeft => '\u{e040}',
            Self::RepeatRight => '\u{e041}',
            Self::RepeatRightLeft => '\u{e042}',
            Self::Rest1024Th => '\u{e4ed}',
            Self::Rest128Th => '\u{e4ea}',
            Self::Rest16Th => '\u{e4e7}',
            Self::Rest256Th => '\u{e4eb}',
            Self::Rest32Nd => '\u{e4e8}',
            Self::Rest512Th => '\u{e4ec}',
            Self::Rest64Th => '\u{e4e9}',
            Self::Rest8Th => '\u{e4e6}',
            Self::RestDoubleWhole => '\u{e4e2}',
            Self::RestDoubleWholeLegerLine => '\u{e4f3}',
            Self::RestHBar => '\u{e4ee}',
            Self::RestHBarLeft => '\u{e4ef}',
            Self::RestHBarMiddle => '\u{e4f0}',
            Self::RestHBarRight => '\u{e4f1}',
            Self::RestHalf => '\u{e4e4}',
            Self::RestHalfLegerLine => '\u{e4f5}',
            Self::RestLonga => '\u{e4e1}',
            Self::RestMaxima => '\u{e4e0}',
            Self::RestQuarter => '\u{e4e5}',
            Self::RestQuarterOld => '\u{e4f2}',
            Self::RestQuarterZ => '\u{e4f6}',
            Self::RestWhole => '\u{e4e3}',
            Self::RestWholeLegerLine => '\u{e4f4}',
            Self::ReversedBrace => '\u{e001}',
            Self::ReversedBracketBottom => '\u{e006}',
            Self::ReversedBracketTop => '\u{e005}',
            Self::RightRepeatSmall => '\u{e04d}',
            Self::ScaleDegree1 => '\u{ef00}',
            Self::ScaleDegree2 => '\u{ef01}',
            Self::ScaleDegree3 => '\u{ef02}',
            Self::ScaleDegree4 => '\u{ef03}',
            Self::ScaleDegree5 => '\u{ef04}',
            Self::ScaleDegree6 => '\u{ef05}',
            Self::ScaleDegree7 => '\u{ef06}',
            Self::ScaleDegree8 => '\u{ef07}',
            Self::ScaleDegree9 => '\u{ef08}',
            Self::SchaefferClef => '\u{e06f}',
            Self::SchaefferFClefToGClef => '\u{e072}',
            Self::SchaefferGClefToFClef => '\u{e071}',
            Self::SchaefferPreviousClef => '\u{e070}',
            Self::Segno => '\u{e047}',
            Self::SegnoSerpent1 => '\u{e04a}',
            Self::SegnoSerpent2 => '\u{e04b}',
            Self::SemipitchedPercussionClef1 => '\u{e06b}',
            Self::SemipitchedPercussionClef2 => '\u{e06c}',
            Self::SmnFlat => '\u{ec52}',
            Self::SmnFlatWhite => '\u{ec53}',
            Self::SmnHistoryDoubleFlat => '\u{ec57}',
            Self::SmnHistoryDoubleSharp => '\u{ec55}',
            Self::SmnHistoryFlat => '\u{ec56}',
            Self::SmnHistorySharp => '\u{ec54}',
            Self::SmnNatural => '\u{ec58}',
            Self::SmnSharp => '\u{ec50}',
            Self::SmnSharpDown => '\u{ec59}',
            Self::SmnSharpWhite => '\u{ec51}',
            Self::SmnSharpWhiteDown => '\u{ec5a}',
            Self::SplitBarDivider => '\u{e00a}',
            Self::Staff1Line => '\u{e010}',
            Self::Staff1LineNarrow => '\u{e01c}',
            Self::Staff1LineWide => '\u{e016}',
            Self::Staff2Lines => '\u{e011}',
            Self::Staff2LinesNarrow => '\u{e01d}',
            Self::Staff2LinesWide => '\u{e017}',
            Self::Staff3Lines => '\u{e012}',
            Self::Staff3LinesNarrow => '\u{e01e}',
            Self::Staff3LinesWide => '\u{e018}',
            Self::Staff4Lines => '\u{e013}',
            Self::Staff4LinesNarrow => '\u{e01f}',
            Self::Staff4LinesWide => '\u{e019}',
            Self::Staff5Lines => '\u{e014}',
            Self::Staff5LinesNarrow => '\u{e020}',
            Self::Staff5LinesWide => '\u{e01a}',
            Self::Staff6Lines => '\u{e015}',
            Self::Staff6LinesNarrow => '\u{e021}',
            Self::Staff6LinesWide => '\u{e01b}',
            Self::StaffDivideArrowDown => '\u{e00b}',
            Self::StaffDivideArrowUp => '\u{e00c}',
            Self::StaffDivideArrowUpDown => '\u{e00d}',
            Self::StaffPosLower1 => '\u{eb98}',
            Self::StaffPosLower2 => '\u{eb99}',
            Self::StaffPosLower3 => '\u{eb9a}',
            Self::StaffPosLower4 => '\u{eb9b}',
            Self::StaffPosLower5 => '\u{eb9c}',
            Self::StaffPosLower6 => '\u{eb9d}',
            Self::StaffPosLower7 => '\u{eb9e}',
            Self::StaffPosLower8 => '\u{eb9f}',
            Self::StaffPosRaise1 => '\u{eb90}',
            Self::StaffPosRaise2 => '\u{eb91}',
            Self::StaffPosRaise3 => '\u{eb92}',
            Self::StaffPosRaise4 => '\u{eb93}',
            Self::StaffPosRaise5 => '\u{eb94}',
            Self::StaffPosRaise6 => '\u{eb95}',
            Self::StaffPosRaise7 => '\u{eb96}',
            Self::StaffPosRaise8 => '\u{eb97}',
            Self::Stem => '\u{e210}',
            Self::StemBowOnBridge => '\u{e215}',
            Self::StemBowOnTailpiece => '\u{e216}',
            Self::StemBuzzRoll => '\u{e217}',
            Self::StemDamp => '\u{e218}',
            Self::StemHarpStringNoise => '\u{e21f}',
            Self::StemMultiphonicsBlack => '\u{e21a}',
            Self::StemMultiphonicsBlackWhite => '\u{e21c}',
            Self::StemMultiphonicsWhite => '\u{e21b}',
            Self::StemPendereckiTremolo => '\u{e213}',
            Self::StemRimShot => '\u{e21e}',
            Self::StemSprechgesang => '\u{e211}',
            Self::StemSulPonticello => '\u{e214}',
            Self::StemSussurando => '\u{e21d}',
            Self::StemSwished => '\u{e212}',
            Self::StemVibratoPulse => '\u{e219}',
            Self::StockhausenTremolo => '\u{e232}',
            Self::StringsBowBehindBridge => '\u{e618}',
            Self::StringsBowBehindBridgeFourStrings => '\u{e62a}',
            Self::StringsBowBehindBridgeOneString => '\u{e627}',
            Self::StringsBowBehindBridgeThreeStrings => '\u{e629}',
            Self::StringsBowBehindBridgeTwoStrings => '\u{e628}',
            Self::StringsBowOnBridge => '\u{e619}',
            Self::StringsBowOnTailpiece => '\u{e61a}',
            Self::StringsChangeBowDirection => '\u{e626}',
            Self::StringsDownBow => '\u{e610}',
            Self::StringsDownBowAwayFromBody => '\u{ee82}',
            Self::StringsDownBowBeyondBridge => '\u{ee84}',
            Self::StringsDownBowTowardsBody => '\u{ee80}',
            Self::StringsDownBowTurned => '\u{e611}',
            Self::StringsFouette => '\u{e622}',
            Self::StringsHalfHarmonic => '\u{e615}',
            Self::StringsHarmonic => '\u{e614}',
            Self::StringsJeteAbove => '\u{e620}',
            Self::StringsJeteBelow => '\u{e621}',
            Self::StringsMuteOff => '\u{e617}',
            Self::StringsMuteOn => '\u{e616}',
            Self::StringsOverpressureDownBow => '\u{e61b}',
            Self::StringsOverpressureNoDirection => '\u{e61f}',
            Self::StringsOverpressurePossibileDownBow => '\u{e61d}',
            Self::StringsOverpressurePossibileUpBow => '\u{e61e}',
            Self::StringsOverpressureUpBow => '\u{e61c}',
            Self::StringsScrapeCircularClockwise => '\u{ee88}',
            Self::StringsScrapeCircularCounterclockwise => '\u{ee89}',
            Self::StringsScrapeParallelInward => '\u{ee86}',
            Self::StringsScrapeParallelOutward => '\u{ee87}',
            Self::StringsThumbPosition => '\u{e624}',
            Self::StringsThumbPositionTurned => '\u{e625}',
            Self::StringsTripleChopInward => '\u{ee8a}',
            Self::StringsTripleChopOutward => '\u{ee8b}',
            Self::StringsUpBow => '\u{e612}',
            Self::StringsUpBowAwayFromBody => '\u{ee83}',
            Self::StringsUpBowBeyondBridge => '\u{ee85}',
            Self::StringsUpBowTowardsBody => '\u{ee81}',
            Self::StringsUpBowTurned => '\u{e613}',
            Self::StringsVibratoPulse => '\u{e623}',
            Self::SwissRudimentsNoteheadBlackDouble => '\u{ee72}',
            Self::SwissRudimentsNoteheadBlackFlam => '\u{ee70}',
            Self::SwissRudimentsNoteheadHalfDouble => '\u{ee73}',
            Self::SwissRudimentsNoteheadHalfFlam => '\u{ee71}',
            Self::SystemDivider => '\u{e007}',
            Self::SystemDividerExtraLong => '\u{e009}',
            Self::SystemDividerLong => '\u{e008}',
            Self::TextAugmentationDot => '\u{e1fc}',
            Self::TextBlackNoteFrac16ThLongStem => '\u{e1f5}',
            Self::TextBlackNoteFrac16ThShortStem => '\u{e1f4}',
            Self::TextBlackNoteFrac32NdLongStem => '\u{e1f6}',
            Self::TextBlackNoteFrac8ThLongStem => '\u{e1f3}',
            Self::TextBlackNoteFrac8ThShortStem => '\u{e1f2}',
            Self::TextBlackNoteLongStem => '\u{e1f1}',
            Self::TextBlackNoteShortStem => '\u{e1f0}',
            Self::TextCont16ThBeamLongStem => '\u{e1fa}',
            Self::TextCont16ThBeamShortStem => '\u{e1f9}',
            Self::TextCont32NdBeamLongStem => '\u{e1fb}',
            Self::TextCont8ThBeamLongStem => '\u{e1f8}',
            Self::TextCont8ThBeamShortStem => '\u{e1f7}',
            Self::TextHeadlessBlackNoteFrac16ThLongStem => '\u{e209}',
            Self::TextHeadlessBlackNoteFrac16ThShortStem => '\u{e208}',
            Self::TextHeadlessBlackNoteFrac32NdLongStem => '\u{e20a}',
            Self::TextHeadlessBlackNoteFrac8ThLongStem => '\u{e207}',
            Self::TextHeadlessBlackNoteFrac8ThShortStem => '\u{e206}',
            Self::TextHeadlessBlackNoteLongStem => '\u{e205}',
            Self::TextHeadlessBlackNoteShortStem => '\u{e204}',
            Self::TextTie => '\u{e1fd}',
            Self::TextTuplet3LongStem => '\u{e202}',
            Self::TextTuplet3ShortStem => '\u{e1ff}',
            Self::TextTupletBracketEndLongStem => '\u{e203}',
            Self::TextTupletBracketEndShortStem => '\u{e200}',
            Self::TextTupletBracketStartLongStem => '\u{e201}',
            Self::TextTupletBracketStartShortStem => '\u{e1fe}',
            Self::TimeSig0 => '\u{e080}',
            Self::TimeSig0Reversed => '\u{ecf0}',
            Self::TimeSig0Turned => '\u{ece0}',
            Self::TimeSig1 => '\u{e081}',
            Self::TimeSig1Reversed => '\u{ecf1}',
            Self::TimeSig1Turned => '\u{ece1}',
            Self::TimeSig2 => '\u{e082}',
            Self::TimeSig2Reversed => '\u{ecf2}',
            Self::TimeSig2Turned => '\u{ece2}',
            Self::TimeSig3 => '\u{e083}',
            Self::TimeSig3Reversed => '\u{ecf3}',
            Self::TimeSig3Turned => '\u{ece3}',
            Self::TimeSig4 => '\u{e084}',
            Self::TimeSig4Reversed => '\u{ecf4}',
            Self::TimeSig4Turned => '\u{ece4}',
            Self::TimeSig5 => '\u{e085}',
            Self::TimeSig5Reversed => '\u{ecf5}',
            Self::TimeSig5Turned => '\u{ece5}',
            Self::TimeSig6 => '\u{e086}',
            Self::TimeSig6Reversed => '\u{ecf6}',
            Self::TimeSig6Turned => '\u{ece6}',
            Self::TimeSig7 => '\u{e087}',
            Self::TimeSig7Reversed => '\u{ecf7}',
            Self::TimeSig7Turned => '\u{ece7}',
            Self::TimeSig8 => '\u{e088}',
            Self::TimeSig8Reversed => '\u{ecf8}',
            Self::TimeSig8Turned => '\u{ece8}',
            Self::TimeSig9 => '\u{e089}',
            Self::TimeSig9Reversed => '\u{ecf9}',
            Self::TimeSig9Turned => '\u{ece9}',
            Self::TimeSigBracketLeft => '\u{ec80}',
            Self::TimeSigBracketLeftSmall => '\u{ec82}',
            Self::TimeSigBracketRight => '\u{ec81}',
            Self::TimeSigBracketRightSmall => '\u{ec83}',
            Self::TimeSigCombDenominator => '\u{e09f}',
            Self::TimeSigCombNumerator => '\u{e09e}',
            Self::TimeSigComma => '\u{e096}',
            Self::TimeSigCommon => '\u{e08a}',
            Self::TimeSigCommonReversed => '\u{ecfa}',
            Self::TimeSigCommonTurned => '\u{ecea}',
            Self::TimeSigCut2 => '\u{ec85}',
            Self::TimeSigCut3 => '\u{ec86}',
            Self::TimeSigCutCommon => '\u{e08b}',
            Self::TimeSigCutCommonReversed => '\u{ecfb}',
            Self::TimeSigCutCommonTurned => '\u{eceb}',
            Self::TimeSigEquals => '\u{e08f}',
            Self::TimeSigFractionHalf => '\u{e098}',
            Self::TimeSigFractionOneThird => '\u{e09a}',
            Self::TimeSigFractionQuarter => '\u{e097}',
            Self::TimeSigFractionThreeQuarters => '\u{e099}',
            Self::TimeSigFractionTwoThirds => '\u{e09b}',
            Self::TimeSigFractionalSlash => '\u{e08e}',
            Self::TimeSigMinus => '\u{e090}',
            Self::TimeSigMultiply => '\u{e091}',
            Self::TimeSigOpenPenderecki => '\u{e09d}',
            Self::TimeSigParensLeft => '\u{e094}',
            Self::TimeSigParensLeftSmall => '\u{e092}',
            Self::TimeSigParensRight => '\u{e095}',
            Self::TimeSigParensRightSmall => '\u{e093}',
            Self::TimeSigPlus => '\u{e08c}',
            Self::TimeSigPlusSmall => '\u{e08d}',
            Self::TimeSigSlash => '\u{ec84}',
            Self::TimeSigX => '\u{e09c}',
            Self::Tremolo1 => '\u{e220}',
            Self::Tremolo2 => '\u{e221}',
            Self::Tremolo3 => '\u{e222}',
            Self::Tremolo4 => '\u{e223}',
            Self::Tremolo5 => '\u{e224}',
            Self::TremoloDivisiDots2 => '\u{e22e}',
            Self::TremoloDivisiDots3 => '\u{e22f}',
            Self::TremoloDivisiDots4 => '\u{e230}',
            Self::TremoloDivisiDots6 => '\u{e231}',
            Self::TremoloFingered1 => '\u{e225}',
            Self::TremoloFingered2 => '\u{e226}',
            Self::TremoloFingered3 => '\u{e227}',
            Self::TremoloFingered4 => '\u{e228}',
            Self::TremoloFingered5 => '\u{e229}',
            Self::TripleTongueAbove => '\u{e5f2}',
            Self::TripleTongueBelow => '\u{e5f3}',
            Self::Tuplet0 => '\u{e880}',
            Self::Tuplet1 => '\u{e881}',
            Self::Tuplet2 => '\u{e882}',
            Self::Tuplet3 => '\u{e883}',
            Self::Tuplet4 => '\u{e884}',
            Self::Tuplet5 => '\u{e885}',
            Self::Tuplet6 => '\u{e886}',
            Self::Tuplet7 => '\u{e887}',
            Self::Tuplet8 => '\u{e888}',
            Self::Tuplet9 => '\u{e889}',
            Self::TupletColon => '\u{e88a}',
            Self::UnmeasuredTremolo => '\u{e22c}',
            Self::UnmeasuredTremoloSimple => '\u{e22d}',
            Self::UnpitchedPercussionClef1 => '\u{e069}',
            Self::UnpitchedPercussionClef2 => '\u{e06a}',
            Self::Ventiduesima => '\u{e517}',
            Self::VentiduesimaAlta => '\u{e518}',
            Self::VentiduesimaBassa => '\u{e519}',
            Self::VentiduesimaBassaMb => '\u{e51e}',
            Self::VocalFingerClickStockhausen => '\u{e649}',
            Self::VocalHalbGesungen => '\u{e64b}',
            Self::VocalMouthClosed => '\u{e640}',
            Self::VocalMouthOpen => '\u{e642}',
            Self::VocalMouthPursed => '\u{e644}',
            Self::VocalMouthSlightlyOpen => '\u{e641}',
            Self::VocalMouthWideOpen => '\u{e643}',
            Self::VocalNasalVoice => '\u{e647}',
            Self::VocalSprechgesang => '\u{e645}',
            Self::VocalTongueClickStockhausen => '\u{e648}',
            Self::VocalTongueFingerClickStockhausen => '\u{e64a}',
            Self::VocalsSussurando => '\u{e646}',
            Self::WiggleArpeggiatoDown => '\u{eaaa}',
            Self::WiggleArpeggiatoDownArrow => '\u{eaae}',
            Self::WiggleArpeggiatoDownSwash => '\u{eaac}',
            Self::WiggleArpeggiatoUp => '\u{eaa9}',
            Self::WiggleArpeggiatoUpArrow => '\u{eaad}',
            Self::WiggleArpeggiatoUpSwash => '\u{eaab}',
            Self::WiggleCircular => '\u{eac9}',
            Self::WiggleCircularConstant => '\u{eac0}',
            Self::WiggleCircularConstantFlipped => '\u{eac1}',
            Self::WiggleCircularConstantFlippedLarge => '\u{eac3}',
            Self::WiggleCircularConstantLarge => '\u{eac2}',
            Self::WiggleCircularEnd => '\u{eacb}',
            Self::WiggleCircularLarge => '\u{eac8}',
            Self::WiggleCircularLarger => '\u{eac7}',
            Self::WiggleCircularLargerStill => '\u{eac6}',
            Self::WiggleCircularLargest => '\u{eac5}',
            Self::WiggleCircularSmall => '\u{eaca}',
            Self::WiggleCircularStart => '\u{eac4}',
            Self::WiggleGlissando => '\u{eaaf}',
            Self::WiggleGlissandoGroup1 => '\u{eabd}',
            Self::WiggleGlissandoGroup2 => '\u{eabe}',
            Self::WiggleGlissandoGroup3 => '\u{eabf}',
            Self::WiggleRandom1 => '\u{eaf0}',
            Self::WiggleRandom2 => '\u{eaf1}',
            Self::WiggleRandom3 => '\u{eaf2}',
            Self::WiggleRandom4 => '\u{eaf3}',
            Self::WiggleSawtooth => '\u{eabb}',
            Self::WiggleSawtoothNarrow => '\u{eaba}',
            Self::WiggleSawtoothWide => '\u{eabc}',
            Self::WiggleSquareWave => '\u{eab8}',
            Self::WiggleSquareWaveNarrow => '\u{eab7}',
            Self::WiggleSquareWaveWide => '\u{eab9}',
            Self::WiggleTrill => '\u{eaa4}',
            Self::WiggleTrillFast => '\u{eaa3}',
            Self::WiggleTrillFaster => '\u{eaa2}',
            Self::WiggleTrillFasterStill => '\u{eaa1}',
            Self::WiggleTrillFastest => '\u{eaa0}',
            Self::WiggleTrillSlow => '\u{eaa5}',
            Self::WiggleTrillSlower => '\u{eaa6}',
            Self::WiggleTrillSlowerStill => '\u{eaa7}',
            Self::WiggleTrillSlowest => '\u{eaa8}',
            Self::WiggleVIbratoLargestSlower => '\u{eaee}',
            Self::WiggleVIbratoMediumSlower => '\u{eae0}',
            Self::WiggleVibrato => '\u{eab0}',
            Self::WiggleVibratoLargeFast => '\u{eae5}',
            Self::WiggleVibratoLargeFaster => '\u{eae4}',
            Self::WiggleVibratoLargeFasterStill => '\u{eae3}',
            Self::WiggleVibratoLargeFastest => '\u{eae2}',
            Self::WiggleVibratoLargeSlow => '\u{eae6}',
            Self::WiggleVibratoLargeSlower => '\u{eae7}',
            Self::WiggleVibratoLargeSlowest => '\u{eae8}',
            Self::WiggleVibratoLargestFast => '\u{eaec}',
            Self::WiggleVibratoLargestFaster => '\u{eaeb}',
            Self::WiggleVibratoLargestFasterStill => '\u{eaea}',
            Self::WiggleVibratoLargestFastest => '\u{eae9}',
            Self::WiggleVibratoLargestSlow => '\u{eaed}',
            Self::WiggleVibratoLargestSlowest => '\u{eaef}',
            Self::WiggleVibratoMediumFast => '\u{eade}',
            Self::WiggleVibratoMediumFaster => '\u{eadd}',
            Self::WiggleVibratoMediumFasterStill => '\u{eadc}',
            Self::WiggleVibratoMediumFastest => '\u{eadb}',
            Self::WiggleVibratoMediumSlow => '\u{eadf}',
            Self::WiggleVibratoMediumSlowest => '\u{eae1}',
            Self::WiggleVibratoSmallFast => '\u{ead7}',
            Self::WiggleVibratoSmallFaster => '\u{ead6}',
            Self::WiggleVibratoSmallFasterStill => '\u{ead5}',
            Self::WiggleVibratoSmallFastest => '\u{ead4}',
            Self::WiggleVibratoSmallSlow => '\u{ead8}',
            Self::WiggleVibratoSmallSlower => '\u{ead9}',
            Self::WiggleVibratoSmallSlowest => '\u{eada}',
            Self::WiggleVibratoSmallestFast => '\u{ead0}',
            Self::WiggleVibratoSmallestFaster => '\u{eacf}',
            Self::WiggleVibratoSmallestFasterStill => '\u{eace}',
            Self::WiggleVibratoSmallestFastest => '\u{eacd}',
            Self::WiggleVibratoSmallestSlow => '\u{ead1}',
            Self::WiggleVibratoSmallestSlower => '\u{ead2}',
            Self::WiggleVibratoSmallestSlowest => '\u{ead3}',
            Self::WiggleVibratoStart => '\u{eacc}',
            Self::WiggleVibratoWide => '\u{eab1}',
            Self::WiggleWavy => '\u{eab5}',
            Self::WiggleWavyNarrow => '\u{eab4}',
            Self::WiggleWavyWide => '\u{eab6}',
            Self::WindClosedHole => '\u{e5f4}',
            Self::WindFlatEmbouchure => '\u{e5fb}',
            Self::WindHalfClosedHole1 => '\u{e5f6}',
            Self::WindHalfClosedHole2 => '\u{e5f7}',
            Self::WindHalfClosedHole3 => '\u{e5f8}',
            Self::WindLessRelaxedEmbouchure => '\u{e5fe}',
            Self::WindLessTightEmbouchure => '\u{e600}',
            Self::WindMouthpiecePop => '\u{e60a}',
            Self::WindMultiphonicsBlackStem => '\u{e607}',
            Self::WindMultiphonicsBlackWhiteStem => '\u{e609}',
            Self::WindMultiphonicsWhiteStem => '\u{e608}',
            Self::WindOpenHole => '\u{e5f9}',
            Self::WindReedPositionIn => '\u{e606}',
            Self::WindReedPositionNormal => '\u{e604}',
            Self::WindReedPositionOut => '\u{e605}',
            Self::WindRelaxedEmbouchure => '\u{e5fd}',
            Self::WindRimOnly => '\u{e60b}',
            Self::WindSharpEmbouchure => '\u{e5fc}',
            Self::WindStrongAirPressure => '\u{e603}',
            Self::WindThreeQuartersClosedHole => '\u{e5f5}',
            Self::WindTightEmbouchure => '\u{e5ff}',
            Self::WindTrillKey => '\u{e5fa}',
            Self::WindVeryTightEmbouchure => '\u{e601}',
            Self::WindWeakAirPressure => '\u{e602}',
            Self::_4StringTabClef => '\u{e06e}',
            Self::_6StringTabClef => '\u{e06d}',
        }
    }

    /// Unicode Musical Symbols range code point
    pub fn alternate_codepoint(&self) -> Option<char> {
        match self {
            Self::AccSagittal11LargeDiesisDown => None,
            Self::AccSagittal11LargeDiesisUp => None,
            Self::AccSagittal11MediumDiesisDown => None,
            Self::AccSagittal11MediumDiesisUp => None,
            Self::AccSagittal11V19LargeDiesisDown => None,
            Self::AccSagittal11V19LargeDiesisUp => None,
            Self::AccSagittal11V19MediumDiesisDown => None,
            Self::AccSagittal11V19MediumDiesisUp => None,
            Self::AccSagittal11V49CommaDown => None,
            Self::AccSagittal11V49CommaUp => None,
            Self::AccSagittal143CommaDown => None,
            Self::AccSagittal143CommaUp => None,
            Self::AccSagittal17CommaDown => None,
            Self::AccSagittal17CommaUp => None,
            Self::AccSagittal17KleismaDown => None,
            Self::AccSagittal17KleismaUp => None,
            Self::AccSagittal19CommaDown => None,
            Self::AccSagittal19CommaUp => None,
            Self::AccSagittal19SchismaDown => None,
            Self::AccSagittal19SchismaUp => None,
            Self::AccSagittal1MinaDown => None,
            Self::AccSagittal1MinaUp => None,
            Self::AccSagittal1TinaDown => None,
            Self::AccSagittal1TinaUp => None,
            Self::AccSagittal23CommaDown => None,
            Self::AccSagittal23CommaUp => None,
            Self::AccSagittal23SmallDiesisDown => None,
            Self::AccSagittal23SmallDiesisUp => None,
            Self::AccSagittal25SmallDiesisDown => None,
            Self::AccSagittal25SmallDiesisUp => None,
            Self::AccSagittal2MinasDown => None,
            Self::AccSagittal2MinasUp => None,
            Self::AccSagittal2TinasDown => None,
            Self::AccSagittal2TinasUp => None,
            Self::AccSagittal35LargeDiesisDown => None,
            Self::AccSagittal35LargeDiesisUp => None,
            Self::AccSagittal35MediumDiesisDown => None,
            Self::AccSagittal35MediumDiesisUp => None,
            Self::AccSagittal3TinasDown => None,
            Self::AccSagittal3TinasUp => None,
            Self::AccSagittal49LargeDiesisDown => None,
            Self::AccSagittal49LargeDiesisUp => None,
            Self::AccSagittal49MediumDiesisDown => None,
            Self::AccSagittal49MediumDiesisUp => None,
            Self::AccSagittal49SmallDiesisDown => None,
            Self::AccSagittal49SmallDiesisUp => None,
            Self::AccSagittal4TinasDown => None,
            Self::AccSagittal4TinasUp => None,
            Self::AccSagittal55CommaDown => None,
            Self::AccSagittal55CommaUp => None,
            Self::AccSagittal5CommaDown => None,
            Self::AccSagittal5CommaUp => None,
            Self::AccSagittal5TinasDown => None,
            Self::AccSagittal5TinasUp => None,
            Self::AccSagittal5V11SmallDiesisDown => None,
            Self::AccSagittal5V11SmallDiesisUp => None,
            Self::AccSagittal5V13LargeDiesisDown => None,
            Self::AccSagittal5V13LargeDiesisUp => None,
            Self::AccSagittal5V13MediumDiesisDown => None,
            Self::AccSagittal5V13MediumDiesisUp => None,
            Self::AccSagittal5V19CommaDown => None,
            Self::AccSagittal5V19CommaUp => None,
            Self::AccSagittal5V23SmallDiesisDown => None,
            Self::AccSagittal5V23SmallDiesisUp => None,
            Self::AccSagittal5V49MediumDiesisDown => None,
            Self::AccSagittal5V49MediumDiesisUp => None,
            Self::AccSagittal5V7KleismaDown => None,
            Self::AccSagittal5V7KleismaUp => None,
            Self::AccSagittal6TinasDown => None,
            Self::AccSagittal6TinasUp => None,
            Self::AccSagittal7CommaDown => None,
            Self::AccSagittal7CommaUp => None,
            Self::AccSagittal7TinasDown => None,
            Self::AccSagittal7TinasUp => None,
            Self::AccSagittal7V11CommaDown => None,
            Self::AccSagittal7V11CommaUp => None,
            Self::AccSagittal7V11KleismaDown => None,
            Self::AccSagittal7V11KleismaUp => None,
            Self::AccSagittal7V19CommaDown => None,
            Self::AccSagittal7V19CommaUp => None,
            Self::AccSagittal8TinasDown => None,
            Self::AccSagittal8TinasUp => None,
            Self::AccSagittal9TinasDown => None,
            Self::AccSagittal9TinasUp => None,
            Self::AccSagittalAcute => None,
            Self::AccSagittalDoubleFlat => None,
            Self::AccSagittalDoubleFlat11V49CUp => None,
            Self::AccSagittalDoubleFlat143CUp => None,
            Self::AccSagittalDoubleFlat17CUp => None,
            Self::AccSagittalDoubleFlat17KUp => None,
            Self::AccSagittalDoubleFlat19CUp => None,
            Self::AccSagittalDoubleFlat19SUp => None,
            Self::AccSagittalDoubleFlat23CUp => None,
            Self::AccSagittalDoubleFlat23SUp => None,
            Self::AccSagittalDoubleFlat25SUp => None,
            Self::AccSagittalDoubleFlat49SUp => None,
            Self::AccSagittalDoubleFlat55CUp => None,
            Self::AccSagittalDoubleFlat5CUp => None,
            Self::AccSagittalDoubleFlat5V11SUp => None,
            Self::AccSagittalDoubleFlat5V19CUp => None,
            Self::AccSagittalDoubleFlat5V23SUp => None,
            Self::AccSagittalDoubleFlat5V7KUp => None,
            Self::AccSagittalDoubleFlat7CUp => None,
            Self::AccSagittalDoubleFlat7V11CUp => None,
            Self::AccSagittalDoubleFlat7V11KUp => None,
            Self::AccSagittalDoubleFlat7V19CUp => None,
            Self::AccSagittalDoubleSharp => None,
            Self::AccSagittalDoubleSharp11V49CDown => None,
            Self::AccSagittalDoubleSharp143CDown => None,
            Self::AccSagittalDoubleSharp17CDown => None,
            Self::AccSagittalDoubleSharp17KDown => None,
            Self::AccSagittalDoubleSharp19CDown => None,
            Self::AccSagittalDoubleSharp19SDown => None,
            Self::AccSagittalDoubleSharp23CDown => None,
            Self::AccSagittalDoubleSharp23SDown => None,
            Self::AccSagittalDoubleSharp25SDown => None,
            Self::AccSagittalDoubleSharp49SDown => None,
            Self::AccSagittalDoubleSharp55CDown => None,
            Self::AccSagittalDoubleSharp5CDown => None,
            Self::AccSagittalDoubleSharp5V11SDown => None,
            Self::AccSagittalDoubleSharp5V19CDown => None,
            Self::AccSagittalDoubleSharp5V23SDown => None,
            Self::AccSagittalDoubleSharp5V7KDown => None,
            Self::AccSagittalDoubleSharp7CDown => None,
            Self::AccSagittalDoubleSharp7V11CDown => None,
            Self::AccSagittalDoubleSharp7V11KDown => None,
            Self::AccSagittalDoubleSharp7V19CDown => None,
            Self::AccSagittalFlat => None,
            Self::AccSagittalFlat11LDown => None,
            Self::AccSagittalFlat11MDown => None,
            Self::AccSagittalFlat11V19LDown => None,
            Self::AccSagittalFlat11V19MDown => None,
            Self::AccSagittalFlat11V49CDown => None,
            Self::AccSagittalFlat11V49CUp => None,
            Self::AccSagittalFlat143CDown => None,
            Self::AccSagittalFlat143CUp => None,
            Self::AccSagittalFlat17CDown => None,
            Self::AccSagittalFlat17CUp => None,
            Self::AccSagittalFlat17KDown => None,
            Self::AccSagittalFlat17KUp => None,
            Self::AccSagittalFlat19CDown => None,
            Self::AccSagittalFlat19CUp => None,
            Self::AccSagittalFlat19SDown => None,
            Self::AccSagittalFlat19SUp => None,
            Self::AccSagittalFlat23CDown => None,
            Self::AccSagittalFlat23CUp => None,
            Self::AccSagittalFlat23SDown => None,
            Self::AccSagittalFlat23SUp => None,
            Self::AccSagittalFlat25SDown => None,
            Self::AccSagittalFlat25SUp => None,
            Self::AccSagittalFlat35LDown => None,
            Self::AccSagittalFlat35MDown => None,
            Self::AccSagittalFlat49LDown => None,
            Self::AccSagittalFlat49MDown => None,
            Self::AccSagittalFlat49SDown => None,
            Self::AccSagittalFlat49SUp => None,
            Self::AccSagittalFlat55CDown => None,
            Self::AccSagittalFlat55CUp => None,
            Self::AccSagittalFlat5CDown => None,
            Self::AccSagittalFlat5CUp => None,
            Self::AccSagittalFlat5V11SDown => None,
            Self::AccSagittalFlat5V11SUp => None,
            Self::AccSagittalFlat5V13LDown => None,
            Self::AccSagittalFlat5V13MDown => None,
            Self::AccSagittalFlat5V19CDown => None,
            Self::AccSagittalFlat5V19CUp => None,
            Self::AccSagittalFlat5V23SDown => None,
            Self::AccSagittalFlat5V23SUp => None,
            Self::AccSagittalFlat5V49MDown => None,
            Self::AccSagittalFlat5V7KDown => None,
            Self::AccSagittalFlat5V7KUp => None,
            Self::AccSagittalFlat7CDown => None,
            Self::AccSagittalFlat7CUp => None,
            Self::AccSagittalFlat7V11CDown => None,
            Self::AccSagittalFlat7V11CUp => None,
            Self::AccSagittalFlat7V11KDown => None,
            Self::AccSagittalFlat7V11KUp => None,
            Self::AccSagittalFlat7V19CDown => None,
            Self::AccSagittalFlat7V19CUp => None,
            Self::AccSagittalFractionalTinaDown => None,
            Self::AccSagittalFractionalTinaUp => None,
            Self::AccSagittalGrave => None,
            Self::AccSagittalShaftDown => None,
            Self::AccSagittalShaftUp => None,
            Self::AccSagittalSharp => None,
            Self::AccSagittalSharp11LUp => None,
            Self::AccSagittalSharp11MUp => None,
            Self::AccSagittalSharp11V19LUp => None,
            Self::AccSagittalSharp11V19MUp => None,
            Self::AccSagittalSharp11V49CDown => None,
            Self::AccSagittalSharp11V49CUp => None,
            Self::AccSagittalSharp143CDown => None,
            Self::AccSagittalSharp143CUp => None,
            Self::AccSagittalSharp17CDown => None,
            Self::AccSagittalSharp17CUp => None,
            Self::AccSagittalSharp17KDown => None,
            Self::AccSagittalSharp17KUp => None,
            Self::AccSagittalSharp19CDown => None,
            Self::AccSagittalSharp19CUp => None,
            Self::AccSagittalSharp19SDown => None,
            Self::AccSagittalSharp19SUp => None,
            Self::AccSagittalSharp23CDown => None,
            Self::AccSagittalSharp23CUp => None,
            Self::AccSagittalSharp23SDown => None,
            Self::AccSagittalSharp23SUp => None,
            Self::AccSagittalSharp25SDown => None,
            Self::AccSagittalSharp25SUp => None,
            Self::AccSagittalSharp35LUp => None,
            Self::AccSagittalSharp35MUp => None,
            Self::AccSagittalSharp49LUp => None,
            Self::AccSagittalSharp49MUp => None,
            Self::AccSagittalSharp49SDown => None,
            Self::AccSagittalSharp49SUp => None,
            Self::AccSagittalSharp55CDown => None,
            Self::AccSagittalSharp55CUp => None,
            Self::AccSagittalSharp5CDown => None,
            Self::AccSagittalSharp5CUp => None,
            Self::AccSagittalSharp5V11SDown => None,
            Self::AccSagittalSharp5V11SUp => None,
            Self::AccSagittalSharp5V13LUp => None,
            Self::AccSagittalSharp5V13MUp => None,
            Self::AccSagittalSharp5V19CDown => None,
            Self::AccSagittalSharp5V19CUp => None,
            Self::AccSagittalSharp5V23SDown => None,
            Self::AccSagittalSharp5V23SUp => None,
            Self::AccSagittalSharp5V49MUp => None,
            Self::AccSagittalSharp5V7KDown => None,
            Self::AccSagittalSharp5V7KUp => None,
            Self::AccSagittalSharp7CDown => None,
            Self::AccSagittalSharp7CUp => None,
            Self::AccSagittalSharp7V11CDown => None,
            Self::AccSagittalSharp7V11CUp => None,
            Self::AccSagittalSharp7V11KDown => None,
            Self::AccSagittalSharp7V11KUp => None,
            Self::AccSagittalSharp7V19CDown => None,
            Self::AccSagittalSharp7V19CUp => None,
            Self::AccSagittalUnused1 => None,
            Self::AccSagittalUnused2 => None,
            Self::AccSagittalUnused3 => None,
            Self::AccSagittalUnused4 => None,
            Self::AccdnCombDot => None,
            Self::AccdnCombLh2RanksEmpty => None,
            Self::AccdnCombLh3RanksEmptySquare => None,
            Self::AccdnCombRh3RanksEmpty => None,
            Self::AccdnCombRh4RanksEmpty => None,
            Self::AccdnDiatonicClef => None,
            Self::AccdnLh2Ranks16Round => None,
            Self::AccdnLh2Ranks8Plus16Round => None,
            Self::AccdnLh2Ranks8Round => None,
            Self::AccdnLh2RanksFullMasterRound => None,
            Self::AccdnLh2RanksMasterPlus16Round => None,
            Self::AccdnLh2RanksMasterRound => None,
            Self::AccdnLh3Ranks2Plus8Square => None,
            Self::AccdnLh3Ranks2Square => None,
            Self::AccdnLh3Ranks8Square => None,
            Self::AccdnLh3RanksDouble8Square => None,
            Self::AccdnLh3RanksTuttiSquare => None,
            Self::AccdnPull => None,
            Self::AccdnPush => None,
            Self::AccdnRh3RanksAccordion => None,
            Self::AccdnRh3RanksAuthenticMusette => None,
            Self::AccdnRh3RanksBandoneon => None,
            Self::AccdnRh3RanksBassoon => None,
            Self::AccdnRh3RanksClarinet => None,
            Self::AccdnRh3RanksDoubleTremoloLower8Ve => None,
            Self::AccdnRh3RanksDoubleTremoloUpper8Ve => None,
            Self::AccdnRh3RanksFullFactory => None,
            Self::AccdnRh3RanksHarmonium => None,
            Self::AccdnRh3RanksImitationMusette => None,
            Self::AccdnRh3RanksLowerTremolo8 => None,
            Self::AccdnRh3RanksMaster => None,
            Self::AccdnRh3RanksOboe => None,
            Self::AccdnRh3RanksOrgan => None,
            Self::AccdnRh3RanksPiccolo => None,
            Self::AccdnRh3RanksTremoloLower8Ve => None,
            Self::AccdnRh3RanksTremoloUpper8Ve => None,
            Self::AccdnRh3RanksTwoChoirs => None,
            Self::AccdnRh3RanksUpperTremolo8 => None,
            Self::AccdnRh3RanksViolin => None,
            Self::AccdnRh4RanksAlto => None,
            Self::AccdnRh4RanksBassAlto => None,
            Self::AccdnRh4RanksMaster => None,
            Self::AccdnRh4RanksSoftBass => None,
            Self::AccdnRh4RanksSoftTenor => None,
            Self::AccdnRh4RanksSoprano => None,
            Self::AccdnRh4RanksTenor => None,
            Self::AccdnRicochet2 => None,
            Self::AccdnRicochet3 => None,
            Self::AccdnRicochet4 => None,
            Self::AccdnRicochet5 => None,
            Self::AccdnRicochet6 => None,
            Self::AccdnRicochetStem2 => None,
            Self::AccdnRicochetStem3 => None,
            Self::AccdnRicochetStem4 => None,
            Self::AccdnRicochetStem5 => None,
            Self::AccdnRicochetStem6 => None,
            Self::Accidental1CommaFlat => None,
            Self::Accidental1CommaSharp => None,
            Self::Accidental2CommaFlat => None,
            Self::Accidental2CommaSharp => None,
            Self::Accidental3CommaFlat => None,
            Self::Accidental3CommaSharp => None,
            Self::Accidental4CommaFlat => None,
            Self::Accidental5CommaSharp => None,
            Self::AccidentalArrowDown => None,
            Self::AccidentalArrowUp => None,
            Self::AccidentalBakiyeFlat => None,
            Self::AccidentalBakiyeSharp => None,
            Self::AccidentalBracketLeft => None,
            Self::AccidentalBracketRight => None,
            Self::AccidentalBuyukMucennebFlat => None,
            Self::AccidentalBuyukMucennebSharp => None,
            Self::AccidentalCombiningCloseCurlyBrace => None,
            Self::AccidentalCombiningLower17Schisma => None,
            Self::AccidentalCombiningLower19Schisma => None,
            Self::AccidentalCombiningLower23Limit29LimitComma => None,
            Self::AccidentalCombiningLower29LimitComma => None,
            Self::AccidentalCombiningLower31Schisma => None,
            Self::AccidentalCombiningLower37Quartertone => None,
            Self::AccidentalCombiningLower41Comma => None,
            Self::AccidentalCombiningLower43Comma => None,
            Self::AccidentalCombiningLower47Quartertone => None,
            Self::AccidentalCombiningLower53LimitComma => None,
            Self::AccidentalCombiningOpenCurlyBrace => None,
            Self::AccidentalCombiningRaise17Schisma => None,
            Self::AccidentalCombiningRaise19Schisma => None,
            Self::AccidentalCombiningRaise23Limit29LimitComma => None,
            Self::AccidentalCombiningRaise29LimitComma => None,
            Self::AccidentalCombiningRaise31Schisma => None,
            Self::AccidentalCombiningRaise37Quartertone => None,
            Self::AccidentalCombiningRaise41Comma => None,
            Self::AccidentalCombiningRaise43Comma => None,
            Self::AccidentalCombiningRaise47Quartertone => None,
            Self::AccidentalCombiningRaise53LimitComma => None,
            Self::AccidentalCommaSlashDown => None,
            Self::AccidentalCommaSlashUp => None,
            Self::AccidentalDoubleFlat => Some('𝄫'),
            Self::AccidentalDoubleFlatArabic => None,
            Self::AccidentalDoubleFlatEqualTempered => None,
            Self::AccidentalDoubleFlatOneArrowDown => None,
            Self::AccidentalDoubleFlatOneArrowUp => None,
            Self::AccidentalDoubleFlatReversed => None,
            Self::AccidentalDoubleFlatThreeArrowsDown => None,
            Self::AccidentalDoubleFlatThreeArrowsUp => None,
            Self::AccidentalDoubleFlatTurned => None,
            Self::AccidentalDoubleFlatTwoArrowsDown => None,
            Self::AccidentalDoubleFlatTwoArrowsUp => None,
            Self::AccidentalDoubleSharp => Some('𝄪'),
            Self::AccidentalDoubleSharpArabic => None,
            Self::AccidentalDoubleSharpEqualTempered => None,
            Self::AccidentalDoubleSharpOneArrowDown => None,
            Self::AccidentalDoubleSharpOneArrowUp => None,
            Self::AccidentalDoubleSharpThreeArrowsDown => None,
            Self::AccidentalDoubleSharpThreeArrowsUp => None,
            Self::AccidentalDoubleSharpTwoArrowsDown => None,
            Self::AccidentalDoubleSharpTwoArrowsUp => None,
            Self::AccidentalEnharmonicAlmostEqualTo => None,
            Self::AccidentalEnharmonicEquals => None,
            Self::AccidentalEnharmonicTilde => None,
            Self::AccidentalFilledReversedFlatAndFlat => None,
            Self::AccidentalFilledReversedFlatAndFlatArrowDown => None,
            Self::AccidentalFilledReversedFlatAndFlatArrowUp => None,
            Self::AccidentalFilledReversedFlatArrowDown => None,
            Self::AccidentalFilledReversedFlatArrowUp => None,
            Self::AccidentalFiveQuarterTonesFlatArrowDown => None,
            Self::AccidentalFiveQuarterTonesSharpArrowUp => None,
            Self::AccidentalFlat => Some('♭'),
            Self::AccidentalFlatArabic => None,
            Self::AccidentalFlatEqualTempered => None,
            Self::AccidentalFlatLoweredStockhausen => None,
            Self::AccidentalFlatOneArrowDown => None,
            Self::AccidentalFlatOneArrowUp => None,
            Self::AccidentalFlatRaisedStockhausen => None,
            Self::AccidentalFlatRepeatedLineStockhausen => None,
            Self::AccidentalFlatRepeatedSpaceStockhausen => None,
            Self::AccidentalFlatThreeArrowsDown => None,
            Self::AccidentalFlatThreeArrowsUp => None,
            Self::AccidentalFlatTurned => None,
            Self::AccidentalFlatTwoArrowsDown => None,
            Self::AccidentalFlatTwoArrowsUp => None,
            Self::AccidentalHabaFlatQuarterToneHigher => None,
            Self::AccidentalHabaFlatThreeQuarterTonesLower => None,
            Self::AccidentalHabaQuarterToneHigher => None,
            Self::AccidentalHabaQuarterToneLower => None,
            Self::AccidentalHabaSharpQuarterToneLower => None,
            Self::AccidentalHabaSharpThreeQuarterTonesHigher => None,
            Self::AccidentalHalfSharpArrowDown => None,
            Self::AccidentalHalfSharpArrowUp => None,
            Self::AccidentalJohnston13 => None,
            Self::AccidentalJohnston31 => None,
            Self::AccidentalJohnstonDown => None,
            Self::AccidentalJohnstonEl => None,
            Self::AccidentalJohnstonMinus => None,
            Self::AccidentalJohnstonPlus => None,
            Self::AccidentalJohnstonSeven => None,
            Self::AccidentalJohnstonUp => None,
            Self::AccidentalKomaFlat => None,
            Self::AccidentalKomaSharp => None,
            Self::AccidentalKoron => None,
            Self::AccidentalKucukMucennebFlat => None,
            Self::AccidentalKucukMucennebSharp => None,
            Self::AccidentalLargeDoubleSharp => None,
            Self::AccidentalLowerOneSeptimalComma => None,
            Self::AccidentalLowerOneTridecimalQuartertone => None,
            Self::AccidentalLowerOneUndecimalQuartertone => None,
            Self::AccidentalLowerTwoSeptimalCommas => None,
            Self::AccidentalLoweredStockhausen => None,
            Self::AccidentalNarrowReversedFlat => None,
            Self::AccidentalNarrowReversedFlatAndFlat => None,
            Self::AccidentalNatural => Some('♮'),
            Self::AccidentalNaturalArabic => None,
            Self::AccidentalNaturalEqualTempered => None,
            Self::AccidentalNaturalFlat => None,
            Self::AccidentalNaturalLoweredStockhausen => None,
            Self::AccidentalNaturalOneArrowDown => None,
            Self::AccidentalNaturalOneArrowUp => None,
            Self::AccidentalNaturalRaisedStockhausen => None,
            Self::AccidentalNaturalReversed => None,
            Self::AccidentalNaturalSharp => None,
            Self::AccidentalNaturalThreeArrowsDown => None,
            Self::AccidentalNaturalThreeArrowsUp => None,
            Self::AccidentalNaturalTwoArrowsDown => None,
            Self::AccidentalNaturalTwoArrowsUp => None,
            Self::AccidentalOneAndAHalfSharpsArrowDown => None,
            Self::AccidentalOneAndAHalfSharpsArrowUp => None,
            Self::AccidentalOneQuarterToneFlatFerneyhough => None,
            Self::AccidentalOneQuarterToneFlatStockhausen => None,
            Self::AccidentalOneQuarterToneSharpFerneyhough => None,
            Self::AccidentalOneQuarterToneSharpStockhausen => None,
            Self::AccidentalOneThirdToneFlatFerneyhough => None,
            Self::AccidentalOneThirdToneSharpFerneyhough => None,
            Self::AccidentalParensLeft => None,
            Self::AccidentalParensRight => None,
            Self::AccidentalQuarterFlatEqualTempered => None,
            Self::AccidentalQuarterSharpEqualTempered => None,
            Self::AccidentalQuarterToneFlat4 => Some('𝄳'),
            Self::AccidentalQuarterToneFlatArabic => None,
            Self::AccidentalQuarterToneFlatArrowUp => Some('𝄬'),
            Self::AccidentalQuarterToneFlatFilledReversed => None,
            Self::AccidentalQuarterToneFlatNaturalArrowDown => Some('𝄯'),
            Self::AccidentalQuarterToneFlatPenderecki => None,
            Self::AccidentalQuarterToneFlatStein => None,
            Self::AccidentalQuarterToneFlatVanBlankenburg => None,
            Self::AccidentalQuarterToneSharp4 => Some('𝄲'),
            Self::AccidentalQuarterToneSharpArabic => None,
            Self::AccidentalQuarterToneSharpArrowDown => Some('𝄱'),
            Self::AccidentalQuarterToneSharpBusotti => None,
            Self::AccidentalQuarterToneSharpNaturalArrowUp => Some('𝄮'),
            Self::AccidentalQuarterToneSharpStein => None,
            Self::AccidentalQuarterToneSharpWiggle => None,
            Self::AccidentalRaiseOneSeptimalComma => None,
            Self::AccidentalRaiseOneTridecimalQuartertone => None,
            Self::AccidentalRaiseOneUndecimalQuartertone => None,
            Self::AccidentalRaiseTwoSeptimalCommas => None,
            Self::AccidentalRaisedStockhausen => None,
            Self::AccidentalReversedFlatAndFlatArrowDown => None,
            Self::AccidentalReversedFlatAndFlatArrowUp => None,
            Self::AccidentalReversedFlatArrowDown => None,
            Self::AccidentalReversedFlatArrowUp => None,
            Self::AccidentalSharp => Some('♯'),
            Self::AccidentalSharpArabic => None,
            Self::AccidentalSharpEqualTempered => None,
            Self::AccidentalSharpLoweredStockhausen => None,
            Self::AccidentalSharpOneArrowDown => None,
            Self::AccidentalSharpOneArrowUp => None,
            Self::AccidentalSharpOneHorizontalStroke => None,
            Self::AccidentalSharpRaisedStockhausen => None,
            Self::AccidentalSharpRepeatedLineStockhausen => None,
            Self::AccidentalSharpRepeatedSpaceStockhausen => None,
            Self::AccidentalSharpReversed => None,
            Self::AccidentalSharpSharp => None,
            Self::AccidentalSharpThreeArrowsDown => None,
            Self::AccidentalSharpThreeArrowsUp => None,
            Self::AccidentalSharpTwoArrowsDown => None,
            Self::AccidentalSharpTwoArrowsUp => None,
            Self::AccidentalSims12Down => None,
            Self::AccidentalSims12Up => None,
            Self::AccidentalSims4Down => None,
            Self::AccidentalSims4Up => None,
            Self::AccidentalSims6Down => None,
            Self::AccidentalSims6Up => None,
            Self::AccidentalSori => None,
            Self::AccidentalTavenerFlat => None,
            Self::AccidentalTavenerSharp => None,
            Self::AccidentalThreeQuarterTonesFlatArabic => None,
            Self::AccidentalThreeQuarterTonesFlatArrowDown => Some('𝄭'),
            Self::AccidentalThreeQuarterTonesFlatArrowUp => None,
            Self::AccidentalThreeQuarterTonesFlatCouper => None,
            Self::AccidentalThreeQuarterTonesFlatGrisey => None,
            Self::AccidentalThreeQuarterTonesFlatTartini => None,
            Self::AccidentalThreeQuarterTonesFlatZimmermann => None,
            Self::AccidentalThreeQuarterTonesSharpArabic => None,
            Self::AccidentalThreeQuarterTonesSharpArrowDown => None,
            Self::AccidentalThreeQuarterTonesSharpArrowUp => Some('𝄰'),
            Self::AccidentalThreeQuarterTonesSharpBusotti => None,
            Self::AccidentalThreeQuarterTonesSharpStein => None,
            Self::AccidentalThreeQuarterTonesSharpStockhausen => None,
            Self::AccidentalTripleFlat => None,
            Self::AccidentalTripleSharp => None,
            Self::AccidentalTwoThirdTonesFlatFerneyhough => None,
            Self::AccidentalTwoThirdTonesSharpFerneyhough => None,
            Self::AccidentalUpsAndDownsDown => None,
            Self::AccidentalUpsAndDownsLess => None,
            Self::AccidentalUpsAndDownsMore => None,
            Self::AccidentalUpsAndDownsUp => None,
            Self::AccidentalWilsonMinus => None,
            Self::AccidentalWilsonPlus => None,
            Self::AccidentalWyschnegradsky10TwelfthsFlat => None,
            Self::AccidentalWyschnegradsky10TwelfthsSharp => None,
            Self::AccidentalWyschnegradsky11TwelfthsFlat => None,
            Self::AccidentalWyschnegradsky11TwelfthsSharp => None,
            Self::AccidentalWyschnegradsky1TwelfthsFlat => None,
            Self::AccidentalWyschnegradsky1TwelfthsSharp => None,
            Self::AccidentalWyschnegradsky2TwelfthsFlat => None,
            Self::AccidentalWyschnegradsky2TwelfthsSharp => None,
            Self::AccidentalWyschnegradsky3TwelfthsFlat => None,
            Self::AccidentalWyschnegradsky3TwelfthsSharp => None,
            Self::AccidentalWyschnegradsky4TwelfthsFlat => None,
            Self::AccidentalWyschnegradsky4TwelfthsSharp => None,
            Self::AccidentalWyschnegradsky5TwelfthsFlat => None,
            Self::AccidentalWyschnegradsky5TwelfthsSharp => None,
            Self::AccidentalWyschnegradsky6TwelfthsFlat => None,
            Self::AccidentalWyschnegradsky6TwelfthsSharp => None,
            Self::AccidentalWyschnegradsky7TwelfthsFlat => None,
            Self::AccidentalWyschnegradsky7TwelfthsSharp => None,
            Self::AccidentalWyschnegradsky8TwelfthsFlat => None,
            Self::AccidentalWyschnegradsky8TwelfthsSharp => None,
            Self::AccidentalWyschnegradsky9TwelfthsFlat => None,
            Self::AccidentalWyschnegradsky9TwelfthsSharp => None,
            Self::AccidentalXenakisOneThirdToneSharp => None,
            Self::AccidentalXenakisTwoThirdTonesSharp => None,
            Self::AnalyticsChoralmelodie => None,
            Self::AnalyticsEndStimme => Some('𝆨'),
            Self::AnalyticsHauptrhythmus => None,
            Self::AnalyticsHauptstimme => Some('𝆦'),
            Self::AnalyticsInversion1 => None,
            Self::AnalyticsNebenstimme => Some('𝆧'),
            Self::AnalyticsStartStimme => None,
            Self::AnalyticsTheme => None,
            Self::AnalyticsTheme1 => None,
            Self::AnalyticsThemeInversion => None,
            Self::AnalyticsThemeRetrograde => None,
            Self::AnalyticsThemeRetrogradeInversion => None,
            Self::Arpeggiato => None,
            Self::ArpeggiatoDown => Some('𝆄'),
            Self::ArpeggiatoUp => Some('𝆃'),
            Self::ArrowBlackDown => None,
            Self::ArrowBlackDownLeft => None,
            Self::ArrowBlackDownRight => None,
            Self::ArrowBlackLeft => None,
            Self::ArrowBlackRight => None,
            Self::ArrowBlackUp => None,
            Self::ArrowBlackUpLeft => None,
            Self::ArrowBlackUpRight => None,
            Self::ArrowOpenDown => None,
            Self::ArrowOpenDownLeft => None,
            Self::ArrowOpenDownRight => None,
            Self::ArrowOpenLeft => None,
            Self::ArrowOpenRight => None,
            Self::ArrowOpenUp => None,
            Self::ArrowOpenUpLeft => None,
            Self::ArrowOpenUpRight => None,
            Self::ArrowWhiteDown => None,
            Self::ArrowWhiteDownLeft => None,
            Self::ArrowWhiteDownRight => None,
            Self::ArrowWhiteLeft => None,
            Self::ArrowWhiteRight => None,
            Self::ArrowWhiteUp => None,
            Self::ArrowWhiteUpLeft => None,
            Self::ArrowWhiteUpRight => None,
            Self::ArrowheadBlackDown => None,
            Self::ArrowheadBlackDownLeft => None,
            Self::ArrowheadBlackDownRight => None,
            Self::ArrowheadBlackLeft => None,
            Self::ArrowheadBlackRight => None,
            Self::ArrowheadBlackUp => None,
            Self::ArrowheadBlackUpLeft => None,
            Self::ArrowheadBlackUpRight => None,
            Self::ArrowheadOpenDown => None,
            Self::ArrowheadOpenDownLeft => None,
            Self::ArrowheadOpenDownRight => None,
            Self::ArrowheadOpenLeft => None,
            Self::ArrowheadOpenRight => None,
            Self::ArrowheadOpenUp => None,
            Self::ArrowheadOpenUpLeft => None,
            Self::ArrowheadOpenUpRight => None,
            Self::ArrowheadWhiteDown => None,
            Self::ArrowheadWhiteDownLeft => None,
            Self::ArrowheadWhiteDownRight => None,
            Self::ArrowheadWhiteLeft => None,
            Self::ArrowheadWhiteRight => None,
            Self::ArrowheadWhiteUp => None,
            Self::ArrowheadWhiteUpLeft => None,
            Self::ArrowheadWhiteUpRight => None,
            Self::ArticAccentAbove => Some('\u{1d17b}'),
            Self::ArticAccentBelow => None,
            Self::ArticAccentStaccatoAbove => Some('\u{1d181}'),
            Self::ArticAccentStaccatoBelow => None,
            Self::ArticLaissezVibrerAbove => None,
            Self::ArticLaissezVibrerBelow => None,
            Self::ArticMarcatoAbove => Some('\u{1d17f}'),
            Self::ArticMarcatoBelow => None,
            Self::ArticMarcatoStaccatoAbove => Some('\u{1d180}'),
            Self::ArticMarcatoStaccatoBelow => None,
            Self::ArticMarcatoTenutoAbove => None,
            Self::ArticMarcatoTenutoBelow => None,
            Self::ArticSoftAccentAbove => None,
            Self::ArticSoftAccentBelow => None,
            Self::ArticSoftAccentStaccatoAbove => None,
            Self::ArticSoftAccentStaccatoBelow => None,
            Self::ArticSoftAccentTenutoAbove => None,
            Self::ArticSoftAccentTenutoBelow => None,
            Self::ArticSoftAccentTenutoStaccatoAbove => None,
            Self::ArticSoftAccentTenutoStaccatoBelow => None,
            Self::ArticStaccatissimoAbove => Some('\u{1d17e}'),
            Self::ArticStaccatissimoBelow => None,
            Self::ArticStaccatissimoStrokeAbove => None,
            Self::ArticStaccatissimoStrokeBelow => None,
            Self::ArticStaccatissimoWedgeAbove => None,
            Self::ArticStaccatissimoWedgeBelow => None,
            Self::ArticStaccatoAbove => Some('\u{1d17c}'),
            Self::ArticStaccatoBelow => None,
            Self::ArticStressAbove => None,
            Self::ArticStressBelow => None,
            Self::ArticTenutoAbove => Some('\u{1d17d}'),
            Self::ArticTenutoAccentAbove => None,
            Self::ArticTenutoAccentBelow => None,
            Self::ArticTenutoBelow => None,
            Self::ArticTenutoStaccatoAbove => Some('\u{1d182}'),
            Self::ArticTenutoStaccatoBelow => None,
            Self::ArticUnstressAbove => None,
            Self::ArticUnstressBelow => None,
            Self::AugmentationDot => Some('𝅭'),
            Self::BarlineDashed => Some('𝄄'),
            Self::BarlineDotted => None,
            Self::BarlineDouble => Some('𝄁'),
            Self::BarlineFinal => Some('𝄂'),
            Self::BarlineHeavy => None,
            Self::BarlineHeavyHeavy => None,
            Self::BarlineReverseFinal => Some('𝄃'),
            Self::BarlineShort => Some('𝄅'),
            Self::BarlineSingle => Some('𝄀'),
            Self::BarlineTick => None,
            Self::BeamAccelRit1 => None,
            Self::BeamAccelRit10 => None,
            Self::BeamAccelRit11 => None,
            Self::BeamAccelRit12 => None,
            Self::BeamAccelRit13 => None,
            Self::BeamAccelRit14 => None,
            Self::BeamAccelRit15 => None,
            Self::BeamAccelRit2 => None,
            Self::BeamAccelRit3 => None,
            Self::BeamAccelRit4 => None,
            Self::BeamAccelRit5 => None,
            Self::BeamAccelRit6 => None,
            Self::BeamAccelRit7 => None,
            Self::BeamAccelRit8 => None,
            Self::BeamAccelRit9 => None,
            Self::BeamAccelRitFinal => None,
            Self::Brace => Some('𝄔'),
            Self::Bracket => Some('𝄕'),
            Self::BracketBottom => None,
            Self::BracketTop => None,
            Self::BrassBend => Some('\u{1d189}'),
            Self::BrassDoitLong => None,
            Self::BrassDoitMedium => None,
            Self::BrassDoitShort => Some('\u{1d185}'),
            Self::BrassFallLipLong => None,
            Self::BrassFallLipMedium => None,
            Self::BrassFallLipShort => Some('\u{1d186}'),
            Self::BrassFallRoughLong => None,
            Self::BrassFallRoughMedium => None,
            Self::BrassFallRoughShort => None,
            Self::BrassFallSmoothLong => None,
            Self::BrassFallSmoothMedium => None,
            Self::BrassFallSmoothShort => None,
            Self::BrassFlip => Some('\u{1d187}'),
            Self::BrassHarmonMuteClosed => None,
            Self::BrassHarmonMuteStemHalfLeft => None,
            Self::BrassHarmonMuteStemHalfRight => None,
            Self::BrassHarmonMuteStemOpen => None,
            Self::BrassJazzTurn => None,
            Self::BrassLiftLong => None,
            Self::BrassLiftMedium => None,
            Self::BrassLiftShort => None,
            Self::BrassLiftSmoothLong => None,
            Self::BrassLiftSmoothMedium => None,
            Self::BrassLiftSmoothShort => None,
            Self::BrassMuteClosed => None,
            Self::BrassMuteHalfClosed => None,
            Self::BrassMuteOpen => None,
            Self::BrassPlop => None,
            Self::BrassScoop => None,
            Self::BrassSmear => Some('\u{1d188}'),
            Self::BrassValveTrill => None,
            Self::BreathMarkComma => Some('𝄒'),
            Self::BreathMarkSalzedo => None,
            Self::BreathMarkTick => None,
            Self::BreathMarkUpbow => None,
            Self::BridgeClef => None,
            Self::BuzzRoll => None,
            Self::CClef => Some('𝄡'),
            Self::CClef8Vb => None,
            Self::CClefArrowDown => None,
            Self::CClefArrowUp => None,
            Self::CClefChange => None,
            Self::CClefCombining => None,
            Self::CClefReversed => None,
            Self::CClefSquare => None,
            Self::Caesura => Some('𝄓'),
            Self::CaesuraCurved => None,
            Self::CaesuraShort => None,
            Self::CaesuraSingleStroke => None,
            Self::CaesuraThick => None,
            Self::ChantAccentusAbove => None,
            Self::ChantAccentusBelow => None,
            Self::ChantAuctumAsc => None,
            Self::ChantAuctumDesc => None,
            Self::ChantAugmentum => None,
            Self::ChantCaesura => None,
            Self::ChantCclef => Some('𝇐'),
            Self::ChantCirculusAbove => None,
            Self::ChantCirculusBelow => None,
            Self::ChantConnectingLineAsc2Nd => None,
            Self::ChantConnectingLineAsc3Rd => None,
            Self::ChantConnectingLineAsc4Th => None,
            Self::ChantConnectingLineAsc5Th => None,
            Self::ChantConnectingLineAsc6Th => None,
            Self::ChantCustosStemDownPosHigh => None,
            Self::ChantCustosStemDownPosHighest => None,
            Self::ChantCustosStemDownPosMiddle => None,
            Self::ChantCustosStemUpPosLow => None,
            Self::ChantCustosStemUpPosLowest => None,
            Self::ChantCustosStemUpPosMiddle => None,
            Self::ChantDeminutumLower => None,
            Self::ChantDeminutumUpper => None,
            Self::ChantDivisioFinalis => None,
            Self::ChantDivisioMaior => None,
            Self::ChantDivisioMaxima => None,
            Self::ChantDivisioMinima => None,
            Self::ChantEntryLineAsc2Nd => None,
            Self::ChantEntryLineAsc3Rd => None,
            Self::ChantEntryLineAsc4Th => None,
            Self::ChantEntryLineAsc5Th => None,
            Self::ChantEntryLineAsc6Th => None,
            Self::ChantEpisema => None,
            Self::ChantFclef => Some('𝇑'),
            Self::ChantIctusAbove => None,
            Self::ChantIctusBelow => None,
            Self::ChantLigaturaDesc2Nd => None,
            Self::ChantLigaturaDesc3Rd => None,
            Self::ChantLigaturaDesc4Th => None,
            Self::ChantLigaturaDesc5Th => None,
            Self::ChantOriscusAscending => None,
            Self::ChantOriscusDescending => None,
            Self::ChantOriscusLiquescens => None,
            Self::ChantPodatusLower => None,
            Self::ChantPodatusUpper => Some('𝇔'),
            Self::ChantPunctum => None,
            Self::ChantPunctumCavum => None,
            Self::ChantPunctumDeminutum => None,
            Self::ChantPunctumInclinatum => None,
            Self::ChantPunctumInclinatumAuctum => None,
            Self::ChantPunctumInclinatumDeminutum => None,
            Self::ChantPunctumLinea => None,
            Self::ChantPunctumLineaCavum => None,
            Self::ChantPunctumVirga => Some('𝇓'),
            Self::ChantPunctumVirgaReversed => None,
            Self::ChantQuilisma => None,
            Self::ChantSemicirculusAbove => None,
            Self::ChantSemicirculusBelow => None,
            Self::ChantStaff => None,
            Self::ChantStaffNarrow => None,
            Self::ChantStaffWide => None,
            Self::ChantStrophicus => None,
            Self::ChantStrophicusAuctus => None,
            Self::ChantStrophicusLiquescens2Nd => None,
            Self::ChantStrophicusLiquescens3Rd => None,
            Self::ChantStrophicusLiquescens4Th => None,
            Self::ChantStrophicusLiquescens5Th => None,
            Self::ChantVirgula => None,
            Self::Clef15 => None,
            Self::Clef8 => None,
            Self::ClefChangeCombining => None,
            Self::Coda => Some('𝄌'),
            Self::CodaSquare => None,
            Self::ConductorBeat2Compound => None,
            Self::ConductorBeat2Simple => None,
            Self::ConductorBeat3Compound => None,
            Self::ConductorBeat3Simple => None,
            Self::ConductorBeat4Compound => None,
            Self::ConductorBeat4Simple => None,
            Self::ConductorLeftBeat => None,
            Self::ConductorRightBeat => None,
            Self::ConductorStrongBeat => None,
            Self::ConductorUnconducted => None,
            Self::ConductorWeakBeat => None,
            Self::ControlBeginBeam => Some('\u{1d173}'),
            Self::ControlBeginPhrase => Some('\u{1d179}'),
            Self::ControlBeginSlur => Some('\u{1d177}'),
            Self::ControlBeginTie => Some('\u{1d175}'),
            Self::ControlEndBeam => Some('\u{1d174}'),
            Self::ControlEndPhrase => Some('\u{1d17a}'),
            Self::ControlEndSlur => Some('\u{1d178}'),
            Self::ControlEndTie => Some('\u{1d176}'),
            Self::CsymAccidentalDoubleFlat => None,
            Self::CsymAccidentalDoubleSharp => None,
            Self::CsymAccidentalFlat => None,
            Self::CsymAccidentalNatural => None,
            Self::CsymAccidentalSharp => None,
            Self::CsymAccidentalTripleFlat => None,
            Self::CsymAccidentalTripleSharp => None,
            Self::CsymAlteredBassSlash => None,
            Self::CsymAugmented => None,
            Self::CsymBracketLeftTall => None,
            Self::CsymBracketRightTall => None,
            Self::CsymDiagonalArrangementSlash => None,
            Self::CsymDiminished => Some('𝆩'),
            Self::CsymHalfDiminished => None,
            Self::CsymMajorSeventh => None,
            Self::CsymMinor => None,
            Self::CsymParensLeftTall => None,
            Self::CsymParensLeftVeryTall => None,
            Self::CsymParensRightTall => None,
            Self::CsymParensRightVeryTall => None,
            Self::CurlewSign => None,
            Self::DaCapo => Some('𝄊'),
            Self::DalSegno => Some('𝄉'),
            Self::DaseianExcellentes1 => None,
            Self::DaseianExcellentes2 => None,
            Self::DaseianExcellentes3 => None,
            Self::DaseianExcellentes4 => None,
            Self::DaseianFinales1 => None,
            Self::DaseianFinales2 => None,
            Self::DaseianFinales3 => None,
            Self::DaseianFinales4 => None,
            Self::DaseianGraves1 => None,
            Self::DaseianGraves2 => None,
            Self::DaseianGraves3 => None,
            Self::DaseianGraves4 => None,
            Self::DaseianResidua1 => None,
            Self::DaseianResidua2 => None,
            Self::DaseianSuperiores1 => None,
            Self::DaseianSuperiores2 => None,
            Self::DaseianSuperiores3 => None,
            Self::DaseianSuperiores4 => None,
            Self::DoubleLateralRollStevens => None,
            Self::DoubleTongueAbove => Some('\u{1d18a}'),
            Self::DoubleTongueBelow => None,
            Self::DynamicCombinedSeparatorColon => None,
            Self::DynamicCombinedSeparatorHyphen => None,
            Self::DynamicCombinedSeparatorSlash => None,
            Self::DynamicCombinedSeparatorSpace => None,
            Self::DynamicCrescendoHairpin => Some('𝆒'),
            Self::DynamicDiminuendoHairpin => Some('𝆓'),
            Self::DynamicFf => None,
            Self::DynamicFff => None,
            Self::DynamicFfff => None,
            Self::DynamicFffff => None,
            Self::DynamicFfffff => None,
            Self::DynamicForte => Some('𝆑'),
            Self::DynamicFortePiano => None,
            Self::DynamicForzando => None,
            Self::DynamicHairpinBracketLeft => None,
            Self::DynamicHairpinBracketRight => None,
            Self::DynamicHairpinParenthesisLeft => None,
            Self::DynamicHairpinParenthesisRight => None,
            Self::DynamicMessaDiVoce => None,
            Self::DynamicMezzo => Some('𝆐'),
            Self::DynamicMf => None,
            Self::DynamicMp => None,
            Self::DynamicNiente => None,
            Self::DynamicNienteForHairpin => None,
            Self::DynamicPf => None,
            Self::DynamicPiano => Some('𝆏'),
            Self::DynamicPp => None,
            Self::DynamicPpp => None,
            Self::DynamicPppp => None,
            Self::DynamicPpppp => None,
            Self::DynamicPppppp => None,
            Self::DynamicRinforzando => Some('𝆌'),
            Self::DynamicRinforzando1 => None,
            Self::DynamicRinforzando2 => None,
            Self::DynamicSforzando => Some('𝆍'),
            Self::DynamicSforzando1 => None,
            Self::DynamicSforzandoPianissimo => None,
            Self::DynamicSforzandoPiano => None,
            Self::DynamicSforzato => None,
            Self::DynamicSforzatoFf => None,
            Self::DynamicSforzatoPiano => None,
            Self::DynamicZ => Some('𝆎'),
            Self::ElecAudioChannelsEight => None,
            Self::ElecAudioChannelsFive => None,
            Self::ElecAudioChannelsFour => None,
            Self::ElecAudioChannelsOne => None,
            Self::ElecAudioChannelsSeven => None,
            Self::ElecAudioChannelsSix => None,
            Self::ElecAudioChannelsThreeFrontal => None,
            Self::ElecAudioChannelsThreeSurround => None,
            Self::ElecAudioChannelsTwo => None,
            Self::ElecAudioIn => None,
            Self::ElecAudioMono => None,
            Self::ElecAudioOut => None,
            Self::ElecAudioStereo => None,
            Self::ElecCamera => None,
            Self::ElecDataIn => None,
            Self::ElecDataOut => None,
            Self::ElecDisc => None,
            Self::ElecDownload => None,
            Self::ElecEject => None,
            Self::ElecFastForward => None,
            Self::ElecHeadphones => None,
            Self::ElecHeadset => None,
            Self::ElecLineIn => None,
            Self::ElecLineOut => None,
            Self::ElecLoop => None,
            Self::ElecLoudspeaker => None,
            Self::ElecMicrophone => None,
            Self::ElecMicrophoneMute => None,
            Self::ElecMicrophoneUnmute => None,
            Self::ElecMidiController0 => None,
            Self::ElecMidiController100 => None,
            Self::ElecMidiController20 => None,
            Self::ElecMidiController40 => None,
            Self::ElecMidiController60 => None,
            Self::ElecMidiController80 => None,
            Self::ElecMidiIn => None,
            Self::ElecMidiOut => None,
            Self::ElecMixingConsole => None,
            Self::ElecMonitor => None,
            Self::ElecMute => None,
            Self::ElecPause => None,
            Self::ElecPlay => None,
            Self::ElecPowerOnOff => None,
            Self::ElecProjector => None,
            Self::ElecReplay => None,
            Self::ElecRewind => None,
            Self::ElecShuffle => None,
            Self::ElecSkipBackwards => None,
            Self::ElecSkipForwards => None,
            Self::ElecStop => None,
            Self::ElecTape => None,
            Self::ElecUnmute => None,
            Self::ElecUpload => None,
            Self::ElecUsb => None,
            Self::ElecVideoCamera => None,
            Self::ElecVideoIn => None,
            Self::ElecVideoOut => None,
            Self::ElecVolumeFader => None,
            Self::ElecVolumeFaderThumb => None,
            Self::ElecVolumeLevel0 => None,
            Self::ElecVolumeLevel100 => None,
            Self::ElecVolumeLevel20 => None,
            Self::ElecVolumeLevel40 => None,
            Self::ElecVolumeLevel60 => None,
            Self::ElecVolumeLevel80 => None,
            Self::FClef => Some('𝄢'),
            Self::FClef15Ma => None,
            Self::FClef15Mb => None,
            Self::FClef8Va => Some('𝄣'),
            Self::FClef8Vb => Some('𝄤'),
            Self::FClefArrowDown => None,
            Self::FClefArrowUp => None,
            Self::FClefChange => None,
            Self::FClefReversed => None,
            Self::FClefTurned => None,
            Self::FermataAbove => Some('𝄐'),
            Self::FermataBelow => Some('𝄑'),
            Self::FermataLongAbove => None,
            Self::FermataLongBelow => None,
            Self::FermataLongHenzeAbove => None,
            Self::FermataLongHenzeBelow => None,
            Self::FermataShortAbove => None,
            Self::FermataShortBelow => None,
            Self::FermataShortHenzeAbove => None,
            Self::FermataShortHenzeBelow => None,
            Self::FermataVeryLongAbove => None,
            Self::FermataVeryLongBelow => None,
            Self::FermataVeryShortAbove => None,
            Self::FermataVeryShortBelow => None,
            Self::Figbass0 => None,
            Self::Figbass1 => None,
            Self::Figbass2 => None,
            Self::Figbass2Raised => None,
            Self::Figbass3 => None,
            Self::Figbass4 => None,
            Self::Figbass4Raised => None,
            Self::Figbass5 => None,
            Self::Figbass5Raised1 => None,
            Self::Figbass5Raised2 => None,
            Self::Figbass5Raised3 => None,
            Self::Figbass6 => None,
            Self::Figbass6Raised => None,
            Self::Figbass6Raised2 => None,
            Self::Figbass7 => None,
            Self::Figbass7Diminished => None,
            Self::Figbass7Raised1 => None,
            Self::Figbass7Raised2 => None,
            Self::Figbass8 => None,
            Self::Figbass9 => None,
            Self::Figbass9Raised => None,
            Self::FigbassBracketLeft => None,
            Self::FigbassBracketRight => None,
            Self::FigbassCombiningLowering => None,
            Self::FigbassCombiningRaising => None,
            Self::FigbassDoubleFlat => None,
            Self::FigbassDoubleSharp => None,
            Self::FigbassFlat => None,
            Self::FigbassNatural => None,
            Self::FigbassParensLeft => None,
            Self::FigbassParensRight => None,
            Self::FigbassPlus => None,
            Self::FigbassSharp => None,
            Self::FigbassTripleFlat => None,
            Self::FigbassTripleSharp => None,
            Self::Fingering0 => None,
            Self::Fingering0Italic => None,
            Self::Fingering1 => None,
            Self::Fingering1Italic => None,
            Self::Fingering2 => None,
            Self::Fingering2Italic => None,
            Self::Fingering3 => None,
            Self::Fingering3Italic => None,
            Self::Fingering4 => None,
            Self::Fingering4Italic => None,
            Self::Fingering5 => None,
            Self::Fingering5Italic => None,
            Self::Fingering6 => None,
            Self::Fingering6Italic => None,
            Self::Fingering7 => None,
            Self::Fingering7Italic => None,
            Self::Fingering8 => None,
            Self::Fingering8Italic => None,
            Self::Fingering9 => None,
            Self::Fingering9Italic => None,
            Self::FingeringALower => None,
            Self::FingeringCLower => None,
            Self::FingeringELower => None,
            Self::FingeringILower => None,
            Self::FingeringLeftBracket => None,
            Self::FingeringLeftBracketItalic => None,
            Self::FingeringLeftParenthesis => None,
            Self::FingeringLeftParenthesisItalic => None,
            Self::FingeringMLower => None,
            Self::FingeringMultipleNotes => None,
            Self::FingeringOLower => None,
            Self::FingeringPLower => None,
            Self::FingeringQLower => None,
            Self::FingeringRightBracket => None,
            Self::FingeringRightBracketItalic => None,
            Self::FingeringRightParenthesis => None,
            Self::FingeringRightParenthesisItalic => None,
            Self::FingeringSLower => None,
            Self::FingeringSeparatorMiddleDot => None,
            Self::FingeringSeparatorMiddleDotWhite => None,
            Self::FingeringSeparatorSlash => None,
            Self::FingeringSubstitutionAbove => None,
            Self::FingeringSubstitutionBelow => None,
            Self::FingeringSubstitutionDash => None,
            Self::FingeringTLower => None,
            Self::FingeringTUpper => None,
            Self::FingeringXLower => None,
            Self::Flag1024ThDown => None,
            Self::Flag1024ThUp => None,
            Self::Flag128ThDown => None,
            Self::Flag128ThUp => Some('\u{1d172}'),
            Self::Flag16ThDown => None,
            Self::Flag16ThUp => Some('\u{1d16f}'),
            Self::Flag256ThDown => None,
            Self::Flag256ThUp => None,
            Self::Flag32NdDown => None,
            Self::Flag32NdUp => Some('\u{1d170}'),
            Self::Flag512ThDown => None,
            Self::Flag512ThUp => None,
            Self::Flag64ThDown => None,
            Self::Flag64ThUp => Some('\u{1d171}'),
            Self::Flag8ThDown => None,
            Self::Flag8ThUp => Some('\u{1d16e}'),
            Self::FlagInternalDown => None,
            Self::FlagInternalUp => None,
            Self::Fretboard3String => None,
            Self::Fretboard3StringNut => None,
            Self::Fretboard4String => Some('𝄝'),
            Self::Fretboard4StringNut => None,
            Self::Fretboard5String => None,
            Self::Fretboard5StringNut => None,
            Self::Fretboard6String => Some('𝄜'),
            Self::Fretboard6StringNut => None,
            Self::FretboardFilledCircle => None,
            Self::FretboardO => None,
            Self::FretboardX => None,
            Self::FunctionAngleLeft => None,
            Self::FunctionAngleRight => None,
            Self::FunctionBracketLeft => None,
            Self::FunctionBracketRight => None,
            Self::FunctionDLower => None,
            Self::FunctionDUpper => None,
            Self::FunctionDd => None,
            Self::FunctionEight => None,
            Self::FunctionFUpper => None,
            Self::FunctionFive => None,
            Self::FunctionFour => None,
            Self::FunctionGLower => None,
            Self::FunctionGUpper => None,
            Self::FunctionGreaterThan => None,
            Self::FunctionILower => None,
            Self::FunctionIUpper => None,
            Self::FunctionKLower => None,
            Self::FunctionKUpper => None,
            Self::FunctionLLower => None,
            Self::FunctionLUpper => None,
            Self::FunctionLessThan => None,
            Self::FunctionMLower => None,
            Self::FunctionMUpper => None,
            Self::FunctionMinus => None,
            Self::FunctionNLower => None,
            Self::FunctionNUpper => None,
            Self::FunctionNUpperSuperscript => None,
            Self::FunctionNine => None,
            Self::FunctionOne => None,
            Self::FunctionPLower => None,
            Self::FunctionPUpper => None,
            Self::FunctionParensLeft => None,
            Self::FunctionParensRight => None,
            Self::FunctionPlus => None,
            Self::FunctionRLower => None,
            Self::FunctionRepetition1 => None,
            Self::FunctionRepetition2 => None,
            Self::FunctionRing => None,
            Self::FunctionSLower => None,
            Self::FunctionSUpper => None,
            Self::FunctionSeven => None,
            Self::FunctionSix => None,
            Self::FunctionSlashedDd => None,
            Self::FunctionSsLower => None,
            Self::FunctionSsUpper => None,
            Self::FunctionTLower => None,
            Self::FunctionTUpper => None,
            Self::FunctionThree => None,
            Self::FunctionTwo => None,
            Self::FunctionVLower => None,
            Self::FunctionVUpper => None,
            Self::FunctionZero => None,
            Self::GClef => Some('𝄞'),
            Self::GClef15Ma => None,
            Self::GClef15Mb => None,
            Self::GClef8Va => Some('𝄟'),
            Self::GClef8Vb => Some('𝄠'),
            Self::GClef8VbCClef => None,
            Self::GClef8VbOld => None,
            Self::GClef8VbParens => None,
            Self::GClefArrowDown => None,
            Self::GClefArrowUp => None,
            Self::GClefChange => None,
            Self::GClefLigatedNumberAbove => None,
            Self::GClefLigatedNumberBelow => None,
            Self::GClefReversed => None,
            Self::GClefTurned => None,
            Self::GlissandoDown => Some('𝆲'),
            Self::GlissandoUp => Some('𝆱'),
            Self::GraceNoteAcciaccaturaStemDown => None,
            Self::GraceNoteAcciaccaturaStemUp => Some('𝆔'),
            Self::GraceNoteAppoggiaturaStemDown => None,
            Self::GraceNoteAppoggiaturaStemUp => Some('𝆕'),
            Self::GraceNoteSlashStemDown => None,
            Self::GraceNoteSlashStemUp => None,
            Self::GuitarBarreFull => None,
            Self::GuitarBarreHalf => None,
            Self::GuitarClosePedal => None,
            Self::GuitarFadeIn => None,
            Self::GuitarFadeOut => None,
            Self::GuitarGolpe => None,
            Self::GuitarHalfOpenPedal => None,
            Self::GuitarLeftHandTapping => None,
            Self::GuitarOpenPedal => None,
            Self::GuitarRightHandTapping => None,
            Self::GuitarShake => None,
            Self::GuitarString0 => None,
            Self::GuitarString1 => None,
            Self::GuitarString10 => None,
            Self::GuitarString11 => None,
            Self::GuitarString12 => None,
            Self::GuitarString13 => None,
            Self::GuitarString2 => None,
            Self::GuitarString3 => None,
            Self::GuitarString4 => None,
            Self::GuitarString5 => None,
            Self::GuitarString6 => None,
            Self::GuitarString7 => None,
            Self::GuitarString8 => None,
            Self::GuitarString9 => None,
            Self::GuitarStrumDown => None,
            Self::GuitarStrumUp => None,
            Self::GuitarVibratoBarDip => None,
            Self::GuitarVibratoBarScoop => None,
            Self::GuitarVibratoStroke => None,
            Self::GuitarVolumeSwell => None,
            Self::GuitarWideVibratoStroke => None,
            Self::HandbellsBelltree => None,
            Self::HandbellsDamp3 => None,
            Self::HandbellsEcho1 => None,
            Self::HandbellsEcho2 => None,
            Self::HandbellsGyro => None,
            Self::HandbellsHandMartellato => None,
            Self::HandbellsMalletBellOnTable => None,
            Self::HandbellsMalletBellSuspended => None,
            Self::HandbellsMalletLft => None,
            Self::HandbellsMartellato => None,
            Self::HandbellsMartellatoLift => None,
            Self::HandbellsMutedMartellato => None,
            Self::HandbellsPluckLift => None,
            Self::HandbellsSwing => None,
            Self::HandbellsSwingDown => None,
            Self::HandbellsSwingUp => None,
            Self::HandbellsTablePairBells => None,
            Self::HandbellsTableSingleBell => None,
            Self::HarpMetalRod => None,
            Self::HarpPedalCentered => None,
            Self::HarpPedalDivider => None,
            Self::HarpPedalLowered => None,
            Self::HarpPedalRaised => None,
            Self::HarpSalzedoAeolianAscending => None,
            Self::HarpSalzedoAeolianDescending => None,
            Self::HarpSalzedoDampAbove => None,
            Self::HarpSalzedoDampBelow => None,
            Self::HarpSalzedoDampBothHands => None,
            Self::HarpSalzedoDampLowStrings => None,
            Self::HarpSalzedoFluidicSoundsLeft => None,
            Self::HarpSalzedoFluidicSoundsRight => None,
            Self::HarpSalzedoIsolatedSounds => None,
            Self::HarpSalzedoMetallicSounds => None,
            Self::HarpSalzedoMetallicSoundsOneString => None,
            Self::HarpSalzedoMuffleTotally => None,
            Self::HarpSalzedoOboicFlux => None,
            Self::HarpSalzedoPlayUpperEnd => None,
            Self::HarpSalzedoSlideWithSuppleness => None,
            Self::HarpSalzedoSnareDrum => None,
            Self::HarpSalzedoTamTamSounds => None,
            Self::HarpSalzedoThunderEffect => None,
            Self::HarpSalzedoTimpanicSounds => None,
            Self::HarpSalzedoWhistlingSounds => None,
            Self::HarpStringNoiseStem => None,
            Self::HarpTuningKey => None,
            Self::HarpTuningKeyGlissando => None,
            Self::HarpTuningKeyHandle => None,
            Self::HarpTuningKeyShank => None,
            Self::IndianDrumClef => None,
            Self::KahnBackChug => None,
            Self::KahnBackFlap => None,
            Self::KahnBackRiff => None,
            Self::KahnBackRip => None,
            Self::KahnBallChange => None,
            Self::KahnBallDig => None,
            Self::KahnBrushBackward => None,
            Self::KahnBrushForward => None,
            Self::KahnChug => None,
            Self::KahnClap => None,
            Self::KahnDoubleSnap => None,
            Self::KahnDoubleWing => None,
            Self::KahnDrawStep => None,
            Self::KahnDrawTap => None,
            Self::KahnFlam => None,
            Self::KahnFlap => None,
            Self::KahnFlapStep => None,
            Self::KahnFlat => None,
            Self::KahnFleaHop => None,
            Self::KahnFleaTap => None,
            Self::KahnGraceTap => None,
            Self::KahnGraceTapChange => None,
            Self::KahnGraceTapHop => None,
            Self::KahnGraceTapStamp => None,
            Self::KahnHeel => None,
            Self::KahnHeelChange => None,
            Self::KahnHeelClick => None,
            Self::KahnHeelDrop => None,
            Self::KahnHeelStep => None,
            Self::KahnHeelTap => None,
            Self::KahnHop => None,
            Self::KahnJumpApart => None,
            Self::KahnJumpTogether => None,
            Self::KahnKneeInward => None,
            Self::KahnKneeOutward => None,
            Self::KahnLeap => None,
            Self::KahnLeapFlatFoot => None,
            Self::KahnLeapHeelClick => None,
            Self::KahnLeftCatch => None,
            Self::KahnLeftCross => None,
            Self::KahnLeftFoot => None,
            Self::KahnLeftToeStrike => None,
            Self::KahnLeftTurn => None,
            Self::KahnOverTheTop => None,
            Self::KahnOverTheTopTap => None,
            Self::KahnPull => None,
            Self::KahnPush => None,
            Self::KahnRiff => None,
            Self::KahnRiffle => None,
            Self::KahnRightCatch => None,
            Self::KahnRightCross => None,
            Self::KahnRightFoot => None,
            Self::KahnRightToeStrike => None,
            Self::KahnRightTurn => None,
            Self::KahnRip => None,
            Self::KahnRipple => None,
            Self::KahnScrape => None,
            Self::KahnScuff => None,
            Self::KahnScuffle => None,
            Self::KahnShuffle => None,
            Self::KahnSlam => None,
            Self::KahnSlap => None,
            Self::KahnSlideStep => None,
            Self::KahnSlideTap => None,
            Self::KahnSnap => None,
            Self::KahnStamp => None,
            Self::KahnStampStamp => None,
            Self::KahnStep => None,
            Self::KahnStepStamp => None,
            Self::KahnStomp => None,
            Self::KahnStompBrush => None,
            Self::KahnTap => None,
            Self::KahnToe => None,
            Self::KahnToeClick => None,
            Self::KahnToeDrop => None,
            Self::KahnToeStep => None,
            Self::KahnToeTap => None,
            Self::KahnTrench => None,
            Self::KahnWing => None,
            Self::KahnWingChange => None,
            Self::KahnZank => None,
            Self::KahnZink => None,
            Self::KeyboardBebung2DotsAbove => None,
            Self::KeyboardBebung2DotsBelow => None,
            Self::KeyboardBebung3DotsAbove => None,
            Self::KeyboardBebung3DotsBelow => None,
            Self::KeyboardBebung4DotsAbove => None,
            Self::KeyboardBebung4DotsBelow => None,
            Self::KeyboardLeftPedalPictogram => None,
            Self::KeyboardMiddlePedalPictogram => None,
            Self::KeyboardPedalD => None,
            Self::KeyboardPedalDot => None,
            Self::KeyboardPedalE => None,
            Self::KeyboardPedalHalf => Some('𝆰'),
            Self::KeyboardPedalHalf2 => None,
            Self::KeyboardPedalHalf3 => None,
            Self::KeyboardPedalHeel1 => None,
            Self::KeyboardPedalHeel2 => None,
            Self::KeyboardPedalHeel3 => None,
            Self::KeyboardPedalHeelToToe => None,
            Self::KeyboardPedalHeelToe => None,
            Self::KeyboardPedalHookEnd => None,
            Self::KeyboardPedalHookStart => None,
            Self::KeyboardPedalHyphen => None,
            Self::KeyboardPedalP => None,
            Self::KeyboardPedalParensLeft => None,
            Self::KeyboardPedalParensRight => None,
            Self::KeyboardPedalPed => Some('𝆮'),
            Self::KeyboardPedalS => None,
            Self::KeyboardPedalSost => None,
            Self::KeyboardPedalToe1 => None,
            Self::KeyboardPedalToe2 => None,
            Self::KeyboardPedalToeToHeel => None,
            Self::KeyboardPedalUp => Some('𝆯'),
            Self::KeyboardPedalUpNotch => None,
            Self::KeyboardPedalUpSpecial => None,
            Self::KeyboardPlayWithLh => None,
            Self::KeyboardPlayWithLhEnd => None,
            Self::KeyboardPlayWithRh => None,
            Self::KeyboardPlayWithRhEnd => None,
            Self::KeyboardPluckInside => None,
            Self::KeyboardRightPedalPictogram => None,
            Self::KievanAccidentalFlat => Some('𝇨'),
            Self::KievanAccidentalSharp => None,
            Self::KievanAugmentationDot => None,
            Self::KievanCClef => Some('𝇞'),
            Self::KievanEndingSymbol => Some('𝇟'),
            Self::KievanNote8ThStemDown => Some('𝇦'),
            Self::KievanNote8ThStemUp => Some('𝇧'),
            Self::KievanNoteBeam => None,
            Self::KievanNoteHalfStaffLine => Some('𝇣'),
            Self::KievanNoteHalfStaffSpace => None,
            Self::KievanNoteQuarterStemDown => Some('𝇤'),
            Self::KievanNoteQuarterStemUp => Some('𝇥'),
            Self::KievanNoteReciting => Some('𝇡'),
            Self::KievanNoteWhole => Some('𝇢'),
            Self::KievanNoteWholeFinal => Some('𝇠'),
            Self::KodalyHandDo => None,
            Self::KodalyHandFa => None,
            Self::KodalyHandLa => None,
            Self::KodalyHandMi => None,
            Self::KodalyHandRe => None,
            Self::KodalyHandSo => None,
            Self::KodalyHandTi => None,
            Self::LeftRepeatSmall => None,
            Self::LegerLine => None,
            Self::LegerLineNarrow => None,
            Self::LegerLineWide => None,
            Self::LuteBarlineEndRepeat => None,
            Self::LuteBarlineFinal => None,
            Self::LuteBarlineStartRepeat => None,
            Self::LuteDuration16Th => None,
            Self::LuteDuration32Nd => None,
            Self::LuteDuration8Th => None,
            Self::LuteDurationDoubleWhole => None,
            Self::LuteDurationHalf => None,
            Self::LuteDurationQuarter => None,
            Self::LuteDurationWhole => None,
            Self::LuteFingeringRhFirst => None,
            Self::LuteFingeringRhSecond => None,
            Self::LuteFingeringRhThird => None,
            Self::LuteFingeringRhThumb => None,
            Self::LuteFrench10ThCourse => None,
            Self::LuteFrench7ThCourse => None,
            Self::LuteFrench8ThCourse => None,
            Self::LuteFrench9ThCourse => None,
            Self::LuteFrenchAppoggiaturaAbove => None,
            Self::LuteFrenchAppoggiaturaBelow => None,
            Self::LuteFrenchFretA => None,
            Self::LuteFrenchFretB => None,
            Self::LuteFrenchFretC => None,
            Self::LuteFrenchFretD => None,
            Self::LuteFrenchFretE => None,
            Self::LuteFrenchFretF => None,
            Self::LuteFrenchFretG => None,
            Self::LuteFrenchFretH => None,
            Self::LuteFrenchFretI => None,
            Self::LuteFrenchFretK => None,
            Self::LuteFrenchFretL => None,
            Self::LuteFrenchFretM => None,
            Self::LuteFrenchFretN => None,
            Self::LuteFrenchMordentInverted => None,
            Self::LuteFrenchMordentLower => None,
            Self::LuteFrenchMordentUpper => None,
            Self::LuteGermanALower => None,
            Self::LuteGermanAUpper => None,
            Self::LuteGermanBLower => None,
            Self::LuteGermanBUpper => None,
            Self::LuteGermanCLower => None,
            Self::LuteGermanCUpper => None,
            Self::LuteGermanDLower => None,
            Self::LuteGermanDUpper => None,
            Self::LuteGermanELower => None,
            Self::LuteGermanEUpper => None,
            Self::LuteGermanFLower => None,
            Self::LuteGermanFUpper => None,
            Self::LuteGermanGLower => None,
            Self::LuteGermanGUpper => None,
            Self::LuteGermanHLower => None,
            Self::LuteGermanHUpper => None,
            Self::LuteGermanILower => None,
            Self::LuteGermanIUpper => None,
            Self::LuteGermanKLower => None,
            Self::LuteGermanKUpper => None,
            Self::LuteGermanLLower => None,
            Self::LuteGermanLUpper => None,
            Self::LuteGermanMLower => None,
            Self::LuteGermanMUpper => None,
            Self::LuteGermanNLower => None,
            Self::LuteGermanNUpper => None,
            Self::LuteGermanOLower => None,
            Self::LuteGermanPLower => None,
            Self::LuteGermanQLower => None,
            Self::LuteGermanRLower => None,
            Self::LuteGermanSLower => None,
            Self::LuteGermanTLower => None,
            Self::LuteGermanVLower => None,
            Self::LuteGermanXLower => None,
            Self::LuteGermanYLower => None,
            Self::LuteGermanZLower => None,
            Self::LuteItalianClefCSolFaUt => None,
            Self::LuteItalianClefFFaUt => None,
            Self::LuteItalianFret0 => None,
            Self::LuteItalianFret1 => None,
            Self::LuteItalianFret2 => None,
            Self::LuteItalianFret3 => None,
            Self::LuteItalianFret4 => None,
            Self::LuteItalianFret5 => None,
            Self::LuteItalianFret6 => None,
            Self::LuteItalianFret7 => None,
            Self::LuteItalianFret8 => None,
            Self::LuteItalianFret9 => None,
            Self::LuteItalianHoldFinger => None,
            Self::LuteItalianHoldNote => None,
            Self::LuteItalianReleaseFinger => None,
            Self::LuteItalianTempoFast => None,
            Self::LuteItalianTempoNeitherFastNorSlow => None,
            Self::LuteItalianTempoSlow => None,
            Self::LuteItalianTempoSomewhatFast => None,
            Self::LuteItalianTempoVerySlow => None,
            Self::LuteItalianTimeTriple => None,
            Self::LuteItalianTremolo => None,
            Self::LuteItalianVibrato => None,
            Self::LuteStaff6Lines => None,
            Self::LuteStaff6LinesNarrow => None,
            Self::LuteStaff6LinesWide => None,
            Self::LyricsElision => None,
            Self::LyricsElisionNarrow => None,
            Self::LyricsElisionWide => None,
            Self::LyricsHyphenBaseline => None,
            Self::LyricsHyphenBaselineNonBreaking => None,
            Self::LyricsTextRepeat => None,
            Self::MedRenFlatHardB => None,
            Self::MedRenFlatSoftB => Some('𝇒'),
            Self::MedRenFlatWithDot => None,
            Self::MedRenGClefCmn => None,
            Self::MedRenLiquescenceCmn => None,
            Self::MedRenLiquescentAscCmn => None,
            Self::MedRenLiquescentDescCmn => None,
            Self::MedRenNatural => None,
            Self::MedRenNaturalWithCross => None,
            Self::MedRenOriscusCmn => None,
            Self::MedRenPlicaCmn => None,
            Self::MedRenPunctumCmn => None,
            Self::MedRenQuilismaCmn => None,
            Self::MedRenSharpCroix => Some('𝇏'),
            Self::MedRenStrophicusCmn => None,
            Self::MensuralAlterationSign => None,
            Self::MensuralBlackBrevis => None,
            Self::MensuralBlackBrevisVoid => None,
            Self::MensuralBlackDragma => None,
            Self::MensuralBlackLonga => None,
            Self::MensuralBlackMaxima => None,
            Self::MensuralBlackMinima => Some('𝆺𝅥'),
            Self::MensuralBlackMinimaVoid => Some('𝆹𝅥'),
            Self::MensuralBlackSemibrevis => Some('𝆺'),
            Self::MensuralBlackSemibrevisCaudata => None,
            Self::MensuralBlackSemibrevisOblique => None,
            Self::MensuralBlackSemibrevisVoid => Some('𝆹'),
            Self::MensuralBlackSemiminima => None,
            Self::MensuralCclef => None,
            Self::MensuralCclefPetrucciPosHigh => None,
            Self::MensuralCclefPetrucciPosHighest => None,
            Self::MensuralCclefPetrucciPosLow => None,
            Self::MensuralCclefPetrucciPosLowest => None,
            Self::MensuralCclefPetrucciPosMiddle => None,
            Self::MensuralColorationEndRound => None,
            Self::MensuralColorationEndSquare => None,
            Self::MensuralColorationStartRound => None,
            Self::MensuralColorationStartSquare => None,
            Self::MensuralCombStemDiagonal => None,
            Self::MensuralCombStemDown => None,
            Self::MensuralCombStemDownFlagExtended => None,
            Self::MensuralCombStemDownFlagFlared => None,
            Self::MensuralCombStemDownFlagFusa => None,
            Self::MensuralCombStemDownFlagLeft => None,
            Self::MensuralCombStemDownFlagRight => None,
            Self::MensuralCombStemDownFlagSemiminima => None,
            Self::MensuralCombStemUp => None,
            Self::MensuralCombStemUpFlagExtended => None,
            Self::MensuralCombStemUpFlagFlared => None,
            Self::MensuralCombStemUpFlagFusa => None,
            Self::MensuralCombStemUpFlagLeft => None,
            Self::MensuralCombStemUpFlagRight => None,
            Self::MensuralCombStemUpFlagSemiminima => None,
            Self::MensuralCustosCheckmark => None,
            Self::MensuralCustosDown => None,
            Self::MensuralCustosTurn => None,
            Self::MensuralCustosUp => None,
            Self::MensuralFclef => None,
            Self::MensuralFclefPetrucci => None,
            Self::MensuralGclef => None,
            Self::MensuralGclefPetrucci => None,
            Self::MensuralModusImperfectumVert => None,
            Self::MensuralModusPerfectumVert => None,
            Self::MensuralNoteheadLongaBlack => None,
            Self::MensuralNoteheadLongaBlackVoid => None,
            Self::MensuralNoteheadLongaVoid => None,
            Self::MensuralNoteheadLongaWhite => None,
            Self::MensuralNoteheadMaximaBlack => None,
            Self::MensuralNoteheadMaximaBlackVoid => None,
            Self::MensuralNoteheadMaximaVoid => None,
            Self::MensuralNoteheadMaximaWhite => None,
            Self::MensuralNoteheadMinimaWhite => None,
            Self::MensuralNoteheadSemibrevisBlack => None,
            Self::MensuralNoteheadSemibrevisBlackVoid => None,
            Self::MensuralNoteheadSemibrevisBlackVoidTurned => None,
            Self::MensuralNoteheadSemibrevisVoid => None,
            Self::MensuralNoteheadSemiminimaWhite => None,
            Self::MensuralObliqueAsc2NdBlack => None,
            Self::MensuralObliqueAsc2NdBlackVoid => None,
            Self::MensuralObliqueAsc2NdVoid => None,
            Self::MensuralObliqueAsc2NdWhite => None,
            Self::MensuralObliqueAsc3RdBlack => None,
            Self::MensuralObliqueAsc3RdBlackVoid => None,
            Self::MensuralObliqueAsc3RdVoid => None,
            Self::MensuralObliqueAsc3RdWhite => None,
            Self::MensuralObliqueAsc4ThBlack => None,
            Self::MensuralObliqueAsc4ThBlackVoid => None,
            Self::MensuralObliqueAsc4ThVoid => None,
            Self::MensuralObliqueAsc4ThWhite => None,
            Self::MensuralObliqueAsc5ThBlack => None,
            Self::MensuralObliqueAsc5ThBlackVoid => None,
            Self::MensuralObliqueAsc5ThVoid => None,
            Self::MensuralObliqueAsc5ThWhite => None,
            Self::MensuralObliqueDesc2NdBlack => None,
            Self::MensuralObliqueDesc2NdBlackVoid => None,
            Self::MensuralObliqueDesc2NdVoid => None,
            Self::MensuralObliqueDesc2NdWhite => None,
            Self::MensuralObliqueDesc3RdBlack => None,
            Self::MensuralObliqueDesc3RdBlackVoid => None,
            Self::MensuralObliqueDesc3RdVoid => None,
            Self::MensuralObliqueDesc3RdWhite => None,
            Self::MensuralObliqueDesc4ThBlack => None,
            Self::MensuralObliqueDesc4ThBlackVoid => None,
            Self::MensuralObliqueDesc4ThVoid => None,
            Self::MensuralObliqueDesc4ThWhite => None,
            Self::MensuralObliqueDesc5ThBlack => None,
            Self::MensuralObliqueDesc5ThBlackVoid => None,
            Self::MensuralObliqueDesc5ThVoid => None,
            Self::MensuralObliqueDesc5ThWhite => None,
            Self::MensuralProlation1 => Some('𝇇'),
            Self::MensuralProlation10 => Some('𝇎'),
            Self::MensuralProlation11 => None,
            Self::MensuralProlation2 => Some('𝇈'),
            Self::MensuralProlation3 => Some('𝇉'),
            Self::MensuralProlation4 => None,
            Self::MensuralProlation5 => Some('𝇊'),
            Self::MensuralProlation6 => Some('𝇋'),
            Self::MensuralProlation7 => Some('𝇌'),
            Self::MensuralProlation8 => None,
            Self::MensuralProlation9 => Some('𝇍'),
            Self::MensuralProlationCombiningDot => None,
            Self::MensuralProlationCombiningDotVoid => None,
            Self::MensuralProlationCombiningStroke => None,
            Self::MensuralProlationCombiningThreeDots => None,
            Self::MensuralProlationCombiningThreeDotsTri => None,
            Self::MensuralProlationCombiningTwoDots => None,
            Self::MensuralProportion1 => None,
            Self::MensuralProportion2 => None,
            Self::MensuralProportion3 => None,
            Self::MensuralProportion4 => None,
            Self::MensuralProportion5 => None,
            Self::MensuralProportion6 => None,
            Self::MensuralProportion7 => None,
            Self::MensuralProportion8 => None,
            Self::MensuralProportion9 => None,
            Self::MensuralProportionMajor => None,
            Self::MensuralProportionMinor => None,
            Self::MensuralProportionProportioDupla1 => None,
            Self::MensuralProportionProportioDupla2 => None,
            Self::MensuralProportionProportioQuadrupla => None,
            Self::MensuralProportionProportioTripla => None,
            Self::MensuralProportionTempusPerfectum => None,
            Self::MensuralRestBrevis => Some('𝇃'),
            Self::MensuralRestFusa => None,
            Self::MensuralRestLongaImperfecta => Some('𝇂'),
            Self::MensuralRestLongaPerfecta => Some('𝇁'),
            Self::MensuralRestMaxima => None,
            Self::MensuralRestMinima => Some('𝇅'),
            Self::MensuralRestSemibrevis => Some('𝇄'),
            Self::MensuralRestSemifusa => None,
            Self::MensuralRestSemiminima => Some('𝇆'),
            Self::MensuralSignumDown => None,
            Self::MensuralSignumUp => None,
            Self::MensuralTempusImperfectumHoriz => None,
            Self::MensuralTempusPerfectumHoriz => None,
            Self::MensuralWhiteBrevis => Some('𝆸'),
            Self::MensuralWhiteFusa => Some('𝆺𝅥𝅮'),
            Self::MensuralWhiteLonga => Some('𝆷'),
            Self::MensuralWhiteMaxima => Some('𝆶'),
            Self::MensuralWhiteMinima => None,
            Self::MensuralWhiteSemibrevis => Some('𝆹'),
            Self::MensuralWhiteSemiminima => None,
            Self::MetAugmentationDot => None,
            Self::MetNote1024ThDown => None,
            Self::MetNote1024ThUp => None,
            Self::MetNote128ThDown => None,
            Self::MetNote128ThUp => None,
            Self::MetNote16ThDown => None,
            Self::MetNote16ThUp => None,
            Self::MetNote256ThDown => None,
            Self::MetNote256ThUp => None,
            Self::MetNote32NdDown => None,
            Self::MetNote32NdUp => None,
            Self::MetNote512ThDown => None,
            Self::MetNote512ThUp => None,
            Self::MetNote64ThDown => None,
            Self::MetNote64ThUp => None,
            Self::MetNote8ThDown => None,
            Self::MetNote8ThUp => None,
            Self::MetNoteDoubleWhole => None,
            Self::MetNoteDoubleWholeSquare => None,
            Self::MetNoteHalfDown => None,
            Self::MetNoteHalfUp => None,
            Self::MetNoteQuarterDown => None,
            Self::MetNoteQuarterUp => None,
            Self::MetNoteWhole => None,
            Self::MetricModulationArrowLeft => None,
            Self::MetricModulationArrowRight => None,
            Self::MiscDoNotCopy => None,
            Self::MiscDoNotPhotocopy => None,
            Self::MiscEyeglasses => None,
            Self::Note1024ThDown => None,
            Self::Note1024ThUp => None,
            Self::Note128ThDown => None,
            Self::Note128ThUp => Some('𝅘𝅥𝅲'),
            Self::Note16ThDown => None,
            Self::Note16ThUp => Some('𝅘𝅥𝅯'),
            Self::Note256ThDown => None,
            Self::Note256ThUp => None,
            Self::Note32NdDown => None,
            Self::Note32NdUp => Some('𝅘𝅥𝅰'),
            Self::Note512ThDown => None,
            Self::Note512ThUp => None,
            Self::Note64ThDown => None,
            Self::Note64ThUp => Some('𝅘𝅥𝅱'),
            Self::Note8ThDown => None,
            Self::Note8ThUp => Some('𝅘𝅥𝅮'),
            Self::NoteABlack => None,
            Self::NoteAFlatBlack => None,
            Self::NoteAFlatHalf => None,
            Self::NoteAFlatWhole => None,
            Self::NoteAHalf => None,
            Self::NoteASharpBlack => None,
            Self::NoteASharpHalf => None,
            Self::NoteASharpWhole => None,
            Self::NoteAWhole => None,
            Self::NoteBBlack => None,
            Self::NoteBFlatBlack => None,
            Self::NoteBFlatHalf => None,
            Self::NoteBFlatWhole => None,
            Self::NoteBHalf => None,
            Self::NoteBSharpBlack => None,
            Self::NoteBSharpHalf => None,
            Self::NoteBSharpWhole => None,
            Self::NoteBWhole => None,
            Self::NoteCBlack => None,
            Self::NoteCFlatBlack => None,
            Self::NoteCFlatHalf => None,
            Self::NoteCFlatWhole => None,
            Self::NoteCHalf => None,
            Self::NoteCSharpBlack => None,
            Self::NoteCSharpHalf => None,
            Self::NoteCSharpWhole => None,
            Self::NoteCWhole => None,
            Self::NoteDBlack => None,
            Self::NoteDFlatBlack => None,
            Self::NoteDFlatHalf => None,
            Self::NoteDFlatWhole => None,
            Self::NoteDHalf => None,
            Self::NoteDSharpBlack => None,
            Self::NoteDSharpHalf => None,
            Self::NoteDSharpWhole => None,
            Self::NoteDWhole => None,
            Self::NoteDiBlack => None,
            Self::NoteDiHalf => None,
            Self::NoteDiWhole => None,
            Self::NoteDoBlack => None,
            Self::NoteDoHalf => None,
            Self::NoteDoWhole => None,
            Self::NoteDoubleWhole => Some('𝅜'),
            Self::NoteDoubleWholeSquare => None,
            Self::NoteEBlack => None,
            Self::NoteEFlatBlack => None,
            Self::NoteEFlatHalf => None,
            Self::NoteEFlatWhole => None,
            Self::NoteEHalf => None,
            Self::NoteESharpBlack => None,
            Self::NoteESharpHalf => None,
            Self::NoteESharpWhole => None,
            Self::NoteEWhole => None,
            Self::NoteEmptyBlack => None,
            Self::NoteEmptyHalf => None,
            Self::NoteEmptyWhole => None,
            Self::NoteFBlack => None,
            Self::NoteFFlatBlack => None,
            Self::NoteFFlatHalf => None,
            Self::NoteFFlatWhole => None,
            Self::NoteFHalf => None,
            Self::NoteFSharpBlack => None,
            Self::NoteFSharpHalf => None,
            Self::NoteFSharpWhole => None,
            Self::NoteFWhole => None,
            Self::NoteFaBlack => None,
            Self::NoteFaHalf => None,
            Self::NoteFaWhole => None,
            Self::NoteFiBlack => None,
            Self::NoteFiHalf => None,
            Self::NoteFiWhole => None,
            Self::NoteGBlack => None,
            Self::NoteGFlatBlack => None,
            Self::NoteGFlatHalf => None,
            Self::NoteGFlatWhole => None,
            Self::NoteGHalf => None,
            Self::NoteGSharpBlack => None,
            Self::NoteGSharpHalf => None,
            Self::NoteGSharpWhole => None,
            Self::NoteGWhole => None,
            Self::NoteHBlack => None,
            Self::NoteHHalf => None,
            Self::NoteHSharpBlack => None,
            Self::NoteHSharpHalf => None,
            Self::NoteHSharpWhole => None,
            Self::NoteHWhole => None,
            Self::NoteHalfDown => None,
            Self::NoteHalfUp => Some('𝅗𝅥'),
            Self::NoteLaBlack => None,
            Self::NoteLaHalf => None,
            Self::NoteLaWhole => None,
            Self::NoteLeBlack => None,
            Self::NoteLeHalf => None,
            Self::NoteLeWhole => None,
            Self::NoteLiBlack => None,
            Self::NoteLiHalf => None,
            Self::NoteLiWhole => None,
            Self::NoteMeBlack => None,
            Self::NoteMeHalf => None,
            Self::NoteMeWhole => None,
            Self::NoteMiBlack => None,
            Self::NoteMiHalf => None,
            Self::NoteMiWhole => None,
            Self::NoteQuarterDown => None,
            Self::NoteQuarterUp => Some('𝅘𝅥'),
            Self::NoteRaBlack => None,
            Self::NoteRaHalf => None,
            Self::NoteRaWhole => None,
            Self::NoteReBlack => None,
            Self::NoteReHalf => None,
            Self::NoteReWhole => None,
            Self::NoteRiBlack => None,
            Self::NoteRiHalf => None,
            Self::NoteRiWhole => None,
            Self::NoteSeBlack => None,
            Self::NoteSeHalf => None,
            Self::NoteSeWhole => None,
            Self::NoteShapeArrowheadLeftBlack => None,
            Self::NoteShapeArrowheadLeftDoubleWhole => None,
            Self::NoteShapeArrowheadLeftWhite => None,
            Self::NoteShapeDiamondBlack => None,
            Self::NoteShapeDiamondDoubleWhole => None,
            Self::NoteShapeDiamondWhite => None,
            Self::NoteShapeIsoscelesTriangleBlack => None,
            Self::NoteShapeIsoscelesTriangleDoubleWhole => None,
            Self::NoteShapeIsoscelesTriangleWhite => None,
            Self::NoteShapeKeystoneBlack => None,
            Self::NoteShapeKeystoneDoubleWhole => None,
            Self::NoteShapeKeystoneWhite => None,
            Self::NoteShapeMoonBlack => None,
            Self::NoteShapeMoonDoubleWhole => None,
            Self::NoteShapeMoonLeftBlack => None,
            Self::NoteShapeMoonLeftDoubleWhole => None,
            Self::NoteShapeMoonLeftWhite => None,
            Self::NoteShapeMoonWhite => None,
            Self::NoteShapeQuarterMoonBlack => None,
            Self::NoteShapeQuarterMoonDoubleWhole => None,
            Self::NoteShapeQuarterMoonWhite => None,
            Self::NoteShapeRoundBlack => None,
            Self::NoteShapeRoundDoubleWhole => None,
            Self::NoteShapeRoundWhite => None,
            Self::NoteShapeSquareBlack => None,
            Self::NoteShapeSquareDoubleWhole => None,
            Self::NoteShapeSquareWhite => None,
            Self::NoteShapeTriangleLeftBlack => None,
            Self::NoteShapeTriangleLeftDoubleWhole => None,
            Self::NoteShapeTriangleLeftWhite => None,
            Self::NoteShapeTriangleRightBlack => None,
            Self::NoteShapeTriangleRightDoubleWhole => None,
            Self::NoteShapeTriangleRightWhite => None,
            Self::NoteShapeTriangleRoundBlack => None,
            Self::NoteShapeTriangleRoundDoubleWhole => None,
            Self::NoteShapeTriangleRoundLeftBlack => None,
            Self::NoteShapeTriangleRoundLeftDoubleWhole => None,
            Self::NoteShapeTriangleRoundLeftWhite => None,
            Self::NoteShapeTriangleRoundWhite => None,
            Self::NoteShapeTriangleUpBlack => None,
            Self::NoteShapeTriangleUpDoubleWhole => None,
            Self::NoteShapeTriangleUpWhite => None,
            Self::NoteSiBlack => None,
            Self::NoteSiHalf => None,
            Self::NoteSiWhole => None,
            Self::NoteSoBlack => None,
            Self::NoteSoHalf => None,
            Self::NoteSoWhole => None,
            Self::NoteTeBlack => None,
            Self::NoteTeHalf => None,
            Self::NoteTeWhole => None,
            Self::NoteTiBlack => None,
            Self::NoteTiHalf => None,
            Self::NoteTiWhole => None,
            Self::NoteWhole => Some('𝅝'),
            Self::NoteheadBlack => Some('𝅘'),
            Self::NoteheadCircleSlash => None,
            Self::NoteheadCircleX => Some('𝅅'),
            Self::NoteheadCircleXDoubleWhole => None,
            Self::NoteheadCircleXHalf => None,
            Self::NoteheadCircleXWhole => None,
            Self::NoteheadCircledBlack => None,
            Self::NoteheadCircledBlackLarge => None,
            Self::NoteheadCircledDoubleWhole => None,
            Self::NoteheadCircledDoubleWholeLarge => None,
            Self::NoteheadCircledHalf => None,
            Self::NoteheadCircledHalfLarge => None,
            Self::NoteheadCircledWhole => None,
            Self::NoteheadCircledWholeLarge => None,
            Self::NoteheadCircledXLarge => None,
            Self::NoteheadClusterDoubleWhole2Nd => None,
            Self::NoteheadClusterDoubleWhole3Rd => None,
            Self::NoteheadClusterDoubleWholeBottom => None,
            Self::NoteheadClusterDoubleWholeMiddle => None,
            Self::NoteheadClusterDoubleWholeTop => None,
            Self::NoteheadClusterHalf2Nd => None,
            Self::NoteheadClusterHalf3Rd => None,
            Self::NoteheadClusterHalfBottom => None,
            Self::NoteheadClusterHalfMiddle => None,
            Self::NoteheadClusterHalfTop => None,
            Self::NoteheadClusterQuarter2Nd => None,
            Self::NoteheadClusterQuarter3Rd => None,
            Self::NoteheadClusterQuarterBottom => None,
            Self::NoteheadClusterQuarterMiddle => None,
            Self::NoteheadClusterQuarterTop => None,
            Self::NoteheadClusterRoundBlack => None,
            Self::NoteheadClusterRoundWhite => None,
            Self::NoteheadClusterSquareBlack => Some('𝅛'),
            Self::NoteheadClusterSquareWhite => Some('𝅚'),
            Self::NoteheadClusterWhole2Nd => None,
            Self::NoteheadClusterWhole3Rd => None,
            Self::NoteheadClusterWholeBottom => None,
            Self::NoteheadClusterWholeMiddle => None,
            Self::NoteheadClusterWholeTop => None,
            Self::NoteheadCowellEleventhNoteSeriesHalf => None,
            Self::NoteheadCowellEleventhNoteSeriesWhole => None,
            Self::NoteheadCowellEleventhSeriesBlack => None,
            Self::NoteheadCowellFifteenthNoteSeriesBlack => None,
            Self::NoteheadCowellFifteenthNoteSeriesHalf => None,
            Self::NoteheadCowellFifteenthNoteSeriesWhole => None,
            Self::NoteheadCowellFifthNoteSeriesBlack => None,
            Self::NoteheadCowellFifthNoteSeriesHalf => None,
            Self::NoteheadCowellFifthNoteSeriesWhole => None,
            Self::NoteheadCowellNinthNoteSeriesBlack => None,
            Self::NoteheadCowellNinthNoteSeriesHalf => None,
            Self::NoteheadCowellNinthNoteSeriesWhole => None,
            Self::NoteheadCowellSeventhNoteSeriesBlack => None,
            Self::NoteheadCowellSeventhNoteSeriesHalf => None,
            Self::NoteheadCowellSeventhNoteSeriesWhole => None,
            Self::NoteheadCowellThirdNoteSeriesBlack => None,
            Self::NoteheadCowellThirdNoteSeriesHalf => None,
            Self::NoteheadCowellThirdNoteSeriesWhole => None,
            Self::NoteheadCowellThirteenthNoteSeriesBlack => None,
            Self::NoteheadCowellThirteenthNoteSeriesHalf => None,
            Self::NoteheadCowellThirteenthNoteSeriesWhole => None,
            Self::NoteheadDiamondBlack => None,
            Self::NoteheadDiamondBlackOld => None,
            Self::NoteheadDiamondBlackWide => None,
            Self::NoteheadDiamondClusterBlack2Nd => None,
            Self::NoteheadDiamondClusterBlack3Rd => None,
            Self::NoteheadDiamondClusterBlackBottom => None,
            Self::NoteheadDiamondClusterBlackMiddle => None,
            Self::NoteheadDiamondClusterBlackTop => None,
            Self::NoteheadDiamondClusterWhite2Nd => None,
            Self::NoteheadDiamondClusterWhite3Rd => None,
            Self::NoteheadDiamondClusterWhiteBottom => None,
            Self::NoteheadDiamondClusterWhiteMiddle => None,
            Self::NoteheadDiamondClusterWhiteTop => None,
            Self::NoteheadDiamondDoubleWhole => None,
            Self::NoteheadDiamondDoubleWholeOld => None,
            Self::NoteheadDiamondHalf => None,
            Self::NoteheadDiamondHalfFilled => None,
            Self::NoteheadDiamondHalfOld => None,
            Self::NoteheadDiamondHalfWide => None,
            Self::NoteheadDiamondOpen => None,
            Self::NoteheadDiamondWhite => None,
            Self::NoteheadDiamondWhiteWide => None,
            Self::NoteheadDiamondWhole => None,
            Self::NoteheadDiamondWholeOld => None,
            Self::NoteheadDoubleWhole => None,
            Self::NoteheadDoubleWholeSquare => None,
            Self::NoteheadDoubleWholeWithX => None,
            Self::NoteheadHalf => Some('𝅗'),
            Self::NoteheadHalfFilled => None,
            Self::NoteheadHalfWithX => None,
            Self::NoteheadHeavyX => None,
            Self::NoteheadHeavyXHat => None,
            Self::NoteheadLargeArrowDownBlack => None,
            Self::NoteheadLargeArrowDownDoubleWhole => None,
            Self::NoteheadLargeArrowDownHalf => None,
            Self::NoteheadLargeArrowDownWhole => None,
            Self::NoteheadLargeArrowUpBlack => None,
            Self::NoteheadLargeArrowUpDoubleWhole => None,
            Self::NoteheadLargeArrowUpHalf => None,
            Self::NoteheadLargeArrowUpWhole => None,
            Self::NoteheadMoonBlack => Some('𝅓'),
            Self::NoteheadMoonWhite => Some('𝅒'),
            Self::NoteheadNancarrowSine => None,
            Self::NoteheadNull => Some('𝅙'),
            Self::NoteheadParenthesis => Some('𝅖'),
            Self::NoteheadParenthesisLeft => None,
            Self::NoteheadParenthesisRight => None,
            Self::NoteheadPlusBlack => Some('𝅄'),
            Self::NoteheadPlusDoubleWhole => None,
            Self::NoteheadPlusHalf => None,
            Self::NoteheadPlusWhole => None,
            Self::NoteheadRectangularClusterBlackBottom => None,
            Self::NoteheadRectangularClusterBlackMiddle => None,
            Self::NoteheadRectangularClusterBlackTop => None,
            Self::NoteheadRectangularClusterWhiteBottom => None,
            Self::NoteheadRectangularClusterWhiteMiddle => None,
            Self::NoteheadRectangularClusterWhiteTop => None,
            Self::NoteheadRoundBlack => None,
            Self::NoteheadRoundBlackDoubleSlashed => None,
            Self::NoteheadRoundBlackLarge => None,
            Self::NoteheadRoundBlackSlashed => None,
            Self::NoteheadRoundBlackSlashedLarge => None,
            Self::NoteheadRoundWhite => None,
            Self::NoteheadRoundWhiteDoubleSlashed => None,
            Self::NoteheadRoundWhiteLarge => None,
            Self::NoteheadRoundWhiteSlashed => None,
            Self::NoteheadRoundWhiteSlashedLarge => None,
            Self::NoteheadRoundWhiteWithDot => None,
            Self::NoteheadRoundWhiteWithDotLarge => None,
            Self::NoteheadSlashDiamondWhite => None,
            Self::NoteheadSlashHorizontalEnds => Some('𝄍'),
            Self::NoteheadSlashHorizontalEndsMuted => None,
            Self::NoteheadSlashVerticalEnds => None,
            Self::NoteheadSlashVerticalEndsMuted => None,
            Self::NoteheadSlashVerticalEndsSmall => None,
            Self::NoteheadSlashWhiteDoubleWhole => None,
            Self::NoteheadSlashWhiteHalf => None,
            Self::NoteheadSlashWhiteMuted => None,
            Self::NoteheadSlashWhiteWhole => None,
            Self::NoteheadSlashX => None,
            Self::NoteheadSlashedBlack1 => None,
            Self::NoteheadSlashedBlack2 => None,
            Self::NoteheadSlashedDoubleWhole1 => None,
            Self::NoteheadSlashedDoubleWhole2 => None,
            Self::NoteheadSlashedHalf1 => None,
            Self::NoteheadSlashedHalf2 => None,
            Self::NoteheadSlashedWhole1 => None,
            Self::NoteheadSlashedWhole2 => None,
            Self::NoteheadSquareBlack => Some('𝅇'),
            Self::NoteheadSquareBlackLarge => None,
            Self::NoteheadSquareBlackWhite => None,
            Self::NoteheadSquareWhite => Some('𝅆'),
            Self::NoteheadTriangleDownBlack => Some('𝅏'),
            Self::NoteheadTriangleDownDoubleWhole => None,
            Self::NoteheadTriangleDownHalf => None,
            Self::NoteheadTriangleDownWhite => Some('𝅎'),
            Self::NoteheadTriangleDownWhole => None,
            Self::NoteheadTriangleLeftBlack => Some('𝅋'),
            Self::NoteheadTriangleLeftWhite => Some('𝅊'),
            Self::NoteheadTriangleRightBlack => Some('𝅍'),
            Self::NoteheadTriangleRightWhite => Some('𝅌'),
            Self::NoteheadTriangleRoundDownBlack => Some('𝅕'),
            Self::NoteheadTriangleRoundDownWhite => Some('𝅔'),
            Self::NoteheadTriangleUpBlack => Some('𝅉'),
            Self::NoteheadTriangleUpDoubleWhole => None,
            Self::NoteheadTriangleUpHalf => None,
            Self::NoteheadTriangleUpRightBlack => Some('𝅑'),
            Self::NoteheadTriangleUpRightWhite => Some('𝅐'),
            Self::NoteheadTriangleUpWhite => Some('𝅈'),
            Self::NoteheadTriangleUpWhole => None,
            Self::NoteheadVoidWithX => None,
            Self::NoteheadWhole => None,
            Self::NoteheadWholeFilled => None,
            Self::NoteheadWholeWithX => None,
            Self::NoteheadXBlack => Some('𝅃'),
            Self::NoteheadXDoubleWhole => None,
            Self::NoteheadXHalf => None,
            Self::NoteheadXOrnate => None,
            Self::NoteheadXOrnateEllipse => None,
            Self::NoteheadXWhole => None,
            Self::OctaveBaselineA => None,
            Self::OctaveBaselineB => None,
            Self::OctaveBaselineM => None,
            Self::OctaveBaselineV => None,
            Self::OctaveBassa => None,
            Self::OctaveLoco => None,
            Self::OctaveParensLeft => None,
            Self::OctaveParensRight => None,
            Self::OctaveSuperscriptA => None,
            Self::OctaveSuperscriptB => None,
            Self::OctaveSuperscriptM => None,
            Self::OctaveSuperscriptV => None,
            Self::OneHandedRollStevens => None,
            Self::OrganGerman2Fusae => None,
            Self::OrganGerman2Minimae => None,
            Self::OrganGerman2OctaveUp => None,
            Self::OrganGerman2Semifusae => None,
            Self::OrganGerman2Semiminimae => None,
            Self::OrganGerman3Fusae => None,
            Self::OrganGerman3Minimae => None,
            Self::OrganGerman3Semifusae => None,
            Self::OrganGerman3Semiminimae => None,
            Self::OrganGerman4Fusae => None,
            Self::OrganGerman4Minimae => None,
            Self::OrganGerman4Semifusae => None,
            Self::OrganGerman4Semiminimae => None,
            Self::OrganGerman5Fusae => None,
            Self::OrganGerman5Minimae => None,
            Self::OrganGerman5Semifusae => None,
            Self::OrganGerman5Semiminimae => None,
            Self::OrganGerman6Fusae => None,
            Self::OrganGerman6Minimae => None,
            Self::OrganGerman6Semifusae => None,
            Self::OrganGerman6Semiminimae => None,
            Self::OrganGermanALower => None,
            Self::OrganGermanAUpper => None,
            Self::OrganGermanAugmentationDot => None,
            Self::OrganGermanBLower => None,
            Self::OrganGermanBUpper => None,
            Self::OrganGermanBuxheimerBrevis2 => None,
            Self::OrganGermanBuxheimerBrevis3 => None,
            Self::OrganGermanBuxheimerMinimaRest => None,
            Self::OrganGermanBuxheimerSemibrevis => None,
            Self::OrganGermanBuxheimerSemibrevisRest => None,
            Self::OrganGermanCLower => None,
            Self::OrganGermanCUpper => None,
            Self::OrganGermanCisLower => None,
            Self::OrganGermanCisUpper => None,
            Self::OrganGermanDLower => None,
            Self::OrganGermanDUpper => None,
            Self::OrganGermanDisLower => None,
            Self::OrganGermanDisUpper => None,
            Self::OrganGermanELower => None,
            Self::OrganGermanEUpper => None,
            Self::OrganGermanFLower => None,
            Self::OrganGermanFUpper => None,
            Self::OrganGermanFisLower => None,
            Self::OrganGermanFisUpper => None,
            Self::OrganGermanFusa => None,
            Self::OrganGermanFusaRest => None,
            Self::OrganGermanGLower => None,
            Self::OrganGermanGUpper => None,
            Self::OrganGermanGisLower => None,
            Self::OrganGermanGisUpper => None,
            Self::OrganGermanHLower => None,
            Self::OrganGermanHUpper => None,
            Self::OrganGermanMinima => None,
            Self::OrganGermanMinimaRest => None,
            Self::OrganGermanOctaveDown => None,
            Self::OrganGermanOctaveUp => None,
            Self::OrganGermanSemibrevis => None,
            Self::OrganGermanSemibrevisRest => None,
            Self::OrganGermanSemifusa => None,
            Self::OrganGermanSemifusaRest => None,
            Self::OrganGermanSemiminima => None,
            Self::OrganGermanSemiminimaRest => None,
            Self::OrganGermanTie => None,
            Self::OrnamentBottomLeftConcaveStroke => None,
            Self::OrnamentBottomLeftConcaveStrokeLarge => Some('𝆡'),
            Self::OrnamentBottomLeftConvexStroke => None,
            Self::OrnamentBottomRightConcaveStroke => Some('𝆟'),
            Self::OrnamentBottomRightConvexStroke => None,
            Self::OrnamentComma => None,
            Self::OrnamentDoubleObliqueLinesAfterNote => None,
            Self::OrnamentDoubleObliqueLinesBeforeNote => None,
            Self::OrnamentDownCurve => None,
            Self::OrnamentHaydn => None,
            Self::OrnamentHighLeftConcaveStroke => None,
            Self::OrnamentHighLeftConvexStroke => Some('𝆢'),
            Self::OrnamentHighRightConcaveStroke => None,
            Self::OrnamentHighRightConvexStroke => None,
            Self::OrnamentHookAfterNote => None,
            Self::OrnamentHookBeforeNote => None,
            Self::OrnamentLeftFacingHalfCircle => None,
            Self::OrnamentLeftFacingHook => None,
            Self::OrnamentLeftPlus => None,
            Self::OrnamentLeftShakeT => None,
            Self::OrnamentLeftVerticalStroke => Some('𝆛'),
            Self::OrnamentLeftVerticalStrokeWithCross => None,
            Self::OrnamentLowLeftConcaveStroke => None,
            Self::OrnamentLowLeftConvexStroke => Some('𝆤'),
            Self::OrnamentLowRightConcaveStroke => Some('𝆣'),
            Self::OrnamentLowRightConvexStroke => None,
            Self::OrnamentMiddleVerticalStroke => Some('𝆠'),
            Self::OrnamentMordent => None,
            Self::OrnamentObliqueLineAfterNote => None,
            Self::OrnamentObliqueLineBeforeNote => None,
            Self::OrnamentObliqueLineHorizAfterNote => None,
            Self::OrnamentObliqueLineHorizBeforeNote => None,
            Self::OrnamentOriscus => None,
            Self::OrnamentPinceCouperin => None,
            Self::OrnamentPortDeVoixV => None,
            Self::OrnamentPrecompAppoggTrill => None,
            Self::OrnamentPrecompAppoggTrillSuffix => None,
            Self::OrnamentPrecompCadence => None,
            Self::OrnamentPrecompCadenceUpperPrefix => None,
            Self::OrnamentPrecompCadenceUpperPrefixTurn => None,
            Self::OrnamentPrecompCadenceWithTurn => None,
            Self::OrnamentPrecompDescendingSlide => None,
            Self::OrnamentPrecompDoubleCadenceLowerPrefix => None,
            Self::OrnamentPrecompDoubleCadenceUpperPrefix => None,
            Self::OrnamentPrecompDoubleCadenceUpperPrefixTurn => None,
            Self::OrnamentPrecompInvertedMordentUpperPrefix => None,
            Self::OrnamentPrecompMordentRelease => None,
            Self::OrnamentPrecompMordentUpperPrefix => None,
            Self::OrnamentPrecompPortDeVoixMordent => None,
            Self::OrnamentPrecompSlide => None,
            Self::OrnamentPrecompSlideTrillBach => None,
            Self::OrnamentPrecompSlideTrillDAnglebert => None,
            Self::OrnamentPrecompSlideTrillMarpurg => None,
            Self::OrnamentPrecompSlideTrillMuffat => None,
            Self::OrnamentPrecompSlideTrillSuffixMuffat => None,
            Self::OrnamentPrecompTrillLowerSuffix => None,
            Self::OrnamentPrecompTrillSuffixDandrieu => None,
            Self::OrnamentPrecompTrillWithMordent => None,
            Self::OrnamentPrecompTurnTrillBach => None,
            Self::OrnamentPrecompTurnTrillDAnglebert => None,
            Self::OrnamentQuilisma => None,
            Self::OrnamentRightFacingHalfCircle => None,
            Self::OrnamentRightFacingHook => None,
            Self::OrnamentRightVerticalStroke => None,
            Self::OrnamentSchleifer => None,
            Self::OrnamentShake3 => None,
            Self::OrnamentShakeMuffat1 => None,
            Self::OrnamentShortObliqueLineAfterNote => None,
            Self::OrnamentShortObliqueLineBeforeNote => None,
            Self::OrnamentShortTrill => None,
            Self::OrnamentTopLeftConcaveStroke => None,
            Self::OrnamentTopLeftConvexStroke => Some('𝆥'),
            Self::OrnamentTopRightConcaveStroke => None,
            Self::OrnamentTopRightConvexStroke => Some('𝆞'),
            Self::OrnamentTremblement => None,
            Self::OrnamentTremblementCouperin => None,
            Self::OrnamentTrill => Some('𝆖'),
            Self::OrnamentTurn => Some('𝆗'),
            Self::OrnamentTurnInverted => Some('𝆘'),
            Self::OrnamentTurnSlash => Some('𝆙'),
            Self::OrnamentTurnUp => Some('𝆚'),
            Self::OrnamentTurnUpS => None,
            Self::OrnamentUpCurve => None,
            Self::OrnamentVerticalLine => None,
            Self::OrnamentZigZagLineNoRightEnd => Some('𝆜'),
            Self::OrnamentZigZagLineWithRightEnd => Some('𝆝'),
            Self::Ottava => None,
            Self::OttavaAlta => Some('𝄶'),
            Self::OttavaBassa => Some('𝄷'),
            Self::OttavaBassaBa => None,
            Self::OttavaBassaVb => None,
            Self::PendereckiTremolo => None,
            Self::PictAgogo => None,
            Self::PictAlmglocken => None,
            Self::PictAnvil => None,
            Self::PictBambooChimes => None,
            Self::PictBambooScraper => None,
            Self::PictBassDrum => None,
            Self::PictBassDrumOnSide => None,
            Self::PictBeaterBow => None,
            Self::PictBeaterBox => None,
            Self::PictBeaterBrassMalletsDown => None,
            Self::PictBeaterBrassMalletsLeft => None,
            Self::PictBeaterBrassMalletsRight => None,
            Self::PictBeaterBrassMalletsUp => None,
            Self::PictBeaterCombiningDashedCircle => None,
            Self::PictBeaterCombiningParentheses => None,
            Self::PictBeaterDoubleBassDrumDown => None,
            Self::PictBeaterDoubleBassDrumUp => None,
            Self::PictBeaterFinger => None,
            Self::PictBeaterFingernails => None,
            Self::PictBeaterFist => None,
            Self::PictBeaterGuiroScraper => None,
            Self::PictBeaterHammer => None,
            Self::PictBeaterHammerMetalDown => None,
            Self::PictBeaterHammerMetalUp => None,
            Self::PictBeaterHammerPlasticDown => None,
            Self::PictBeaterHammerPlasticUp => None,
            Self::PictBeaterHammerWoodDown => None,
            Self::PictBeaterHammerWoodUp => None,
            Self::PictBeaterHand => None,
            Self::PictBeaterHardBassDrumDown => None,
            Self::PictBeaterHardBassDrumUp => None,
            Self::PictBeaterHardGlockenspielDown => None,
            Self::PictBeaterHardGlockenspielLeft => None,
            Self::PictBeaterHardGlockenspielRight => None,
            Self::PictBeaterHardGlockenspielUp => None,
            Self::PictBeaterHardTimpaniDown => None,
            Self::PictBeaterHardTimpaniLeft => None,
            Self::PictBeaterHardTimpaniRight => None,
            Self::PictBeaterHardTimpaniUp => None,
            Self::PictBeaterHardXylophoneDown => None,
            Self::PictBeaterHardXylophoneLeft => None,
            Self::PictBeaterHardXylophoneRight => None,
            Self::PictBeaterHardXylophoneUp => None,
            Self::PictBeaterHardYarnDown => None,
            Self::PictBeaterHardYarnLeft => None,
            Self::PictBeaterHardYarnRight => None,
            Self::PictBeaterHardYarnUp => None,
            Self::PictBeaterJazzSticksDown => None,
            Self::PictBeaterJazzSticksUp => None,
            Self::PictBeaterKnittingNeedle => None,
            Self::PictBeaterMallet => None,
            Self::PictBeaterMalletDown => None,
            Self::PictBeaterMediumBassDrumDown => None,
            Self::PictBeaterMediumBassDrumUp => None,
            Self::PictBeaterMediumTimpaniDown => None,
            Self::PictBeaterMediumTimpaniLeft => None,
            Self::PictBeaterMediumTimpaniRight => None,
            Self::PictBeaterMediumTimpaniUp => None,
            Self::PictBeaterMediumXylophoneDown => None,
            Self::PictBeaterMediumXylophoneLeft => None,
            Self::PictBeaterMediumXylophoneRight => None,
            Self::PictBeaterMediumXylophoneUp => None,
            Self::PictBeaterMediumYarnDown => None,
            Self::PictBeaterMediumYarnLeft => None,
            Self::PictBeaterMediumYarnRight => None,
            Self::PictBeaterMediumYarnUp => None,
            Self::PictBeaterMetalBassDrumDown => None,
            Self::PictBeaterMetalBassDrumUp => None,
            Self::PictBeaterMetalDown => None,
            Self::PictBeaterMetalHammer => None,
            Self::PictBeaterMetalLeft => None,
            Self::PictBeaterMetalRight => None,
            Self::PictBeaterMetalUp => None,
            Self::PictBeaterSnareSticksDown => None,
            Self::PictBeaterSnareSticksUp => None,
            Self::PictBeaterSoftBassDrumDown => None,
            Self::PictBeaterSoftBassDrumUp => None,
            Self::PictBeaterSoftGlockenspielDown => None,
            Self::PictBeaterSoftGlockenspielLeft => None,
            Self::PictBeaterSoftGlockenspielRight => None,
            Self::PictBeaterSoftGlockenspielUp => None,
            Self::PictBeaterSoftTimpaniDown => None,
            Self::PictBeaterSoftTimpaniLeft => None,
            Self::PictBeaterSoftTimpaniRight => None,
            Self::PictBeaterSoftTimpaniUp => None,
            Self::PictBeaterSoftXylophone => None,
            Self::PictBeaterSoftXylophoneDown => None,
            Self::PictBeaterSoftXylophoneLeft => None,
            Self::PictBeaterSoftXylophoneRight => None,
            Self::PictBeaterSoftXylophoneUp => None,
            Self::PictBeaterSoftYarnDown => None,
            Self::PictBeaterSoftYarnLeft => None,
            Self::PictBeaterSoftYarnRight => None,
            Self::PictBeaterSoftYarnUp => None,
            Self::PictBeaterSpoonWoodenMallet => None,
            Self::PictBeaterSuperballDown => None,
            Self::PictBeaterSuperballLeft => None,
            Self::PictBeaterSuperballRight => None,
            Self::PictBeaterSuperballUp => None,
            Self::PictBeaterTriangleDown => None,
            Self::PictBeaterTrianglePlain => None,
            Self::PictBeaterTriangleUp => None,
            Self::PictBeaterWireBrushesDown => None,
            Self::PictBeaterWireBrushesUp => None,
            Self::PictBeaterWoodTimpaniDown => None,
            Self::PictBeaterWoodTimpaniLeft => None,
            Self::PictBeaterWoodTimpaniRight => None,
            Self::PictBeaterWoodTimpaniUp => None,
            Self::PictBeaterWoodXylophoneDown => None,
            Self::PictBeaterWoodXylophoneLeft => None,
            Self::PictBeaterWoodXylophoneRight => None,
            Self::PictBeaterWoodXylophoneUp => None,
            Self::PictBell => None,
            Self::PictBellOfCymbal => None,
            Self::PictBellPlate => None,
            Self::PictBellTree => None,
            Self::PictBirdWhistle => None,
            Self::PictBoardClapper => None,
            Self::PictBongos => None,
            Self::PictBrakeDrum => None,
            Self::PictCabasa => None,
            Self::PictCannon => None,
            Self::PictCarHorn => None,
            Self::PictCastanets => None,
            Self::PictCastanetsWithHandle => None,
            Self::PictCelesta => None,
            Self::PictCencerro => None,
            Self::PictCenter1 => None,
            Self::PictCenter2 => None,
            Self::PictCenter3 => None,
            Self::PictChainRattle => None,
            Self::PictChimes => None,
            Self::PictChineseCymbal => None,
            Self::PictChokeCymbal => None,
            Self::PictClaves => None,
            Self::PictCoins => None,
            Self::PictConga => None,
            Self::PictCowBell => None,
            Self::PictCrashCymbals => None,
            Self::PictCrotales => None,
            Self::PictCrushStem => None,
            Self::PictCuica => None,
            Self::PictCymbalTongs => None,
            Self::PictDamp1 => None,
            Self::PictDamp2 => None,
            Self::PictDamp3 => None,
            Self::PictDamp4 => None,
            Self::PictDeadNoteStem => None,
            Self::PictDrumStick => None,
            Self::PictDuckCall => None,
            Self::PictEdgeOfCymbal => None,
            Self::PictEmptyTrap => None,
            Self::PictFingerCymbals => None,
            Self::PictFlexatone => None,
            Self::PictFootballRatchet => None,
            Self::PictGlassHarmonica => None,
            Self::PictGlassHarp => None,
            Self::PictGlassPlateChimes => None,
            Self::PictGlassTubeChimes => None,
            Self::PictGlsp => None,
            Self::PictGlspSmithBrindle => None,
            Self::PictGobletDrum => None,
            Self::PictGong => None,
            Self::PictGongWithButton => None,
            Self::PictGuiro => None,
            Self::PictGumHardDown => None,
            Self::PictGumHardLeft => None,
            Self::PictGumHardRight => None,
            Self::PictGumHardUp => None,
            Self::PictGumMediumDown => None,
            Self::PictGumMediumLeft => None,
            Self::PictGumMediumRight => None,
            Self::PictGumMediumUp => None,
            Self::PictGumSoftDown => None,
            Self::PictGumSoftLeft => None,
            Self::PictGumSoftRight => None,
            Self::PictGumSoftUp => None,
            Self::PictHalfOpen1 => None,
            Self::PictHalfOpen2 => None,
            Self::PictHandbell => None,
            Self::PictHiHat => None,
            Self::PictHiHatOnStand => None,
            Self::PictJawHarp => None,
            Self::PictJingleBells => None,
            Self::PictKlaxonHorn => None,
            Self::PictLeftHandCircle => None,
            Self::PictLionsRoar => None,
            Self::PictLithophone => None,
            Self::PictLogDrum => None,
            Self::PictLotusFlute => None,
            Self::PictMar => None,
            Self::PictMarSmithBrindle => None,
            Self::PictMaraca => None,
            Self::PictMaracas => None,
            Self::PictMegaphone => None,
            Self::PictMetalPlateChimes => None,
            Self::PictMetalTubeChimes => None,
            Self::PictMusicalSaw => None,
            Self::PictNormalPosition => None,
            Self::PictOnRim => None,
            Self::PictOpen => None,
            Self::PictOpenRimShot => None,
            Self::PictPistolShot => None,
            Self::PictPoliceWhistle => None,
            Self::PictQuijada => None,
            Self::PictRainstick => None,
            Self::PictRatchet => None,
            Self::PictRecoReco => None,
            Self::PictRightHandSquare => None,
            Self::PictRim1 => None,
            Self::PictRim2 => None,
            Self::PictRim3 => None,
            Self::PictRimShotOnStem => None,
            Self::PictSandpaperBlocks => None,
            Self::PictScrapeAroundRim => None,
            Self::PictScrapeAroundRimClockwise => None,
            Self::PictScrapeCenterToEdge => None,
            Self::PictScrapeEdgeToCenter => None,
            Self::PictShellBells => None,
            Self::PictShellChimes => None,
            Self::PictSiren => None,
            Self::PictSistrum => None,
            Self::PictSizzleCymbal => None,
            Self::PictSleighBell => None,
            Self::PictSlideBrushOnGong => None,
            Self::PictSlideWhistle => None,
            Self::PictSlitDrum => None,
            Self::PictSnareDrum => None,
            Self::PictSnareDrumMilitary => None,
            Self::PictSnareDrumSnaresOff => None,
            Self::PictSteelDrums => None,
            Self::PictStickShot => None,
            Self::PictSuperball => None,
            Self::PictSuspendedCymbal => None,
            Self::PictSwishStem => None,
            Self::PictTabla => None,
            Self::PictTamTam => None,
            Self::PictTamTamWithBeater => None,
            Self::PictTambourine => None,
            Self::PictTempleBlocks => None,
            Self::PictTenorDrum => None,
            Self::PictThundersheet => None,
            Self::PictTimbales => None,
            Self::PictTimpani => None,
            Self::PictTomTom => None,
            Self::PictTomTomChinese => None,
            Self::PictTomTomIndoAmerican => None,
            Self::PictTomTomJapanese => None,
            Self::PictTriangle => None,
            Self::PictTubaphone => None,
            Self::PictTubularBells => None,
            Self::PictTurnLeftStem => None,
            Self::PictTurnRightLeftStem => None,
            Self::PictTurnRightStem => None,
            Self::PictVib => None,
            Self::PictVibMotorOff => None,
            Self::PictVibSmithBrindle => None,
            Self::PictVibraslap => None,
            Self::PictVietnameseHat => None,
            Self::PictWhip => None,
            Self::PictWindChimesGlass => None,
            Self::PictWindMachine => None,
            Self::PictWindWhistle => None,
            Self::PictWoodBlock => None,
            Self::PictWoundHardDown => None,
            Self::PictWoundHardLeft => None,
            Self::PictWoundHardRight => None,
            Self::PictWoundHardUp => None,
            Self::PictWoundSoftDown => None,
            Self::PictWoundSoftLeft => None,
            Self::PictWoundSoftRight => None,
            Self::PictWoundSoftUp => None,
            Self::PictXyl => None,
            Self::PictXylBass => None,
            Self::PictXylSmithBrindle => None,
            Self::PictXylTenor => None,
            Self::PictXylTenorTrough => None,
            Self::PictXylTrough => None,
            Self::PluckedBuzzPizzicato => None,
            Self::PluckedDamp => Some('𝆴'),
            Self::PluckedDampAll => Some('𝆵'),
            Self::PluckedDampOnStem => None,
            Self::PluckedFingernailFlick => None,
            Self::PluckedLeftHandPizzicato => None,
            Self::PluckedPlectrum => None,
            Self::PluckedSnapPizzicatoAbove => None,
            Self::PluckedSnapPizzicatoBelow => Some('\u{1d1ad}'),
            Self::PluckedWithFingernails => Some('𝆳'),
            Self::Quindicesima => None,
            Self::QuindicesimaAlta => None,
            Self::QuindicesimaBassa => Some('𝄹'),
            Self::QuindicesimaBassaMb => None,
            Self::Repeat1Bar => Some('𝄎'),
            Self::Repeat2Bars => Some('𝄏'),
            Self::Repeat4Bars => None,
            Self::RepeatBarLowerDot => None,
            Self::RepeatBarSlash => None,
            Self::RepeatBarUpperDot => None,
            Self::RepeatDot => None,
            Self::RepeatDots => Some('𝄈'),
            Self::RepeatLeft => Some('𝄆'),
            Self::RepeatRight => Some('𝄇'),
            Self::RepeatRightLeft => None,
            Self::Rest1024Th => None,
            Self::Rest128Th => Some('𝅂'),
            Self::Rest16Th => Some('𝄿'),
            Self::Rest256Th => None,
            Self::Rest32Nd => Some('𝅀'),
            Self::Rest512Th => None,
            Self::Rest64Th => Some('𝅁'),
            Self::Rest8Th => Some('𝄾'),
            Self::RestDoubleWhole => Some('𝄺'),
            Self::RestDoubleWholeLegerLine => None,
            Self::RestHBar => Some('𝄩'),
            Self::RestHBarLeft => None,
            Self::RestHBarMiddle => None,
            Self::RestHBarRight => None,
            Self::RestHalf => Some('𝄼'),
            Self::RestHalfLegerLine => None,
            Self::RestLonga => None,
            Self::RestMaxima => None,
            Self::RestQuarter => Some('𝄽'),
            Self::RestQuarterOld => None,
            Self::RestQuarterZ => None,
            Self::RestWhole => Some('𝄻'),
            Self::RestWholeLegerLine => None,
            Self::ReversedBrace => None,
            Self::ReversedBracketBottom => None,
            Self::ReversedBracketTop => None,
            Self::RightRepeatSmall => None,
            Self::ScaleDegree1 => None,
            Self::ScaleDegree2 => None,
            Self::ScaleDegree3 => None,
            Self::ScaleDegree4 => None,
            Self::ScaleDegree5 => None,
            Self::ScaleDegree6 => None,
            Self::ScaleDegree7 => None,
            Self::ScaleDegree8 => None,
            Self::ScaleDegree9 => None,
            Self::SchaefferClef => None,
            Self::SchaefferFClefToGClef => None,
            Self::SchaefferGClefToFClef => None,
            Self::SchaefferPreviousClef => None,
            Self::Segno => Some('𝄋'),
            Self::SegnoSerpent1 => None,
            Self::SegnoSerpent2 => None,
            Self::SemipitchedPercussionClef1 => None,
            Self::SemipitchedPercussionClef2 => None,
            Self::SmnFlat => None,
            Self::SmnFlatWhite => None,
            Self::SmnHistoryDoubleFlat => None,
            Self::SmnHistoryDoubleSharp => None,
            Self::SmnHistoryFlat => None,
            Self::SmnHistorySharp => None,
            Self::SmnNatural => None,
            Self::SmnSharp => None,
            Self::SmnSharpDown => None,
            Self::SmnSharpWhite => None,
            Self::SmnSharpWhiteDown => None,
            Self::SplitBarDivider => None,
            Self::Staff1Line => Some('𝄖'),
            Self::Staff1LineNarrow => None,
            Self::Staff1LineWide => None,
            Self::Staff2Lines => Some('𝄗'),
            Self::Staff2LinesNarrow => None,
            Self::Staff2LinesWide => None,
            Self::Staff3Lines => Some('𝄘'),
            Self::Staff3LinesNarrow => None,
            Self::Staff3LinesWide => None,
            Self::Staff4Lines => Some('𝄙'),
            Self::Staff4LinesNarrow => None,
            Self::Staff4LinesWide => None,
            Self::Staff5Lines => Some('𝄚'),
            Self::Staff5LinesNarrow => None,
            Self::Staff5LinesWide => None,
            Self::Staff6Lines => Some('𝄛'),
            Self::Staff6LinesNarrow => None,
            Self::Staff6LinesWide => None,
            Self::StaffDivideArrowDown => None,
            Self::StaffDivideArrowUp => None,
            Self::StaffDivideArrowUpDown => None,
            Self::StaffPosLower1 => None,
            Self::StaffPosLower2 => None,
            Self::StaffPosLower3 => None,
            Self::StaffPosLower4 => None,
            Self::StaffPosLower5 => None,
            Self::StaffPosLower6 => None,
            Self::StaffPosLower7 => None,
            Self::StaffPosLower8 => None,
            Self::StaffPosRaise1 => None,
            Self::StaffPosRaise2 => None,
            Self::StaffPosRaise3 => None,
            Self::StaffPosRaise4 => None,
            Self::StaffPosRaise5 => None,
            Self::StaffPosRaise6 => None,
            Self::StaffPosRaise7 => None,
            Self::StaffPosRaise8 => None,
            Self::Stem => Some('\u{1d165}'),
            Self::StemBowOnBridge => None,
            Self::StemBowOnTailpiece => None,
            Self::StemBuzzRoll => None,
            Self::StemDamp => None,
            Self::StemHarpStringNoise => None,
            Self::StemMultiphonicsBlack => None,
            Self::StemMultiphonicsBlackWhite => None,
            Self::StemMultiphonicsWhite => None,
            Self::StemPendereckiTremolo => None,
            Self::StemRimShot => None,
            Self::StemSprechgesang => Some('𝅦'),
            Self::StemSulPonticello => None,
            Self::StemSussurando => None,
            Self::StemSwished => None,
            Self::StemVibratoPulse => None,
            Self::StockhausenTremolo => None,
            Self::StringsBowBehindBridge => None,
            Self::StringsBowBehindBridgeFourStrings => None,
            Self::StringsBowBehindBridgeOneString => None,
            Self::StringsBowBehindBridgeThreeStrings => None,
            Self::StringsBowBehindBridgeTwoStrings => None,
            Self::StringsBowOnBridge => None,
            Self::StringsBowOnTailpiece => None,
            Self::StringsChangeBowDirection => None,
            Self::StringsDownBow => Some('\u{1d1aa}'),
            Self::StringsDownBowAwayFromBody => None,
            Self::StringsDownBowBeyondBridge => None,
            Self::StringsDownBowTowardsBody => None,
            Self::StringsDownBowTurned => None,
            Self::StringsFouette => None,
            Self::StringsHalfHarmonic => None,
            Self::StringsHarmonic => Some('\u{1d1ac}'),
            Self::StringsJeteAbove => None,
            Self::StringsJeteBelow => None,
            Self::StringsMuteOff => None,
            Self::StringsMuteOn => None,
            Self::StringsOverpressureDownBow => None,
            Self::StringsOverpressureNoDirection => None,
            Self::StringsOverpressurePossibileDownBow => None,
            Self::StringsOverpressurePossibileUpBow => None,
            Self::StringsOverpressureUpBow => None,
            Self::StringsScrapeCircularClockwise => None,
            Self::StringsScrapeCircularCounterclockwise => None,
            Self::StringsScrapeParallelInward => None,
            Self::StringsScrapeParallelOutward => None,
            Self::StringsThumbPosition => None,
            Self::StringsThumbPositionTurned => None,
            Self::StringsTripleChopInward => None,
            Self::StringsTripleChopOutward => None,
            Self::StringsUpBow => Some('\u{1d1ab}'),
            Self::StringsUpBowAwayFromBody => None,
            Self::StringsUpBowBeyondBridge => None,
            Self::StringsUpBowTowardsBody => None,
            Self::StringsUpBowTurned => None,
            Self::StringsVibratoPulse => None,
            Self::SwissRudimentsNoteheadBlackDouble => None,
            Self::SwissRudimentsNoteheadBlackFlam => None,
            Self::SwissRudimentsNoteheadHalfDouble => None,
            Self::SwissRudimentsNoteheadHalfFlam => None,
            Self::SystemDivider => None,
            Self::SystemDividerExtraLong => None,
            Self::SystemDividerLong => None,
            Self::TextAugmentationDot => None,
            Self::TextBlackNoteFrac16ThLongStem => None,
            Self::TextBlackNoteFrac16ThShortStem => None,
            Self::TextBlackNoteFrac32NdLongStem => None,
            Self::TextBlackNoteFrac8ThLongStem => None,
            Self::TextBlackNoteFrac8ThShortStem => None,
            Self::TextBlackNoteLongStem => None,
            Self::TextBlackNoteShortStem => None,
            Self::TextCont16ThBeamLongStem => None,
            Self::TextCont16ThBeamShortStem => None,
            Self::TextCont32NdBeamLongStem => None,
            Self::TextCont8ThBeamLongStem => None,
            Self::TextCont8ThBeamShortStem => None,
            Self::TextHeadlessBlackNoteFrac16ThLongStem => None,
            Self::TextHeadlessBlackNoteFrac16ThShortStem => None,
            Self::TextHeadlessBlackNoteFrac32NdLongStem => None,
            Self::TextHeadlessBlackNoteFrac8ThLongStem => None,
            Self::TextHeadlessBlackNoteFrac8ThShortStem => None,
            Self::TextHeadlessBlackNoteLongStem => None,
            Self::TextHeadlessBlackNoteShortStem => None,
            Self::TextTie => None,
            Self::TextTuplet3LongStem => None,
            Self::TextTuplet3ShortStem => None,
            Self::TextTupletBracketEndLongStem => None,
            Self::TextTupletBracketEndShortStem => None,
            Self::TextTupletBracketStartLongStem => None,
            Self::TextTupletBracketStartShortStem => None,
            Self::TimeSig0 => None,
            Self::TimeSig0Reversed => None,
            Self::TimeSig0Turned => None,
            Self::TimeSig1 => None,
            Self::TimeSig1Reversed => None,
            Self::TimeSig1Turned => None,
            Self::TimeSig2 => None,
            Self::TimeSig2Reversed => None,
            Self::TimeSig2Turned => None,
            Self::TimeSig3 => None,
            Self::TimeSig3Reversed => None,
            Self::TimeSig3Turned => None,
            Self::TimeSig4 => None,
            Self::TimeSig4Reversed => None,
            Self::TimeSig4Turned => None,
            Self::TimeSig5 => None,
            Self::TimeSig5Reversed => None,
            Self::TimeSig5Turned => None,
            Self::TimeSig6 => None,
            Self::TimeSig6Reversed => None,
            Self::TimeSig6Turned => None,
            Self::TimeSig7 => None,
            Self::TimeSig7Reversed => None,
            Self::TimeSig7Turned => None,
            Self::TimeSig8 => None,
            Self::TimeSig8Reversed => None,
            Self::TimeSig8Turned => None,
            Self::TimeSig9 => None,
            Self::TimeSig9Reversed => None,
            Self::TimeSig9Turned => None,
            Self::TimeSigBracketLeft => None,
            Self::TimeSigBracketLeftSmall => None,
            Self::TimeSigBracketRight => None,
            Self::TimeSigBracketRightSmall => None,
            Self::TimeSigCombDenominator => None,
            Self::TimeSigCombNumerator => None,
            Self::TimeSigComma => None,
            Self::TimeSigCommon => Some('𝄴'),
            Self::TimeSigCommonReversed => None,
            Self::TimeSigCommonTurned => None,
            Self::TimeSigCut2 => None,
            Self::TimeSigCut3 => None,
            Self::TimeSigCutCommon => Some('𝄵'),
            Self::TimeSigCutCommonReversed => None,
            Self::TimeSigCutCommonTurned => None,
            Self::TimeSigEquals => None,
            Self::TimeSigFractionHalf => None,
            Self::TimeSigFractionOneThird => None,
            Self::TimeSigFractionQuarter => None,
            Self::TimeSigFractionThreeQuarters => None,
            Self::TimeSigFractionTwoThirds => None,
            Self::TimeSigFractionalSlash => None,
            Self::TimeSigMinus => None,
            Self::TimeSigMultiply => None,
            Self::TimeSigOpenPenderecki => None,
            Self::TimeSigParensLeft => None,
            Self::TimeSigParensLeftSmall => None,
            Self::TimeSigParensRight => None,
            Self::TimeSigParensRightSmall => None,
            Self::TimeSigPlus => None,
            Self::TimeSigPlusSmall => None,
            Self::TimeSigSlash => None,
            Self::TimeSigX => None,
            Self::Tremolo1 => Some('\u{1d167}'),
            Self::Tremolo2 => Some('\u{1d168}'),
            Self::Tremolo3 => Some('\u{1d169}'),
            Self::Tremolo4 => None,
            Self::Tremolo5 => None,
            Self::TremoloDivisiDots2 => None,
            Self::TremoloDivisiDots3 => None,
            Self::TremoloDivisiDots4 => None,
            Self::TremoloDivisiDots6 => None,
            Self::TremoloFingered1 => Some('𝅪'),
            Self::TremoloFingered2 => Some('𝅫'),
            Self::TremoloFingered3 => Some('𝅬'),
            Self::TremoloFingered4 => None,
            Self::TremoloFingered5 => None,
            Self::TripleTongueAbove => Some('\u{1d18b}'),
            Self::TripleTongueBelow => None,
            Self::Tuplet0 => None,
            Self::Tuplet1 => None,
            Self::Tuplet2 => None,
            Self::Tuplet3 => None,
            Self::Tuplet4 => None,
            Self::Tuplet5 => None,
            Self::Tuplet6 => None,
            Self::Tuplet7 => None,
            Self::Tuplet8 => None,
            Self::Tuplet9 => None,
            Self::TupletColon => None,
            Self::UnmeasuredTremolo => None,
            Self::UnmeasuredTremoloSimple => None,
            Self::UnpitchedPercussionClef1 => Some('𝄥'),
            Self::UnpitchedPercussionClef2 => Some('𝄦'),
            Self::Ventiduesima => None,
            Self::VentiduesimaAlta => None,
            Self::VentiduesimaBassa => None,
            Self::VentiduesimaBassaMb => None,
            Self::VocalFingerClickStockhausen => None,
            Self::VocalHalbGesungen => None,
            Self::VocalMouthClosed => None,
            Self::VocalMouthOpen => None,
            Self::VocalMouthPursed => None,
            Self::VocalMouthSlightlyOpen => None,
            Self::VocalMouthWideOpen => None,
            Self::VocalNasalVoice => None,
            Self::VocalSprechgesang => None,
            Self::VocalTongueClickStockhausen => None,
            Self::VocalTongueFingerClickStockhausen => None,
            Self::VocalsSussurando => None,
            Self::WiggleArpeggiatoDown => None,
            Self::WiggleArpeggiatoDownArrow => None,
            Self::WiggleArpeggiatoDownSwash => None,
            Self::WiggleArpeggiatoUp => None,
            Self::WiggleArpeggiatoUpArrow => None,
            Self::WiggleArpeggiatoUpSwash => None,
            Self::WiggleCircular => None,
            Self::WiggleCircularConstant => None,
            Self::WiggleCircularConstantFlipped => None,
            Self::WiggleCircularConstantFlippedLarge => None,
            Self::WiggleCircularConstantLarge => None,
            Self::WiggleCircularEnd => None,
            Self::WiggleCircularLarge => None,
            Self::WiggleCircularLarger => None,
            Self::WiggleCircularLargerStill => None,
            Self::WiggleCircularLargest => None,
            Self::WiggleCircularSmall => None,
            Self::WiggleCircularStart => None,
            Self::WiggleGlissando => None,
            Self::WiggleGlissandoGroup1 => None,
            Self::WiggleGlissandoGroup2 => None,
            Self::WiggleGlissandoGroup3 => None,
            Self::WiggleRandom1 => None,
            Self::WiggleRandom2 => None,
            Self::WiggleRandom3 => None,
            Self::WiggleRandom4 => None,
            Self::WiggleSawtooth => None,
            Self::WiggleSawtoothNarrow => None,
            Self::WiggleSawtoothWide => None,
            Self::WiggleSquareWave => None,
            Self::WiggleSquareWaveNarrow => None,
            Self::WiggleSquareWaveWide => None,
            Self::WiggleTrill => None,
            Self::WiggleTrillFast => None,
            Self::WiggleTrillFaster => None,
            Self::WiggleTrillFasterStill => None,
            Self::WiggleTrillFastest => None,
            Self::WiggleTrillSlow => None,
            Self::WiggleTrillSlower => None,
            Self::WiggleTrillSlowerStill => None,
            Self::WiggleTrillSlowest => None,
            Self::WiggleVIbratoLargestSlower => None,
            Self::WiggleVIbratoMediumSlower => None,
            Self::WiggleVibrato => None,
            Self::WiggleVibratoLargeFast => None,
            Self::WiggleVibratoLargeFaster => None,
            Self::WiggleVibratoLargeFasterStill => None,
            Self::WiggleVibratoLargeFastest => None,
            Self::WiggleVibratoLargeSlow => None,
            Self::WiggleVibratoLargeSlower => None,
            Self::WiggleVibratoLargeSlowest => None,
            Self::WiggleVibratoLargestFast => None,
            Self::WiggleVibratoLargestFaster => None,
            Self::WiggleVibratoLargestFasterStill => None,
            Self::WiggleVibratoLargestFastest => None,
            Self::WiggleVibratoLargestSlow => None,
            Self::WiggleVibratoLargestSlowest => None,
            Self::WiggleVibratoMediumFast => None,
            Self::WiggleVibratoMediumFaster => None,
            Self::WiggleVibratoMediumFasterStill => None,
            Self::WiggleVibratoMediumFastest => None,
            Self::WiggleVibratoMediumSlow => None,
            Self::WiggleVibratoMediumSlowest => None,
            Self::WiggleVibratoSmallFast => None,
            Self::WiggleVibratoSmallFaster => None,
            Self::WiggleVibratoSmallFasterStill => None,
            Self::WiggleVibratoSmallFastest => None,
            Self::WiggleVibratoSmallSlow => None,
            Self::WiggleVibratoSmallSlower => None,
            Self::WiggleVibratoSmallSlowest => None,
            Self::WiggleVibratoSmallestFast => None,
            Self::WiggleVibratoSmallestFaster => None,
            Self::WiggleVibratoSmallestFasterStill => None,
            Self::WiggleVibratoSmallestFastest => None,
            Self::WiggleVibratoSmallestSlow => None,
            Self::WiggleVibratoSmallestSlower => None,
            Self::WiggleVibratoSmallestSlowest => None,
            Self::WiggleVibratoStart => None,
            Self::WiggleVibratoWide => None,
            Self::WiggleWavy => None,
            Self::WiggleWavyNarrow => None,
            Self::WiggleWavyWide => None,
            Self::WindClosedHole => None,
            Self::WindFlatEmbouchure => None,
            Self::WindHalfClosedHole1 => None,
            Self::WindHalfClosedHole2 => None,
            Self::WindHalfClosedHole3 => None,
            Self::WindLessRelaxedEmbouchure => None,
            Self::WindLessTightEmbouchure => None,
            Self::WindMouthpiecePop => None,
            Self::WindMultiphonicsBlackStem => None,
            Self::WindMultiphonicsBlackWhiteStem => None,
            Self::WindMultiphonicsWhiteStem => None,
            Self::WindOpenHole => None,
            Self::WindReedPositionIn => None,
            Self::WindReedPositionNormal => None,
            Self::WindReedPositionOut => None,
            Self::WindRelaxedEmbouchure => None,
            Self::WindRimOnly => None,
            Self::WindSharpEmbouchure => None,
            Self::WindStrongAirPressure => None,
            Self::WindThreeQuartersClosedHole => None,
            Self::WindTightEmbouchure => None,
            Self::WindTrillKey => None,
            Self::WindVeryTightEmbouchure => None,
            Self::WindWeakAirPressure => None,
            Self::_4StringTabClef => None,
            Self::_6StringTabClef => None,
        }
    }
}
