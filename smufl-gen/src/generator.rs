use std::collections::HashMap;

use codegen::{Impl, Scope};
use convert_case::{Case, Casing};

use crate::glyph::Glyph;

const GLYPH_ENUM_NAME: &str = "Glyph";
const CODEPOINT_FN_NAME: &str = "codepoint";
const ALTERNATE_CODEPOINT_FN_NAME: &str = "alternate_codepoint";

pub fn generate(glyphs: HashMap<String, Glyph>) -> String {
    let mut glyphs = glyphs
        .into_iter()
        .map(|(name, glyph)| (variant_name(&name), name, glyph))
        .collect::<Vec<_>>();
    glyphs.sort_by_key(|(name, _, _)| name.to_owned());

    let mut scope = Scope::new();

    scope.import("serde", "Deserialize");

    add_glyph_enum(&glyphs, &mut scope);
    add_glyph_impl(&glyphs, &mut scope);

    scope.to_string()
}

fn variant_name(name: &str) -> String {
    let name = name.to_case(Case::Pascal);
    let needs_underscore = name.chars().next().map_or(false, |c| !c.is_alphabetic());

    if needs_underscore {
        format!("_{name}")
    } else {
        name
    }
}

fn add_glyph_enum(glyphs: &[(String, String, Glyph)], scope: &mut Scope) {
    let glyph_enum = scope
        .new_enum(GLYPH_ENUM_NAME)
        .vis("pub")
        .doc("SMuFL glyphs")
        .derive("Clone")
        .derive("Copy")
        .derive("Debug")
        .derive("Deserialize")
        .derive("Eq")
        .derive("Hash")
        .derive("PartialEq");

    for (name, original_name, glyph) in glyphs {
        glyph_enum
            .new_variant(name)
            .annotation(format!(
                "/// {}",
                // The descriptions include square brackets which the Rust documentation generator
                // treats as Markdown, so escape those.
                glyph.description.replace('[', "\\[").replace(']', "\\]")
            ))
            .annotation(format!(r#"#[serde(rename = "{original_name}")]"#));
    }
}

fn add_glyph_impl(glyphs: &[(String, String, Glyph)], scope: &mut Scope) {
    let glyph_impl = scope.new_impl(GLYPH_ENUM_NAME);

    add_codepoint_fn(glyphs, glyph_impl);
    add_alternate_codepoint_fn(glyphs, glyph_impl);
}

fn add_codepoint_fn(glyphs: &[(String, String, Glyph)], glyph_impl: &mut Impl) {
    let codepoint_fn = glyph_impl
        .new_fn(CODEPOINT_FN_NAME)
        .vis("pub")
        .arg_ref_self()
        .ret("char")
        .doc("SMuFL code point")
        .line("match self {");

    for (name, _, glyph) in glyphs {
        codepoint_fn.line(format!("Self::{name} => {},", glyph.codepoint));
    }

    codepoint_fn.line("}");
}

fn add_alternate_codepoint_fn(glyphs: &[(String, String, Glyph)], glyph_impl: &mut Impl) {
    let codepoint_fn = glyph_impl
        .new_fn(ALTERNATE_CODEPOINT_FN_NAME)
        .vis("pub")
        .arg_ref_self()
        .ret("Option<char>")
        .doc("Unicode Musical Symbols range code point")
        .line("match self {");

    for (name, _, glyph) in glyphs {
        let value = match &glyph.alternate_codepoint {
            Some(codepoint) => format!("Some({codepoint})"),
            None => "None".to_owned(),
        };

        codepoint_fn.line(format!("Self::{name} => {value},"));
    }

    codepoint_fn.line("}");
}
