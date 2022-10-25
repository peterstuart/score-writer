use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(transparent)]
pub struct StaffSpaces(f64);
