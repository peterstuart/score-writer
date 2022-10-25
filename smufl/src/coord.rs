use serde::Deserialize;

use crate::StaffSpaces;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Coord(StaffSpaces, StaffSpaces);
