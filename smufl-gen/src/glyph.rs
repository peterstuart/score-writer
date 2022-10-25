use std::{collections::HashMap, fmt::Display, io::Read};

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Glyph {
    pub codepoint: Codepoint,
    pub alternate_codepoint: Option<Codepoint>,
    pub description: String,
}

impl Glyph {
    pub fn from_reader(reader: impl Read) -> anyhow::Result<HashMap<String, Glyph>> {
        Ok(serde_json::from_reader(reader)?)
    }
}

#[derive(Debug)]
pub struct Codepoint(char);

impl Display for Codepoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{:?}", self.0).fmt(f)
    }
}

impl<'de> Deserialize<'de> for Codepoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Codepoints are serialized as "U+E06", etc.
        let string: String = Deserialize::deserialize(deserializer)?;
        let hex = &string[2..];
        let value = u32::from_str_radix(hex, 16).unwrap();
        let char = char::from_u32(value).unwrap();

        Ok(Self(char))
    }
}
