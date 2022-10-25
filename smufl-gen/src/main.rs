mod generator;
mod glyph;

use std::{fs::File, io::BufReader};

use anyhow::Result;
use clap::Parser;

use crate::{generator::generate, glyph::Glyph};

#[derive(clap::Parser)]
struct Args {
    /// Path to the glyphnames.json metadata file from the smufl repo
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let glyphs = Glyph::from_reader(reader)?;
    let code = generate(glyphs);

    println!("{code}");

    Ok(())
}
